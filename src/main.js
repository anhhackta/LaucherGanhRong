const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const { Window } = window.__TAURI__.window;

// State logic
let currentConfig = {};
let latestManifest = null;
let localVersion = "0.0.0";
let gameStatus = "Checking";

// Localization Map
const STRINGS = {
    "en": { "download": "Download", "play": "Play", "update": "Update", "checking": "Checking...", "offline": "Offline" },
    "vi": { "download": "Tải Game", "play": "Chơi Ngay", "update": "Cập Nhật", "checking": "Đang kiểm tra...", "offline": "Mất mạng" },
    "jp": { "download": "ダウンロード", "play": "プレイ", "update": "更新", "checking": "確認中...", "offline": "オフライン" },
    "zh": { "download": "下载", "play": "开始游戏", "update": "更新", "checking": "检查中...", "offline": "离线" }
};
let currentLang = "en";

// Elements
const elBackground = document.getElementById('background');
const elActionBtn = document.getElementById('action-btn');
const elBtnText = document.getElementById('btn-text');
const elProgressInfo = document.getElementById('progress-info');
const elProgressText = document.getElementById('progress-text');
const elProgressPercent = document.getElementById('progress-percent');
const elOfflineBanner = document.getElementById('offline-banner');

// News Carousel Elements
const elNewsSlides = document.getElementById('news-slides');
const elNewsTitle = document.getElementById('news-title');
const elNewsDate = document.getElementById('news-date');
const elNewsDots = document.getElementById('news-dots');
const elNewsPrev = document.getElementById('news-prev');
const elNewsNext = document.getElementById('news-next');

// Modals
const elSettingsModal = document.getElementById('settings-modal');
const elSettingsBtn = document.getElementById('settings-btn');
const elCloseModalBtn = document.getElementById('close-modal-btn');

// Win Controls
const elMinimizeBtn = document.getElementById('minimize-btn');
const elCloseBtn = document.getElementById('close-btn');

// Sidebar
const navItems = document.querySelectorAll('.nav-item');

// Slideshow
let bgIndex = 0;
let bgInterval = null;

// News Carousel
let newsData = [];
let newsIndex = 0;
let newsInterval = null;

window.addEventListener('DOMContentLoaded', async () => {
    // Basic Listeners
    elSettingsBtn.onclick = () => elSettingsModal.style.display = 'flex';
    elCloseModalBtn.onclick = () => elSettingsModal.style.display = 'none';

    // Sidebar interaction
    navItems.forEach(item => {
        item.onclick = () => {
            navItems.forEach(n => n.classList.remove('active'));
            item.classList.add('active');
        };
    });

    // Language Radio Listeners (Instant Switch)
    document.querySelectorAll('input[name="lang"]').forEach(radio => {
        radio.addEventListener('change', (e) => {
            setLanguage(e.target.value);
            saveConfigOnly();
        });
    });

    // Close Behavior Radio Listeners (Instant Save)
    document.querySelectorAll('input[name="close"]').forEach(radio => {
        radio.addEventListener('change', () => {
            saveConfigOnly();
        });
    });

    // Logo click - open website
    const logoLink = document.getElementById('logo-link');
    if (logoLink) {
        logoLink.onclick = (e) => {
            e.preventDefault();
            window.__TAURI__.opener.openUrl('https://ganhrong.tech');
        };
    }

    bindWindowControls();

    // 1. Get Config
    currentConfig = await invoke('get_config').catch(console.error);
    applyConfig(currentConfig);
    setLanguage(currentConfig.language || 'en');

    // 2. Get Local Version
    localVersion = await invoke('get_local_version').catch(() => "0.0.0");

    // 3. Get Manifest
    fetchManifest();

    // Listeners
    listen('download-progress', (event) => {
        const payload = event.payload;
        updateProgress(payload.progress, payload.status);
    });

    listen('download-complete', () => {
        gameStatus = "ReadyToPlay";
        updateUI();
        invoke('get_local_version').then(v => localVersion = v);
    });

    listen('download-error', (event) => {
        alert("Error: " + event.payload);
        gameStatus = "UpdateAvailable";
        updateUI();
    });

    listen('manifest-updated', () => {
        fetchManifest(true);
    });

    elActionBtn.onclick = handleAction;
});

