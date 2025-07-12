use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    Text { content: String },
    Typing { user_id: String },
    Read { message_id: String },
    Join { room_id: String, user_id: String },
    Leave { room_id: String, user_id: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    Text { content: String, user_id: String },
    Typing { user_id: String },
    Read { message_id: String, user_id: String },
    Join { room_id: String, user_id: String },
    Leave { room_id: String, user_id: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
