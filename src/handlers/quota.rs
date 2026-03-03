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
            if let Some(api_key_store) = &state.api_key_store {
                match api_key_store.get(&key).await {
                    Some(api_key_info) => {
                        let response = QuotaResponse {
                            api_key: api_key_info.key.clone(),
                            limit: api_key_info.rate_limit.burst_capacity,
                            remaining: api_key_info.rate_limit.burst_capacity,
                            reset_seconds: 0,
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
