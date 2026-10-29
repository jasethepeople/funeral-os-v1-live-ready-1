
use axum::{extract::State, Json}; use crate::core::AppState;
#[derive(serde::Deserialize)] pub struct DeathCert { pub obituary_id: uuid::Uuid, pub state_code: String, pub data_encrypted: String }
pub async fn submit_certificate(State(s): State<AppState>, Json(p): Json<DeathCert>) -> Json<serde_json::Value> { sqlx::query!("INSERT INTO compliance_docs (id, obituary_id, state_code, status) VALUES ($1,$2,$3,'submitted')", uuid::Uuid::new_v4(), p.obituary_id, p.state_code).execute(&s.db).await.unwrap(); Json(serde_json::json!({"status": "submitted"})) }
