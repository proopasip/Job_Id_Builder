// Locales
const locales = {
    "en": {
        "settings": "Settings",
        "limit": "Server Limit",
        "pingFilter": "Max Ping",
        "playerFilter": "Min Players",
        "gameInput": "Game link or PlaceId",
        "fetchBtn": "Find Servers 🔍",
        "resultDisplay": "Select a server from the table below",
        "copyBtn": "Copy 📋",
        "searchPlaceholder": "Search games...",
        "trending": "Trending Games",
        "statusSearching": "Searching for servers...",
        "statusFound": "Found {qty} matching servers",
        "statusError": "Error: {msg}",
        "statusNoServers": "No servers found",
        "statusFiltered": "No servers match your filters",
        "statusCopied": "Copied to clipboard!",
        "statusReady": "Ready",
        "themeLight": "Light Theme",
        "themeDark": "Dark Theme",
        "langTitle": "Select Language",
        "settingsClose": "Close",
        "settingsTitle": "Settings",
        "themeTitle": "Theme"
    },
    "ru": {
        "settings": "Настройки",
        "limit": "Просканировать серверов",
        "pingFilter": "Макс. пинг",
        "playerFilter": "Мин. игроков",
        "gameInput": "Ссылка на игру или PlaceId",
        "fetchBtn": "Найти серверы 🔍",
        "resultDisplay": "Выберите сервер из таблицы ниже",
        "copyBtn": "Копировать 📋",
        "searchPlaceholder": "Поиск игр...",
        "trending": "Популярные игры",
        "statusSearching": "Поиск серверов...",
        "statusFound": "Найдено {qty} подходящих серверов",
        "statusError": "Ошибка: {msg}",
        "statusNoServers": "Серверы не найдены",
        "statusFiltered": "Нет серверов, подходящих под фильтры",
        "statusCopied": "Скопировано в буфер обмена!",
        "statusReady": "Готов к работе",
        "themeLight": "Светлая тема",
        "themeDark": "Темная тема",
        "langTitle": "Выберите язык",
        "settingsClose": "Закрыть",
        "settingsTitle": "Настройки",
        "themeTitle": "Тема"
    },
    "uk": {
        "settings": "Налаштування",
        "limit": "Ліміт серверів",
        "pingFilter": "Макс. пінг",
        "playerFilter": "Мін. гравців",
        "gameInput": "Посилання на гру або PlaceId",
        "fetchBtn": "Знайти сервери 🔍",
        "resultDisplay": "Виберіть сервер із таблиці нижче",
        "copyBtn": "Копіювати 📋",
        "searchPlaceholder": "Пошук ігор...",
        "trending": "Популярні ігри",
        "statusSearching": "Пошук серверів...",
        "statusFound": "Знайдено {qty} відповідних серверів",
        "statusError": "Помилка: {msg}",
        "statusNoServers": "Сервери не знайдені",
        "statusFiltered": "Немає серверів, що відповідають фільтрам",
        "statusCopied": "Скопійовано в буфер обміну!",
        "statusReady": "Готовий до роботи",
        "themeLight": "Світла тема",
        "themeDark": "Темна тема",
        "langTitle": "Виберіть мову",
        "settingsClose": "Закрити",
        "settingsTitle": "Налаштування",
        "themeTitle": "Тема"
    },
    "zh": {
        "settings": "设置",
        "limit": "服务器限制",
        "pingFilter": "最大延迟",
        "playerFilter": "最小人数",
        "gameInput": "游戏链接或 PlaceId",
        "fetchBtn": "查找服务器 🔍",
        "resultDisplay": "从下方表格中选择一个服务器",
        "copyBtn": "复制 📋",
        "searchPlaceholder": "搜索游戏...",
        "trending": "热门游戏",
        "statusSearching": "正在搜索服务器...",
        "statusFound": "找到 {qty} 个匹配的服务器",
        "statusError": "错误: {msg}",
        "statusNoServers": "未找到服务器",
        "statusFiltered": "没有符合过滤条件的服务器",
        "statusCopied": "已复制到剪贴板！",
        "statusReady": "准备就绪",
        "themeLight": "浅色主题",
        "themeDark": "深色主题",
        "langTitle": "选择语言",
        "settingsClose": "关闭",
        "settingsTitle": "设置",
        "themeTitle": "主题"
    },
    "ja": {
        "settings": "設定",
        "limit": "サーバー制限",
        "pingFilter": "最大Ping",
        "playerFilter": "最小プレイヤー数",
        "gameInput": "ゲームリンクまたはPlaceId",
        "fetchBtn": "サーバーを探す 🔍",
        "resultDisplay": "下の表からサーバーを選択してください",
        "copyBtn": "コピー 📋",
        "searchPlaceholder": "ゲームを検索...",
        "trending": "人気のゲーム",
        "statusSearching": "サーバーを検索中...",
        "statusFound": "{qty} 個のサーバーが見つかりました",
        "statusError": "エラー: {msg}",
        "statusNoServers": "サーバーが見つかりませんでした",
        "statusFiltered": "フィルターに一致するサーバーはありません",
        "statusCopied": "クリップボードにコピーされました！",
        "statusReady": "準備完了",
        "themeLight": "ライトテーマ",
        "themeDark": "ダークテーマ",
        "langTitle": "言語を選択",
        "settingsClose": "閉じる",
        "settingsTitle": "設定",
        "themeTitle": "テーマ"
    },
    "ko": {
        "settings": "설정",
        "limit": "서버 제한",
        "pingFilter": "최대 핑",
        "playerFilter": "최소 플레이어",
        "gameInput": "게임 링크 또는 PlaceId",
        "fetchBtn": "서버 찾기 🔍",
        "resultDisplay": "아래 표에서 서버를 선택하세요",
        "copyBtn": "복사 📋",
        "searchPlaceholder": "게임 검색...",
        "trending": "인기 게임",
        "statusSearching": "서버 검색 중...",
        "statusFound": "{qty}개의 일치하는 서버를 찾았습니다",
        "statusError": "오류: {msg}",
        "statusNoServers": "서버를 찾을 수 없습니다",
        "statusFiltered": "필터와 일치하는 서버가 없습니다",
        "statusCopied": "클립보드에 복사되었습니다!",
        "statusReady": "준비 완료",
        "themeLight": "라이트 테마",
        "themeDark": "다크 테마",
        "langTitle": "언어 선택",
        "settingsClose": "닫기",
        "settingsTitle": "설정",
        "themeTitle": "테마"
    },
    "it": {
        "settings": "Impostazioni",
        "limit": "Limite Server",
        "pingFilter": "Ping Massimo",
        "playerFilter": "Giocatori Minimi",
        "gameInput": "Link del gioco o PlaceId",
        "fetchBtn": "Trova Server 🔍",
        "resultDisplay": "Seleziona un server dalla tabella sottostante",
        "copyBtn": "Copia 📋",
        "searchPlaceholder": "Cerca giochi...",
        "trending": "Giochi di tendenza",
        "statusSearching": "Ricerca server in corso...",
        "statusFound": "Trovati {qty} server corrispondenti",
        "statusError": "Errore: {msg}",
        "statusNoServers": "Nessun server trovato",
        "statusFiltered": "Nessun server corrisponde ai filtri",
        "statusCopied": "Copiato negli appunti!",
        "statusReady": "Pronto",
        "themeLight": "Tema Chiaro",
        "themeDark": "Tema Scuro",
        "langTitle": "Seleziona Lingua",
        "settingsClose": "Chiudi",
        "settingsTitle": "Impostazioni",
        "themeTitle": "Tema"
    }
};
let currentLang = 'ru';

