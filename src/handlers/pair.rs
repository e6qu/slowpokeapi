use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;

use crate::cache::Cache;
use crate::models::{
    is_crypto_code, is_metal_code, DataSourceInfo, PairResponse, RateCollection, ResponseResult,
};
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

    if base == target {
        return Err((
            StatusCode::BAD_REQUEST,
            "Base and target currencies must be different".to_string(),
        ));
    }

    let is_crypto = is_crypto_code(&base) || is_crypto_code(&target);
    let is_metal = is_metal_code(&base) || is_metal_code(&target);
    let is_fiat = (base.len() == 3 && base.chars().all(|c| c.is_ascii_uppercase()))
        && (target.len() == 3 && target.chars().all(|c| c.is_ascii_uppercase()));

    if !is_fiat && !is_crypto && !is_metal {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid currency code. Must be a 3-letter fiat code (USD, EUR), crypto (BTC, ETH), or metal (XAU, XAG)".to_string(),
        ));
    }

    if let Some(amount) = amount {
        if !amount.is_finite() || amount <= 0.0 {
            return Err((
                StatusCode::BAD_REQUEST,
                "Amount must be a finite positive number".to_string(),
            ));
        }
    }

    let cache_key = format!("pair:{base}:{target}");

    // Try cache with metadata first
    if let Some(ref cache) = state.rate_cache {
        match cache.get_with_metadata(&cache_key).await {
            Ok(Some(cache_result)) => {
                let rate = cache_result.value.rates.get(&target).copied().ok_or((
                    StatusCode::NOT_FOUND,
                    format!("Currency not found: {target}"),
                ))?;
                let source_str = cache_result.value.source.to_string();
                return Ok(Json(build_response_with_rate(
                    &cache_result.value,
                    &target,
                    rate,
                    amount,
                    build_data_source_info(&source_str, true, Some(&cache_result)),
                )));
            }
            Ok(None) => {}
            Err(e) => tracing::warn!("Cache get error for {}: {}", cache_key, e),
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
            let rate = rates.rates.get(&target).copied().ok_or((
                StatusCode::NOT_FOUND,
                format!("Currency not found: {target}"),
            ))?;
            let source_str = rates.source.to_string();

            if let Some(ref cache) = state.rate_cache {
                if let Err(e) = cache.set(cache_key, rates.clone(), None).await {
                    tracing::warn!("Cache set error: {}", e);
                }
            }
            Ok(Json(build_response_with_rate(
                &rates,
                &target,
                rate,
                amount,
                build_data_source_info(&source_str, false, None),
            )))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

fn build_data_source_info(
    source: &str,
    _cached: bool,
    cache_result: Option<&crate::cache::CacheResult<RateCollection>>,
) -> DataSourceInfo {
    let now = Utc::now();

    if let Some(cr) = cache_result {
        DataSourceInfo {
            source: source.to_string(),
            source_timestamp_unix: cr.source_timestamp.timestamp(),
            source_timestamp_utc: cr.source_timestamp.to_rfc3339(),
            cached: true,
            cache_timestamp_unix: cr.cached_at.map(|t| t.timestamp()),
            cache_timestamp_utc: cr.cached_at.map(|t| t.to_rfc3339()),
        }
    } else {
        DataSourceInfo {
            source: source.to_string(),
            source_timestamp_unix: now.timestamp(),
            source_timestamp_utc: now.to_rfc3339(),
            cached: false,
            cache_timestamp_unix: None,
            cache_timestamp_utc: None,
        }
    }
}

fn build_response_with_rate(
    rates: &RateCollection,
    target: &str,
    rate: f64,
    amount: Option<f64>,
    data_source: DataSourceInfo,
) -> PairResponse {
    let now = chrono::Utc::now();
    let next_update = now + chrono::Duration::hours(24);

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
        data_source,
    }
}
