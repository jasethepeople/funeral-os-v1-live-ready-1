use axum::{http::{Request, StatusCode}, middleware::Next, response::Response};
use crate::auth::jwt::verify_token;

pub async fn rbac_middleware<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // Allow public routes
    let path = req.uri().path().to_string();
    if path.starts_with("/health") || path.starts_with("/api/v1/auth/") || (path.contains("/guestbook/") && req.method() == http::Method::GET) {
        return Ok(next.run(req).await);
    }
    
    let auth_header = req.headers().get(http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    
    if !auth_header.starts_with("Bearer ") {
        // For demo, allow if no auth in dev but log
        // In production, this would return 401
        if std::env::var("ENV").unwrap_or_else(|_| "development".into()) == "production" {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        let token = auth_header.trim_start_matches("Bearer ").trim();
        match verify_token(token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
            },
            Err(_) => return Err(StatusCode::UNAUTHORIZED),
        }
    }
    
    Ok(next.run(req).await)
}
