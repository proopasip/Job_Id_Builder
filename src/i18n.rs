//! Localisation for the UI.
//!
//! The translations themselves are ported verbatim from the original Electron build
//! (`locales.json` in the original project), so nothing that was already translated is lost.
//! This module is dependency-free and unit-tested, like [`crate::core`].

#![allow(clippy::doc_markdown)]

/// A language the UI can be displayed in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Lang {
    /// Russian, the default.
    #[default]
    Ru,
    /// English
    En,
    /// Українська
    Uk,
    /// 中文
    Zh,
    /// 日本語
    Ja,
    /// 한국어
    Ko,
    /// Italiano
    It,
}

impl Lang {
    /// Every supported language, in the order shown in the settings window.
    pub const ALL: [Lang; 7] = [
        Lang::Ru, Lang::En, Lang::Uk, Lang::Zh, Lang::Ja, Lang::Ko, Lang::It,
    ];

    /// Short label for the language button.
    #[must_use]
    pub fn code(self) -> &'static str {
        match self {
            Lang::Ru => "RU",
            Lang::En => "EN",
            Lang::Uk => "UA",
            Lang::Zh => "CN",
            Lang::Ja => "JP",
            Lang::Ko => "KR",
            Lang::It => "IT",
        }
    }

    /// The language's own name for itself.
    #[must_use]
    pub fn native_name(self) -> &'static str {
        match self {
            Lang::Ru => "Русский",
            Lang::En => "English",
            Lang::Uk => "Українська",
            Lang::Zh => "中文",
            Lang::Ja => "日本語",
            Lang::Ko => "한국어",
            Lang::It => "Italiano",
        }
    }

    /// Parse a language back from the code returned by [`Lang::code`].
    ///
    /// Unknown codes fall back to Russian rather than failing, so a stale
    /// settings file can never stop the app from starting.
    #[must_use]
    pub fn from_code(code: &str) -> Lang {
        match code.trim().to_ascii_uppercase().as_str() {
            "RU" => Lang::Ru,
            "EN" => Lang::En,
            "UA" => Lang::Uk,
            "CN" => Lang::Zh,
            "JP" => Lang::Ja,
            "KR" => Lang::Ko,
            "IT" => Lang::It,
            _ => Lang::Ru,
        }
    }
}

/// Every translatable string in the UI.
#[derive(Debug, Clone, Copy)]
pub struct Strings {
    /// `settings`
    pub settings: &'static str,
    /// `limit`
    pub limit: &'static str,
    /// `pingFilter`
    pub ping_filter: &'static str,
    /// `playerFilter`
    pub player_filter: &'static str,
    /// `gameInput`
    pub game_input: &'static str,
    /// `fetchBtn`
    pub fetch_btn: &'static str,
    /// `resultDisplay`
    pub result_display: &'static str,
    /// `copyBtn`
    pub copy_btn: &'static str,
    /// `statusSearching`
    pub status_searching: &'static str,
    /// `statusFound`
    pub status_found: &'static str,
    /// `statusError`
    pub status_error: &'static str,
    /// `statusNoServers`
    pub status_no_servers: &'static str,
    /// `statusFiltered`
    pub status_filtered: &'static str,
    /// `statusCopied`
    pub status_copied: &'static str,
    /// `statusReady`
    pub status_ready: &'static str,
    /// `themeLight`
    pub theme_light: &'static str,
    /// `themeDark`
    pub theme_dark: &'static str,
    /// `langTitle`
    pub lang_title: &'static str,
    /// `settingsClose`
    pub settings_close: &'static str,
    /// `settingsTitle`
    pub settings_title: &'static str,
    /// `themeTitle`
    pub theme_title: &'static str,
    /// `col_ping`
    pub col_ping: &'static str,
    /// `col_players`
    pub col_players: &'static str,
    /// `col_job_id`
    pub col_job_id: &'static str,
    /// `launch`
    pub launch: &'static str,
    /// `invalid_place_id`
    pub invalid_place_id: &'static str,
    /// `no_limit`
    pub no_limit: &'static str,
    /// `game_page`
    pub game_page: &'static str,
}

