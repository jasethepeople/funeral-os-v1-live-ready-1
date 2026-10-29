
use axum::{extract::{State, Path}, Json}; use crate::core::AppState; use uuid::Uuid;
pub async fn get_tree(State(s): State<AppState>, Path(id): Path<Uuid>) -> Json<serde_json::Value> { let rows = sqlx::query!("SELECT data FROM family_nodes WHERE root_id=$1", id).fetch_all(&s.db).await.unwrap(); Json(serde_json::json!({"root": id, "nodes": rows.into_iter().map(|r| r.data).collect::<Vec<_>>()})) }
