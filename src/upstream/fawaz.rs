use async_trait::async_trait;
use chrono::NaiveDate;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::sync::Arc;

use super::{HttpClient, Upstream};
use crate::models::{HistoricalRate, RateCollection, Source};
use crate::upstream::metrics::UPSTREAM_METRICS;
use crate::{Error, Result};

const BASE_URL: &str = "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1";

pub struct FawazClient {
    http: Arc<HttpClient>,
    healthy: std::sync::atomic::AtomicBool,
}

impl FawazClient {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self {
            http,
            healthy: std::sync::atomic::AtomicBool::new(true),
        }
    }
}

#[async_trait]
impl Upstream for FawazClient {
    async fn get_latest_rates(&self, base: &str) -> Result<RateCollection> {
        let start = std::time::Instant::now();
        let base_lower = base.to_lowercase();
        let url = format!("{BASE_URL}/currencies/{base_lower}.json");

        UPSTREAM_METRICS.record_request("fawaz");

        let response = self.http.inner().get(&url).send().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("fawaz");
            self.healthy
                .store(false, std::sync::atomic::Ordering::SeqCst);
            Error::Internal(format!("FawazAhmed request failed: {e}"))
        })?;

        UPSTREAM_METRICS.observe_latency("fawaz", start.elapsed());

        if response.status() == StatusCode::NOT_FOUND {
            return Err(Error::NotFound(format!("Currency not found: {base}")));
        }

        if !response.status().is_success() {
            UPSTREAM_METRICS.record_error("fawaz");
            return Err(Error::Internal(format!(
                "FawazAhmed returned status: {}",
                response.status()
            )));
        }

        let json: serde_json::Value = response.json().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("fawaz");
            Error::Internal(format!("Failed to parse FawazAhmed response: {e}"))
        })?;

        let rates_map: HashMap<String, f64> = json
            .get(&base_lower)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .ok_or_else(|| {
                UPSTREAM_METRICS.record_error("fawaz");
                Error::Internal("Invalid response from FawazAhmed: missing rates".to_string())
            })?;

        Ok(RateCollection {
            base_code: base.to_uppercase(),
            rates: rates_map,
            timestamp: chrono::Utc::now(),
            source: Source::FawazAhmed,
        })
    }

    async fn get_historical_rates(&self, _base: &str, _date: NaiveDate) -> Result<HistoricalRate> {
        Err(Error::Internal(
            "Historical rates are not supported by this upstream provider".to_string(),
        ))
    }

    fn name(&self) -> &'static str {
        "fawaz"
    }

    fn is_healthy(&self) -> bool {
        self.healthy.load(std::sync::atomic::Ordering::SeqCst)
    }
}
