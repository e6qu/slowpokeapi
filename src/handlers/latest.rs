use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;

use crate::cache::Cache;
use crate::models::{
    is_crypto_code, is_metal_code, DataSourceInfo, LatestRatesResponse, RateCollection,
    ResponseResult,
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

    // Try cache with metadata first
    if let Some(ref cache) = state.rate_cache {
        match cache.get_with_metadata(&cache_key).await {
            Ok(Some(cache_result)) => {
                let source_str = cache_result.value.source.to_string();
                return Ok(Json(build_response(
                    &cache_result.value,
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
            ))
        }
    };

    match upstream_manager.get_latest_rates(&base).await {
        Ok(rates) => {
            let source_str = rates.source.to_string();
            if let Some(ref cache) = state.rate_cache {
                let key = cache_key.clone();
                if let Err(e) = cache.set(cache_key, rates.clone(), None).await {
                    tracing::warn!("Cache set error for {}: {}", key, e);
                }
            }
            Ok(Json(build_response(
                &rates,
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

fn build_response(rates: &RateCollection, data_source: DataSourceInfo) -> LatestRatesResponse {
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
        data_source,
    }
}
