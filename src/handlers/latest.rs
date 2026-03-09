use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use std::collections::HashMap;

use crate::cache::Cache;
use crate::models::{DataSourceInfo, LatestRatesResponse, ResponseResult, UpstreamRequestInfo};
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
                    build_data_source_info_from_cache(&source_str, &cache_result),
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
            let upstream_request = build_upstream_request(&source_str, &base);

            if let Some(ref cache) = state.rate_cache {
                let key = cache_key.clone();
                if let Err(e) = cache
                    .set_with_metadata(
                        cache_key,
                        rates.clone(),
                        crate::cache::UpstreamRequestDetails {
                            endpoint: upstream_request.endpoint.clone(),
                            method: upstream_request
                                .method
                                .clone()
                                .unwrap_or_else(|| "GET".to_string()),
                            headers: upstream_request.headers.clone(),
                            payload: upstream_request.payload.clone(),
                        },
                        None,
                    )
                    .await
                {
                    tracing::warn!("Cache set error for {}: {}", key, e);
                }
            }
            Ok(Json(build_response(
                &rates,
                build_data_source_info_fresh(&source_str, upstream_request),
            )))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

fn build_upstream_request(source: &str, base: &str) -> UpstreamRequestInfo {
    let endpoint = match source {
        "frankfurter" => format!("https://api.frankfurter.app/latest?from={}", base),
        "fawazahmed0" => format!(
            "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1/currencies/{}.json",
            base.to_lowercase()
        ),
        "coingecko" => format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
            base.to_lowercase()
        ),
        "coincap" => "https://api.coincap.io/v2/assets".to_string(),
        _ => "unknown".to_string(),
    };

    // Determine if non-GET method or special headers/payload needed
    let (method, headers, payload) = match source {
        "coingecko" => {
            // CoinGecko often requires API key header for higher rate limits
            let mut headers = HashMap::new();
            headers.insert("Accept".to_string(), "application/json".to_string());
            (None, Some(headers), None)
        }
        "coincap" => {
            let mut headers = HashMap::new();
            headers.insert("Accept".to_string(), "application/json".to_string());
            (None, Some(headers), None)
        }
        _ => (None, None, None), // Standard GET request
    };

    UpstreamRequestInfo {
        endpoint,
        method,
        headers,
        payload,
    }
}

fn build_data_source_info_from_cache<V>(
    source: &str,
    cache_result: &crate::cache::CacheResult<V>,
) -> DataSourceInfo {
    let upstream_request = UpstreamRequestInfo {
        endpoint: cache_result.upstream_request.endpoint.clone(),
        method: if cache_result.upstream_request.method == "GET" {
            None
        } else {
            Some(cache_result.upstream_request.method.clone())
        },
        headers: cache_result.upstream_request.headers.clone(),
        payload: cache_result.upstream_request.payload.clone(),
    };

    DataSourceInfo {
        source: source.to_string(),
        last_retrieved_unix: cache_result.retrieved_at.timestamp(),
        last_retrieved_utc: cache_result.retrieved_at.to_rfc3339(),
        last_cached_unix: cache_result.cached_at.map(|t| t.timestamp()),
        last_cached_utc: cache_result.cached_at.map(|t| t.to_rfc3339()),
        upstream_request,
    }
}

fn build_data_source_info_fresh(
    source: &str,
    upstream_request: UpstreamRequestInfo,
) -> DataSourceInfo {
    let now = Utc::now();

    DataSourceInfo {
        source: source.to_string(),
        last_retrieved_unix: now.timestamp(),
        last_retrieved_utc: now.to_rfc3339(),
        last_cached_unix: None,
        last_cached_utc: None,
        upstream_request,
    }
}

fn build_response(
    rates: &crate::models::RateCollection,
    data_source: DataSourceInfo,
) -> LatestRatesResponse {
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

use crate::models::{is_crypto_code, is_metal_code};
