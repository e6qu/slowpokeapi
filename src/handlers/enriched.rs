use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use std::collections::HashMap;

use crate::cache::Cache;
use crate::models::{
    CurrencyMetadata, DataSourceInfo, EnrichedResponse, ResponseResult, UpstreamRequestInfo,
};
use crate::server::AppState;

fn get_metadata(code: &str) -> Option<CurrencyMetadata> {
    match code {
        "USD" => Some(CurrencyMetadata {
            code: "USD".to_string(),
            locale: "en-US".to_string(),
            two_letter_country_code: "US".to_string(),
            currency_name: "United States Dollar".to_string(),
            currency_name_short: "Dollar".to_string(),
            display_symbol: "$".to_string(),
            flag_url: "https://flagcdn.com/w640/us.png".to_string(),
        }),
        "EUR" => Some(CurrencyMetadata {
            code: "EUR".to_string(),
            locale: "de-DE".to_string(),
            two_letter_country_code: "DE".to_string(),
            currency_name: "Euro".to_string(),
            currency_name_short: "Euro".to_string(),
            display_symbol: "€".to_string(),
            flag_url: "https://flagcdn.com/w640/eu.png".to_string(),
        }),
        "GBP" => Some(CurrencyMetadata {
            code: "GBP".to_string(),
            locale: "en-GB".to_string(),
            two_letter_country_code: "GB".to_string(),
            currency_name: "British Pound Sterling".to_string(),
            currency_name_short: "Pound".to_string(),
            display_symbol: "£".to_string(),
            flag_url: "https://flagcdn.com/w640/gb.png".to_string(),
        }),
        "JPY" => Some(CurrencyMetadata {
            code: "JPY".to_string(),
            locale: "ja-JP".to_string(),
            two_letter_country_code: "JP".to_string(),
            currency_name: "Japanese Yen".to_string(),
            currency_name_short: "Yen".to_string(),
            display_symbol: "¥".to_string(),
            flag_url: "https://flagcdn.com/w640/jp.png".to_string(),
        }),
        "CAD" => Some(CurrencyMetadata {
            code: "CAD".to_string(),
            locale: "en-CA".to_string(),
            two_letter_country_code: "CA".to_string(),
            currency_name: "Canadian Dollar".to_string(),
            currency_name_short: "Dollar".to_string(),
            display_symbol: "C$".to_string(),
            flag_url: "https://flagcdn.com/w640/ca.png".to_string(),
        }),
        "AUD" => Some(CurrencyMetadata {
            code: "AUD".to_string(),
            locale: "en-AU".to_string(),
            two_letter_country_code: "AU".to_string(),
            currency_name: "Australian Dollar".to_string(),
            currency_name_short: "Dollar".to_string(),
            display_symbol: "A$".to_string(),
            flag_url: "https://flagcdn.com/w640/au.png".to_string(),
        }),
        "CHF" => Some(CurrencyMetadata {
            code: "CHF".to_string(),
            locale: "de-CH".to_string(),
            two_letter_country_code: "CH".to_string(),
            currency_name: "Swiss Franc".to_string(),
            currency_name_short: "Franc".to_string(),
            display_symbol: "CHF".to_string(),
            flag_url: "https://flagcdn.com/w640/ch.png".to_string(),
        }),
        "CNY" => Some(CurrencyMetadata {
            code: "CNY".to_string(),
            locale: "zh-CN".to_string(),
            two_letter_country_code: "CN".to_string(),
            currency_name: "Chinese Yuan".to_string(),
            currency_name_short: "Yuan".to_string(),
            display_symbol: "¥".to_string(),
            flag_url: "https://flagcdn.com/w640/cn.png".to_string(),
        }),
        "INR" => Some(CurrencyMetadata {
            code: "INR".to_string(),
            locale: "hi-IN".to_string(),
            two_letter_country_code: "IN".to_string(),
            currency_name: "Indian Rupee".to_string(),
            currency_name_short: "Rupee".to_string(),
            display_symbol: "₹".to_string(),
            flag_url: "https://flagcdn.com/w640/in.png".to_string(),
        }),
        "MXN" => Some(CurrencyMetadata {
            code: "MXN".to_string(),
            locale: "es-MX".to_string(),
            two_letter_country_code: "MX".to_string(),
            currency_name: "Mexican Peso".to_string(),
            currency_name_short: "Peso".to_string(),
            display_symbol: "$".to_string(),
            flag_url: "https://flagcdn.com/w640/mx.png".to_string(),
        }),
        _ => None,
    }
}

