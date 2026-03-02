//! SlowPokeAPI - Currency exchange rate API with distributed sync

use slowpokeapi::{
    config::Settings,
    server::{create_router, AppState},
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let settings = Settings::load().expect("Failed to load configuration");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&settings.logging.level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::new(settings.clone());
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
