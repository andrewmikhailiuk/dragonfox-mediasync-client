use parking_lot::RwLock;
use serde::Serialize;
use tokio::sync::mpsc::UnboundedSender;

use crate::protocol::Message;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected {
        room: String,
        #[serde(rename = "latencyMs")]
        latency_ms: Option<u32>,
    },
    Reconnecting {
        attempt: u32,
    },
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self::Disconnected
    }
}

pub struct AppState {
    pub ws_sender: RwLock<Option<UnboundedSender<Message>>>,
    pub connection_status: RwLock<ConnectionStatus>,
    pub current_room: RwLock<Option<String>>,
    pub current_server: RwLock<Option<String>>,
    pub last_ping_time: RwLock<Option<i64>>,
    pub simulate_cooldown_until: RwLock<i64>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ws_sender: RwLock::new(None),
            connection_status: RwLock::new(ConnectionStatus::Disconnected),
            current_room: RwLock::new(None),
            current_server: RwLock::new(None),
            last_ping_time: RwLock::new(None),
            simulate_cooldown_until: RwLock::new(0),
        }
    }

    pub fn set_status(&self, status: ConnectionStatus) {
        *self.connection_status.write() = status;
    }

    pub fn get_status(&self) -> ConnectionStatus {
        self.connection_status.read().clone()
    }

    pub fn set_sender(&self, sender: Option<UnboundedSender<Message>>) {
        *self.ws_sender.write() = sender;
    }

    pub fn send_message(&self, msg: Message) -> Result<(), String> {
        let guard = self.ws_sender.read();
        if let Some(sender) = guard.as_ref() {
            sender.send(msg).map_err(|e| e.to_string())
        } else {
            Err("Not connected".into())
        }
    }

    pub fn set_room(&self, room: Option<String>) {
        *self.current_room.write() = room;
    }

    pub fn set_server(&self, server: Option<String>) {
        *self.current_server.write() = server;
    }

    pub fn get_room(&self) -> Option<String> {
        self.current_room.read().clone()
    }

    pub fn set_last_ping(&self, time: i64) {
        *self.last_ping_time.write() = Some(time);
    }

    pub fn take_last_ping(&self) -> Option<i64> {
        self.last_ping_time.write().take()
    }

    pub fn set_cooldown(&self, until: i64) {
        *self.simulate_cooldown_until.write() = until;
    }
}
