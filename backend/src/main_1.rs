
use axum::{routing::{get, post}, Router, middleware};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod auth; mod core; mod modules;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "funeral_os_api=info,tower_http=info".into()),
    )).with(tracing_subscriber::fmt::layer().json()).init();
    let db = core::db::init_pool().await?;
    sqlx::migrate!("./migrations").run(&db).await?;
    let nats = core::nats::init().await?;
    let state = core::AppState { db, nats };
    let app = Router::new()
        .route("/health", get(core::health::check))
        .route("/api/v1/obituaries", post(modules::obituary::create).get(modules::obituary::list))
        .route("/api/v1/obituaries/:id", get(modules::obituary::get).put(modules::obituary::update))
        .route("/api/v1/streams", post(modules::streaming::create_stream))
        .route("/api/v1/guestbook/:obituary_id", get(modules::guestbook::list).post(modules::guestbook::sign))
        .route("/api/v1/compliance/death-certificate", post(modules::compliance::submit_certificate))
        .route("/api/v1/family/tree/:id", get(modules::family::get_tree))
        .layer(middleware::from_fn(auth::audit::audit_middleware))
        .layer(middleware::from_fn(auth::rbac::rbac_middleware))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(5 * 1024 * 1024))
        .with_state(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("FuneralOS API listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
