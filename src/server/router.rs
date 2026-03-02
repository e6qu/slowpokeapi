use axum::routing::get;
use axum::Router;

use crate::handlers::health;
use crate::server::openapi::swagger_ui;
use crate::server::AppState;

pub fn create_router(state: AppState) -> Router {
    let health_routes = Router::new()
        .route("/healthz", get(health::healthz))
        .route("/readyz", get(health::readyz))
        .route("/livez", get(health::livez))
        .route("/health", get(health::health));

    Router::new()
        .merge(health_routes)
        .merge(swagger_ui())
        .with_state(state)
}
