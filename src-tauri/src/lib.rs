// LE.GO.LAS Desktop — Tauri application entry point (library crate).
//
// Architecture:
//   - eimzo    : Native WebSocket bridge to local E-IMZO daemon (replaces eimzo-proxy.js)
//   - tray     : System tray icon with context menu (minimize-to-tray behaviour)
//   - shortcuts: Global keyboard shortcuts (Ctrl+Shift+N, Ctrl+Shift+F)

mod eimzo;
mod shortcuts;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // ── Plugins ───────────────────────────────────────────────────────
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        // ── Commands (invocable from JS via invoke()) ──────────────────────
        .invoke_handler(tauri::generate_handler![
            eimzo::eimzo_send,
            eimzo::eimzo_check,
        ])
        // ── App setup ─────────────────────────────────────────────────────
        .setup(|app| {
            // System tray
            tray::setup_tray(&app.handle())?;

            // Global shortcuts — registered after tray so window handle is ready
            shortcuts::register_shortcuts(&app.handle())?;

            Ok(())
        })
        // ── Window close → minimize to tray ───────────────────────────────
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Hide instead of closing; user can quit from tray menu
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running LE.GO.LAS desktop application");
}
