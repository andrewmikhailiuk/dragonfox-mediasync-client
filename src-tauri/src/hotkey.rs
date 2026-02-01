use std::sync::Arc;

use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use tracing::{error, info};

use crate::media;
use crate::protocol::Message;
use crate::state::AppState;

pub fn register(app: &AppHandle, state: Arc<AppState>, shortcut_str: &str) -> Result<(), String> {
    let shortcut: Shortcut = shortcut_str.parse().map_err(|e| format!("{:?}", e))?;

    let app_clone = app.clone();
    let state_clone = state.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            // Only handle key press, not release
            if event.state != ShortcutState::Pressed {
                return;
            }

            info!("Global hotkey triggered");

            // Send toggle to server
            if let Err(e) = state_clone.send_message(Message::toggle()) {
                error!("Failed to send toggle: {}", e);
            }

            // Simulate media key locally so player responds
            media::simulate_toggle(&state_clone);

            // Emit to frontend
            let _ = app_clone.emit("sync-event", Message::toggle());
        })
        .map_err(|e| e.to_string())
}

pub fn unregister_all(app: &AppHandle) -> Result<(), String> {
    app.global_shortcut()
        .unregister_all()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_hotkey(
    app: AppHandle,
    state: tauri::State<'_, Arc<AppState>>,
    shortcut: String,
) -> Result<(), String> {
    unregister_all(&app)?;
    if !shortcut.is_empty() {
        register(&app, state.inner().clone(), &shortcut)?;
    }
    Ok(())
}
