use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum ResponseResult {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum ErrorType {
    #[serde(rename = "missing-key")]
    MissingKey,
    #[serde(rename = "invalid-key")]
    InvalidKey,
    #[serde(rename = "inactive-account")]
    InactiveAccount,
    #[serde(rename = "quota-reached")]
    QuotaReached,
    #[serde(rename = "not-found")]
    NotFound,
    #[serde(rename = "invalid-currency")]
    InvalidCurrency,
    #[serde(rename = "invalid-date")]
    InvalidDate,
    #[serde(rename = "malformed-request")]
    MalformedRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LatestRatesResponse {
    pub result: ResponseResult,
    pub documentation: String,
    pub time_last_update_unix: i64,
    pub time_last_update_utc: String,
    pub time_next_update_unix: i64,
    pub time_next_update_utc: String,
    pub base_code: String,
    pub conversion_rates: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PairResponse {
    pub result: ResponseResult,
    pub documentation: String,
    pub time_last_update_unix: i64,
    pub time_last_update_utc: String,
    pub time_next_update_unix: i64,
    pub time_next_update_utc: String,
    pub base_code: String,
    pub target_code: String,
    pub conversion_rate: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_result: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HistoricalResponse {
    pub result: ResponseResult,
    pub documentation: String,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub base_code: String,
    pub conversion_rates: HashMap<String, f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_results: Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EnrichedResponse {
    pub result: ResponseResult,
    pub time_last_update_unix: i64,
    pub time_last_update_utc: String,
    pub base_code: String,
    pub target_code: String,
    pub conversion_rate: f64,
    pub target_data: crate::models::CurrencyMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct QuotaResponse {
    pub result: ResponseResult,
    pub quota_used: u64,
    pub quota_limit: u64,
    pub quota_remaining: u64,
    pub reset_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CurrenciesResponse {
    #[serde(flatten)]
    pub currencies: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub result: ResponseResult,
    pub error_type: ErrorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
