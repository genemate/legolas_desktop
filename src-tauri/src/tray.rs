// System tray setup for LE.GO.LAS Desktop.
//
// The tray icon provides quick access without keeping the main window open.
// Closing the main window hides it (minimize to tray) rather than quitting.

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

pub fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "open", "Открыть LE.GO.LAS", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Выйти из приложения", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&open, &separator, &quit])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("LE.GO.LAS — Корпоративная платформа")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event({
            let app = app.clone();
            move |_tray, event| match event.id.as_ref() {
                "open" => {
                    if let Some(win) = app.get_webview_window("main") {
                        let _ = win.show();
                        let _ = win.unminimize();
                        let _ = win.set_focus();
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event({
            let app = app.clone();
            move |_tray, event| {
                // Double-click left button → bring window to front
                if let TrayIconEvent::DoubleClick {
                    button: MouseButton::Left,
                    ..
                } = event
                {
                    if let Some(win) = app.get_webview_window("main") {
                        let _ = win.show();
                        let _ = win.unminimize();
                        let _ = win.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
