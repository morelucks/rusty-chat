use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub full_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub full_name: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub username: String,
    pub exp: i64, // expiration time
    pub iat: i64, // issued at
}

impl Claims {
    pub fn new(user_id: Uuid, username: String) -> Self {
        let now = Utc::now().timestamp();
        Self {
            sub: user_id.to_string(),
            username,
            exp: now + (24 * 60 * 60), // 24 hours
            iat: now,
        }
    }
} 