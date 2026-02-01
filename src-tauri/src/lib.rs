mod commands;
mod hotkey;
mod media;
mod protocol;
mod state;
mod tray;
mod websocket;

use std::sync::Arc;

use state::AppState;
use tracing_subscriber::EnvFilter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("media_sync=debug".parse().unwrap()),
        )
        .init();

    let app_state = Arc::new(AppState::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(app_state)
        .setup(|app| {
            // Create system tray
            tray::create(app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::connect,
            commands::disconnect,
            commands::get_connection_status,
            commands::send_toggle,
            commands::send_ping,
            hotkey::set_hotkey,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