static RU: Strings = Strings {
    settings: "Настройки",
    limit: "Просканировать серверов",
    ping_filter: "Макс. пинг",
    player_filter: "Мин. игроков",
    game_input: "Ссылка на игру или PlaceId",
    fetch_btn: "Найти серверы 🔍",
    result_display: "Выберите сервер из таблицы ниже",
    copy_btn: "Копировать 📋",
    status_searching: "Поиск серверов...",
    status_found: "Найдено {qty} подходящих серверов",
    status_error: "Ошибка: {msg}",
    status_no_servers: "Серверы не найдены",
    status_filtered: "Нет серверов, подходящих под фильтры",
    status_copied: "Скопировано в буфер обмена!",
    status_ready: "Готов к работе",
    theme_light: "Светлая тема",
    theme_dark: "Темная тема",
    lang_title: "Выберите язык",
    settings_close: "Закрыть",
    settings_title: "Настройки",
    theme_title: "Тема",
    col_ping: "Пинг (мс)",
    col_players: "Игроки",
    col_job_id: "Job ID",
    launch: "Запустить 🚀",
    invalid_place_id: "Неверная ссылка на игру или PlaceId",
    no_limit: "Без лимита",
    game_page: "Страница игры 🌐",
};

static EN: Strings = Strings {
    settings: "Settings",
    limit: "Scan Servers",
    ping_filter: "Max Ping",
    player_filter: "Min Players",
    game_input: "Game link or PlaceId",
    fetch_btn: "Find Servers 🔍",
    result_display: "Select a server from the table below",
    copy_btn: "Copy 📋",
    status_searching: "Searching for servers...",
    status_found: "Found {qty} matching servers",
    status_error: "Error: {msg}",
    status_no_servers: "No servers found",
    status_filtered: "No servers match your filters",
    status_copied: "Copied to clipboard!",
    status_ready: "Ready",
    theme_light: "Light Theme",
    theme_dark: "Dark Theme",
    lang_title: "Select Language",
    settings_close: "Close",
    settings_title: "Settings",
    theme_title: "Theme",
    col_ping: "Ping (ms)",
    col_players: "Players",
    col_job_id: "Job ID",
    launch: "Launch 🚀",
    invalid_place_id: "Invalid game link or PlaceId",
    no_limit: "No limit",
    game_page: "Game page 🌐",
};

static UK: Strings = Strings {
    settings: "Налаштування",
    limit: "Просканувати сервери",
    ping_filter: "Макс. пінг",
    player_filter: "Мін. гравців",
    game_input: "Посилання на гру або PlaceId",
    fetch_btn: "Знайти сервери 🔍",
    result_display: "Виберіть сервер із таблиці нижче",
    copy_btn: "Копіювати 📋",
    status_searching: "Пошук серверів...",
    status_found: "Знайдено {qty} відповідних серверів",
    status_error: "Помилка: {msg}",
    status_no_servers: "Сервери не знайдені",
    status_filtered: "Немає серверів, що відповідають фільтрам",
    status_copied: "Скопійовано в буфер обміну!",
    status_ready: "Готовий до роботи",
    theme_light: "Світла тема",
    theme_dark: "Темна тема",
    lang_title: "Виберіть мову",
    settings_close: "Закрити",
    settings_title: "Налаштування",
    theme_title: "Тема",
    col_ping: "Пінг (мс)",
    col_players: "Гравці",
    col_job_id: "Job ID",
    launch: "Запустити 🚀",
    invalid_place_id: "Невірне посилання на гру або PlaceId",
    no_limit: "Без ліміту",
    game_page: "Сторінка гри 🌐",
};

static ZH: Strings = Strings {
    settings: "设置",
    limit: "扫描服务器",
    ping_filter: "最大延迟",
    player_filter: "最小人数",
    game_input: "游戏链接或 PlaceId",
    fetch_btn: "查找服务器 🔍",
    result_display: "从下方表格中选择一个服务器",
    copy_btn: "复制 📋",
    status_searching: "正在搜索服务器...",
    status_found: "找到 {qty} 个匹配的服务器",
    status_error: "错误: {msg}",
    status_no_servers: "未找到服务器",
    status_filtered: "没有符合过滤条件的服务器",
    status_copied: "已复制到剪贴板！",
    status_ready: "准备就绪",
    theme_light: "浅色主题",
    theme_dark: "深色主题",
    lang_title: "选择语言",
    settings_close: "关闭",
    settings_title: "设置",
    theme_title: "主题",
    col_ping: "延迟 (毫秒)",
    col_players: "玩家",
    col_job_id: "Job ID",
    launch: "启动 🚀",
    invalid_place_id: "无效的游戏链接或 PlaceId",
    no_limit: "无限制",
    game_page: "游戏页面 🌐",
};

