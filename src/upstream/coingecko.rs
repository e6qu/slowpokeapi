use async_trait::async_trait;
use chrono::NaiveDate;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::sync::Arc;

use super::{HttpClient, Upstream};
use crate::models::{HistoricalRate, RateCollection, Source};
use crate::upstream::metrics::UPSTREAM_METRICS;
use crate::{Error, Result};

const BASE_URL: &str = "https://api.coingecko.com/api/v3";

pub const COINGECKO_ID_MAP: &[(&str, &str)] = &[
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
    ("MATIC", "matic-network"),
    ("AVAX", "avalanche-2"),
    ("UNI", "uniswap"),
    ("ATOM", "cosmos"),
];

pub fn code_to_coingecko_id(code: &str) -> Option<&'static str> {
    COINGECKO_ID_MAP
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, id)| *id)
}

pub fn coingecko_id_to_code(id: &str) -> Option<&'static str> {
    COINGECKO_ID_MAP
        .iter()
        .find(|(_, i)| *i == id)
        .map(|(c, _)| *c)
}

pub fn is_crypto_currency(code: &str) -> bool {
    code_to_coingecko_id(code).is_some()
}

pub struct CoinGeckoClient {
    http: Arc<HttpClient>,
    healthy: std::sync::atomic::AtomicBool,
}

impl CoinGeckoClient {
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
impl Upstream for CoinGeckoClient {
    async fn get_latest_rates(&self, base: &str) -> Result<RateCollection> {
        let start = std::time::Instant::now();

        let coingecko_id = code_to_coingecko_id(base)
            .ok_or_else(|| Error::NotFound(format!("Cryptocurrency not supported: {base}")))?;

        let vs_currencies = "usd,eur,gbp,jpy,aud,cad,chf,cny,inr,mxn,brl,krw,hkd,sgd,nok,sek,dkk,rub,zar,try,egp,php,idr,myr,thb,vnd,sar,aed,kwd,bdt,pkr,lkr,inr,ngn";
        let url =
            format!("{BASE_URL}/simple/price?ids={coingecko_id}&vs_currencies={vs_currencies}");

        UPSTREAM_METRICS.record_request("coingecko");

        let response = self.http.inner().get(&url).send().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("coingecko");
            self.healthy
                .store(false, std::sync::atomic::Ordering::SeqCst);
            Error::Internal(format!("CoinGecko request failed: {e}"))
        })?;

        UPSTREAM_METRICS.observe_latency("coingecko", start.elapsed());

        if response.status() == StatusCode::NOT_FOUND {
            return Err(Error::NotFound(format!("Cryptocurrency not found: {base}")));
        }

        if !response.status().is_success() {
            UPSTREAM_METRICS.record_error("coingecko");
            return Err(Error::Internal(format!(
                "CoinGecko returned status: {}",
                response.status()
            )));
        }

        let json: serde_json::Value = response.json().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("coingecko");
            Error::Internal(format!("Failed to parse CoinGecko response: {e}"))
        })?;

        let prices = json.get(coingecko_id).ok_or_else(|| {
            UPSTREAM_METRICS.record_error("coingecko");
            Error::Internal("Invalid response from CoinGecko: missing prices".to_string())
        })?;

        let mut rates: HashMap<String, f64> = HashMap::new();

        if let Some(obj) = prices.as_object() {
            for (currency, price_val) in obj {
                if let Some(price) = price_val.as_f64() {
                    if price > 0.0 {
                        rates.insert(currency.to_uppercase(), 1.0 / price);
                    }
                }
            }
        }

        rates.insert(base.to_uppercase(), 1.0);

        Ok(RateCollection {
            base_code: base.to_uppercase(),
            rates,
            timestamp: chrono::Utc::now(),
            source: Source::CoinGecko,
        })
    }

    async fn get_historical_rates(&self, base: &str, date: NaiveDate) -> Result<HistoricalRate> {
        let start = std::time::Instant::now();

        let coingecko_id = code_to_coingecko_id(base)
            .ok_or_else(|| Error::NotFound(format!("Cryptocurrency not supported: {base}")))?;

        let now = chrono::Utc::now();
        let date_dt = chrono::TimeZone::from_utc_datetime(
            &chrono::Utc,
            &date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| Error::Internal("Invalid date for historical lookup".to_string()))?,
        );
        let days = (now - date_dt).num_days().max(0);

        let url = format!(
            "{BASE_URL}/coins/{coingecko_id}/market_chart?vs_currency=usd&days={days}&interval=daily"
        );

        UPSTREAM_METRICS.record_request("coingecko");

        let response = self.http.inner().get(&url).send().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("coingecko");
            self.healthy
                .store(false, std::sync::atomic::Ordering::SeqCst);
            Error::Internal(format!("CoinGecko historical request failed: {e}"))
        })?;

        UPSTREAM_METRICS.observe_latency("coingecko", start.elapsed());

        if !response.status().is_success() {
            UPSTREAM_METRICS.record_error("coingecko");
            return Err(Error::Internal(format!(
                "CoinGecko returned status: {}",
                response.status()
            )));
        }

        let json: serde_json::Value = response.json().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("coingecko");
            Error::Internal(format!("Failed to parse CoinGecko response: {e}"))
        })?;

        let prices = json
            .get("prices")
            .and_then(|p| p.as_array())
            .ok_or_else(|| {
                UPSTREAM_METRICS.record_error("coingecko");
                Error::Internal("Invalid response from CoinGecko: missing prices".to_string())
            })?;

        let date_ts = date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| Error::Internal("Invalid date for historical lookup".to_string()))?
            .and_utc()
            .timestamp_millis();
        let mut closest_price: Option<f64> = None;
        let mut min_diff = i64::MAX;

        for price_point in prices {
            if let Some(arr) = price_point.as_array() {
                if arr.len() >= 2 {
                    if let (Some(ts), Some(price)) = (arr[0].as_i64(), arr[1].as_f64()) {
                        let diff = (ts - date_ts).abs();
                        if diff < min_diff {
                            min_diff = diff;
                            closest_price = Some(price);
                        }
                    }
                }
            }
        }

        let usd_rate = closest_price.ok_or_else(|| {
            Error::NotFound(format!("Historical price not found for {base} on {date}"))
        })?;

        if usd_rate <= 0.0 {
            return Err(Error::Internal(format!(
                "Invalid price for {base} on {date}: {usd_rate}"
            )));
        }

        let mut rates: HashMap<String, f64> = HashMap::new();
        rates.insert("USD".to_string(), 1.0 / usd_rate);
        rates.insert(base.to_uppercase(), 1.0);

        Ok(HistoricalRate {
            base_code: base.to_uppercase(),
            date,
            rates,
            source: Source::CoinGecko,
        })
    }

    fn name(&self) -> &'static str {
        "coingecko"
    }

    fn is_healthy(&self) -> bool {
        self.healthy.load(std::sync::atomic::Ordering::SeqCst)
    }
}
