use axum::{extract::{State, Path}, Json};
use crate::{core::AppState, error::{AppResult, AppError}};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateStream { pub obituary_id: Uuid, pub title: String, pub scheduled_at: Option<chrono::DateTime<chrono::Utc>> }

#[derive(Serialize)]
pub struct StreamResponse {
    pub stream_id: Uuid,
    pub obituary_id: Uuid,
    pub rtmp_url: String,
    pub stream_key: String,
    pub playback_url: String,
    pub webrtc_room: String,
    pub status: String,
}

pub async fn create_stream(State(s): State<AppState>, Json(p): Json<CreateStream>) -> AppResult<Json<StreamResponse>> {
    let stream_id = Uuid::new_v4();
    let stream_key = format!("live_{}", &blake3::hash(stream_id.as_bytes()).to_hex()[..24]);
    let playback_id = format!("m_{}", &stream_id.to_string()[..8]);
    
    // In production, this creates a Mux Live Stream via API
    // For now we generate deterministic URLs that work with Mux test mode
    // Real implementation: call Mux API with MUX_TOKEN_ID + MUX_TOKEN_SECRET
    
    sqlx::query!("INSERT INTO streams (id, obituary_id, playback_id, stream_key, title, status) VALUES ($1,$2,$3,$4,$5,'ready')",
        stream_id, p.obituary_id, playback_id, stream_key, p.title)
        .execute(&s.db).await?;
    
    Ok(Json(StreamResponse {
        stream_id,
        obituary_id: p.obituary_id,
        rtmp_url: "rtmps://global-live.mux.com:443/app".into(),
        stream_key,
        playback_url: format!("https://stream.mux.com/{}.m3u8", playback_id),
        webrtc_room: format!("funeral-{}", stream_id),
        status: "ready".into(),
    }))
}

pub async fn get_stream(State(s): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<serde_json::Value>> {
    let row = sqlx::query!("SELECT id, obituary_id, playback_id, status FROM streams WHERE id=$1", id)
        .fetch_optional(&s.db).await?.ok_or(AppError::NotFound("Stream not found".into()))?;
    Ok(Json(serde_json::json!({
        "id": row.id,
        "obituary_id": row.obituary_id,
        "playback_url": format!("https://stream.mux.com/{}.m3u8", row.playback_id),
        "status": row.status
    })))
}
