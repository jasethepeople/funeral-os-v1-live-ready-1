use axum::{extract::{State, Path}, Json};
use crate::{core::AppState, error::{AppResult, AppError}};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FamilyMember {
    pub full_name: String,
    pub relationship: String, // spouse, child, parent, sibling, etc
    pub birth_date: Option<chrono::NaiveDate>,
    pub is_deceased: bool,
    pub parent_id: Option<Uuid>,
    pub obituary_id: Option<Uuid>,
}

pub async fn get_tree(State(s): State<AppState>, Path(root_id): Path<Uuid>) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query!("SELECT id, root_id, data FROM family_nodes WHERE root_id=$1 ORDER BY created_at", root_id)
        .fetch_all(&s.db).await?;
    Ok(Json(serde_json::json!({
        "root_id": root_id,
        "members": rows.into_iter().map(|r| serde_json::json!({"id": r.id, "data": r.data})).collect::<Vec<_>>(),
        "count": rows.len()
    })))
}

pub async fn add_member(State(s): State<AppState>, Path(root_id): Path<Uuid>, Json(member): Json<FamilyMember>) -> AppResult<Json<serde_json::Value>> {
    let id = Uuid::new_v4();
    let data = serde_json::to_value(&member).unwrap();
    sqlx::query!("INSERT INTO family_nodes (id, root_id, data) VALUES ($1,$2,$3)", id, root_id, data)
        .execute(&s.db).await?;
    Ok(Json(serde_json::json!({"id": id, "root_id": root_id})))
}