// Elements
const gameInput = document.getElementById('gameInput');
const fetchBtn = document.getElementById('fetchBtn');
const serverTable = document.querySelector('#serverTable tbody');
const statusBar = document.getElementById('statusBar');
const finalLink = document.getElementById('finalLink');
const resultDisplay = document.getElementById('resultDisplay');
const copyBtn = document.getElementById('copyBtn');
const openSettingsBtn = document.getElementById('openSettingsBtn');
const settingsModal = document.getElementById('settingsModal');
const closeSettingsBtn = document.getElementById('closeSettingsBtn');

function setLanguage(lang) {
    currentLang = lang;
    document.querySelectorAll('.lang-btn').forEach(btn => {
        btn.classList.toggle('active', btn.getAttribute('data-lang') === lang);
    });
    
    const t = locales[lang];
    document.getElementById('openSettingsBtn').innerText = '⚙️ ' + t.settings;
    document.getElementById('txt-limit').innerText = t.limit;
    document.getElementById('txt-pingFilter').innerText = t.pingFilter;
    document.getElementById('txt-playerFilter').innerText = t.playerFilter;
    document.getElementById('txt-gameInput').innerText = t.gameInput;
    document.getElementById('fetchBtn').innerText = t.fetchBtn;
    document.getElementById('finalLink').innerText = t.resultDisplay;
    document.getElementById('copyBtn').innerText = t.copyBtn;
    
    // Modal
    document.getElementById('txt-settingsTitle').innerText = t.settingsTitle;
    document.getElementById('txt-langTitle').innerText = t.langTitle;
    document.getElementById('txt-themeTitle').innerText = t.themeTitle;
    document.getElementById('txt-themeLight').innerText = t.themeLight;
    document.getElementById('txt-themeDark').innerText = t.themeDark;
    document.getElementById('closeSettingsBtn').innerText = t.settingsClose;
    
    document.getElementById('th-ping').innerText = lang === 'ru' ? 'Пинг (ms)' : 'Ping (ms)';
    document.getElementById('th-players').innerText = lang === 'ru' ? 'Игроки' : 'Players';
    document.getElementById('th-jobid').innerText = lang === 'ru' ? 'Job ID' : 'Job ID';
    
    updateStatus(t.statusReady, '');
}

