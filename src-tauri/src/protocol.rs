use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    pub timestamp: i64,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

impl Message {
    pub fn toggle() -> Self {
        Self {
            msg_type: "toggle".into(),
            position: None,
            timestamp: now_ms(),
            client_id: None,
        }
    }

    pub fn ping() -> Self {
        Self {
            msg_type: "ping".into(),
            position: None,
            timestamp: now_ms(),
            client_id: None,
        }
    }

    pub fn is_toggle(&self) -> bool {
        self.msg_type == "toggle"
    }

    pub fn is_play(&self) -> bool {
        self.msg_type == "play"
    }

    pub fn is_pause(&self) -> bool {
        self.msg_type == "pause"
    }

    pub fn is_pong(&self) -> bool {
        self.msg_type == "pong"
    }
}

pub fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