#[utoipa::path(
    get,
    path = "/v1/enriched/{base_code}/{target_code}",
    tag = "rates",
    params(
        ("base_code" = String, description = "Three-letter base currency code (e.g., USD)"),
        ("target_code" = String, description = "Three-letter target currency code (e.g., EUR)")
    ),
    responses(
        (status = 200, description = "Rate with target currency metadata", body = EnrichedResponse),
        (status = 400, description = "Invalid currency code"),
        (status = 404, description = "Currency metadata not found"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn get_enriched(
    State(state): State<AppState>,
    Path((base_code, target_code)): Path<(String, String)>,
) -> Result<Json<EnrichedResponse>, (StatusCode, String)> {
    let base = base_code.to_uppercase();
    let target = target_code.to_uppercase();

    if base == target {
        return Err((
            StatusCode::BAD_REQUEST,
            "Base and target currencies must be different".to_string(),
        ));
    }

    if base.len() != 3 || !base.chars().all(|c| c.is_ascii_uppercase()) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Invalid base currency code: {base}"),
        ));
    }

    if target.len() != 3 || !target.chars().all(|c| c.is_ascii_uppercase()) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Invalid target currency code: {target}"),
        ));
    }

    let target_metadata = get_metadata(&target).ok_or((
        StatusCode::NOT_FOUND,
        format!("Currency metadata not found for: {target}"),
    ))?;

    let cache_key = format!("enriched:{base}:{target}");

    // Try cache with metadata first
    if let Some(ref cache) = state.rate_cache {
        match cache.get_with_metadata(&cache_key).await {
            Ok(Some(cache_result)) => {
                let rate = cache_result.value.rates.get(&target).copied().ok_or((
                    StatusCode::NOT_FOUND,
                    format!("Currency rate not found: {target}"),
                ))?;
                let source_str = cache_result.value.source.to_string();
                return Ok(Json(build_response(
                    &base,
                    rate,
                    target_metadata,
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
            ));
        }
    };

    match upstream_manager.get_latest_rates(&base).await {
        Ok(rates) => {
            let rate = rates.rates.get(&target).copied().ok_or((
                StatusCode::NOT_FOUND,
                format!("Currency rate not found: {target}"),
            ))?;
            let source_str = rates.source.to_string();
            let upstream_request = build_upstream_request(&source_str, &base);

            if let Some(ref cache) = state.rate_cache {
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
                    tracing::warn!("Cache set error: {}", e);
                }
            }
            Ok(Json(build_response(
                &base,
                rate,
                target_metadata,
                build_data_source_info_fresh(&source_str, upstream_request),
            )))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

fn build_upstream_request(source: &str, base: &str) -> UpstreamRequestInfo {
    let endpoint = match source {
        "frankfurter" => format!("https://api.frankfurter.app/latest?from={base}"),
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

    let (method, headers, payload) = match source {
        "coingecko" | "coincap" => {
            let mut headers = HashMap::new();
            headers.insert("Accept".to_string(), "application/json".to_string());
            (None, Some(headers), None)
        }
        _ => (None, None, None),
    };

    UpstreamRequestInfo {
        endpoint,
        method,
        headers,
        payload,
    }
}

fn build_data_source_info_from_cache(
    source: &str,
    cache_result: &crate::cache::CacheResult<crate::models::RateCollection>,
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
        last_retrieved: cache_result.retrieved_at.to_rfc3339(),
        last_cached: cache_result.cached_at.map(|t| t.to_rfc3339()),
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
        last_retrieved: now.to_rfc3339(),
        last_cached: None,
        upstream_request,
    }
}

fn build_response(
    base_code: &str,
    conversion_rate: f64,
    target_data: CurrencyMetadata,
    data_source: DataSourceInfo,
) -> EnrichedResponse {
    let now = chrono::Utc::now();

    EnrichedResponse {
        result: ResponseResult::Success,
        time_last_update_unix: now.timestamp(),
        time_last_update_utc: now.to_rfc3339(),
        base_code: base_code.to_uppercase(),
        target_code: target_data.code.clone(),
        conversion_rate,
        target_data,
        data_source,
    }
}
