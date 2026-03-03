use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use crate::cache::Cache;
use crate::models::{PairResponse, RateCollection, ResponseResult};
use crate::server::AppState;

const DOCUMENTATION_URL: &str = "https://github.com/e6qu/slowpokeapi";

#[derive(Debug, serde::Deserialize)]
pub struct PairQueryParams {
    pub amount: Option<f64>,
}

#[utoipa::path(
    get,
    path = "/v1/pair/{base_code}/{target_code}",
    tag = "rates",
    params(
        ("base_code" = String, description = "Three-letter base currency code (e.g., USD)"),
        ("target_code" = String, description = "Three-letter target currency code (e.g., EUR)")
    ),
    responses(
        (status = 200, description = "Rate for currency pair", body = PairResponse),
        (status = 400, description = "Invalid currency code"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn get_pair(
    State(state): State<AppState>,
    Path((base_code, target_code)): Path<(String, String)>,
    Query(params): Query<PairQueryParams>,
) -> Result<Json<PairResponse>, (StatusCode, String)> {
    let base = base_code.to_uppercase();
    let target = target_code.to_uppercase();
    let amount = params.amount;

    if base.len() != 3 || !base.chars().all(|c| c.is_ascii_uppercase()) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Invalid base currency code: {}", base),
        ));
    }

 return Ok(Json(build_response(&rates, &target, amount))

    if let Some(amount) = amount {
        if amount <= 0.0 {
            return Err((
                StatusCode::BAD_REQUEST,
                "Amount must be positive".to_string(),
            ));
        }
    }

    let cache_key = format!("pair:{base}:{target}");

    if let Some(ref cache) = state.rate_cache {
        if let Ok(Some(rates)) = cache.get(&cache_key).await {
            return Ok(Json(build_response(&rates, &target, amount)));
        }
    }

    let upstream_manager = match state.upstream_manager.as_ref() {
        Some(m) => m,
        None => {
            return Err((
                StatusCode::SERVICE_UNAVAILABLE,
                "Rate service not available".to_string(),
            ));
        }
    };

    match upstream_manager.get_latest_rates(&base).await {
        Ok(rates) => {
            if let Some(ref cache) = state.rate_cache {
                let _ = cache.set(cache_key.clone(), rates.clone(), None).await;
            }
            Ok(Json(build_response(&rates, &target, amount)))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

fn build_response(rates: &RateCollection, target: &str, amount: Option<f64>) -> PairResponse {
    let now = chrono::Utc::now();
    let next_update = now + chrono::Duration::hours(24);

    let rate = rates.rates.get(target).copied().unwrap_or(0.0);

    let conversion_result = amount.map(|a| a * rate);

    PairResponse {
        result: ResponseResult::Success,
        documentation: DOCUMENTATION_URL.to_string(),
        time_last_update_unix: rates.timestamp.timestamp(),
        time_last_update_utc: rates.timestamp.to_rfc3339(),
        time_next_update_unix: next_update.timestamp(),
        time_next_update_utc: next_update.to_rfc3339(),
        base_code: rates.base_code.clone(),
        target_code: target.to_uppercase(),
        conversion_rate: rate,
        conversion_result,
    }
}
