Game Launcher Walkthrough
=========================

I have successfully created a minimal, production-ready single-game launcher using Tauri + Rust.

Architecture
------------

1.  **Backend (Rust)**
    
    *   state.rs: Managed application state (Config, Manifest, Status).
        
    *   manifest.rs: Fetches manifest.json from R2 (or cache if offline).
        
    *   downloader.rs: Handles downloads, SHA256 verification, and atomic extraction. **Ensures game/version.txt is created.**
        
    *   game.rs: Checks local version vs manifest and launches the game.
        
    *   config.rs: Persists user settings (config.json).
        
    *   tray.rs: Handles system tray interactions.
        
2.  **Frontend (Vanilla JS)**
    
    *   Lightweight index.html and main.js.
        
    *   Glassmorphism UI with dark theme.
        
    *   Real-time stats updates via Tauri Events (download-progress, download-complete).
        
    *   Offline mode support.
        

Verification
------------

### Build

Run the following command to build the production executable:

Plain textANTLR4BashCC#CSSCoffeeScriptCMakeDartDjangoDockerEJSErlangGitGoGraphQLGroovyHTMLJavaJavaScriptJSONJSXKotlinLaTeXLessLuaMakefileMarkdownMATLABMarkupObjective-CPerlPHPPowerShell.propertiesProtocol BuffersPythonRRubySass (Sass)Sass (Scss)SchemeSQLShellSwiftSVGTSXTypeScriptWebAssemblyYAMLXML`   npm run tauri build   `

This generates the installer in src-tauri/target/release/bundle/.

### Manual Testing Guide

1.  **Startup**:
    
    *   Launcher opens.
        
    *   Checks internet.
        
    *   Loads manifest from URL (or cache).
        
    *   Shows "Install Game" if game missing.
        
2.  **Download Flow**:
    
    *   Click "Install".
        
    *   Progress bar appears.
        
    *   Game downloaded to cache/game.tmp.zip.
        
    *   Verified against checksum.
        
    *   Extracted to game/.
        
    *   version.txt created.
        
    *   Button changes to "Play".
        
3.  **Features**:
    
    *   **Manifest-Driven**: Configures game version, download URL, **background slideshow**, and news.
        
    *   **R2 Integration**: All assets hosted on Cloudflare R2.
        
    *   **Atomic Updates**: Safe update process (download -> verify -> extract -> swap).
        
    *   **Frameless UI**: Custom window with rounded corners and slideshow background.
        
    *   **Instant Localization**: Support for English, Vietnamese, and Japanese.
        
4.  **Offline Mode**:
    
    *   Disconnect internet.
        
    *   Restart Launcher.
        
    *   Shows "Offline Mode" banner.
        
    *   "Play" button disabled (per strict rules).
        
5.  **Settings**:
    
    *   Change "Close Behavior" to "Minimize to Tray".
        
    *   Close window -> App stays in tray.
        
    *   Right-click Tray -> Show/Quit.
        

Configuration
-------------

Update MANIFEST\_URL in 

src-tauri/src/lib.rs to point to your Cloudflare R2 manifest.json.
