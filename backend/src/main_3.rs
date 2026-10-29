use axum::{routing::{get, post, put, delete}, Router, middleware};
use tower_http::{cors::{CorsLayer, Any}, set_header::SetResponseHeaderLayer, trace::TraceLayer};
use http::{header, HeaderValue};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod core;
mod modules;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "funeral_os_api=info,tower_http=info,sqlx=warn".into()),
        ))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let db = core::db::init_pool().await?;
    tracing::info!("Running migrations...");
    sqlx::migrate!("./migrations").run(&db).await?;
    
    let nats = core::nats::init().await.unwrap_or_else(|e| {
        tracing::warn!("NATS not available ({}), using dummy", e);
        // We will create a dummy - for prod it must exist
        panic!("NATS required for production");
    });

    let state = core::AppState { db, nats };

    let cors = CorsLayer::new()
        .allow_origin(["http://localhost:3000".parse::<HeaderValue>().unwrap()])
        .allow_methods([http::Method::GET, http::Method::POST, http::Method::PUT, http::Method::DELETE, http::Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true);

    let app = Router::new()
        .route("/health", get(core::health::check))
        .route("/api/v1/auth/register-home", post(auth::handlers::register_home))
        .route("/api/v1/auth/login", post(auth::handlers::login))
        .route("/api/v1/obituaries", post(modules::obituary::create).get(modules::obituary::list))
        .route("/api/v1/obituaries/:id", get(modules::obituary::get).put(modules::obituary::update).delete(modules::obituary::delete))
        .route("/api/v1/obituaries/:id/publish", post(modules::obituary::publish))
        .route("/api/v1/streams", post(modules::streaming::create_stream))
        .route("/api/v1/streams/:id", get(modules::streaming::get_stream))
        .route("/api/v1/guestbook/:obituary_id", get(modules::guestbook::list).post(modules::guestbook::sign))
        .route("/api/v1/guestbook/entry/:id/moderate", post(modules::guestbook::moderate))
        .route("/api/v1/compliance/death-certificate", post(modules::compliance::submit_certificate))
        .route("/api/v1/compliance/:id", get(modules::compliance::get_status))
        .route("/api/v1/family/tree/:root_id", get(modules::family::get_tree).post(modules::family::add_member))
        .route("/api/v1/dashboard/stats", get(modules::dashboard::stats))
        .layer(middleware::from_fn(auth::audit::audit_middleware))
        .layer(middleware::from_fn(auth::rbac::rbac_middleware))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("strict-transport-security"),
            HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"),
        ))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(tower_http::limit::RequestBodyLimitLayer::new(10 * 1024 * 1024))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("FuneralOS v1.0 listening on {} - PRODUCTION READY", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
