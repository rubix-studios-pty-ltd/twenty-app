use tauri::{
    AppHandle, Manager, Wry,
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

use crate::autostart::{is_autostart_enabled, toggle_autostart};
use crate::tray::{check_updates, exit_app, reset_instance};
use crate::windows::main::{toggle_main_window, window_tray_label};

pub struct TrayMenuState {
    open_item: MenuItem<Wry>,
}

pub fn sync_tray_label(app: &AppHandle) {
    let label = window_tray_label(app);

    if let Some(state) = app.try_state::<TrayMenuState>() {
        let _ = state.open_item.set_text(label);
    }
}

pub fn setup_tray(app: &tauri::App) -> tauri::Result<()> {
    let window_status = window_tray_label(app.handle());
    let open = MenuItem::with_id(app, "open", window_status, true, None::<&str>)?;

    app.manage(TrayMenuState {
        open_item: open.clone(),
    });

    let autostart_enabled = is_autostart_enabled();
    let autostart = CheckMenuItem::with_id(
        app,
        "autostart",
        "Startup",
        true,
        autostart_enabled,
        None::<&str>,
    )?;

    let reset_instance_item = MenuItem::with_id(app, "reset_instance", "Reset", true, None::<&str>)?;
    let updates = MenuItem::with_id(app, "check_updates", "Update", true, None::<&str>)?;
    let exit = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;

    let separator = PredefinedMenuItem::separator(app)?;

    let settings_menu = Submenu::with_id_and_items(
        app,
        "settings",
        "Settings",
        true,
        &[&autostart, &separator, &reset_instance_item],
    )?;

    let menu = Menu::with_items(
        app,
        &[
            &open,
            &separator,
            &settings_menu,
            &separator,
            &updates,
            &exit,
        ],
    )?;

    let Some(icon) = app.default_window_icon().cloned() else {
        return Ok(());
    };

    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .tooltip("Twenty")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| {
            match event.id.as_ref() {
            "open" => {
                toggle_main_window(app);
                sync_tray_label(app);
            }
            "autostart" => {
                if let Err(error) = toggle_autostart() {
                    eprintln!("Failed to toggle autostart: {error}");
                } else if let Some(item) = app.menu().and_then(|menu| menu.get("autostart"))
                    && let Some(check_item) = item.as_check_menuitem()
                {
                    let enabled = is_autostart_enabled();
                    let _ = check_item.set_checked(enabled);
                }
            }
            "check_updates" => {
                check_updates(app);
            }
            "reset_instance" => {
                reset_instance(app);
            }
            "exit" => {
                exit_app(app);
            }
            _ => {}
        }
    })
    .on_tray_icon_event(move |tray, event| {
        if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();

                toggle_main_window(app);
                sync_tray_label(app);
            }
        })
        .build(app)?;

    Ok(())
}