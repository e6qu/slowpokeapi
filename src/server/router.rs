use axum::routing::get;
use axum::Router;

use crate::handlers::health;
use crate::server::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health::healthz))
        .route("/readyz", get(health::readyz))
        .route("/livez", get(health::livez))
        .route("/health", get(health::health))
        .with_state(state)
}
