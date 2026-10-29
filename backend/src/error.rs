use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation: {0}")]
    Validation(String),
    #[error("Internal: {0}")]
    Internal(String),
}
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self { Self::Internal(e.to_string()) }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            Self::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            Self::Validation(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            Self::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".into()),
        };
        (status, Json(json!({"error": msg}))).into_response()
    }
}
pub type AppResult<T> = Result<T, AppError>;
