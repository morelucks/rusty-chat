use uuid::Uuid;
use actix::{prelude::*, Addr};
use std::collections::{HashMap, HashSet};

type SessionAddr = Addr<super::ws_session::ChatSession>;

pub struct ChatServer {
    pub sessions: HashMap<Uuid, SessionAddr>,
    rooms: HashMap<String, HashSet<SessionAddr>>,
}
pub struct RegisterSession {
    pub user_id: Uuid,
    pub addr: SessionAddr,
}
impl Message for RegisterSession {
    type Result = ();
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
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
        self.rooms.entry(msg.room_id).or_default().insert(msg.addr);
    }
}

impl Handler<RegisterSession> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: RegisterSession, _: &mut Context<Self>) {
        self.sessions.insert(msg.user_id, msg.addr);
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

pub struct SendDirectMessage {
    pub from: Uuid,
    pub to: Uuid,
    pub content: String,
}
impl Message for SendDirectMessage {
    type Result = ();
}

impl Handler<SendDirectMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: SendDirectMessage, _: &mut Context<Self>) {
        if let Some(addr) = self.sessions.get(&msg.to) {
            addr.do_send(super::ws_session::ServerMessage(format!("From {}: {}", msg.from, msg.content)));
        }
        
        if let Some(addr) = self.sessions.get(&msg.from) {
            addr.do_send(super::ws_session::ServerMessage(format!("To {}: {}", msg.to, msg.content)));
        }
    }
}
