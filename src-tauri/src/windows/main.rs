use tauri::WindowEvent;

pub fn setup_window(window: &tauri::Window, event: &WindowEvent) {
    if window.label() != "main" {
        return;
    }

    if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();

        let _ = window.hide();
    }
}
