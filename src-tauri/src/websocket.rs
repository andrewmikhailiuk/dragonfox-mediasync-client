use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
use tracing::{error, info, warn};
use url::Url;

use crate::protocol::{self, Message};
use crate::state::{AppState, ConnectionStatus};
use crate::tray;

const MAX_BACKOFF_MS: u64 = 30_000;
const INITIAL_BACKOFF_MS: u64 = 1_000;
const PING_INTERVAL_MS: u64 = 5_000;

pub async fn connect(
    app: AppHandle,
    state: Arc<AppState>,
    server_url: String,
    room: String,
) -> Result<(), String> {
    // Disconnect existing connection
    state.set_sender(None);

    state.set_server(Some(server_url.clone()));
    state.set_room(Some(room.clone()));
    state.set_status(ConnectionStatus::Connecting);
    emit_status(&app, &state);
    tray::update_icon(&app, &ConnectionStatus::Connecting);

    let ws_url = build_ws_url(&server_url, &room)?;

    // Create channel for outgoing messages
    let (tx, rx) = unbounded_channel::<Message>();
    state.set_sender(Some(tx.clone()));

    // Spawn connection task
    let app_clone = app.clone();
    let state_clone = state.clone();
    tokio::spawn(async move {
        run_connection_loop(app_clone, state_clone, ws_url, rx).await;
    });

    Ok(())
}

pub fn disconnect(app: &AppHandle, state: &AppState) {
    state.set_sender(None);
    state.set_room(None);
    state.set_server(None);
    state.set_status(ConnectionStatus::Disconnected);
    emit_status(app, state);
    tray::update_icon(app, &ConnectionStatus::Disconnected);
}

fn build_ws_url(server_url: &str, room: &str) -> Result<Url, String> {
    let base = if server_url.ends_with("/ws") {
        server_url.to_string()
    } else if server_url.ends_with('/') {
        format!("{}ws", server_url)
    } else {
        format!("{}/ws", server_url)
    };

    Url::parse(&format!("{}?room={}", base, room)).map_err(|e| e.to_string())
}

async fn run_connection_loop(
    app: AppHandle,
    state: Arc<AppState>,
    url: Url,
    mut rx: UnboundedReceiver<Message>,
) {
    let mut backoff_ms = INITIAL_BACKOFF_MS;
    let mut attempt = 0u32;

    loop {
        // Check if we should stop (sender was cleared = intentional disconnect)
        if state.ws_sender.read().is_none() {
            info!("Connection loop stopped: sender cleared");
            break;
        }

        info!("Connecting to {}", url);

        match connect_async(url.as_str()).await {
            Ok((ws_stream, _)) => {
                info!("Connected to WebSocket");
                backoff_ms = INITIAL_BACKOFF_MS;
                attempt = 0;

                let room = state.get_room().unwrap_or_default();
                state.set_status(ConnectionStatus::Connected {
                    room: room.clone(),
                    latency_ms: None,
                });
                emit_status(&app, &state);
                tray::update_icon(
                    &app,
                    &ConnectionStatus::Connected {
                        room,
                        latency_ms: None,
                    },
                );

                let (mut write, mut read) = ws_stream.split();

                // Ping interval
                let ping_state = state.clone();
                let ping_tx = {
                    let guard = state.ws_sender.read();
                    guard.clone()
                };

                let ping_handle = tokio::spawn(async move {
                    let mut interval = tokio::time::interval(Duration::from_millis(PING_INTERVAL_MS));
                    loop {
                        interval.tick().await;
                        if let Some(ref tx) = ping_tx {
                            let ping = Message::ping();
                            ping_state.set_last_ping(ping.timestamp);
                            if tx.send(ping).is_err() {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                });

                loop {
                    tokio::select! {
                        // Outgoing messages
                        Some(msg) = rx.recv() => {
                            let json = match serde_json::to_string(&msg) {
                                Ok(j) => j,
                                Err(e) => {
                                    error!("Failed to serialize message: {}", e);
                                    continue;
                                }
                            };
                            if let Err(e) = write.send(WsMessage::Text(json.into())).await {
                                error!("Failed to send message: {}", e);
                                break;
                            }
                        }
                        // Incoming messages
                        Some(result) = read.next() => {
                            match result {
                                Ok(WsMessage::Text(text)) => {
                                    handle_message(&app, &state, &text);
                                }
                                Ok(WsMessage::Close(_)) => {
                                    info!("WebSocket closed by server");
                                    break;
                                }
                                Err(e) => {
                                    error!("WebSocket error: {}", e);
                                    break;
                                }
                                _ => {}
                            }
                        }
                        else => {
                            break;
                        }
                    }

                    // Check if intentionally disconnected
                    if state.ws_sender.read().is_none() {
                        ping_handle.abort();
                        return;
                    }
                }

                ping_handle.abort();
            }
            Err(e) => {
                error!("Failed to connect: {}", e);
            }
        }

        // Check if intentionally disconnected
        if state.ws_sender.read().is_none() {
            break;
        }

        // Reconnect with backoff
        attempt += 1;
        state.set_status(ConnectionStatus::Reconnecting { attempt });
        emit_status(&app, &state);
        tray::update_icon(&app, &ConnectionStatus::Reconnecting { attempt });

        warn!("Reconnecting in {}ms (attempt {})", backoff_ms, attempt);
        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;

        backoff_ms = (backoff_ms * 2).min(MAX_BACKOFF_MS);

        // Recreate receiver channel
        let (tx, new_rx) = unbounded_channel::<Message>();
        rx = new_rx;
        state.set_sender(Some(tx));
    }
}

fn handle_message(app: &AppHandle, state: &AppState, text: &str) {
    let msg: Message = match serde_json::from_str(text) {
        Ok(m) => m,
        Err(e) => {
            warn!("Failed to parse message: {}", e);
            return;
        }
    };

    if msg.is_pong() {
        if let Some(ping_time) = state.take_last_ping() {
            let latency = (protocol::now_ms() - ping_time) as u32;
            if let ConnectionStatus::Connected { room, .. } = state.get_status() {
                state.set_status(ConnectionStatus::Connected {
                    room,
                    latency_ms: Some(latency),
                });
                emit_status(app, state);
            }
        }
        return;
    }

    // Emit event to frontend
    let _ = app.emit("sync-event", &msg);

    // Simulate media key if it's a playback control message
    if msg.is_toggle() || msg.is_play() || msg.is_pause() {
        crate::media::simulate_toggle(state);
    }
}

fn emit_status(app: &AppHandle, state: &AppState) {
    let _ = app.emit("connection-status", state.get_status());
}
