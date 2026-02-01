use std::sync::Arc;

use tauri::{AppHandle, State};

use crate::protocol::Message;
use crate::state::{AppState, ConnectionStatus};
use crate::websocket;

#[tauri::command]
pub async fn connect(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    server_url: String,
    room: String,
) -> Result<(), String> {
    websocket::connect(app, state.inner().clone(), server_url, room).await
}

#[tauri::command]
pub fn disconnect(app: AppHandle, state: State<'_, Arc<AppState>>) {
    websocket::disconnect(&app, state.inner());
}

#[tauri::command]
pub fn get_connection_status(state: State<'_, Arc<AppState>>) -> ConnectionStatus {
    state.get_status()
}

#[tauri::command]
pub fn send_toggle(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    state.send_message(Message::toggle())
}

#[tauri::command]
pub fn send_ping(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    state.send_message(Message::ping())
}
