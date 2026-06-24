mod autostart;
mod commands;
mod config;
mod listeners;
mod tray;
mod windows;

use listeners::inject_link;
use tray::setup_tray;
use windows::main::setup_window;

pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(commands::handler())
        .on_page_load(|webview, _payload| {
            inject_link(webview);
        })
        .on_window_event(|window, event| {
            setup_window(window, event);
        })
        .setup(|app| {
            setup_tray(app)?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Error while building application");

    app.run(|_app_handle, _event| {});
}
