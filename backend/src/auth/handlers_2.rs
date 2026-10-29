use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::{core::AppState, error::{AppResult, AppError}};
use uuid::Uuid;
use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::{rand_core::OsRng, SaltString}};
use crate::auth::jwt::create_token;

#[derive(Deserialize)]
pub struct RegisterHome {
    pub home_name: String,
    pub owner_email: String,
    pub password: String,
    pub phone: String,
    pub address: String,
}

#[derive(Deserialize)]
pub struct Login { pub email: String, pub password: String }

#[derive(Serialize)]
pub struct AuthResponse { pub token: String, pub home_id: Uuid, pub user_id: Uuid }

pub async fn register_home(State(s): State<AppState>, Json(p): Json<RegisterHome>) -> AppResult<Json<AuthResponse>> {
    if p.password.len() < 12 {
        return Err(AppError::Validation("Password must be 12+ chars".into()));
    }
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(p.password.as_bytes(), &salt).map_err(|_| AppError::Internal("hash failed".into()))?.to_string();
    
    let home_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    
    // Create home + owner in transaction
    let mut tx = s.db.begin().await?;
    sqlx::query!("INSERT INTO funeral_homes (id, name, email, phone, address) VALUES ($1,$2,$3,$4,$5)", home_id, p.home_name, p.owner_email, p.phone, p.address)
        .execute(&mut *tx).await?;
    sqlx::query!("INSERT INTO users (id, home_id, email, password_hash, role) VALUES ($1,$2,$3,$4,'owner')", user_id, home_id, p.owner_email, hash)
        .execute(&mut *tx).await?;
    tx.commit().await?;
    
    let token = create_token(user_id, home_id, "owner".into()).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(AuthResponse { token, home_id, user_id }))
}

pub async fn login(State(s): State<AppState>, Json(p): Json<Login>) -> AppResult<Json<AuthResponse>> {
    let row = sqlx::query!("SELECT id, home_id, password_hash, role FROM users WHERE email=$1", p.email)
        .fetch_optional(&s.db).await?.ok_or(AppError::Unauthorized)?;
    
    let parsed = argon2::PasswordHash::new(&row.password_hash).map_err(|_| AppError::Internal("hash parse".into()))?;
    Argon2::default().verify_password(p.password.as_bytes(), &parsed).map_err(|_| AppError::Unauthorized)?;
    
    let token = create_token(row.id, row.home_id, row.role.clone()).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(AuthResponse { token, home_id: row.home_id, user_id: row.id }))
}
