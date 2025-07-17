use actix::{Actor, Addr, AsyncContext, Context, Handler, StreamHandler};
use actix_web_actors::ws;
use serde::Deserialize;
use crate::utils::types::{ClientMessage, ServerMessage};

pub struct ChatSession {
    pub user_id: String,
    pub room_id: String,
    pub server_addr: Addr<super::ws_server::ChatServer>,
}

impl ChatSession {
    pub fn new(user_id: String, room_id: String, server_addr: Addr<super::ws_server::ChatServer>) -> Self {
        Self {
            user_id,
            room_id,
            server_addr,
        }
    }

    fn send_server_message(&self, msg: ServerMessage, ctx: &mut ws::WebsocketContext<Self>) {
        if let Ok(json) = serde_json::to_string(&msg) {
            ctx.text(json);
        }
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.server_addr.do_send(super::ws_server::JoinRoom {
            user_id: self.user_id.clone(),
            room_id: self.room_id.clone(),
            addr: ctx.address(),
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.server_addr.do_send(super::ws_server::LeaveRoom {
            user_id: self.user_id.clone(),
        });
    }
}

// Accepts a raw JSON string (from ws_server.rs) and sends it to the client
pub struct OutgoingRawMessage(pub String);
impl actix::Message for OutgoingRawMessage {
    type Result = ();
}

impl Handler<OutgoingRawMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: OutgoingRawMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                match client_msg {
                    ClientMessage::Text { content } => {
                        self.server_addr.do_send(super::ws_server::BroadcastMessage {
                            room_id: self.room_id.clone(),
                            message: content,
                            sender_id: self.user_id.clone(),
                        });
                    },
                    ClientMessage::Private { to, content } => {
                        self.server_addr.do_send(super::ws_server::PrivateMessage {
                            to,
                            from: self.user_id.clone(),
                            content,
                        });
                    },
                    _ => {}
                }
            }
        }
    }
}

#[derive(Deserialize)]
struct IncomingMessage {
    #[serde(rename = "type")]
    msg_type: String,
    content: String,
    room_id: Option<String>,
}
