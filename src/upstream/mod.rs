pub mod circuit_breaker;
pub mod client;
pub mod coincap;
pub mod coingecko;
pub mod fawaz;
pub mod frankfurter;
pub mod manager;
pub mod metrics;

use async_trait::async_trait;
use chrono::NaiveDate;
use std::collections::HashMap;

use crate::models::{HistoricalRate, RateCollection};
use crate::Result;

/// Metadata about the upstream API request
#[derive(Debug, Clone)]
pub struct UpstreamCall {
    /// Full endpoint URL
    pub endpoint: String,
    /// HTTP method used
    pub method: String,
    /// Headers sent (if any)
    pub headers: Option<HashMap<String, String>>,
    /// Payload/body sent (if any)
    pub payload: Option<String>,
}

impl UpstreamCall {
    /// Create a simple GET request
    pub fn get(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            method: "GET".to_string(),
            headers: None,
            payload: None,
        }
    }

    /// Add headers
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Add payload
    pub fn with_payload(mut self, payload: impl Into<String>) -> Self {
        self.payload = Some(payload.into());
        self
    }
}

/// Response from upstream including the data and call metadata
#[derive(Debug, Clone)]
pub struct UpstreamResponse<T> {
    /// The actual data
    pub data: T,
    /// Details of the upstream API call
    pub call: UpstreamCall,
}

#[async_trait]
pub trait Upstream: Send + Sync {
    async fn get_latest_rates(&self, base: &str) -> Result<RateCollection>;
    async fn get_historical_rates(&self, base: &str, date: NaiveDate) -> Result<HistoricalRate>;
    fn name(&self) -> &'static str;
    fn is_healthy(&self) -> bool;
}

pub use client::HttpClient;
pub use coincap::CoinCapClient;
pub use coingecko::CoinGeckoClient;
pub use fawaz::FawazClient;
pub use frankfurter::FrankfurterClient;
pub use manager::UpstreamManager;

pub fn is_crypto_currency(code: &str) -> bool {
    coingecko::is_crypto_currency(code)
}

pub fn is_metal_currency(code: &str) -> bool {
    matches!(code.to_uppercase().as_str(), "XAU" | "XAG" | "XPT" | "XPD")
}