function bindWindowControls() {
    const appWindow = window.__TAURI__.window.getCurrentWindow();
    elMinimizeBtn.onclick = () => appWindow.minimize();
    elCloseBtn.onclick = async () => {
        // If MinimizeToTray is selected, hide window instead of closing
        if (currentConfig && currentConfig.close_behavior === 'MinimizeToTray') {
            appWindow.hide();
        } else {
            appWindow.close();
        }
    };
}

// Language Logic
function setLanguage(lang) {
    if (!STRINGS[lang]) lang = 'en';
    currentLang = lang;

    // Update Radio UI
    document.querySelectorAll(`input[name="lang"]`).forEach(r => r.checked = (r.value === lang));

    // Update UI Texts
    updateUI();
}

async function saveConfigOnly() {
    // Updates config in backend without closing modal or explicit save
    // Get current radio state for Close behavior too
    const closeRadios = document.getElementsByName('close');
    let close = 'MinimizeToTray';
    for (const r of closeRadios) { if (r.checked) close = r.value; }

    const newConfig = {
        language: currentLang,
        close_behavior: close,
        launch_at_startup: false
    };

    currentConfig = newConfig;
    await invoke('save_config', { config: newConfig });
}

// Slideshow Logic
function startSlideshow(backgrounds) {
    if (!backgrounds || backgrounds.length === 0) return;

    // Clear existing
    if (bgInterval) clearInterval(bgInterval);

    // Set initial
    bgIndex = 0;
    elBackground.style.backgroundImage = `url('${backgrounds[bgIndex]}')`;

    if (backgrounds.length > 1) {
        bgInterval = setInterval(() => {
            bgIndex = (bgIndex + 1) % backgrounds.length;
            elBackground.style.backgroundImage = `url('${backgrounds[bgIndex]}')`;
        }, 60000); // 60s
    }
}

async function fetchManifest(force = false) {
    elBtnText.innerText = STRINGS[currentLang].checking;
    try {
        latestManifest = await invoke('get_manifest', { forceRefresh: force });
        elOfflineBanner.style.display = 'none';

        // Backgrounds
        if (latestManifest.backgrounds && latestManifest.backgrounds.length > 0) {
            startSlideshow(latestManifest.backgrounds);
        } else if (latestManifest.background) {
            // Fallback for old manifest format just in case
            startSlideshow([latestManifest.background]);
        }

        renderNews(latestManifest.news);

        if (localVersion === "0.0.0") {
            gameStatus = "Missing";
        } else if (localVersion !== latestManifest.latest_version) {
            gameStatus = "UpdateAvailable";
        } else {
            gameStatus = "ReadyToPlay";
        }
    } catch (err) {
        if (err && err.toString().includes("Offline")) {
            gameStatus = "Offline";
            elOfflineBanner.style.display = 'block';
        } else {
            gameStatus = "Error";
            elBtnText.innerText = "Error";
            console.error(err);
        }
    }
    updateUI();
}

function updateUI() {
    elProgressInfo.style.display = 'none';
    elActionBtn.disabled = false;

    const txt = STRINGS[currentLang];

    if (gameStatus === "Offline") {
        elBtnText.innerText = txt.offline;
        elActionBtn.disabled = true;
    } else if (gameStatus === "Missing") {
        elBtnText.innerText = txt.download;
    } else if (gameStatus === "UpdateAvailable") {
        elBtnText.innerText = txt.update;
    } else if (gameStatus === "ReadyToPlay") {
        elBtnText.innerText = txt.play;
    } else if (gameStatus.startsWith("Downloading")) {
        elBtnText.innerText = "Installing..."; // Keep simple or localized
        elActionBtn.disabled = true;
        elProgressInfo.style.display = 'block';
    } else {
        elBtnText.innerText = txt.checking;
        elActionBtn.disabled = true;
    }
}

