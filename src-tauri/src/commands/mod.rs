pub mod settings;

pub fn handler() -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        settings::get_url,
        settings::set_url,
        settings::open_external
    ]
}
