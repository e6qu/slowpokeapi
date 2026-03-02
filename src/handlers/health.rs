use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Instant;
use utoipa::ToSchema;

use crate::server::AppState;

#[allow(clippy::disallowed_methods)]
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<HashMap<String, HealthCheck>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthCheck {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
}

#[utoipa::path(
    get,
    path = "/healthz",
    tag = "system",
    responses(
        (status = 200, description = "Service is alive", body = String, example = "ok")
    )
)]
pub async fn healthz() -> &'static str {
    "ok"
}

#[utoipa::path(
    get,
    path = "/readyz",
    tag = "system",
    responses(
        (status = 200, description = "Service is ready", body = String),
        (status = 503, description = "Service not ready", body = String)
    )
)]
pub async fn readyz(State(state): State<AppState>) -> impl IntoResponse {
    let mut checks = HashMap::new();
    let mut all_healthy = true;

    if let Some(db_healthy) = state.db_health() {
        checks.insert(
            "database".to_string(),
            HealthCheck {
                status: if db_healthy { "pass" } else { "fail" }.to_string(),
                message: None,
                latency_ms: None,
            },
        );
        if !db_healthy {
            all_healthy = false;
        }
    }

    let status = if all_healthy { "ok" } else { "not ready" };

    if all_healthy {
        (StatusCode::OK, status)
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, status)
    }
}

#[utoipa::path(
    get,
    path = "/livez",
    tag = "system",
    responses(
        (status = 200, description = "Service is running", body = String, example = "ok")
    )
)]
pub async fn livez() -> &'static str {
    "ok"
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "system",
    responses(
        (status = 200, description = "Deep health check", body = HealthResponse)
    )
)]
pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    let mut checks = HashMap::new();
    let mut overall_status = "healthy";

    let start = Instant::now();
    if let Some(db_healthy) = state.db_health() {
        let latency = start.elapsed().as_millis() as u64;
        let status = if db_healthy { "pass" } else { "fail" };
        if status == "fail" {
            overall_status = "unhealthy";
        }
        checks.insert(
            "database".to_string(),
            HealthCheck {
                status: status.to_string(),
                message: if db_healthy {
                    Some("SQLite connection healthy".to_string())
                } else {
                    Some("Database connection failed".to_string())
                },
                latency_ms: Some(latency),
            },
        );
    }

    let uptime = state.start_time.elapsed().as_secs();

    let response = HealthResponse {
        status: overall_status.to_string(),
        version: Some(env!("CARGO_PKG_VERSION").to_string()),
        uptime_seconds: Some(uptime),
        checks: Some(checks),
    };

    Json(response)
}
