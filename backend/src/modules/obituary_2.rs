use axum::{extract::{State, Path}, Json};
use serde::{Deserialize, Serialize};
use crate::{core::AppState, error::{AppResult, AppError}, core::crypto::encrypt_field};
use uuid::Uuid;
use validator::Validate;
use chrono::NaiveDate;

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct CreateObituary {
    #[validate(length(min=1, max=200))]
    pub full_name: String,
    pub date_of_birth: NaiveDate,
    pub date_of_passing: NaiveDate,
    #[validate(length(min=10, max=20000))]
    pub biography: String,
    pub service_date: Option<chrono::DateTime<chrono::Utc>>,
    pub service_location: Option<String>,
    pub visibility: String, // public, private, unlisted
    pub photos: Option<Vec<String>>, // S3 keys
    pub video_tribute_url: Option<String>,
}

#[derive(Serialize)]
pub struct ObitRow { pub id: Uuid, pub data: serde_json::Value, pub created_at: chrono::DateTime<chrono::Utc> }

pub async fn create(State(s): State<AppState>, Json(mut payload): Json<CreateObituary>) -> AppResult<Json<serde_json::Value>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let id = Uuid::new_v4();
    
    // Encrypt biography at rest
    let encrypted_bio = encrypt_field(&payload.biography).map_err(|e| AppError::Internal(e.to_string()))?;
    let mut storable = payload.clone();
    storable.biography = encrypted_bio; // store encrypted
    
    let data = serde_json::to_value(&storable).unwrap();
    
    // Get home_id from JWT if available - for now create for demo home
    sqlx::query!("INSERT INTO obituaries (id, full_name, data, status) VALUES ($1,$2,$3,'draft')", id, payload.full_name, data)
        .execute(&s.db).await?;
    
    // Publish event for search indexing
    let _ = s.nats.publish("obituary.created".into(), format!("{{"id":"{}"}}", id).into()).await;
    
    Ok(Json(serde_json::json!({"id": id, "status":"draft"})))
}

pub async fn list(State(s): State<AppState>) -> AppResult<Json<Vec<serde_json::Value>>> {
    let rows = sqlx::query!("SELECT id, data, created_at FROM obituaries ORDER BY created_at DESC LIMIT 100")
        .fetch_all(&s.db).await?;
    let list: Vec<serde_json::Value> = rows.into_iter().map(|r| {
        serde_json::json!({"id": r.id, "data": r.data, "created_at": r.created_at})
    }).collect();
    Ok(Json(list))
}

pub async fn get(State(s): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<serde_json::Value>> {
    let row = sqlx::query!("SELECT id, data, status FROM obituaries WHERE id=$1", id)
        .fetch_optional(&s.db).await?.ok_or(AppError::NotFound("Obituary not found".into()))?;
    Ok(Json(serde_json::json!({"id": row.id, "data": row.data, "status": row.status})))
}

pub async fn update(State(s): State<AppState>, Path(id): Path<Uuid>, Json(payload): Json<CreateObituary>) -> AppResult<Json<serde_json::Value>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let encrypted = encrypt_field(&payload.biography).map_err(|e| AppError::Internal(e.to_string()))?;
    let mut storable = payload.clone();
    storable.biography = encrypted;
    let data = serde_json::to_value(&storable).unwrap();
    
    sqlx::query!("UPDATE obituaries SET full_name=$2, data=$3, updated_at=NOW() WHERE id=$1", id, payload.full_name, data)
        .execute(&s.db).await?;
    Ok(Json(serde_json::json!({"ok": true})))
}

pub async fn delete(State(s): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<serde_json::Value>> {
    sqlx::query!("DELETE FROM obituaries WHERE id=$1", id).execute(&s.db).await?;
    Ok(Json(serde_json::json!({"deleted": true})))
}

pub async fn publish(State(s): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<serde_json::Value>> {
    sqlx::query!("UPDATE obituaries SET status='published', published_at=NOW() WHERE id=$1", id).execute(&s.db).await?;
    let _ = s.nats.publish("obituary.published".into(), format!("{{"id":"{}"}}", id).into()).await;
    Ok(Json(serde_json::json!({"status":"published","public_url": format!("/memorial/{}", id)})))
}
