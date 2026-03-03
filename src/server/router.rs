use axum::routing::get;
use axum::Router;

use crate::handlers::{currencies, health, metrics};
use crate::metrics::PROMETHEUS_LAYER;
use crate::server::openapi::swagger_ui;
use crate::server::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health::healthz))
        .route("/readyz", get(health::readyz))
        .route("/livez", get(health::livez))
        .route("/health", get(health::health))
        .route("/v1/currencies", get(currencies::list_currencies))
        .route("/v1/currencies.min", get(currencies::list_currencies_min))
        .route("/metrics", get(metrics::metrics))
        .merge(swagger_ui())
        .layer(PROMETHEUS_LAYER.clone())
        .with_state(state)
}
