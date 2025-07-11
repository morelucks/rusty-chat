use actix::prelude::*;
use std::collections::{HashMap, HashSet};

type SessionAddr = Addr<super::ws_session::ChatSession>;

pub struct ChatServer {
    rooms: HashMap<String, HashSet<SessionAddr>>,
}

impl ChatServer {
    pub fn new() -> Self {
        Self { rooms: HashMap::new() }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

pub struct JoinRoom {
    pub room_id: String,
    pub addr: SessionAddr,
}
impl Message for JoinRoom {
    type Result = ();
}

impl Handler<JoinRoom> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) {
        self.rooms.entry(msg.room_id)
            .or_default()
            .insert(msg.addr);
    }
}

pub struct BroadcastMessage {
    pub room_id: String,
    pub message: String,
}
impl Message for BroadcastMessage {
    type Result = ();
}

impl Handler<BroadcastMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Context<Self>) {
        if let Some(sessions) = self.rooms.get(&msg.room_id) {
            for session in sessions {
                session.do_send(super::ws_session::ServerMessage(msg.message.clone()));
            }
        }
    }
}
