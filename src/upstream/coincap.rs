use async_trait::async_trait;
use chrono::NaiveDate;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::sync::Arc;

use super::{HttpClient, Upstream};
use crate::models::{HistoricalRate, RateCollection, Source};
use crate::upstream::metrics::UPSTREAM_METRICS;
use crate::{Error, Result};

const BASE_URL: &str = "https://api.coincap.io/v2";

pub const COINCAP_ID_MAP: &[(&str, &str)] = &[
    ("BTC", "bitcoin"),
    ("ETH", "ethereum"),
    ("XRP", "ripple"),
    ("LTC", "litecoin"),
    ("BCH", "bitcoin-cash"),
    ("ADA", "cardano"),
    ("DOT", "polkadot"),
    ("LINK", "chainlink"),
    ("XLM", "stellar"),
    ("DOGE", "dogecoin"),
    ("SOL", "solana"),
    ("MATIC", "polygon"),
    ("AVAX", "avalanche"),
    ("UNI", "uniswap"),
    ("ATOM", "cosmos"),
];

pub fn code_to_coincap_id(code: &str) -> Option<&'static str> {
    COINCAP_ID_MAP
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, id)| *id)
}

pub fn coincap_id_to_code(id: &str) -> Option<&'static str> {
    COINCAP_ID_MAP
        .iter()
        .find(|(_, i)| *i == id)
        .map(|(c, _)| *c)
}

pub fn is_crypto_currency(code: &str) -> bool {
    code_to_coincap_id(code).is_some()
}

pub struct CoinCapClient {
    http: Arc<HttpClient>,
    healthy: std::sync::atomic::AtomicBool,
}

impl CoinCapClient {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self {
            http,
            healthy: std::sync::atomic::AtomicBool::new(true),
        }
    }

    pub fn supports_currency(&self, code: &str) -> bool {
        is_crypto_currency(code)
    }
}

#[async_trait]
impl Upstream for CoinCapClient {
    async fn get_latest_rates(&self, base: &str) -> Result<RateCollection> {
        let start = std::time::Instant::now();

        let coincap_id = code_to_coincap_id(base)
            .ok_or_else(|| Error::NotFound(format!("Cryptocurrency not supported: {base}")))?;

        let url = format!("{BASE_URL}/assets/{coincap_id}");

        UPSTREAM_METRICS.record_request("coincap");

        let response = self.http.inner().get(&url).send().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("coincap");
            self.healthy
                .store(false, std::sync::atomic::Ordering::SeqCst);
            Error::Internal(format!("CoinCap request failed: {e}"))
        })?;

        UPSTREAM_METRICS.observe_latency("coincap", start.elapsed());

        if response.status() == StatusCode::NOT_FOUND {
            return Err(Error::NotFound(format!("Cryptocurrency not found: {base}")));
        }

        if !response.status().is_success() {
            UPSTREAM_METRICS.record_error("coincap");
            return Err(Error::Internal(format!(
                "CoinCap returned status: {}",
                response.status()
            )));
        }

        let json: serde_json::Value = response.json().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("coincap");
            Error::Internal(format!("Failed to parse CoinCap response: {e}"))
        })?;

        let data = json.get("data").ok_or_else(|| {
            UPSTREAM_METRICS.record_error("coincap");
            Error::Internal("Invalid response from CoinCap: missing data".to_string())
        })?;

        let usd_price = data
            .get("priceUsd")
            .and_then(|p| p.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| {
                UPSTREAM_METRICS.record_error("coincap");
                Error::Internal("Invalid response from CoinCap: missing priceUsd".to_string())
            })?;

        if usd_price <= 0.0 {
            return Err(Error::Internal(format!(
                "Invalid price for {base}: {usd_price}"
            )));
        }

        let mut rates: HashMap<String, f64> = HashMap::new();
        rates.insert("USD".to_string(), 1.0 / usd_price);
        rates.insert(base.to_uppercase(), 1.0);

        Ok(RateCollection {
            base_code: base.to_uppercase(),
            rates,
            timestamp: chrono::Utc::now(),
            source: Source::CoinCap,
        })
    }

    async fn get_historical_rates(&self, base: &str, date: NaiveDate) -> Result<HistoricalRate> {
        let start = std::time::Instant::now();

        let coincap_id = code_to_coincap_id(base)
            .ok_or_else(|| Error::NotFound(format!("Cryptocurrency not supported: {base}")))?;

        let date_start = date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| Error::Internal("Invalid date for historical lookup".to_string()))?
            .and_utc()
            .timestamp_millis();
        let date_end = date
            .and_hms_opt(23, 59, 59)
            .ok_or_else(|| Error::Internal("Invalid date for historical lookup".to_string()))?
            .and_utc()
            .timestamp_millis();

        let url = format!(
            "{BASE_URL}/assets/{coincap_id}/history?interval=d1&start={date_start}&end={date_end}"
        );

        UPSTREAM_METRICS.record_request("coincap");

        let response = self.http.inner().get(&url).send().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("coincap");
            self.healthy
                .store(false, std::sync::atomic::Ordering::SeqCst);
            Error::Internal(format!("CoinCap historical request failed: {e}"))
        })?;

        UPSTREAM_METRICS.observe_latency("coincap", start.elapsed());

        if !response.status().is_success() {
            UPSTREAM_METRICS.record_error("coincap");
            return Err(Error::Internal(format!(
                "CoinCap returned status: {}",
                response.status()
            )));
        }

        let json: serde_json::Value = response.json().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("coincap");
            Error::Internal(format!("Failed to parse CoinCap response: {e}"))
        })?;

        let data = json.get("data").and_then(|d| d.as_array()).ok_or_else(|| {
            UPSTREAM_METRICS.record_error("coincap");
            Error::Internal("Invalid response from CoinCap: missing data".to_string())
        })?;

        let price_entry = data.first().ok_or_else(|| {
            Error::NotFound(format!("Historical price not found for {base} on {date}"))
        })?;

        let usd_price = price_entry
            .get("priceUsd")
            .and_then(|p| p.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| {
                UPSTREAM_METRICS.record_error("coincap");
                Error::Internal("Invalid response from CoinCap: missing priceUsd".to_string())
            })?;

        if usd_price <= 0.0 {
            return Err(Error::Internal(format!(
                "Invalid price for {base} on {date}: {usd_price}"
            )));
        }

        let mut rates: HashMap<String, f64> = HashMap::new();
        rates.insert("USD".to_string(), 1.0 / usd_price);
        rates.insert(base.to_uppercase(), 1.0);

        Ok(HistoricalRate {
            base_code: base.to_uppercase(),
            date,
            rates,
            source: Source::CoinCap,
        })
    }

    fn name(&self) -> &'static str {
        "coincap"
    }

    fn is_healthy(&self) -> bool {
        self.healthy.load(std::sync::atomic::Ordering::SeqCst)
    }
}
