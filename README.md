# Gánh Rồng Launcher

Minimal, high-performance single-game desktop launcher built with **Tauri + Rust**.

## Features

- **Manifest-Driven**: Dynamic configuration fetched from Cloudflare R2
- **Background Slideshow**: Rotating backgrounds (every 60s)
- **Atomic Updates**: Safe `download → verify → extract → swap` process
- **Frameless UI**: Modern, rounded-corner window with custom controls
- **Multi-Language**: Vietnamese, English, Japanese, Chinese
- **Instant Language Switch**: No restart required
- **System Tray**: Minimize to tray / Exit options
- **Offline Detection**: Graceful handling when offline

## Requirements

- Node.js 18+
- Rust 1.70+
- Windows 10/11

## Setup

```bash
# Install dependencies
npm install

# Run in development
npm run tauri dev

# Build for production
npm run tauri build
```

## Folder Structure

```
LaucherGanhRong/
├── src/                    # Frontend (HTML/CSS/JS)
│   ├── index.html
│   ├── styles.css
│   └── main.js
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── lib.rs          # Main entry, commands
│   │   ├── config.rs       # Settings persistence
│   │   ├── manifest.rs     # Manifest fetch/cache
│   │   ├── downloader.rs   # Download & install logic
│   │   ├── game.rs         # Version check, launch
│   │   └── tray.rs         # System tray
│   └── icons/
│       └── logo.png        # App logo
├── cache/                  # Temporary download files
├── game/                   # Installed game files
├── manifest.json           # Sample manifest for R2
└── README.md
```

## Cloudflare R2 Setup

1. Create an R2 bucket in Cloudflare dashboard
2. Enable **Public Access** on the bucket
3. Upload files:
   - `manifest.json` (from project root)
   - `game-v1.0.0.zip` (your game archive)
   - Background images (`bg1.jpg`, `bg2.jpg`, etc.)
   - News images

4. Get public URLs and update `manifest.json`:
   ```json
   {
     "game_zip": "https://pub-XXXXX.r2.dev/game/game-v1.0.0.zip",
     "backgrounds": ["https://pub-XXXXX.r2.dev/assets/bg1.jpg"],
     ...
   }
   ```

5. Update `MANIFEST_URL` in `src-tauri/src/lib.rs`:
   ```rust
   const MANIFEST_URL: &str = "https://pub-XXXXX.r2.dev/manifest.json";
   ```

## Manifest Format

```json
{
  "game_name": "Gánh Rồng",
  "latest_version": "1.0.0",
  "game_zip": "https://...",
  "checksum": "sha256:HASH_OF_ZIP",
  "server_status": "online",
  "maintenance_message": "",
  "backgrounds": ["https://...", "https://..."],
  "news": [
    {
      "title": "Update 1.0",
      "image": "https://...",
      "date": "2024.12.30",
      "link": "https://yoursite.com/news/1"
    }
  ],
  "languages": ["vi", "en", "jp", "zh"]
}
```

### Server Status Values

| Value | Description |
|-------|-------------|
| `online` | Game is playable |
| `maintenance` | Server maintenance, show message |
| `closed` | Game is closed/offline |

## Game ZIP Naming Convention

Name your game zip file using this format:

```
{game-name}-v{version}.zip
```

**Examples:**
- `ganhrong-v1.0.0.zip`
- `ganhrong-v1.0.1.zip`
- `ganhrong-v2.0.0.zip`

**Rules:**
1. Use lowercase letters
2. Use hyphens `-` instead of spaces
3. Prefix version with `v`
4. Version format: `major.minor.patch` (e.g., `1.0.0`)

**Folder Structure Inside ZIP:**
```
ganhrong-v1.0.0.zip
└── game.exe           (required - main executable)
└── assets/            (game assets)
└── data/              (game data)
└── ...                (other files)
```

> **Important:** The `version.txt` file is created automatically by the launcher after successful installation. Do NOT include it in your zip.

## Button Logic

| Condition | Button |
|-----------|--------|
| `game/` folder missing | **Download** |
| Local version ≠ Manifest version | **Update** |
| Version matches | **Play** |
| Offline | Disabled |

## Performance Targets

- Idle RAM: < 80MB
- Startup: < 1 second
- Binary size: < 15MB

## Generate Checksum

```bash
# Windows PowerShell
Get-FileHash game.zip -Algorithm SHA256

# Linux/Mac
sha256sum game.zip
```

## License

MIT
