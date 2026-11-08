#!/usr/bin/env python3
"""Generate src/i18n.rs from the original Electron locale table."""
import json
import re

SRC = "legacy/electron/src/locales.json"
OUT = "src/i18n.rs"

with open(SRC, encoding="utf-8") as fh:
    data = json.load(fh)

# Keys that only exist in the Rust UI. The original Electron build hardcoded its
# table headers (and had only RU/EN variants), so these are new translations.
EXTRA = {
    "col_ping": {
        "ru": "Пинг (мс)", "en": "Ping (ms)", "uk": "Пінг (мс)",
        "zh": "延迟 (毫秒)", "ja": "Ping (ミリ秒)", "ko": "핑 (ms)", "it": "Ping (ms)",
    },
    "col_players": {
        "ru": "Игроки", "en": "Players", "uk": "Гравці",
        "zh": "玩家", "ja": "プレイヤー", "ko": "플레이어", "it": "Giocatori",
    },
    "col_job_id": {
        "ru": "Job ID", "en": "Job ID", "uk": "Job ID",
        "zh": "Job ID", "ja": "Job ID", "ko": "Job ID", "it": "Job ID",
    },
    "launch": {
        "ru": "Запустить 🚀", "en": "Launch 🚀", "uk": "Запустити 🚀",
        "zh": "启动 🚀", "ja": "起動 🚀", "ko": "실행 🚀", "it": "Avvia 🚀",
    },
    "invalid_place_id": {
        "ru": "Неверная ссылка на игру или PlaceId",
        "en": "Invalid game link or PlaceId",
        "uk": "Невірне посилання на гру або PlaceId",
        "zh": "无效的游戏链接或 PlaceId",
        "ja": "無効なゲームリンクまたは PlaceId",
        "ko": "잘못된 게임 링크 또는 PlaceId",
        "it": "Link del gioco o PlaceId non valido",
    },
    "no_limit": {
        "ru": "Без лимита", "en": "No limit", "uk": "Без ліміту",
        "zh": "无限制", "ja": "制限なし", "ko": "제한 없음", "it": "Nessun limite",
    },
    "game_page": {
        "ru": "Страница игры 🌐", "en": "Game page 🌐", "uk": "Сторінка гри 🌐",
        "zh": "游戏页面 🌐", "ja": "ゲームページ 🌐", "ko": "게임 페이지 🌐",
        "it": "Pagina del gioco 🌐",
    },
}

# Keys that belong to features the Rust rewrite does not have (game search,
# trending games, saved games). They are still kept in
# `legacy/electron/src/locales.json` if those features ever come back, but
# carrying them here would just be dead weight.
SKIP = {"searchPlaceholder", "trending", "savedGames", "saveBtn"}

LANGS = ["ru", "en", "uk", "zh", "ja", "ko", "it"]
ENUM = {"ru": "Ru", "en": "En", "uk": "Uk", "zh": "Zh", "ja": "Ja", "ko": "Ko", "it": "It"}
NATIVE = {
    "ru": "Русский", "en": "English", "uk": "Українська", "zh": "中文",
    "ja": "日本語", "ko": "한국어", "it": "Italiano",
}
CODE = {"ru": "RU", "en": "EN", "uk": "UA", "zh": "CN", "ja": "JP", "ko": "KR", "it": "IT"}

def snake(name: str) -> str:
    name = re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()
    return name.replace("__", "_")

def rust_str(s: str) -> str:
    out = s.replace("\\", "\\\\").replace('"', '\\"')
    out = out.replace("\n", "\\n").replace("\t", "\\t").replace("\r", "\\r")
    return '"' + out + '"'

# Field order: original keys first (keeps them diffable against locales.json), then the new ones.
fields = [k for k in data["en"] if k not in SKIP] + list(EXTRA.keys())
snake_fields = [snake(f) for f in fields]

lines = []
w = lines.append
w("//! Localisation for the UI.")
w("//!")
w("//! The translations themselves are ported verbatim from the original Electron build")
w("//! (`legacy/electron/src/locales.json`), so nothing that was already translated is lost.")
w("//! This module is dependency-free and unit-tested, like [`crate::core`].")
w("")
w("#![allow(clippy::doc_markdown)]")
w("")
w("/// A language the UI can be displayed in.")
w("#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]")
w("pub enum Lang {")
w("    /// Russian, the default.")
w("    #[default]")
for lang in LANGS:
    if lang == "ru":
        w("    Ru,")
    else:
        w(f"    /// {NATIVE[lang]}")
        w(f"    {ENUM[lang]},")
