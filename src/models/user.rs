use crate::database::connection::DbPool;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "online_status")]
pub enum OnlineStatus {
    #[sqlx(rename = "online")]
    Online,
    #[sqlx(rename = "offline")]
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub username: String,
    pub email: String,
    pub status: OnlineStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub full_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub status: OnlineStatus,
}

impl User {
    pub async fn create(pool: &DbPool, user: CreateUser) -> Result<Self, sqlx::Error> {
        let now = Utc::now();
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, full_name, username, email, password, status, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) 
             RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(user.full_name)
        .bind(user.username)
        .bind(user.email)
        .bind(user.password)
        .bind(user.status)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(pool: &DbPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn find_all(pool: &DbPool) -> Result<Vec<Self>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(pool)
            .await?;

        Ok(users)
    }
}
