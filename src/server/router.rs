use axum::routing::get;
use axum::Router;

use crate::handlers::{health, metrics};
use crate::metrics::PROMETHEUS_LAYER;
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
        .route("/metrics", get(metrics::metrics))
        .merge(swagger_ui())
        .layer(PROMETHEUS_LAYER.clone())
        .with_state(state)
}
