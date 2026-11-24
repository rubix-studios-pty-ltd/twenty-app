use ksni::TrayMethods;
use tauri::Manager;

use crate::autostart::{is_autostart_enabled, toggle_autostart};
use crate::tray::{check_updates, exit_app, reset_instance};
use crate::windows::main::{toggle_main_window, window_tray_label};

pub struct TwentyTray {
    app: tauri::AppHandle,
    label: String,
    startup_enabled: bool,
}

struct TrayHandle(ksni::Handle<TwentyTray>);

impl ksni::Tray for TwentyTray {
    fn id(&self) -> String {
        "twenty_app".to_string()
    }

    fn title(&self) -> String {
        "twenty".to_string()
    }

    fn icon_name(&self) -> String {
        "twenty_app".to_string()
    }

    fn category(&self) -> ksni::Category {
        ksni::Category::ApplicationStatus
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        toggle_window(&self.app);
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;

        vec![
            StandardItem {
                label: self.label.clone(),
                activate: Box::new(|tray: &mut Self| {
                    toggle_window(&tray.app);
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            SubMenu {
                label: "Settings".into(),
                submenu: vec![
                    CheckmarkItem {
                        label: "Startup".into(),
                        checked: self.startup_enabled,
                        activate: Box::new(|tray: &mut Self| {
                            toggle_startup(&tray.app);
                        }),
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: "Reset".into(),
                        activate: Box::new(|tray: &mut Self| {
                            dispatch(&tray.app, |app| {
                                reset_instance(app);
                            });
                        }),
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Update".into(),
                activate: Box::new(|tray: &mut Self| {
                    dispatch(&tray.app, |app| {
                        check_updates(app);
                    });
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Exit".into(),
                activate: Box::new(|tray: &mut Self| {
                    let app = tray.app.clone();

                    std::thread::spawn(move || {
                        exit_app(&app);
                    });
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

pub fn setup_tray(app: &tauri::App) -> tauri::Result<()> {
    let tray = TwentyTray {
        app: app.handle().clone(),
        label: window_tray_label(app.handle()).into(),
        startup_enabled: is_autostart_enabled(app.handle()),
    };

    match tauri::async_runtime::block_on(tray.disable_dbus_name(true).spawn()) {
        Ok(handle) => {
            app.manage(TrayHandle(handle));
        }
        Err(error) => {
            eprintln!("Failed to spawn Linux ksni tray: {error}");
        }
    }

    Ok(())
}

pub fn sync_tray_label(app: &tauri::AppHandle) {
    dispatch(app, |app| {
        let label: String = window_tray_label(app).into();

        update_tray(app, move |tray| {
            tray.label = label;
        });
    });
}

fn toggle_window(app: &tauri::AppHandle) {
    dispatch(app, |app| {
        toggle_main_window(app);

        sync_tray_label(app);
    });
}

fn toggle_startup(app: &tauri::AppHandle) {
    dispatch(app, |app| {
        if let Err(error) = toggle_autostart(app) {
            eprintln!("Failed to toggle autostart: {error}");
        }

        let enabled = is_autostart_enabled(app);

        update_tray(app, move |tray| {
            tray.startup_enabled = enabled;
        });
    });
}

fn update_tray<F>(app: &tauri::AppHandle, update: F)
where
    F: FnOnce(&mut TwentyTray) + Send + 'static,
{
    let Some(state) = app.try_state::<TrayHandle>() else {
        return;
    };

    let handle = state.0.clone();

    tauri::async_runtime::spawn(async move {
        handle.update(update).await;
    });
}

fn dispatch<F>(app: &tauri::AppHandle, action: F)
where
    F: FnOnce(&tauri::AppHandle) + Send + 'static,
{
    let app_handle = app.clone();

    if let Err(error) = app.run_on_main_thread(move || {
        action(&app_handle);
    }) {
        eprintln!("Failed to dispatch tray action: {error}");
    }
}
