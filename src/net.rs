//! Roblox Games API client.
//!
//! Kept deliberately thin: it turns a place id into a `Vec<Server>` and maps
//! every failure mode onto [`Error`]. All ranking and filtering lives in
//! [`crate::domain`], which is why this module contains no filtering at all.

use std::fmt;
use std::time::Duration;

use serde::Deserialize;

use crate::domain::{ScanDepth, Server};

/// Base URL of the public games API.
const GAMES_API: &str = "https://games.roblox.com/v1/games";

/// Anything slower than this is almost certainly a dead connection.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

/// How much of an error body to keep for display.
const MAX_ERROR_BODY: usize = 300;

/// Why a server lookup failed.
#[derive(Debug)]
pub enum Error {
    /// The request never completed, or the connection broke mid-flight.
    Transport(String),
    /// Roblox answered, but not with a 2xx status.
    Api { status: u16, body: String },
    /// The response was not the JSON we expected.
    Decode(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(detail) => write!(f, "{detail}"),
            Self::Api { status, body } if body.is_empty() => {
                write!(f, "Roblox returned HTTP {status}")
            }
            Self::Api { status, body } => write!(f, "Roblox returned HTTP {status}: {body}"),
            Self::Decode(detail) => write!(f, "Unexpected response: {detail}"),
        }
    }
}

impl std::error::Error for Error {}

/// One page of the `.../servers/Public` endpoint.
#[derive(Debug, Deserialize)]
struct ServersPage {
    #[serde(default)]
    data: Vec<RawServer>,
    #[serde(default)]
    next_page_cursor: Option<String>,
}

/// The wire shape of a server. Roblox uses camelCase and omits some fields.
#[derive(Debug, Deserialize)]
struct RawServer {
    #[serde(default)]
    id: String,
    #[serde(default)]
    playing: u32,
    #[serde(rename = "maxPlayers", default)]
    max_players: u32,
    #[serde(default)]
    ping: Option<u32>,
}

impl From<RawServer> for Server {
    fn from(raw: RawServer) -> Self {
        Self {
            id: raw.id,
            playing: raw.playing,
            max_players: raw.max_players,
            ping: raw.ping,
        }
    }
}

/// A blocking HTTP client for the Roblox games API.
///
/// Blocking is fine here because every call is made from a worker thread; the
/// GUI thread never waits on the network.
pub struct Client {
    inner: reqwest::blocking::Client,
}

impl Client {
    /// Build a client with a timeout and an identifying user agent.
    ///
    /// # Errors
    /// Returns [`Error::Transport`] if the underlying HTTP client cannot start.
    pub fn new() -> Result<Self, Error> {
        let inner = reqwest::blocking::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .map_err(|err| Error::Transport(err.to_string()))?;

        Ok(Self { inner })
    }

    /// Fetch up to `depth.requested` public servers for `place_id`.
    ///
    /// Roblox returns at most [`ScanDepth::PAGE_SIZE`] servers per request, so
    /// deeper scans follow `nextPageCursor` until enough servers have been
    /// collected or the API runs out of pages.
    ///
    /// # Errors
    /// Returns [`Error::Transport`], [`Error::Api`] or [`Error::Decode`]
    /// depending on where the lookup failed.
    pub fn public_servers(&self, place_id: u64, depth: ScanDepth) -> Result<Vec<Server>, Error> {
        let url = format!("{GAMES_API}/{place_id}/servers/Public");
        let mut servers: Vec<Server> = Vec::new();
        let mut cursor: Option<String> = None;

        while depth.wants_more(servers.len()) {
            let page_size = depth.next_page_size(servers.len());

            let mut query = vec![
                ("sortOrder", "Asc".to_owned()),
                ("limit", page_size.to_string()),
            ];
            if let Some(cursor) = cursor.take() {
                query.push(("cursor", cursor));
            }

            let response = self
                .inner
                .get(&url)
                .query(&query)
                .send()
                .map_err(|err| Error::Transport(err.to_string()))?;

            let status = response.status();
            let body = response
                .text()
                .map_err(|err| Error::Transport(err.to_string()))?;

            if !status.is_success() {
                return Err(Error::Api {
                    status: status.as_u16(),
                    body: truncate(&body),
                });
            }

            let page: ServersPage =
                serde_json::from_str(&body).map_err(|err| Error::Decode(err.to_string()))?;

            // An empty page means there is nothing left to page through.
            let page_was_empty = page.data.is_empty();
            servers.extend(page.data.into_iter().map(Server::from));

            match page.next_page_cursor {
                Some(next) if !page_was_empty && !next.is_empty() => cursor = Some(next),
                _ => break,
            }
        }

        servers.truncate(depth.requested);
        Ok(servers)
    }
}

/// Shorten an API error body so it stays readable in the status bar.
fn truncate(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.chars().count() <= MAX_ERROR_BODY {
        return trimmed.to_owned();
    }
    let shortened: String = trimmed.chars().take(MAX_ERROR_BODY).collect();
    format!("{shortened}…")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_bodies_are_truncated() {
        let long = "x".repeat(500);
        let shown = truncate(&long);
        assert_eq!(shown.chars().count(), MAX_ERROR_BODY + 1); // + the ellipsis
        assert!(shown.ends_with('…'));
    }

    #[test]
    fn short_error_bodies_survive_intact() {
        assert_eq!(truncate("  not found  "), "not found");
    }

    #[test]
    fn wire_shape_is_mapped_to_the_domain_type() {
        let raw: RawServer = serde_json::from_str(
            r#"{"id":"abc","playing":7,"maxPlayers":50,"ping":42}"#,
        )
        .expect("valid payload");

        let server = Server::from(raw);
        assert_eq!(server.id, "abc");
        assert_eq!(server.playing, 7);
        assert_eq!(server.max_players, 50);
        assert_eq!(server.ping, Some(42));
    }

    #[test]
    fn missing_fields_are_tolerated() {
        let raw: RawServer = serde_json::from_str(r#"{"id":"abc"}"#).expect("valid payload");
        let server = Server::from(raw);
        assert_eq!(server.playing, 0);
        assert_eq!(server.max_players, 0);
        assert_eq!(server.ping, None);
    }
}
