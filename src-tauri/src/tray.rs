use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle, Manager,
};
use tracing::error;

use crate::state::ConnectionStatus;

const TRAY_ID: &str = "main-tray";

pub fn create(app: &AppHandle) -> Result<TrayIcon, tauri::Error> {
    let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let disconnect = MenuItem::with_id(app, "disconnect", "Disconnect", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show, &disconnect, &quit])?;

    let icon = load_icon("disconnected");

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .icon_as_template(true)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "disconnect" => {
                if let Some(state) = app.try_state::<std::sync::Arc<crate::state::AppState>>() {
                    crate::websocket::disconnect(app, state.inner());
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click { .. } = event {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)
}

pub fn update_icon(app: &AppHandle, status: &ConnectionStatus) {
    let icon_name = match status {
        ConnectionStatus::Disconnected => "disconnected",
        ConnectionStatus::Connecting => "connecting",
        ConnectionStatus::Connected { .. } => "connected",
        ConnectionStatus::Reconnecting { .. } => "connecting",
    };

    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        let icon = load_icon(icon_name);
        if let Err(e) = tray.set_icon(Some(icon)) {
            error!("Failed to set tray icon: {}", e);
        }
    }
}

fn load_icon(name: &str) -> Image<'static> {
    // Create a simple colored icon
    let color: [u8; 4] = match name {
        "connected" => [0, 200, 0, 255],
        "connecting" => [200, 200, 0, 255],
        _ => [128, 128, 128, 255],
    };

    let size = 16;
    let mut rgba = Vec::with_capacity(size * size * 4);
    for _ in 0..size * size {
        rgba.extend_from_slice(&color);
    }

    Image::new_owned(rgba, size as u32, size as u32)
}
