use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

use crate::models::{
    is_crypto_code, is_metal_code, HistoricalRate, HistoricalResponse, ResponseResult,
};
use crate::server::AppState;

const DOCUMENTATION_URL: &str = "https://github.com/e6qu/slowpokeapi";
const MIN_HISTORICAL_DATE: &str = "1999-01-04";

#[utoipa::path(
    get,
    path = "/v1/history/{base_code}/{year}/{month}/{day}",
    tag = "rates",
    params(
        ("base_code" = String, description = "Three-letter base currency code (e.g., USD)"),
        ("year" = i32, description = "Year (e.g., 2024)"),
        ("month" = u32, description = "Month (1-12)"),
        ("day" = u32, description = "Day (1-31)")
    ),
    responses(
        (status = 200, description = "Historical rates", body = HistoricalResponse),
        (status = 400, description = "Invalid currency code or date"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn get_history(
    State(state): State<AppState>,
    Path((base_code, year, month, day)): Path<(String, i32, u32, u32)>,
) -> Result<Json<HistoricalResponse>, (StatusCode, String)> {
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

    if is_crypto || is_metal {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Historical rates are only available for fiat currencies, not {base}"),
        ));
    }

    let date = chrono::NaiveDate::from_ymd_opt(year, month, day).ok_or((
        StatusCode::BAD_REQUEST,
        format!("Invalid date: {year}-{month}-{day}"),
    ))?;

    let today = chrono::Utc::now().date_naive();
    if date > today {
        return Err((
            StatusCode::BAD_REQUEST,
            "Cannot fetch rates for future dates".to_string(),
        ));
    }

    let min_date = chrono::NaiveDate::parse_from_str(MIN_HISTORICAL_DATE, "%Y-%m-%d")
        .expect("MIN_HISTORICAL_DATE constant is valid");
    if date < min_date {
        return Err((
            StatusCode::BAD_REQUEST,
            "Historical data only available from 1999-01-04 onwards".to_string(),
        ));
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

    match upstream_manager.get_historical_rates(&base, date).await {
        Ok(rates) => Ok(Json(build_response(&rates, year, month, day))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

fn build_response(rates: &HistoricalRate, year: i32, month: u32, day: u32) -> HistoricalResponse {
    HistoricalResponse {
        result: ResponseResult::Success,
        documentation: DOCUMENTATION_URL.to_string(),
        year,
        month,
        day,
        base_code: rates.base_code.clone(),
        conversion_rates: rates.rates.clone(),
        conversion_results: None,
    }
}
