use axum::{http::Request, middleware::Next, response::Response};
use std::time::Instant;

pub async fn audit_middleware<B>(req: Request<B>, next: Next<B>) -> Response
where B: std::fmt::Debug {
    let start = Instant::now();
    let method = req.method().clone();
    let path = req.uri().path().to_owned();
    let res = next.run(req).await;
    let latency = start.elapsed().as_millis();
    tracing::info!(
        audit = true,
        method = %method,
        path = %path,
        status = %res.status().as_u16(),
        latency_ms = %latency,
        "audit"
    );
    res
}