static JA: Strings = Strings {
    settings: "設定",
    limit: "サーバーをスキャン",
    ping_filter: "最大Ping",
    player_filter: "最小プレイヤー数",
    game_input: "ゲームリンクまたはPlaceId",
    fetch_btn: "サーバーを探す 🔍",
    result_display: "下の表からサーバーを選択してください",
    copy_btn: "コピー 📋",
    status_searching: "サーバーを検索中...",
    status_found: "{qty} 個のサーバーが見つかりました",
    status_error: "エラー: {msg}",
    status_no_servers: "サーバーが見つかりませんでした",
    status_filtered: "フィルターに一致するサーバーはありません",
    status_copied: "クリップボードにコピーされました！",
    status_ready: "準備完了",
    theme_light: "ライトテーマ",
    theme_dark: "ダークテーマ",
    lang_title: "言語を選択",
    settings_close: "閉じる",
    settings_title: "設定",
    theme_title: "テーマ",
    col_ping: "Ping (ミリ秒)",
    col_players: "プレイヤー",
    col_job_id: "Job ID",
    launch: "起動 🚀",
    invalid_place_id: "無効なゲームリンクまたは PlaceId",
    no_limit: "制限なし",
    game_page: "ゲームページ 🌐",
};

static KO: Strings = Strings {
    settings: "설정",
    limit: "서버 스캔",
    ping_filter: "최대 핑",
    player_filter: "최소 플레이어",
    game_input: "게임 링크 또는 PlaceId",
    fetch_btn: "서버 찾기 🔍",
    result_display: "아래 표에서 서버를 선택하세요",
    copy_btn: "복사 📋",
    status_searching: "서버 검색 중...",
    status_found: "{qty}개의 일치하는 서버를 찾았습니다",
    status_error: "오류: {msg}",
    status_no_servers: "서버를 찾을 수 없습니다",
    status_filtered: "필터와 일치하는 서버가 없습니다",
    status_copied: "클립보드에 복사되었습니다!",
    status_ready: "준비 완료",
    theme_light: "라이트 테마",
    theme_dark: "다크 테마",
    lang_title: "언어 선택",
    settings_close: "닫기",
    settings_title: "설정",
    theme_title: "테마",
    col_ping: "핑 (ms)",
    col_players: "플레이어",
    col_job_id: "Job ID",
    launch: "실행 🚀",
    invalid_place_id: "잘못된 게임 링크 또는 PlaceId",
    no_limit: "제한 없음",
    game_page: "게임 페이지 🌐",
};

static IT: Strings = Strings {
    settings: "Impostazioni",
    limit: "Scansiona Server",
    ping_filter: "Ping Massimo",
    player_filter: "Giocatori Minimi",
    game_input: "Link del gioco o PlaceId",
    fetch_btn: "Trova Server 🔍",
    result_display: "Seleziona un server dalla tabella sottostante",
    copy_btn: "Copia 📋",
    status_searching: "Ricerca server in corso...",
    status_found: "Trovati {qty} server corrispondenti",
    status_error: "Errore: {msg}",
    status_no_servers: "Nessun server trovato",
    status_filtered: "Nessun server corrisponde ai filtri",
    status_copied: "Copiato negli appunti!",
    status_ready: "Pronto",
    theme_light: "Tema Chiaro",
    theme_dark: "Tema Scuro",
    lang_title: "Seleziona Lingua",
    settings_close: "Chiudi",
    settings_title: "Impostazioni",
    theme_title: "Tema",
    col_ping: "Ping (ms)",
    col_players: "Giocatori",
    col_job_id: "Job ID",
    launch: "Avvia 🚀",
    invalid_place_id: "Link del gioco o PlaceId non valido",
    no_limit: "Nessun limite",
    game_page: "Pagina del gioco 🌐",
};

