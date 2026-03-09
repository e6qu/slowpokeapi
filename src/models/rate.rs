use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
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

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Source::Frankfurter => write!(f, "frankfurter"),
            Source::FawazAhmed => write!(f, "fawazahmed0"),
            Source::CoinGecko => write!(f, "coingecko"),
            Source::CoinCap => write!(f, "coincap"),
            Source::Cached => write!(f, "cache"),
        }
    }
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
