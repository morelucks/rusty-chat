use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::utils::types::ServerMessage;
use crate::ws_session::OutgoingRawMessage;

type SessionAddr = Addr<super::ws_session::ChatSession>;

pub struct ChatServer {
    rooms: HashMap<String, HashSet<SessionAddr>>,
    user_sessions: HashMap<String, SessionAddr>,
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            user_sessions: HashMap::new(),
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

pub struct JoinRoom {
    pub user_id: String,
    pub room_id: String,
    pub addr: SessionAddr,
}
impl Message for JoinRoom {
    type Result = ();
}

pub struct LeaveRoom {
    pub user_id: String,
}
impl Message for LeaveRoom {
    type Result = ();
}

impl Handler<JoinRoom> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) {
        self.rooms.entry(msg.room_id).or_default().insert(msg.addr.clone());
        self.user_sessions.insert(msg.user_id, msg.addr);
    }
}

impl Handler<LeaveRoom> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: LeaveRoom, _: &mut Context<Self>) {
        self.user_sessions.remove(&msg.user_id);
    }
}

pub struct BroadcastMessage {
    pub room_id: String,
    pub message: String,
    pub sender_id: String,
}
impl Message for BroadcastMessage {
    type Result = ();
}

impl Handler<BroadcastMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Context<Self>) {
        if let Some(sessions) = self.rooms.get(&msg.room_id) {
            for session in sessions {
                session.do_send(OutgoingRawMessage(
                    serde_json::to_string(&ServerMessage::Text {
                        content: msg.message.clone(),
                        user_id: msg.sender_id.clone(),
                    }).unwrap()
                ));
            }
        }
    }
}

pub struct PrivateMessage {
    pub to: String,
    pub from: String,
    pub content: String,
}
impl Message for PrivateMessage {
    type Result = ();
}

impl Handler<PrivateMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: PrivateMessage, _: &mut Context<Self>) {
        if let Some(session) = self.user_sessions.get(&msg.to) {
            session.do_send(OutgoingRawMessage(
                serde_json::to_string(&ServerMessage::Private {
                    from: msg.from,
                    content: msg.content,
                }).unwrap()
            ));
        }
    }
}
