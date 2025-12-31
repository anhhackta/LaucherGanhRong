use tauri::{
    image::Image,
    menu::{Menu, MenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, Runtime,
};

pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    // Main menu items
    let toggle_i = MenuItem::with_id(app, "toggle", "Show/Hide Window", true, None::<&str>)?;
    let reload_i = MenuItem::with_id(app, "reload", "Reload", true, None::<&str>)?;
    
    // Language submenu
    let lang_vi = MenuItem::with_id(app, "lang_vi", "Tiếng Việt", true, None::<&str>)?;
    let lang_en = MenuItem::with_id(app, "lang_en", "English", true, None::<&str>)?;
    let lang_jp = MenuItem::with_id(app, "lang_jp", "日本語", true, None::<&str>)?;
    let lang_zh = MenuItem::with_id(app, "lang_zh", "中文", true, None::<&str>)?;
    let lang_submenu = Submenu::with_items(app, "Language", true, &[&lang_vi, &lang_en, &lang_jp, &lang_zh])?;
    
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    
    let menu = Menu::with_items(app, &[&toggle_i, &reload_i, &lang_submenu, &quit_i])?;

    // Embed icon directly into binary
    let icon = Image::from_bytes(include_bytes!("../icons/icon.png"))
        .expect("Failed to load embedded icon");

    let _tray = TrayIconBuilder::with_id("main")
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip("Ganh Rong Launcher")
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "toggle" => {
                    if let Some(window) = app.get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
                "reload" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("location.reload()");
                    }
                }
                "lang_vi" | "lang_en" | "lang_jp" | "lang_zh" => {
                    let lang = event.id.as_ref().replace("lang_", "");
                    // Emit to frontend to change language
                    let _ = app.emit("change-language", lang);
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            match event {
                TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } => {
                    let app = tray.app_handle();
                    if let Some(window) = app.get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}
