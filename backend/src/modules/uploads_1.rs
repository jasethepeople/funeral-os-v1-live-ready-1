use axum::{extract::{State, Multipart}, Json};
use crate::{core::AppState, error::{AppResult, AppError}};
use uuid::Uuid;
use aws_config::BehaviorVersion;

pub async fn upload_photo(State(s): State<AppState>, mut multipart: Multipart) -> AppResult<Json<serde_json::Value>> {
    let bucket = std::env::var("S3_BUCKET").unwrap_or_else(|_| "funeral-os-uploads".into());
    let region = std::env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".into());
    
    // Load AWS config - real S3, works with AWS, Cloudflare R2, MinIO
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3 = aws_sdk_s3::Client::new(&config);

    let mut uploaded_keys = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::Internal(e.to_string()))? {
        let filename = field.file_name().unwrap_or("upload.jpg").to_string();
        let content_type = field.content_type().map(|c| c.to_string()).unwrap_or("image/jpeg".into());
        let data = field.bytes().await.map_err(|e| AppError::Internal(e.to_string()))?;

        if data.len() > 20 * 1024 * 1024 {
            return Err(AppError::Validation("File max 20MB".into()));
        }

        let ext = filename.split('.').last().unwrap_or("jpg");
        let key = format!("obituaries/{}/{}.{}", Uuid::new_v4(), Uuid::new_v4(), ext);

        s3.put_object()
            .bucket(&bucket)
            .key(&key)
            .body(data.to_vec().into())
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("S3 upload failed: {} - set AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, S3_BUCKET", e)))?;

        uploaded_keys.push(serde_json::json!({
            "key": key,
            "url": format!("https://{}.s3.{}.amazonaws.com/{}", bucket, region, key),
            "original_name": filename
        }));
    }

    if uploaded_keys.is_empty() {
        return Err(AppError::Validation("No files uploaded".into()));
    }

    Ok(Json(serde_json::json!({"uploaded": uploaded_keys})))
}

pub async fn presign_upload(State(_s): State<AppState>, Json(p): Json<serde_json::Value>) -> AppResult<Json<serde_json::Value>> {
    let bucket = std::env::var("S3_BUCKET").unwrap_or_else(|_| "funeral-os-uploads".into());
    let key = format!("obituaries/{}/{}", Uuid::new_v4(), p.get("filename").and_then(|v| v.as_str()).unwrap_or("photo.jpg"));
    
    // Real presigned URL - client uploads directly to S3
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3 = aws_sdk_s3::Client::new(&config);
    let presigned = s3.put_object()
        .bucket(&bucket)
        .key(&key)
        .presigned(aws_sdk_s3::presigning::PresigningConfig::expires_in(std::time::Duration::from_secs(3600)).unwrap())
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({"url": presigned.uri().to_string(), "key": key, "method":"PUT"})))
}