w("}")
w("")
w("impl Lang {")
w("    /// Every supported language, in the order shown in the settings window.")
w("    pub const ALL: [Lang; %d] = [" % len(LANGS))
w("        " + ", ".join(f"Lang::{ENUM[l]}" for l in LANGS) + ",")
w("    ];")
w("")
w("    /// Short label for the language button.")
w("    #[must_use]")
w("    pub fn code(self) -> &'static str {")
w("        match self {")
for lang in LANGS:
    w(f'            Lang::{ENUM[lang]} => "{CODE[lang]}",')
w("        }")
w("    }")
w("")
w("    /// The language's own name for itself.")
w("    #[must_use]")
w("    pub fn native_name(self) -> &'static str {")
w("        match self {")
for lang in LANGS:
    w(f'            Lang::{ENUM[lang]} => "{NATIVE[lang]}",')
w("        }")
w("    }")
w("")
w("    /// Parse a language back from the code returned by [`Lang::code`].")
w("    ///")
w("    /// Unknown codes fall back to Russian rather than failing, so a stale")
w("    /// settings file can never stop the app from starting.")
w("    #[must_use]")
w("    pub fn from_code(code: &str) -> Lang {")
w("        match code.trim().to_ascii_uppercase().as_str() {")
for lang in LANGS:
    w(f'            "{CODE[lang]}" => Lang::{ENUM[lang]},')
w("            _ => Lang::Ru,")
w("        }")
w("    }")
w("}")
w("")
w("/// Every translatable string in the UI.")
w("#[derive(Debug, Clone, Copy)]")
w("pub struct Strings {")
for f, sf in zip(fields, snake_fields):
    w(f"    /// `{f}`")
    w(f"    pub {sf}: &'static str,")
w("}")
w("")
for lang in LANGS:
    w(f"static {ENUM[lang].upper()}: Strings = Strings {{")
    for f, sf in zip(fields, snake_fields):
        val = data[lang][f] if f in data[lang] else EXTRA[f][lang]
        w(f"    {sf}: {rust_str(val)},")
    w("};")
    w("")
w("/// Look up the string table for `lang`.")
w("#[must_use]")
w("pub fn strings(lang: Lang) -> &'static Strings {")
w("    match lang {")
for lang in LANGS:
    w(f"        Lang::{ENUM[lang]} => &{ENUM[lang].upper()},")
w("    }")
w("}")
w("")
w("/// `statusFound` with the `{qty}` placeholder filled in.")
w("#[must_use]")
w("pub fn status_found(lang: Lang, qty: usize) -> String {")
w("    strings(lang).status_found.replace(\"{qty}\", &qty.to_string())")
w("}")
w("")
w("/// `statusError` with the `{msg}` placeholder filled in.")
w("#[must_use]")
w("pub fn status_error(lang: Lang, msg: &str) -> String {")
w("    strings(lang).status_error.replace(\"{msg}\", msg)")
w("}")
w("")
w("#[cfg(test)]")
w("mod tests {")
w("    use super::*;")
w("")
w("    #[test]")
w("    fn every_language_defines_every_key() {")
w("        // Non-ASCII text would show up as an empty string if a translation went missing.")
w("        for lang in Lang::ALL {")
w("            let s = strings(lang);")
for f, sf in zip(fields, snake_fields):
    w(f"            assert!(!s.{sf}.is_empty(), \"{sf} empty for {{lang:?}}\");")
w("        }")
w("    }")
w("")
w("    #[test]")
w("    fn placeholders_are_substituted() {")
w("        for lang in Lang::ALL {")
w("            assert!(!status_found(lang, 42).contains(\"{qty}\"));")
w("            assert!(status_found(lang, 42).contains(\"42\"));")
w("            assert_eq!(")
w("                status_error(Lang::En, \"boom\"),")
w("                \"Error: boom\"")
w("            );")
w("        }")
w("    }")
w("")
w("    #[test]")
w("    fn language_codes_round_trip() {")
w("        for lang in Lang::ALL {")
w("            assert_eq!(Lang::from_code(lang.code()), lang);")
w("        }")
w('        assert_eq!(Lang::from_code("xx"), Lang::Ru, "unknown codes fall back");')
w('        assert_eq!(Lang::from_code(" en "), Lang::En, "codes are trimmed");')
w("    }")
w("")
w("    #[test]")
w("    fn english_is_the_reference_locale() {")
w("        assert_eq!(strings(Lang::En).fetch_btn, \"Find Servers 🔍\");")
w("        assert_eq!(status_error(Lang::En, \"boom\"), \"Error: boom\");")
w("    }")
w("}")
w("")

with open(OUT, "w", encoding="utf-8") as fh:
    fh.write("\n".join(lines))

print(f"wrote {OUT} ({len(lines)} lines, {len(fields)} keys x {len(LANGS)} langs)")
