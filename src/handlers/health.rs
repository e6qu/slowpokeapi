use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use std::time::Instant;

use crate::server::AppState;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<std::collections::HashMap<String, HealthCheck>>,
}

#[derive(Debug, Serialize)]
pub struct HealthCheck {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
}

pub async fn healthz() -> &'static str {
    "ok"
}

pub async fn readyz(State(state): State<AppState>) -> impl IntoResponse {
    let mut checks = std::collections::HashMap::new();
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

pub async fn livez() -> &'static str {
    "ok"
}

pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    let mut checks = std::collections::HashMap::new();
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
