use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid, // user_id
    pub home_id: Uuid, // funeral_home_id
    pub role: String,
    pub exp: i64,
}

pub fn create_token(user_id: Uuid, home_id: Uuid, role: String) -> anyhow::Result<String> {
    let exp = (Utc::now() + Duration::hours(12)).timestamp();
    let claims = Claims { sub: user_id, home_id, role, exp };
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-in-prod-32-chars-min".into());
    Ok(encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))?)
}

pub fn verify_token(token: &str) -> anyhow::Result<Claims> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-in-prod-32-chars-min".into());
    let data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())?;
    Ok(data.claims)
}
