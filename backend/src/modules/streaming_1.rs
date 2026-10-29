
use axum::{extract::State, Json}; use crate::core::AppState; use uuid::Uuid;
#[derive(serde::Deserialize)] pub struct CreateStream { pub obituary_id: Uuid, pub title: String }
pub async fn create_stream(State(s): State<AppState>, Json(p): Json<CreateStream>) -> Json<serde_json::Value> {
    let stream_id = Uuid::new_v4(); let playback_id = format!("play_{}", &stream_id.to_string()[..8]);
    sqlx::query!("INSERT INTO streams (id, obituary_id, playback_id) VALUES ($1,$2,$3)", stream_id, p.obituary_id, playback_id).execute(&s.db).await.unwrap();
    Json(serde_json::json!({"stream_id": stream_id, "rtmp_url": "rtmps://global-live.mux.com:443/app", "playback_url": format!("https://stream.mux.com/{}.m3u8", playback_id)}))
}
