# 🚀 Search Server for Roblox game

Find the fastest public server of any Roblox game and join it with one click.

Paste a game link (or a bare `PlaceId`), and the app asks Roblox for that game's
public servers, filters them by ping and player count, and lists them best-ping
first. Pick a row and it hands the exact server to your Roblox client.

This is a **native Rust rewrite** of the original Electron/Node application: one
self-contained binary, with no Node.js and no bundled Chromium.

---

## ✨ Features

- 🔍 **Search by link or id** — `1818`, `https://www.roblox.com/games/1818/Crossroads`,
  or anything with a `placeId=` parameter.
- 📉 **Filters that work** — cap the maximum ping, require a minimum number of
  players. Servers with no reported ping sort last instead of being faked.
- ⚡ **Deep scans** — follows the API's pagination cursor, so you can scan up to
  1000 servers instead of being stuck at the 100-row first page.
- 🚀 **One-click join** — generates the modern
  `roblox://experiences/start?placeId=…&gameInstanceId=…` deep link and hands it
  to your operating system. A normal https link to the game is one click away too.
- 🌐 **7 languages** — Русский, English, Українська, 中文, 日本語, 한국어, Italiano.
- 🌓 **Dark and light themes** — and both are remembered between runs.

---

## 🆚 What changed compared to the Electron version

| | Electron version | Rust version |
| --- | --- | --- |
| Runtime | Node.js + Chromium (~150 MB) | one native binary |
| Join link | `launchData=TARGETGAMEID/<job>` (retired by Roblox) | `roblox://experiences/start?…&gameInstanceId=…` |
| Place id parsing | regex `\d{7,}` — classic ids like `1818` were rejected | structured parse, short ids accepted |
| Paging | one page, 100 servers max | follows `nextPageCursor` up to 1000 |
| Missing ping | faked as `9999` ms | `Option<u32>`, sorted last |
| Settings (language, theme) | reset on every launch | persisted through eframe's storage |
| Locales | duplicated: `locales.json` **and** an inline copy that had already drifted | single table in `src/i18n.rs` |
| Trending / game search | called `/v1/games/list`, deprecated by Roblox and returning nothing | removed rather than shipped broken |
| Business logic | mixed into the renderer | isolated in `src/domain.rs` and unit-tested |

---

## 🛠 Requirements

- **Rust 1.95 or newer** (that is egui 0.37's minimum supported version).
- On Linux, eframe needs a few system libraries first:

  ```bash
  sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev \
      libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
  ```

Windows and macOS need nothing extra.

---

## 🚀 Build and run

```bash
cargo run --release        # run it
cargo build --release      # -> target/release/roblox-server-search
cargo test                 # 20 unit tests
```

---

## 📁 Project layout

```
src/
  main.rs     entry point: window options + eframe startup
  domain.rs   Server, filters, place-id parsing, join links — zero dependencies
  i18n.rs     the 7 languages — zero dependencies
  net.rs      Roblox Games API client (reqwest)
  ui.rs       the egui interface
```

`domain.rs` and `i18n.rs` deliberately depend on nothing but the standard
library, so the logic that actually matters can be compiled and tested without a
display, a network connection, or any third-party crate:

```bash
rustc --edition 2021 --test src/domain.rs -o /tmp/domain_test && /tmp/domain_test
```

---

## 📡 Notes on the Roblox API

- Server lists come from
  `https://games.roblox.com/v1/games/{placeId}/servers/Public`, which returns at
  most **100 rows per page** and a `nextPageCursor`. The client follows that
  cursor until it has enough servers.
- The old "trending games" and "search games by name" features were **not**
  ported: they relied on `/v1/games/list`, which Roblox deprecated in 2024 and
  which now returns an empty list.
- Joining a server uses the `roblox://` protocol, which browsers refuse to open,
  so the link is handed straight to the OS (`start` on Windows, `open` on macOS,
  `xdg-open` elsewhere).

---

## 📜 License

ISC. Original concept by [Proopasip](https://github.com/proopasip),
fork development by [egefefefefef](https://github.com/egefefefefef).