function setTheme(theme) {
    if (theme === 'light') {
        document.body.classList.add('light-theme');
        document.body.classList.remove('dark-theme');
        document.getElementById('themeLightBtn').classList.add('active');
        document.getElementById('themeDarkBtn').classList.remove('active');
    } else {
        document.body.classList.add('dark-theme');
        document.body.classList.remove('light-theme');
        document.getElementById('themeDarkBtn').classList.add('active');
        document.getElementById('themeLightBtn').classList.remove('active');
    }
}

function extractPlaceId(input) {
    const regex = /(\d{7,})/; 
    const match = input.match(regex);
    return match ? match[0] : null;
}

async function fetchServers() {
    const input = gameInput.value.trim();
    const placeId = extractPlaceId(input);
    const limit = document.getElementById('limitSelect').value;
    const pingFilter = parseInt(document.getElementById('pingFilter').value) || 9999;
    const playerFilter = parseInt(document.getElementById('playerFilter').value) || 0;

    if (!placeId) {
        updateStatus(locales[currentLang].statusError.replace('{msg}', 'Invalid PlaceId'), 'error');
        return;
    }

    currentPlaceId = placeId;
    fetchBtn.disabled = true;
    updateStatus(locales[currentLang].statusSearching, '');
    serverTable.innerHTML = '';

    const response = await window.electronAPI.fetchServers({ placeId, limit });
    fetchBtn.disabled = false;

    if (response.success) {
        const servers = response.data || [];
        if (servers.length === 0) {
            updateStatus(locales[currentLang].statusNoServers, 'error');
            return;
        }

        const filteredServers = servers
            .filter(s => (s.ping || 9999) <= pingFilter && (s.playing || 0) >= playerFilter)
            .sort((a, b) => (a.ping || 9999) - (b.ping || 9999));

        if (filteredServers.length === 0) {
            updateStatus(locales[currentLang].statusFiltered, 'error');
            return;
        }

        renderTable(filteredServers);
        updateStatus(locales[currentLang].statusFound.replace('{qty}', filteredServers.length), 'success');
    } else {
        updateStatus(locales[currentLang].statusError.replace('{msg}', response.error), 'error');
    }
}

function renderTable(servers) {
    serverTable.innerHTML = '';
    servers.forEach(server => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td>${server.ping || 'N/A'}</td>
            <td>${server.playing}/${server.maxPlayers}</td>
            <td style="font-size: 0.8em; color: #bbb;">${server.id}</td>
        `;
        row.onclick = () => buildLink(server.id);
        serverTable.appendChild(row);
    });
}

function buildLink(jobId) {
    const link = `https://www.roblox.com/games/start?placeId=${currentPlaceId}&launchData=TARGETGAMEID/${jobId}`;
    finalLink.textContent = link;
    resultDisplay.style.display = 'block';
    updateStatus(locales[currentLang].statusCopied, 'success');
}

function updateStatus(msg, type) {
    statusBar.textContent = msg;
    statusBar.className = 'status-bar ' + type;
}

// Event Listeners
copyBtn.onclick = () => {
    navigator.clipboard.writeText(finalLink.textContent);
    updateStatus(locales[currentLang].statusCopied, 'success');
};

fetchBtn.onclick = fetchServers;

openSettingsBtn.onclick = () => settingsModal.style.display = 'flex';
closeSettingsBtn.onclick = () => settingsModal.style.display = 'none';

document.querySelectorAll('.lang-btn').forEach(btn => {
    btn.onclick = () => setLanguage(btn.getAttribute('data-lang'));
});

document.getElementById('themeDarkBtn').onclick = () => setTheme('dark');
document.getElementById('themeLightBtn').onclick = () => setTheme('light');

// Init
setLanguage('ru');
setTheme('dark');
