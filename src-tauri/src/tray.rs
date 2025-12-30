use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime,
};

pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let reload_i = MenuItem::with_id(app, "reload", "Reload", true, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    
    let menu = Menu::with_items(app, &[&reload_i, &show_i, &quit_i])?;

    // Load custom icon from logo.png
    let icon = Image::from_path("icons/logo.png").unwrap_or_else(|_| {
        // Fallback to default icon
        Image::from_path("icons/icon.png").unwrap_or_else(|_| {
            Image::from_bytes(include_bytes!("../icons/icon.png")).expect("Failed to load icon")
        })
    });

    let _tray = TrayIconBuilder::with_id("main")
        .icon(icon)
        .menu(&menu)
        .tooltip("Gánh Rồng Launcher")
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "reload" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("location.reload()");
                    }
                }
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            match event {
                TrayIconEvent::Click { .. } => {
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
