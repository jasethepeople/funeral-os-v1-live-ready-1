
use axum::{extract::{State, Path}, Json}; use crate::core::AppState; use uuid::Uuid;
pub async fn list(State(s): State<AppState>, Path(obit_id): Path<Uuid>) -> Json<Vec<serde_json::Value>> { let rows = sqlx::query!("SELECT data FROM guestbook WHERE obituary_id=$1 ORDER BY created_at", obit_id).fetch_all(&s.db).await.unwrap(); Json(rows.into_iter().map(|r| r.data).collect()) }
pub async fn sign(State(s): State<AppState>, Path(obit_id): Path<Uuid>, Json(entry): Json<serde_json::Value>) -> Json<serde_json::Value> { let id = Uuid::new_v4(); sqlx::query!("INSERT INTO guestbook (id, obituary_id, data) VALUES ($1,$2,$3)", id, obit_id, entry).execute(&s.db).await.unwrap(); Json(serde_json::json!({"id": id, "moderation": "pending"})) }