/// Look up the string table for `lang`.
#[must_use]
pub fn strings(lang: Lang) -> &'static Strings {
    match lang {
        Lang::Ru => &RU,
        Lang::En => &EN,
        Lang::Uk => &UK,
        Lang::Zh => &ZH,
        Lang::Ja => &JA,
        Lang::Ko => &KO,
        Lang::It => &IT,
    }
}

/// `statusFound` with the `{qty}` placeholder filled in.
#[must_use]
pub fn status_found(lang: Lang, qty: usize) -> String {
    strings(lang).status_found.replace("{qty}", &qty.to_string())
}

/// `statusError` with the `{msg}` placeholder filled in.
#[must_use]
pub fn status_error(lang: Lang, msg: &str) -> String {
    strings(lang).status_error.replace("{msg}", msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_language_defines_every_key() {
        // Non-ASCII text would show up as an empty string if a translation went missing.
        for lang in Lang::ALL {
            let s = strings(lang);
            assert!(!s.settings.is_empty(), "settings empty for {lang:?}");
            assert!(!s.limit.is_empty(), "limit empty for {lang:?}");
            assert!(!s.ping_filter.is_empty(), "ping_filter empty for {lang:?}");
            assert!(!s.player_filter.is_empty(), "player_filter empty for {lang:?}");
            assert!(!s.game_input.is_empty(), "game_input empty for {lang:?}");
            assert!(!s.fetch_btn.is_empty(), "fetch_btn empty for {lang:?}");
            assert!(!s.result_display.is_empty(), "result_display empty for {lang:?}");
            assert!(!s.copy_btn.is_empty(), "copy_btn empty for {lang:?}");
            assert!(!s.status_searching.is_empty(), "status_searching empty for {lang:?}");
            assert!(!s.status_found.is_empty(), "status_found empty for {lang:?}");
            assert!(!s.status_error.is_empty(), "status_error empty for {lang:?}");
            assert!(!s.status_no_servers.is_empty(), "status_no_servers empty for {lang:?}");
            assert!(!s.status_filtered.is_empty(), "status_filtered empty for {lang:?}");
            assert!(!s.status_copied.is_empty(), "status_copied empty for {lang:?}");
            assert!(!s.status_ready.is_empty(), "status_ready empty for {lang:?}");
            assert!(!s.theme_light.is_empty(), "theme_light empty for {lang:?}");
            assert!(!s.theme_dark.is_empty(), "theme_dark empty for {lang:?}");
            assert!(!s.lang_title.is_empty(), "lang_title empty for {lang:?}");
            assert!(!s.settings_close.is_empty(), "settings_close empty for {lang:?}");
            assert!(!s.settings_title.is_empty(), "settings_title empty for {lang:?}");
            assert!(!s.theme_title.is_empty(), "theme_title empty for {lang:?}");
            assert!(!s.col_ping.is_empty(), "col_ping empty for {lang:?}");
            assert!(!s.col_players.is_empty(), "col_players empty for {lang:?}");
            assert!(!s.col_job_id.is_empty(), "col_job_id empty for {lang:?}");
            assert!(!s.launch.is_empty(), "launch empty for {lang:?}");
            assert!(!s.invalid_place_id.is_empty(), "invalid_place_id empty for {lang:?}");
            assert!(!s.no_limit.is_empty(), "no_limit empty for {lang:?}");
            assert!(!s.game_page.is_empty(), "game_page empty for {lang:?}");
        }
    }

    #[test]
    fn placeholders_are_substituted() {
        for lang in Lang::ALL {
            assert!(!status_found(lang, 42).contains("{qty}"));
            assert!(status_found(lang, 42).contains("42"));
            assert_eq!(
                status_error(Lang::En, "boom"),
                "Error: boom"
            );
        }
    }

    #[test]
    fn language_codes_round_trip() {
        for lang in Lang::ALL {
            assert_eq!(Lang::from_code(lang.code()), lang);
        }
        assert_eq!(Lang::from_code("xx"), Lang::Ru, "unknown codes fall back");
        assert_eq!(Lang::from_code(" en "), Lang::En, "codes are trimmed");
    }

    #[test]
    fn english_is_the_reference_locale() {
        assert_eq!(strings(Lang::En).fetch_btn, "Find Servers 🔍");
        assert_eq!(status_error(Lang::En, "boom"), "Error: boom");
    }
}
