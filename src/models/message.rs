use crate::database::connection::DbPool;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub room_id: Uuid,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessage {
    pub room_id: Uuid,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub content: String,
}

impl Message {
    pub async fn create(pool: &DbPool, message: CreateMessage) -> Result<Self, sqlx::Error> {
        let now = Utc::now();
        let message = sqlx::query_as::<_, Message>(
            "INSERT INTO messages (id, room_id, sender_id, recipient_id, content, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) 
             RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(message.room_id)
        .bind(message.sender_id)
        .bind(message.recipient_id)
        .bind(message.content)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await?;

        Ok(message)
    }

    pub async fn find_by_id(pool: &DbPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        let message = sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(message)
    }

    pub async fn find_all(pool: &DbPool) -> Result<Vec<Self>, sqlx::Error> {
        let messages =
            sqlx::query_as::<_, Message>("SELECT * FROM messages ORDER BY created_at DESC")
                .fetch_all(pool)
                .await?;

        Ok(messages)
    }

    pub async fn find_by_room_id(pool: &DbPool, room_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        let messages = sqlx::query_as::<_, Message>(
            "SELECT * FROM messages WHERE room_id = $1 ORDER BY created_at DESC",
        )
        .bind(room_id)
        .fetch_all(pool)
        .await?;

        Ok(messages)
    }

    pub async fn find_by_recipient_id(
        pool: &DbPool,
        recipient_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let messages = sqlx::query_as::<_, Message>(
            "SELECT * FROM messages WHERE recipient_id = $1 ORDER BY created_at DESC",
        )
        .bind(recipient_id)
        .fetch_all(pool)
        .await?;

        Ok(messages)
    }
}
