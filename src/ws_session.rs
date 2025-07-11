use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler, Context};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

pub struct ChatSession {
    pub room_id: String,
    pub server_addr: Addr<super::ws_server::ChatServer>,
}

impl ChatSession {
    pub fn new(room_id: String, server_addr: Addr<super::ws_server::ChatServer>) -> Self {
        Self { room_id, server_addr }
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.server_addr.do_send(super::ws_server::JoinRoom {
            room_id: self.room_id.clone(),
            addr: ctx.address(),
        });
    }
}

pub struct ServerMessage(pub String);
impl Message for ServerMessage {
    type Result = ();
}

impl Handler<ServerMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            // Expect JSON: { "type": "message", "content": "...", "room_id": "..." }
            if let Ok(incoming) = serde_json::from_str::<IncomingMessage>(&text) {
                if incoming.msg_type == "message" {
                    self.server_addr.do_send(super::ws_server::BroadcastMessage {
                        room_id: self.room_id.clone(),
                        message: incoming.content,
                    });
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