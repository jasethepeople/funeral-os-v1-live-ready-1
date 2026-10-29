use axum::{extract::State, Json};
use crate::core::AppState;

pub async fn stats(State(s): State<AppState>) -> Json<serde_json::Value> {
    let obits = sqlx::query!("SELECT COUNT(*) as count FROM obituaries").fetch_one(&s.db).await.unwrap();
    let streams = sqlx::query!("SELECT COUNT(*) as count FROM streams").fetch_one(&s.db).await.unwrap();
    let guestbook = sqlx::query!("SELECT COUNT(*) as count FROM guestbook").fetch_one(&s.db).await.unwrap();
    let pending = sqlx::query!("SELECT COUNT(*) as count FROM guestbook WHERE status='pending_review'").fetch_one(&s.db).await.unwrap();
    
    Json(serde_json::json!({
        "obituaries": obits.count,
        "streams": streams.count,
        "guestbook_entries": guestbook.count,
        "pending_moderation": pending.count,
        "mrr_estimate": obits.count.unwrap_or(0) * 49 // $49/mo per active memorial
    }))
}