function updateProgress(progress, status) {
    gameStatus = "Downloading";
    elProgressInfo.style.display = 'block';
    elProgressText.innerText = status;
    elProgressPercent.innerText = Math.floor(progress) + "%";

    elBtnText.innerText = "Installing...";
    elActionBtn.disabled = true;
}

function renderNews(news) {
    newsData = news || [];
    if (newsData.length === 0) {
        elNewsSlides.innerHTML = '<div class="news-slide"><div style="padding:40px;text-align:center;color:#888">No news available</div></div>';
        elNewsTitle.innerText = '';
        elNewsDate.innerText = '';
        return;
    }

    // Create slides
    let slidesHtml = '';
    let dotsHtml = '';
    newsData.forEach((item, i) => {
        slidesHtml += `<div class="news-slide" data-index="${i}" data-link="${item.link || ''}"><img src="${item.image}" alt=""></div>`;
        dotsHtml += `<span class="dot ${i === 0 ? 'active' : ''}" data-index="${i}"></span>`;
    });

    elNewsSlides.innerHTML = slidesHtml;
    elNewsDots.innerHTML = dotsHtml;

    // Set initial
    newsIndex = 0;
    updateNewsDisplay();

    // Add click listeners to slides
    document.querySelectorAll('.news-slide').forEach(slide => {
        slide.onclick = () => {
            const link = slide.dataset.link;
            if (link) window.__TAURI__.opener.openUrl(link);
        };
    });

    // Add click listeners to dots
    document.querySelectorAll('.news-dots .dot').forEach(dot => {
        dot.onclick = () => {
            newsIndex = parseInt(dot.dataset.index);
            updateNewsDisplay();
            resetNewsInterval();
        };
    });

    // Navigation buttons
    elNewsPrev.onclick = () => {
        newsIndex = (newsIndex - 1 + newsData.length) % newsData.length;
        updateNewsDisplay();
        resetNewsInterval();
    };

    elNewsNext.onclick = () => {
        newsIndex = (newsIndex + 1) % newsData.length;
        updateNewsDisplay();
        resetNewsInterval();
    };

    // Start auto-slide
    startNewsInterval();
}

function updateNewsDisplay() {
    if (newsData.length === 0) return;

    // Move slides
    elNewsSlides.style.transform = `translateX(-${newsIndex * 100}%)`;

    // Update title and date
    const current = newsData[newsIndex];
    elNewsTitle.innerText = current.title;
    elNewsDate.innerText = current.date;

    // Update dots
    document.querySelectorAll('.news-dots .dot').forEach((dot, i) => {
        dot.classList.toggle('active', i === newsIndex);
    });
}

function startNewsInterval() {
    if (newsInterval) clearInterval(newsInterval);
    newsInterval = setInterval(() => {
        newsIndex = (newsIndex + 1) % newsData.length;
        updateNewsDisplay();
    }, 15000); // 15 seconds
}

function resetNewsInterval() {
    startNewsInterval();
}

async function handleAction() {
    if (gameStatus === "Missing" || gameStatus === "UpdateAvailable") {
        startDownload();
    } else if (gameStatus === "ReadyToPlay") {
        try {
            await invoke('launch_game');
        } catch (e) {
            alert(e);
        }
    }
}

async function startDownload() {
    try {
        await invoke('start_download');
    } catch (err) {
        alert("Failed to start download: " + err);
    }
}

async function saveSettings() {
    // Legacy save button, instant switch already saves language. 
    // This might be for "Close Behavior".
    saveConfigOnly();
    elSettingsModal.style.display = 'none';
}

function applyConfig(cfg) {
    if (cfg.language) {
        setLanguage(cfg.language);
    }
    if (cfg.close_behavior) {
        const r = document.querySelector(`input[name="close"][value="${cfg.close_behavior}"]`);
        if (r) r.checked = true;
    }
}
