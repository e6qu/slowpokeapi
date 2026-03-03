//! SlowPokeAPI - Currency exchange rate API with distributed sync

use slowpokeapi::{
    config::Settings,
    server::{create_router, AppState},
    storage::sqlite::create_pool,
    upstream::UpstreamManager,
};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let settings = Settings::load().expect("Failed to load configuration");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&settings.logging.level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_pool = if settings.database.url.starts_with("sqlite:")
        && !settings.database.url.contains(":memory:")
    {
        let db_path = settings
            .database
            .url
            .strip_prefix("sqlite:")
            .unwrap_or(&settings.database.url);
        if let Some(parent) = Path::new(db_path).parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).expect("Failed to create database directory");
            }
        }
        create_pool(&settings.database.url)
            .await
            .expect("Failed to create database pool")
    } else {
        create_pool(&settings.database.url)
            .await
            .expect("Failed to create database pool")
    };

    let http_client = Arc::new(slowpokeapi::upstream::HttpClient::new(10));
    let upstream_manager = UpstreamManager::new(http_client);

    let state = AppState::new(settings.clone())
        .with_db(db_pool)
        .with_upstream(upstream_manager);
    let app = create_router(state);

    let addr = SocketAddr::new(
        settings.server.host.parse().expect("Invalid host address"),
        settings.server.port,
    );
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
