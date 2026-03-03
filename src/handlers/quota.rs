use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::server::AppState;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct QuotaResponse {
    pub api_key: String,
    pub limit: u64,
    pub remaining: u64,
    pub reset_seconds: u64,
}

#[utoipa::path(
    get,
    path = "/v1/quota",
    tag = "quota",
    responses(
        (status = 200, description = "Quota information", body = QuotaResponse),
        (status = 401, description = "Missing or invalid API key")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_quota(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let api_key = headers
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    match api_key {
        Some(key) => {
            if let Some(rate_limiter) = &state.rate_limiter {
                match rate_limiter.get_quota_info(&key).await {
                    Some(info) => {
                        let response = QuotaResponse {
                            api_key: key,
                            limit: info.limit,
                            remaining: info.remaining,
                            reset_seconds: info.reset_after.as_secs(),
                        };
                        (StatusCode::OK, Json(response))
                    }
                    None => (
                        StatusCode::UNAUTHORIZED,
                        Json(QuotaResponse {
                            api_key: String::new(),
                            limit: 0,
                            remaining: 0,
                            reset_seconds: 0,
                        }),
                    ),
                }
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(QuotaResponse {
                        api_key: String::new(),
                        limit: 0,
                        remaining: 0,
                        reset_seconds: 0,
                    }),
                )
            }
        }
        None => (
            StatusCode::UNAUTHORIZED,
            Json(QuotaResponse {
                api_key: String::new(),
                limit: 0,
                remaining: 0,
                reset_seconds: 0,
            }),
        ),
    }
}
