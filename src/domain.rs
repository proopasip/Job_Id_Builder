//! Pure domain logic: the parts of the app that need neither a network nor a GUI.
//!
//! Every type and function in here is free of third-party crates and of I/O, so
//! the behaviour that actually matters — parsing what the user pasted, filtering
//! and ranking servers, building the join link — can be compiled and tested on
//! its own:
//!
//! ```sh
//! rustc --edition 2021 --test src/domain.rs -o /tmp/core_test && /tmp/core_test
//! ```

#![allow(clippy::doc_markdown)]

use std::fmt;

/// One public game server, as reported by the Roblox Games API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Server {
    /// The server's job id (a UUID).
    pub id: String,
    /// Players currently in the server.
    pub playing: u32,
    /// Capacity of the server.
    pub max_players: u32,
    /// Round-trip latency in milliseconds. Roblox omits this for some servers,
    /// which is why it is optional rather than a large sentinel value.
    pub ping: Option<u32>,
}

impl Server {
    /// `playing / max_players`, e.g. `"12/50"`.
    #[must_use]
    pub fn occupancy(&self) -> String {
        format!("{}/{}", self.playing, self.max_players)
    }

    /// The ping for display, or `"—"` when Roblox didn't report one.
    #[must_use]
    pub fn ping_label(&self) -> String {
        match self.ping {
            Some(ping) => ping.to_string(),
            None => "—".to_owned(),
        }
    }
}

/// What to keep out of the server list.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Filters {
    /// Drop servers slower than this. `None` means "don't filter on ping".
    pub max_ping: Option<u32>,
    /// Drop servers with fewer players than this.
    pub min_players: u32,
}

impl Filters {
    /// Whether `server` survives these filters.
    #[must_use]
    pub fn matches(self, server: &Server) -> bool {
        if server.playing < self.min_players {
            return false;
        }
        match self.max_ping {
            // A server with no reported ping can't be proven fast, so it is
            // dropped as soon as the user asks for a ping limit.
            Some(max) => server.ping.is_some_and(|ping| ping <= max),
            None => true,
        }
    }
}

/// Keep the servers matching `filters`, ordered by ping, fastest first.
///
/// Servers with no reported ping sort last. Ties are broken by job id so the
/// table order is stable between runs instead of flickering on every refresh.
#[must_use]
pub fn apply_filters(servers: &[Server], filters: Filters) -> Vec<Server> {
    let mut kept: Vec<Server> = servers
        .iter()
        .filter(|server| filters.matches(server))
        .cloned()
        .collect();

    kept.sort_by(|a, b| {
        a.ping
            .unwrap_or(u32::MAX)
            .cmp(&b.ping.unwrap_or(u32::MAX))
            .then_with(|| a.id.cmp(&b.id))
    });

    kept
}

/// Pull a Roblox place id out of whatever the user pasted.
///
/// Understands a bare id (`1818`), a game URL
/// (`https://www.roblox.com/games/1818/Crossroads`), a `placeId=` query
/// parameter, and — as a last resort — the first number anywhere in the string.
///
/// Returns `None` for empty input and for input containing no positive number.
#[must_use]
pub fn extract_place_id(input: &str) -> Option<u64> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    query_param(input, "placeid")
        .or_else(|| games_path(input))
        .or_else(|| first_number(input))
}

/// Read `?key=<digits>` out of a URL, case-insensitively.
fn query_param(input: &str, key: &str) -> Option<u64> {
    let needle = format!("{}=", key.to_ascii_lowercase());

    for part in input.split(['?', '&', '#', ';']) {
        let lowercased = part.to_ascii_lowercase();
        if let Some(value) = lowercased.strip_prefix(&needle) {
            if let Some(id) = parse_positive(value.trim()) {
                return Some(id);
            }
        }
    }
    None
}

/// Read the id out of a `.../games/<digits>/...` path.
fn games_path(input: &str) -> Option<u64> {
    const NEEDLE: &str = "/games/";

    let start = input.find(NEEDLE)? + NEEDLE.len();
    let rest = &input[start..];
    let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());

    parse_positive(&rest[..end])
}

/// First run of digits in the string, if it is a positive number.
fn first_number(input: &str) -> Option<u64> {
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            if let Some(id) = parse_positive(&input[start..i]) {
                return Some(id);
            }
        }
        i += 1;
    }
    None
}

/// Parse a run of ASCII digits into a place id. Zero and overflows are rejected.
fn parse_positive(text: &str) -> Option<u64> {
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    text.parse::<u64>().ok().filter(|&id| id > 0)
}

/// The deep link that makes the Roblox client join one specific server.
///
/// This is the `roblox://experiences/start` scheme. The Electron build used the
/// older `https://www.roblox.com/games/start?...&launchData=TARGETGAMEID/<job>`
/// form, which Roblox superseded with the `gameInstanceId` parameter.
#[must_use]
pub fn join_link(place_id: u64, job_id: &str) -> String {
    format!("roblox://experiences/start?placeId={place_id}&gameInstanceId={job_id}")
}

/// A plain https link to the game, for pasting into a browser.
#[must_use]
pub fn web_link(place_id: u64) -> String {
    format!("https://www.roblox.com/games/{place_id}")
}

/// How many servers to ask the API for, per request and in total.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScanDepth {
    /// What the user picked.
    pub requested: usize,
}

impl ScanDepth {
    /// Roblox returns at most 100 servers per page.
    pub const PAGE_SIZE: usize = 100;

    /// Number of servers to request in the next API call.
    #[must_use]
    pub fn next_page_size(self, already_have: usize) -> usize {
        self.requested.saturating_sub(already_have).min(Self::PAGE_SIZE)
    }

