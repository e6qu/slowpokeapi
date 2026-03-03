use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

use crate::cache::Cache;
use crate::models::{
    is_crypto_code, is_metal_code, LatestRatesResponse, RateCollection, ResponseResult,
};
use crate::server::AppState;

const DOCUMENTATION_URL: &str = "https://github.com/e6qu/slowpokeapi";

#[utoipa::path(
    get,
    path = "/v1/latest/{base_code}",
    tag = "rates",
    params(
        ("base_code" = String, description = "Three-letter currency code (e.g., USD, EUR) or crypto code (e.g., BTC, ETH)")
    ),
    responses(
        (status = 200, description = "Successful response", body = LatestRatesResponse),
        (status = 400, description = "Invalid currency code"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn get_latest(
    State(state): State<AppState>,
    Path(base_code): Path<String>,
) -> Result<Json<LatestRatesResponse>, (StatusCode, String)> {
    let base = base_code.to_uppercase();

    let is_crypto = is_crypto_code(&base);
    let is_metal = is_metal_code(&base);
    let is_fiat = base.len() == 3 && base.chars().all(|c| c.is_ascii_uppercase());

    if !is_fiat && !is_crypto && !is_metal {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid currency code. Must be a 3-letter fiat code (USD, EUR), crypto (BTC, ETH), or metal (XAU, XAG)".to_string(),
        ));
    }

    let cache_key = format!("latest:{base}");

    if let Some(ref cache) = state.rate_cache {
        if let Ok(Some(rates)) = cache.get(&cache_key).await {
            return Ok(Json(build_response(&rates)));
        }
    }

    let upstream_manager = match state.upstream_manager.as_ref() {
        Some(m) => m,
        None => {
            return Err((
                StatusCode::SERVICE_UNAVAILABLE,
                "Rate service not available".to_string(),
            ))
        }
    };

    match upstream_manager.get_latest_rates(&base).await {
        Ok(rates) => {
            if let Some(ref cache) = state.rate_cache {
                let _ = cache.set(cache_key.clone(), rates.clone(), None).await;
            }
            Ok(Json(build_response(&rates)))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

fn build_response(rates: &RateCollection) -> LatestRatesResponse {
    let now = chrono::Utc::now();
    let next_update = now + chrono::Duration::hours(24);

    LatestRatesResponse {
        result: ResponseResult::Success,
        documentation: DOCUMENTATION_URL.to_string(),
        time_last_update_unix: rates.timestamp.timestamp(),
        time_last_update_utc: rates.timestamp.to_rfc3339(),
        time_next_update_unix: next_update.timestamp(),
        time_next_update_utc: next_update.to_rfc3339(),
        base_code: rates.base_code.clone(),
        conversion_rates: rates.rates.clone(),
    }
}
