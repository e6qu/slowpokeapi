use std::sync::Arc;

use super::circuit_breaker::CircuitBreaker;
use super::{CoinCapClient, CoinGeckoClient, FawazClient, FrankfurterClient, HttpClient, Upstream};
use crate::models::{HistoricalRate, RateCollection};
use crate::{Error, Result};

pub struct UpstreamManager {
    fiat_clients: Vec<Arc<dyn Upstream>>,
    fiat_circuit_breakers: Vec<Arc<CircuitBreaker>>,
    crypto_clients: Vec<Arc<dyn Upstream>>,
    crypto_circuit_breakers: Vec<Arc<CircuitBreaker>>,
}

impl UpstreamManager {
    pub fn new(http: Arc<HttpClient>) -> Self {
        let frankfurter = Arc::new(FrankfurterClient::new(http.clone()));
        let fawaz = Arc::new(FawazClient::new(http.clone()));

        let fiat_clients: Vec<Arc<dyn Upstream>> = vec![frankfurter, fawaz];
        let fiat_circuit_breakers = fiat_clients
            .iter()
            .map(|_| Arc::new(CircuitBreaker::default()))
            .collect();

        let coingecko = Arc::new(CoinGeckoClient::new(http.clone()));
        let coincap = Arc::new(CoinCapClient::new(http));

        let crypto_clients: Vec<Arc<dyn Upstream>> = vec![coingecko, coincap];
        let crypto_circuit_breakers = crypto_clients
            .iter()
            .map(|_| Arc::new(CircuitBreaker::default()))
            .collect();

        Self {
            fiat_clients,
            fiat_circuit_breakers,
            crypto_clients,
            crypto_circuit_breakers,
        }
    }

    fn is_crypto_currency(code: &str) -> bool {
        crate::upstream::is_crypto_currency(code)
    }

    async fn get_latest_from_clients(
        clients: &[Arc<dyn Upstream>],
        circuit_breakers: &[Arc<CircuitBreaker>],
        base: &str,
    ) -> Result<RateCollection> {
        let mut last_error = None;

        for (i, client) in clients.iter().enumerate() {
            let circuit = &circuit_breakers[i];

            if !circuit.is_call_allowed().await {
                continue;
            }

            match client.get_latest_rates(base).await {
                Ok(rates) => {
                    circuit.record_success().await;
                    return Ok(rates);
                }
                Err(e) => {
                    circuit.record_failure().await;
                    last_error = Some(e);
                    continue;
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| Error::Internal("All upstreams are unavailable".to_string())))
    }

    async fn get_historical_from_clients(
        clients: &[Arc<dyn Upstream>],
        circuit_breakers: &[Arc<CircuitBreaker>],
        base: &str,
        date: chrono::NaiveDate,
    ) -> Result<HistoricalRate> {
        let mut last_error = None;

        for (i, client) in clients.iter().enumerate() {
            let circuit = &circuit_breakers[i];

            if !circuit.is_call_allowed().await {
                continue;
            }

            match client.get_historical_rates(base, date).await {
                Ok(rates) => {
                    circuit.record_success().await;
                    return Ok(rates);
                }
                Err(e) => {
                    circuit.record_failure().await;
                    last_error = Some(e);
                    continue;
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| Error::Internal("All upstreams are unavailable".to_string())))
    }

    pub async fn get_latest_rates(&self, base: &str) -> Result<RateCollection> {
        if Self::is_crypto_currency(base) {
            Self::get_latest_from_clients(&self.crypto_clients, &self.crypto_circuit_breakers, base)
                .await
        } else {
            Self::get_latest_from_clients(&self.fiat_clients, &self.fiat_circuit_breakers, base)
                .await
        }
    }

    pub async fn get_historical_rates(
        &self,
        base: &str,
        date: chrono::NaiveDate,
    ) -> Result<HistoricalRate> {
        if Self::is_crypto_currency(base) {
            Self::get_historical_from_clients(
                &self.crypto_clients,
                &self.crypto_circuit_breakers,
                base,
                date,
            )
            .await
        } else {
            Self::get_historical_from_clients(
                &self.fiat_clients,
                &self.fiat_circuit_breakers,
                base,
                date,
            )
            .await
        }
    }

    pub fn healthy_count(&self) -> usize {
        self.fiat_clients.iter().filter(|c| c.is_healthy()).count()
            + self
                .crypto_clients
                .iter()
                .filter(|c| c.is_healthy())
                .count()
    }

    pub fn total_count(&self) -> usize {
        self.fiat_clients.len() + self.crypto_clients.len()
    }
}
