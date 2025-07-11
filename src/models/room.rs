use crate::database::connection::DbPool;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub created_by: Uuid,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoom {
    pub name: String,
    pub created_by: Uuid,
    pub is_private: bool,
}

impl Room {
    pub async fn create(pool: &DbPool, room: CreateRoom) -> Result<Self, sqlx::Error> {
        let now = Utc::now();
        let room = sqlx::query_as::<_, Room>(
            "INSERT INTO rooms (id, name, created_by, is_private, created_at) 
             VALUES ($1, $2, $3, $4, $5) 
             RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(room.name)
        .bind(room.created_by)
        .bind(room.is_private)
        .bind(now)
        .fetch_one(pool)
        .await?;

        Ok(room)
    }

    pub async fn find_by_id(pool: &DbPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        let room = sqlx::query_as::<_, Room>("SELECT * FROM rooms WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(room)
    }

    pub async fn find_all(pool: &DbPool) -> Result<Vec<Self>, sqlx::Error> {
        let rooms = sqlx::query_as::<_, Room>("SELECT * FROM rooms ORDER BY created_at DESC")
            .fetch_all(pool)
            .await?;

        Ok(rooms)
    }
}
