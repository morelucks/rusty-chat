use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, errors::Result as JwtResult};
use serde::{Deserialize, Serialize};

const SECRET: &[u8] = b"your_secret_key_here"; // Replace with env/config in production

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: &str, expires_in_minutes: i64) -> JwtResult<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(expires_in_minutes))
        .expect("valid timestamp")
        .timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
}

pub fn decode_jwt(token: &str) -> JwtResult<Claims> {
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &Validation::default())
        .map(|data| data.claims)
} 