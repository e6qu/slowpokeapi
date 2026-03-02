use axum::http::{header, StatusCode};
use axum::response::IntoResponse;

use crate::metrics;

#[utoipa::path(
    get,
    path = "/metrics",
    tag = "system",
    responses(
        (status = 200, description = "Prometheus metrics", body = String)
    )
)]
pub async fn metrics() -> impl IntoResponse {
    let body = metrics::metrics_handler();
    (
        StatusCode::OK,
        [(
            header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        body,
    )
}
