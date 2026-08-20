// Global keyboard shortcuts for LE.GO.LAS Desktop.
//
// These shortcuts work even when the app window is in the background or minimized.
//
// Registered shortcuts:
//   Ctrl+Shift+N  — bring window to front and navigate to new task creation
//   Ctrl+Shift+F  — bring window to front and open global search

use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

pub fn register_shortcuts(app: &AppHandle) -> tauri::Result<()> {
    use tauri_plugin_global_shortcut::Builder;

    app.plugin(
        Builder::new()
            .with_handler({
                let app = app.clone();
                move |_app, shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }

                    let mods = shortcut.mods;
                    let required = Modifiers::CONTROL | Modifiers::SHIFT;

                    if mods != required {
                        return;
                    }

                    match shortcut.key {
                        Code::KeyN => {
                            // Ctrl+Shift+N → open window + trigger new task modal
                            bring_window_to_front(&app);
                            navigate(&app, "window.__legolas_openNewTask?.()");
                        }
                        Code::KeyF => {
                            // Ctrl+Shift+F → open window + focus global search
                            bring_window_to_front(&app);
                            navigate(&app, "window.__legolas_openSearch?.()");
                        }
                        _ => {}
                    }
                }
            })
            .build(),
    )?;

    Ok(())
}

fn bring_window_to_front(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.unminimize();
        let _ = win.set_focus();
    }
}

fn navigate(app: &AppHandle, js: &str) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.eval(js);
    }
}
