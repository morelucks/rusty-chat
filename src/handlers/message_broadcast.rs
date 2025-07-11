use crate::database::connection::DbPool;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Internal struct for broadcasting messages to all users in a room except the sender
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastMessage {
    pub room_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub message_type: BroadcastMessageType,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BroadcastMessageType {
    Text,
    Typing,
    Read,
    Join,
    Leave,
    System,
}

/// Room connection manager for handling websocket connections
#[derive(Debug, Clone)]
pub struct RoomConnectionManager {
    connections: Arc<RwLock<HashMap<Uuid, HashMap<Uuid, broadcast::Sender<String>>>>>,
}

impl RoomConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a user to a room's connection pool
    pub async fn add_user_to_room(
        &self,
        room_id: Uuid,
        user_id: Uuid,
    ) -> broadcast::Receiver<String> {
        let mut connections = self.connections.write().await;

        let room_connections = connections.entry(room_id).or_insert_with(HashMap::new);
        let (tx, rx) = broadcast::channel(100); // Buffer size of 100 messages
        room_connections.insert(user_id, tx);

        rx
    }

    /// Remove a user from a room's connection pool
    pub async fn remove_user_from_room(&self, room_id: Uuid, user_id: Uuid) {
        let mut connections = self.connections.write().await;

        if let Some(room_connections) = connections.get_mut(&room_id) {
            room_connections.remove(&user_id);

            // Remove room if no users left
            if room_connections.is_empty() {
                connections.remove(&room_id);
            }
        }
    }

    /// Broadcast message to all users in a room except the sender
    pub async fn broadcast_to_room(
        &self,
        room_id: Uuid,
        sender_id: Uuid,
        message: &BroadcastMessage,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let connections = self.connections.read().await;

        if let Some(room_connections) = connections.get(&room_id) {
            let message_json = serde_json::to_string(&message)?;

            for (user_id, sender) in room_connections.iter() {
                // Skip the sender
                if *user_id != sender_id {
                    if let Err(e) = sender.send(message_json.clone()) {
                        tracing::warn!("Failed to send message to user {}: {}", user_id, e);
                    }
                }
            }
        }

        Ok(())
    }

    /// Get all users in a room
    pub async fn get_room_users(&self, room_id: Uuid) -> Vec<Uuid> {
        let connections = self.connections.read().await;

        if let Some(room_connections) = connections.get(&room_id) {
            room_connections.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Check if a user is in a room
    pub async fn is_user_in_room(&self, room_id: Uuid, user_id: Uuid) -> bool {
        let connections = self.connections.read().await;

        if let Some(room_connections) = connections.get(&room_id) {
            room_connections.contains_key(&user_id)
        } else {
            false
        }
    }
}

/// Broadcast service for handling message broadcasting operations
pub struct BroadcastService {
    pool: DbPool,
    connection_manager: RoomConnectionManager,
}

impl BroadcastService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            pool,
            connection_manager: RoomConnectionManager::new(),
        }
    }

    /// Create and broadcast a text message
    pub async fn broadcast_text_message(
        &self,
        room_id: Uuid,
        sender_id: Uuid,
        content: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = BroadcastMessage {
            room_id,
            sender_id,
            content,
            message_type: BroadcastMessageType::Text,
            timestamp: Utc::now(),
        };

        self.connection_manager
            .broadcast_to_room(room_id, sender_id, &message)
            .await
    }

    /// Broadcast typing indicator
    pub async fn broadcast_typing(
        &self,
        room_id: Uuid,
        sender_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = BroadcastMessage {
            room_id,
            sender_id,
            content: "typing".to_string(),
            message_type: BroadcastMessageType::Typing,
            timestamp: Utc::now(),
        };

        self.connection_manager
            .broadcast_to_room(room_id, sender_id, &message)
            .await
    }

    /// Broadcast read receipt
    pub async fn broadcast_read_receipt(
        &self,
        room_id: Uuid,
        sender_id: Uuid,
        message_id: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = BroadcastMessage {
            room_id,
            sender_id,
            content: message_id,
            message_type: BroadcastMessageType::Read,
            timestamp: Utc::now(),
        };

        self.connection_manager
            .broadcast_to_room(room_id, sender_id, &message)
            .await
    }

    /// Broadcast user join event
    pub async fn broadcast_user_join(
        &self,
        room_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = BroadcastMessage {
            room_id,
            sender_id: user_id,
            content: "joined".to_string(),
            message_type: BroadcastMessageType::Join,
            timestamp: Utc::now(),
        };

        self.connection_manager
            .broadcast_to_room(room_id, user_id, &message)
            .await
    }

    /// Broadcast user leave event
    pub async fn broadcast_user_leave(
        &self,
        room_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = BroadcastMessage {
            room_id,
            sender_id: user_id,
            content: "left".to_string(),
            message_type: BroadcastMessageType::Leave,
            timestamp: Utc::now(),
        };

        self.connection_manager
            .broadcast_to_room(room_id, user_id, &message)
            .await
    }

    /// Broadcast system message
    pub async fn broadcast_system_message(
        &self,
        room_id: Uuid,
        content: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = BroadcastMessage {
            room_id,
            sender_id: Uuid::nil(), // System messages have nil sender
            content,
            message_type: BroadcastMessageType::System,
            timestamp: Utc::now(),
        };

        // System messages are sent to all users including the "sender" (which is nil)
        self.connection_manager
            .broadcast_to_room(room_id, Uuid::nil(), &message)
            .await
    }

    /// Add user to room and broadcast join event
    pub async fn add_user_to_room(
        &self,
        room_id: Uuid,
        user_id: Uuid,
    ) -> Result<broadcast::Receiver<String>, Box<dyn std::error::Error + Send + Sync>> {
        let receiver = self
            .connection_manager
            .add_user_to_room(room_id, user_id)
            .await;

        // Broadcast join event to other users
        self.broadcast_user_join(room_id, user_id).await?;

        Ok(receiver)
    }

    /// Remove user from room and broadcast leave event
    pub async fn remove_user_from_room(
        &self,
        room_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Broadcast leave event to other users first
        self.broadcast_user_leave(room_id, user_id).await?;

        // Remove from connection manager
        self.connection_manager
            .remove_user_from_room(room_id, user_id)
            .await;

        Ok(())
    }

    /// Get connection manager reference
    pub fn get_connection_manager(&self) -> &RoomConnectionManager {
        &self.connection_manager
    }
}

