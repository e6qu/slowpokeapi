use std::sync::Arc;

use super::circuit_breaker::CircuitBreaker;
use super::{FawazClient, FrankfurterClient, HttpClient, Upstream};
use crate::models::{HistoricalRate, RateCollection};
use crate::{Error, Result};

pub struct UpstreamManager {
    clients: Vec<Arc<dyn Upstream>>,
    circuit_breakers: Vec<Arc<CircuitBreaker>>,
}

impl UpstreamManager {
    pub fn new(http: Arc<HttpClient>) -> Self {
        let frankfurter = Arc::new(FrankfurterClient::new(http.clone()));
        let fawaz = Arc::new(FawazClient::new(http));

        let clients: Vec<Arc<dyn Upstream>> = vec![frankfurter, fawaz];
        let circuit_breakers = clients
            .iter()
            .map(|_| Arc::new(CircuitBreaker::default()))
            .collect();

        Self {
            clients,
            circuit_breakers,
        }
    }

    pub async fn get_latest_rates(&self, base: &str) -> Result<RateCollection> {
        let mut last_error = None;

        for (i, client) in self.clients.iter().enumerate() {
            let circuit = &self.circuit_breakers[i];

            if !circuit.is_call_allowed() {
                continue;
            }

            match client.get_latest_rates(base).await {
                Ok(rates) => {
                    circuit.record_success();
                    return Ok(rates);
                }
                Err(e) => {
                    circuit.record_failure();
                    last_error = Some(e);
                    continue;
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| Error::Internal("All upstreams are unavailable".to_string())))
    }

    pub async fn get_historical_rates(
        &self,
        base: &str,
        date: chrono::NaiveDate,
    ) -> Result<HistoricalRate> {
        let mut last_error = None;

        for (i, client) in self.clients.iter().enumerate() {
            let circuit = &self.circuit_breakers[i];

            if !circuit.is_call_allowed() {
                continue;
            }

            match client.get_historical_rates(base, date).await {
                Ok(rates) => {
                    circuit.record_success();
                    return Ok(rates);
                }
                Err(e) => {
                    circuit.record_failure();
                    last_error = Some(e);
                    continue;
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| Error::Internal("All upstreams are unavailable".to_string())))
    }

    pub fn healthy_count(&self) -> usize {
        self.clients.iter().filter(|c| c.is_healthy()).count()
    }

    pub fn total_count(&self) -> usize {
        self.clients.len()
    }
}
