mod autostart;
mod commands;
mod config;
mod listeners;
mod tray;
mod windows;

use listeners::inject_link;
use tray::system::setup_tray;
use windows::main::setup_window_events;

pub fn run() {
    #[cfg(target_os = "linux")]
    unsafe {
        std::env::set_var("GDK_BACKEND", "x11");
    }

    let app = tauri::Builder::default()
        .plugin(
            tauri_plugin_autostart::Builder::new()
                .app_name("Twenty")
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(commands::handler())
        .on_window_event(|window, event| {
            setup_window_events(window, event);
        })
        .setup(|app| {
            setup_tray(app)?;
            Ok(())
        })
        .on_page_load(|window, _payload| {
            inject_link(window);
        })
        .build(tauri::generate_context!())
        .expect("Error while building application");

    app.run(|_app_handle, _event| {});
}
