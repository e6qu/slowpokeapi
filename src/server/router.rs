use axum::routing::get;
use axum::Router;

use crate::handlers::{currencies, enriched, health, history, latest, metrics, pair};
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
        .route("/v1/latest/:base_code", get(latest::get_latest))
        .route("/v1/pair/:base_code/:target_code", get(pair::get_pair))
        .route(
            "/v1/enriched/:base_code/:target_code",
            get(enriched::get_enriched),
        )
        .route(
            "/v1/history/:base_code/:year/:month/:day",
            get(history::get_history),
        )
        .route("/metrics", get(metrics::metrics))
        .merge(swagger_ui())
        .layer(PROMETHEUS_LAYER.clone())
        .with_state(state)
}
