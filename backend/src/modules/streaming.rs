use axum::{extract::{State, Path}, Json};
use crate::{core::AppState, error::{AppResult, AppError}};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose::STANDARD as B64};
use reqwest::Client;

#[derive(Deserialize)]
pub struct CreateStream { pub obituary_id: Uuid, pub title: String, pub scheduled_at: Option<chrono::DateTime<chrono::Utc>> }

#[derive(Serialize)]
pub struct StreamResponse {
    pub stream_id: Uuid,
    pub obituary_id: Uuid,
    pub mux_live_stream_id: Option<String>,
    pub rtmp_url: String,
    pub stream_key: String,
    pub playback_id: String,
    pub playback_url: String,
    pub status: String,
}

pub async fn create_stream(State(s): State<AppState>, Json(p): Json<CreateStream>) -> AppResult<Json<StreamResponse>> {
    let stream_id = Uuid::new_v4();
    let mux_token_id = std::env::var("MUX_TOKEN_ID").ok();
    let mux_secret = std::env::var("MUX_TOKEN_SECRET").ok();

    let (mux_id, playback_id, stream_key) = if let (Some(token_id), Some(secret)) = (mux_token_id, mux_secret) {
        // REAL MUX API CALL - production
        let client = Client::new();
        let auth = B64.encode(format!("{}:{}", token_id, secret));
        
        let body = serde_json::json!({
            "playback_policy": ["public"],
            "new_asset_settings": {"playback_policy": ["public"]},
            "reconnect_window": 60
        });

        let resp = client.post("https://api.mux.com/video/v1/live-streams")
            .header("Authorization", format!("Basic {}", auth))
            .json(&body)
            .send().await.map_err(|e| AppError::Internal(e.to_string()))?;

        if !resp.status().is_success() {
            let txt = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!("Mux error: {}", txt)));
        }

        let json: serde_json::Value = resp.json().await.map_err(|e| AppError::Internal(e.to_string()))?;
        let data = json.get("data").cloned().unwrap_or_default();
        let mid = data.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let pb = data.get("playback_ids").and_then(|v| v.as_array()).and_then(|a| a.get(0)).and_then(|o| o.get("id")).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let sk = data.get("stream_key").and_then(|v| v.as_str()).unwrap_or("").to_string();
        (mid, pb, sk)
    } else {
        // Fallback deterministic for dev without Mux creds - still real structure
        let pb = format!("m_{}", &stream_id.to_string()[..8]);
        let sk = format!("live_{}", &blake3::hash(stream_id.as_bytes()).to_hex()[..24]);
        ("dev_mode".into(), pb, sk)
    };

    sqlx::query!("INSERT INTO streams (id, obituary_id, playback_id, stream_key, title, status, mux_id) VALUES ($1,$2,$3,$4,$5,'ready',$6)",
        stream_id, p.obituary_id, playback_id, stream_key, p.title, mux_id)
        .execute(&s.db).await?;

    Ok(Json(StreamResponse {
        stream_id,
        obituary_id: p.obituary_id,
        mux_live_stream_id: Some(mux_id),
        rtmp_url: "rtmps://global-live.mux.com:443/app".into(),
        stream_key,
        playback_id: playback_id.clone(),
        playback_url: format!("https://stream.mux.com/{}.m3u8", playback_id),
        status: "ready".into(),
    }))
}

pub async fn get_stream(State(s): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<serde_json::Value>> {
    let row = sqlx::query!("SELECT id, obituary_id, playback_id, stream_key, status, mux_id FROM streams WHERE id=$1", id)
        .fetch_optional(&s.db).await?.ok_or(AppError::NotFound("Stream not found".into()))?;
    Ok(Json(serde_json::json!({
        "id": row.id,
        "obituary_id": row.obituary_id,
        "playback_id": row.playback_id,
        "playback_url": format!("https://stream.mux.com/{}.m3u8", row.playback_id),
        "rtmp_url": "rtmps://global-live.mux.com:443/app",
        "status": row.status,
        "mux_id": row.mux_id
    })))
}

pub async fn end_stream(State(s): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<serde_json::Value>> {
    sqlx::query!("UPDATE streams SET status='ended', ended_at=NOW() WHERE id=$1", id).execute(&s.db).await?;
    Ok(Json(serde_json::json!({"status":"ended"})))
}
