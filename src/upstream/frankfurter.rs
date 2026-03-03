use async_trait::async_trait;
use chrono::{NaiveDate, TimeZone};
use reqwest::StatusCode;
use std::collections::HashMap;
use std::sync::Arc;

use super::{HttpClient, Upstream};
use crate::models::{HistoricalRate, RateCollection, Source};
use crate::upstream::metrics::UPSTREAM_METRICS;
use crate::{Error, Result};

const BASE_URL: &str = "https://api.frankfurter.app";

pub struct FrankfurterClient {
    http: Arc<HttpClient>,
    healthy: std::sync::atomic::AtomicBool,
}

impl FrankfurterClient {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self {
            http,
            healthy: std::sync::atomic::AtomicBool::new(true),
        }
    }
}

#[async_trait]
impl Upstream for FrankfurterClient {
    async fn get_latest_rates(&self, base: &str) -> Result<RateCollection> {
        let start = std::time::Instant::now();
        let url = format!("{BASE_URL}/latest?from={base}");

        UPSTREAM_METRICS.record_request("frankfurter");

        let response = self.http.inner().get(&url).send().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("frankfurter");
            self.healthy
                .store(false, std::sync::atomic::Ordering::SeqCst);
            Error::Internal(format!("Frankfurter request failed: {e}"))
        })?;

        UPSTREAM_METRICS.observe_latency("frankfurter", start.elapsed());

        if response.status() == StatusCode::NOT_FOUND {
            return Err(Error::NotFound(format!("Currency not found: {base}")));
        }

        if !response.status().is_success() {
            UPSTREAM_METRICS.record_error("frankfurter");
            return Err(Error::Internal(format!(
                "Frankfurter returned status: {}",
                response.status()
            )));
        }

        let json: serde_json::Value = response.json().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("frankfurter");
            Error::Internal(format!("Failed to parse Frankfurter response: {e}"))
        })?;

        let rates_map: HashMap<String, f64> = json
            .get("rates")
            .and_then(|r| serde_json::from_value(r.clone()).ok())
            .ok_or_else(|| {
                UPSTREAM_METRICS.record_error("frankfurter");
                Error::Internal("Invalid response from Frankfurter: missing rates".to_string())
            })?;

        let date_str = json
            .get("date")
            .and_then(|d| d.as_str())
            .unwrap_or("2024-01-01");

        let timestamp = chrono::Utc.from_utc_datetime(
            &chrono::NaiveDateTime::parse_from_str(
                &format!("{date_str} 00:00:00"),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap_or_else(|_| chrono::Utc::now().naive_utc()),
        );

        Ok(RateCollection {
            base_code: base.to_uppercase(),
            rates: rates_map,
            timestamp,
            source: Source::Frankfurter,
        })
    }

    async fn get_historical_rates(&self, base: &str, date: NaiveDate) -> Result<HistoricalRate> {
        let start = std::time::Instant::now();
        let date_str = date.format("%Y-%m-%d");
        let url = format!("{BASE_URL}/{date_str}?from={base}");

        UPSTREAM_METRICS.record_request("frankfurter");

        let response = self.http.inner().get(&url).send().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("frankfurter");
            self.healthy
                .store(false, std::sync::atomic::Ordering::SeqCst);
            Error::Internal(format!("Frankfurter request failed: {e}"))
        })?;

        UPSTREAM_METRICS.observe_latency("frankfurter", start.elapsed());

        if response.status() == StatusCode::NOT_FOUND {
            return Err(Error::NotFound(format!("Rates not found for date: {date}")));
        }

        if !response.status().is_success() {
            UPSTREAM_METRICS.record_error("frankfurter");
            return Err(Error::Internal(format!(
                "Frankfurter returned status: {}",
                response.status()
            )));
        }

        let json: serde_json::Value = response.json().await.map_err(|e| {
            UPSTREAM_METRICS.record_error("frankfurter");
            Error::Internal(format!("Failed to parse Frankfurter response: {e}"))
        })?;

        let rates_map: HashMap<String, f64> = json
            .get("rates")
            .and_then(|r| serde_json::from_value(r.clone()).ok())
            .ok_or_else(|| {
                UPSTREAM_METRICS.record_error("frankfurter");
                Error::Internal("Invalid response from Frankfurter: missing rates".to_string())
            })?;

        Ok(HistoricalRate {
            base_code: base.to_uppercase(),
            date,
            rates: rates_map,
            source: Source::Frankfurter,
        })
    }

    fn name(&self) -> &'static str {
        "frankfurter"
    }

    fn is_healthy(&self) -> bool {
        self.healthy.load(std::sync::atomic::Ordering::SeqCst)
    }
}
