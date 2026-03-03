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

use crate::models::{HistoricalRate, RateCollection};
use crate::Result;

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
