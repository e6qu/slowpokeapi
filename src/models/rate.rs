use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum Source {
    Frankfurter,
    FawazAhmed,
    CoinGecko,
    CoinCap,
    Cached,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExchangeRate {
    pub base_code: String,
    pub target_code: String,
    pub rate: f64,
    pub timestamp: DateTime<Utc>,
    pub source: Source,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RateCollection {
    pub base_code: String,
    pub rates: HashMap<String, f64>,
    pub timestamp: DateTime<Utc>,
    pub source: Source,
}
