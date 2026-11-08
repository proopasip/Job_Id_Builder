# Code Review — Search Server for Roblox game

> **Note:** this review describes the **Electron/Node implementation**, which has
> since been rewritten in Rust. The files it refers to now live under
> [`legacy/electron/`](legacy/electron). It is kept because most of the findings
> explain *why* the Rust version is built the way it is.

Reviewed: `arena/87dae12f-search-server-for-roblox-game` @ `c2c20fe`
Method: static analysis + dependency/API verification.
**Note:** the app could not be launched in the review sandbox (Electron's binary download is blocked by network egress rules), so this is a static review, not a runtime test.

## Stack snapshot

| File | Lines | Role |
| --- | --- | --- |
| `src/main.js` | 67 | Electron main process + 3 IPC handlers, all Roblox HTTP calls |
| `src/preload.js` | 7 | contextBridge API (`fetchServers`, `fetchTrending`) |
| `src/ui/renderer.js` | 333 | UI logic + hardcoded inline locale table (7 languages) |
| `src/ui/index.html` | 380 | Single-page UI (markup + all CSS inline) |
| `src/locales.json` | 190 | Locale data — **never loaded by anything** |

## Verdict

The core loop (paste link → fetch servers → filter → copy join link) is sound, and the security baseline in the renderer is correct. But **the app's headline feature — the generated join link — almost certainly no longer works**, because it uses a deep-link format Roblox replaced in 2023. On top of that there are two dead API endpoints, one duplicated locale file, and a build config that is missing entirely.

---

## 🔴 Critical — feature is likely broken at runtime

### 1. Join link uses the retired `launchData=TARGETGAMEID/` format
`src/ui/renderer.js:302`
```js
const link = `https://www.roblox.com/games/start?placeId=${currentPlaceId}&launchData=TARGETGAMEID/${jobId}`;
```
The `TARGETGAMEID` launch-data trick is the pre-2023 approach. Roblox now supports a first-class `gameInstanceId` parameter on its deep-link scheme [3](https://devforum.roblox.com/t/developer-deeplinking-beta/1904069?page=10), and the documented, supported form is [4](https://github.com/bloxstraplabs/bloxstrap/wiki/A-deep-dive-on-how-the-Roblox-bootstrapper-works):
```
roblox://experiences/start?placeId=<PLACEID>&gameInstanceId=<JOBID>
```
Recommended fix: emit the `roblox://experiences/start?...` URI (and keep a `https://www.roblox.com/games/<placeId>` fallback for copying into a browser).

### 2. Trending + search both call a deprecated endpoint
`src/main.js:44` and `src/main.js:54` hit `https://games.roblox.com/v1/games/list`. Roblox formally deprecated `/v1/games/list` (and `/v1/games/sorts`) in May 2024 while revamping Discover [1](https://devforum.roblox.com/t/official-list-of-deprecated-web-endpoints/62889/59); it is reported as returning empty results since [4](https://devforum.roblox.com/t/is-there-any-way-to-get-the-most-popular-games-now-with-the-api/3396525). Replacement: `apis.roblox.com/explore-api/v1/get-sort-content`.

Compounding this: the `search-games` handler is **never exposed in `preload.js`**, so it is unreachable dead code. `fetch-trending` *is* exposed but has no caller in the renderer.

## 🟠 Bugs

3. **`currentPlaceId` is never declared** (`renderer.js:256`, read at `:302`). It becomes an implicit global — it happens to work in sloppy mode, but it throws `ReferenceError` the moment the file is loaded as a module or under `'use strict'`. Clicking a row before any search also yields `placeId=undefined`. → `let currentPlaceId = null;`
4. **Wrong status on row click.** `buildLink()` shows `statusCopied` ("Скопировано в буфер обмена!") even though nothing was copied — the copy only happens in `copyBtn.onclick`.
5. **No error handling on the clipboard write.** `navigator.clipboard.writeText(...)` returns a promise that can reject (permissions/focus); unhandled rejection today.
6. **`extractPlaceId` regex requires 7+ digits** (`/(\d{7,})/`), so classic/low-ID places are rejected — e.g. Crossroads is `placeId=1818`.
7. **No pagination.** `/v1/games/{id}/servers/Public` caps at 100 rows per page and requires a `cursor` to advance, so `limit=100` is a hard ceiling, not a configurable scan depth.
8. **`innerHTML` with API data** in `renderTable()` — `ping`, `playing`, `maxPlayers`, `id` are interpolated straight into markup. Low risk (local app, JSON from Roblox) but trivially avoidable with `textContent` + `createElement`.
9. **`setLanguage()` clobbers UI state**: it calls `updateStatus(statusReady)` (wipes the last result message) and rewrites `#finalLink` with the placeholder string, discarding a link the user just generated.
10. **Language/theme are not persisted** — every launch resets to RU + dark.
11. **Modal ✕ becomes "Закрыть"** — `setLanguage()` overwrites `closeSettingsBtn.innerText`, removing the glyph. Minor, but the button also has no Esc-to-close and no overlay click-to-close.

## 🟡 Duplication & dead code

12. **`src/locales.json` is orphaned.** Renderer embeds its own copy of the same table. The copies have **already drifted**: the JSON has 25 keys per language, the inline copy 23 — `savedGames` and `saveBtn` are missing from all 7 inline languages. Two sources of truth for translations will keep diverging. → pick one (load the JSON via `fs` in the preload, or delete it).
13. Unused locale entries: `searchPlaceholder`, `trending` (referenced by no UI element).
14. `resultDisplay` doubles as both the empty-state hint and the link container.

## 🟡 Build & packaging

15. **No `build` block in `package.json`.** `npm run build` → `electron-builder` with zero configuration: no `appId`, no `icon`, no `files` filter, no `win`/`mac`/`linux` targets. Result builds for the host platform only, ships the default Electron icon, and pulls in the whole project directory. `build.bat` implies a Windows `.exe` that is never actually configured.
16. **`name` is `job-id-builder-js`** (inherited from the upstream fork) while the product is "Search Server for Roblox game"; no `productName`, no `author`, no `repository`, no `homepage`.
17. README documents `dist/` output and a `.exe`; with the current config the artifact type depends entirely on the host OS.
18. No `LICENSE` file, though `package.json` declares `ISC`.

## 🟢 Hardening suggestions (nice-to-have)

19. No CSP `<meta>` tag in `index.html`.
20. No `setWindowOpenHandler` / `will-navigate` guard on the BrowserWindow.
21. Consider `sandbox: true` in `webPreferences` (currently `nodeIntegration: false` + `contextIsolation: true`, which is already the important part).

## 🟢 What's done well

- Correct Electron security baseline: `nodeIntegration: false`, `contextIsolation: true`, all network access funnelled through a narrow `contextBridge` API.
- All three JS files pass `node --check`; both JSON files parse.
- 10s timeout on the servers request; IPC handlers return `{success, error}` instead of throwing across the bridge.
- Clean CSS-variable theming that makes the dark/light switch a one-class change.
- `package-lock.json` is committed.