/// Helper struct for creating broadcast messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBroadcastMessage {
    pub room_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub message_type: BroadcastMessageType,
}

impl CreateBroadcastMessage {
    pub fn new(
        room_id: Uuid,
        sender_id: Uuid,
        content: String,
        message_type: BroadcastMessageType,
    ) -> Self {
        Self {
            room_id,
            sender_id,
            content,
            message_type,
        }
    }

    pub fn text(room_id: Uuid, sender_id: Uuid, content: String) -> Self {
        Self::new(room_id, sender_id, content, BroadcastMessageType::Text)
    }

    pub fn typing(room_id: Uuid, sender_id: Uuid) -> Self {
        Self::new(
            room_id,
            sender_id,
            "typing".to_string(),
            BroadcastMessageType::Typing,
        )
    }

    pub fn read(room_id: Uuid, sender_id: Uuid, message_id: String) -> Self {
        Self::new(room_id, sender_id, message_id, BroadcastMessageType::Read)
    }

    pub fn join(room_id: Uuid, user_id: Uuid) -> Self {
        Self::new(
            room_id,
            user_id,
            "joined".to_string(),
            BroadcastMessageType::Join,
        )
    }

    pub fn leave(room_id: Uuid, user_id: Uuid) -> Self {
        Self::new(
            room_id,
            user_id,
            "left".to_string(),
            BroadcastMessageType::Leave,
        )
    }

    pub fn system(room_id: Uuid, content: String) -> Self {
        Self::new(room_id, Uuid::nil(), content, BroadcastMessageType::System)
    }
}
