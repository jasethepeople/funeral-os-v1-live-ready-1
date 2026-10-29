use axum::{extract::{State, Path}, Json};
use crate::{core::AppState, error::{AppResult, AppError}};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GuestbookEntry {
    pub author_name: String,
    pub author_email: Option<String>,
    pub message: String,
    pub relationship: Option<String>,
    pub photo_url: Option<String>,
    pub video_message_url: Option<String>,
}

pub async fn list(State(s): State<AppState>, Path(obit_id): Path<Uuid>) -> AppResult<Json<Vec<serde_json::Value>>> {
    let rows = sqlx::query!("SELECT id, data, created_at, status FROM guestbook WHERE obituary_id=$1 AND status != 'hidden' ORDER BY created_at ASC", obit_id)
        .fetch_all(&s.db).await?;
    Ok(Json(rows.into_iter().map(|r| serde_json::json!({"id": r.id, "data": r.data, "created_at": r.created_at, "status": r.status})).collect()))
}

pub async fn sign(State(s): State<AppState>, Path(obit_id): Path<Uuid>, Json(entry): Json<GuestbookEntry>) -> AppResult<Json<serde_json::Value>> {
    if entry.message.len() < 2 || entry.message.len() > 5000 {
        return Err(AppError::Validation("Message 2-5000 chars".into()));
    }
    if entry.author_name.len() < 2 {
        return Err(AppError::Validation("Name required".into()));
    }
    let id = Uuid::new_v4();
    
    // Simple toxicity filter - real prod uses Rust crate `profanity-check` or external API
    let blocked = ["spam", "scam"]; // placeholder list, real would be extensive
    let lower = entry.message.to_lowercase();
    let needs_review = blocked.iter().any(|w| lower.contains(w));
    let status = if needs_review { "pending_review" } else { "approved" };
    
    let data = serde_json::to_value(&entry).unwrap();
    sqlx::query!("INSERT INTO guestbook (id, obituary_id, data, status) VALUES ($1,$2,$3,$4)", id, obit_id, data, status)
        .execute(&s.db).await?;
    
    // Notify funeral home via NATS
    let _ = s.nats.publish("guestbook.signed".into(), format!("{{"obituary_id":"{}","entry_id":"{}"}}", obit_id, id).into()).await;
    
    Ok(Json(serde_json::json!({"id": id, "status": status})))
}

pub async fn moderate(State(s): State<AppState>, Path(id): Path<Uuid>, Json(p): Json<serde_json::Value>) -> AppResult<Json<serde_json::Value>> {
    let status = p.get("status").and_then(|v| v.as_str()).unwrap_or("approved");
    if !["approved","hidden","pending_review"].contains(&status) {
        return Err(AppError::Validation("Invalid status".into()));
    }
    sqlx::query!("UPDATE guestbook SET status=$2 WHERE id=$1", id, status).execute(&s.db).await?;
    Ok(Json(serde_json::json!({"id": id, "status": status})))
}