    /// Whether another page is worth fetching.
    #[must_use]
    pub fn wants_more(self, already_have: usize) -> bool {
        already_have < self.requested
    }
}

impl fmt::Display for ScanDepth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.requested)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn server(id: &str, playing: u32, max_players: u32, ping: Option<u32>) -> Server {
        Server {
            id: id.to_owned(),
            playing,
            max_players,
            ping,
        }
    }

    // ---------------------------------------------------------------- placeId

    #[test]
    fn reads_a_bare_place_id() {
        assert_eq!(extract_place_id("1818"), Some(1818));
        assert_eq!(extract_place_id("  920587237  "), Some(920_587_237));
    }

    #[test]
    fn short_classic_place_ids_are_accepted() {
        // The Electron build only matched `\d{7,}`, so Crossroads (1818) and
        // every other classic place silently failed to parse.
        assert_eq!(extract_place_id("1818"), Some(1818));
        assert_eq!(
            extract_place_id("https://www.roblox.com/games/1818/Crossroads"),
            Some(1818)
        );
    }

    #[test]
    fn reads_a_game_url() {
        assert_eq!(
            extract_place_id("https://www.roblox.com/games/920587237/Adopt-Me"),
            Some(920_587_237)
        );
        assert_eq!(
            extract_place_id("www.roblox.com/games/12345/some-game?anything=1"),
            Some(12345)
        );
    }

    #[test]
    fn prefers_an_explicit_place_id_parameter() {
        assert_eq!(
            extract_place_id("roblox://experiences/start?placeId=999&gameInstanceId=abc"),
            Some(999)
        );
        assert_eq!(
            extract_place_id("https://www.roblox.com/games/1818/x?PLACEID=42"),
            Some(42)
        );
    }

    #[test]
    fn falls_back_to_the_first_number() {
        assert_eq!(extract_place_id("Brookhaven 4924924662"), Some(4_924_924_662));
    }

    #[test]
    fn rejects_input_without_a_place_id() {
        assert_eq!(extract_place_id(""), None);
        assert_eq!(extract_place_id("   "), None);
        assert_eq!(extract_place_id("https://www.roblox.com/discover"), None);
        assert_eq!(extract_place_id("0"), None);
    }

    // ----------------------------------------------------------------- filters

    #[test]
    fn keeps_only_servers_matching_both_filters() {
        let servers = vec![
            server("a", 10, 50, Some(80)),
            server("b", 2, 50, Some(40)),
            server("c", 30, 50, Some(400)),
        ];

        let filtered = apply_filters(
            &servers,
            Filters {
                max_ping: Some(100),
                min_players: 5,
            },
        );

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "a");
    }

    #[test]
    fn no_ping_filter_keeps_fast_and_slow_alike() {
        let servers = vec![server("slow", 1, 10, Some(999)), server("fast", 1, 10, Some(5))];

        let filtered = apply_filters(&servers, Filters::default());

        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn a_ping_filter_drops_servers_with_unknown_ping() {
        // The Electron build treated a missing ping as 9999ms, which happened to
        // work but only by accident of the sentinel value.
        let servers = vec![server("unknown", 5, 10, None), server("known", 5, 10, Some(300))];

        let filtered = apply_filters(
            &servers,
            Filters {
                max_ping: Some(500),
                min_players: 0,
            },
        );

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "known");
    }

    #[test]
    fn sorts_by_ping_with_unknown_ping_last() {
        let servers = vec![
            server("slow", 1, 10, Some(500)),
            server("unknown", 1, 10, None),
            server("fast", 1, 10, Some(12)),
        ];

        let filtered = apply_filters(&servers, Filters::default());

        let order: Vec<&str> = filtered.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(order, vec!["fast", "slow", "unknown"]);
    }

    #[test]
    fn equal_pings_are_broken_ties_deterministically() {
        let servers = vec![
            server("zzz", 1, 10, Some(50)),
            server("aaa", 1, 10, Some(50)),
            server("mmm", 1, 10, Some(50)),
        ];

        let filtered = apply_filters(&servers, Filters::default());
        let order: Vec<&str> = filtered.iter().map(|s| s.id.as_str()).collect();

        assert_eq!(order, vec!["aaa", "mmm", "zzz"]);
    }

    #[test]
    fn filtering_does_not_reorder_the_original_list() {
        let servers = vec![server("b", 1, 10, Some(90)), server("a", 1, 10, Some(10))];

        let filtered = apply_filters(&servers, Filters::default());

        assert_eq!(filtered[0].id, "a");
        assert_eq!(servers[0].id, "b", "input slice must not be mutated");
    }

    // ------------------------------------------------------------------- links

    #[test]
    fn builds_the_modern_deep_link() {
        assert_eq!(
            join_link(1818, "abc-123"),
            "roblox://experiences/start?placeId=1818&gameInstanceId=abc-123"
        );
    }

    #[test]
    fn builds_a_browsable_game_link() {
        assert_eq!(web_link(1818), "https://www.roblox.com/games/1818");
    }

    // -------------------------------------------------------------- scan depth

    #[test]
    fn paging_stops_at_the_requested_depth() {
        let depth = ScanDepth { requested: 250 };

        assert_eq!(depth.next_page_size(0), 100);
        assert_eq!(depth.next_page_size(100), 100);
        assert_eq!(depth.next_page_size(200), 50);
        assert_eq!(depth.next_page_size(250), 0);
        assert!(depth.wants_more(0));
        assert!(depth.wants_more(249));
        assert!(!depth.wants_more(250));
    }

    #[test]
    fn display_helpers() {
        let s = server("x", 12, 50, Some(30));
        assert_eq!(s.occupancy(), "12/50");
        assert_eq!(s.ping_label(), "30");
        assert_eq!(server("y", 0, 10, None).ping_label(), "—");
    }
}
