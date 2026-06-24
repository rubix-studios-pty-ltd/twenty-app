use crate::config::{load_config, save_config};

#[tauri::command]
pub fn get_url(app_handle: tauri::AppHandle) -> Option<String> {
    let config = load_config(&app_handle);
    config.instance_url
}

#[tauri::command]
pub fn set_url(app_handle: tauri::AppHandle, url: String) -> Result<(), String> {
    let mut config = load_config(&app_handle);
    config.instance_url = Some(url);

    save_config(&app_handle, &config)
}

#[tauri::command]
pub fn open_external(url: String) -> Result<(), String> {
    let parsed = url::Url::parse(&url).map_err(|e| e.to_string())?;

    match parsed.scheme() {
        "http" | "https" | "tel" => {
            tauri_plugin_opener::open_url(parsed.as_str(), None::<&str>)
                .map_err(|e| e.to_string())
        }
        scheme => Err(format!("Blocked unsupported URL scheme: {scheme}")),
    }
}