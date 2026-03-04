use crate::storage::ApiKeyStore;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

const SAFETY_FACTOR: f64 = 0.5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub global_requests_per_second: u64,
    pub global_burst_capacity: u64,
    pub authenticated_requests_per_second: u64,
    pub authenticated_burst_capacity: u64,
    pub anonymous_requests_per_second: u64,
    pub anonymous_burst_capacity: u64,
    pub jitter_max_seconds: u64,
    pub backoff_base_seconds: u64,
    pub backoff_max_seconds: u64,
    pub backpressure_threshold: f64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            global_requests_per_second: 500,
            global_burst_capacity: 1000,
            authenticated_requests_per_second: 50,
            authenticated_burst_capacity: 100,
            anonymous_requests_per_second: 10,
            anonymous_burst_capacity: 20,
            jitter_max_seconds: 5,
            backoff_base_seconds: 1,
            backoff_max_seconds: 60,
            backpressure_threshold: 0.8,
        }
    }
}

impl RateLimitConfig {
    pub fn effective_global_rate(&self) -> u64 {
        ((self.global_requests_per_second as f64) * SAFETY_FACTOR) as u64
    }

    pub fn effective_global_burst(&self) -> u64 {
        ((self.global_burst_capacity as f64) * SAFETY_FACTOR) as u64
    }

    pub fn effective_authenticated_rate(&self) -> u64 {
        ((self.authenticated_requests_per_second as f64) * SAFETY_FACTOR) as u64
    }

    pub fn effective_authenticated_burst(&self) -> u64 {
        ((self.authenticated_burst_capacity as f64) * SAFETY_FACTOR) as u64
    }

    pub fn effective_anonymous_rate(&self) -> u64 {
        ((self.anonymous_requests_per_second as f64) * SAFETY_FACTOR) as u64
    }

    pub fn effective_anonymous_burst(&self) -> u64 {
        ((self.anonymous_burst_capacity as f64) * SAFETY_FACTOR) as u64
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub name: String,
    pub is_active: bool,
}

impl ApiKey {
    pub fn new(key: String, name: String) -> Self {
        Self {
            key,
            name,
            is_active: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenBucket {
    capacity: u64,
    tokens: f64,
    refill_rate: f64,
    last_refill: Instant,
}

impl TokenBucket {
    pub fn new(capacity: u64, refill_rate: u64) -> Self {
        Self {
            capacity,
            tokens: capacity as f64,
            refill_rate: refill_rate as f64,
            last_refill: Instant::now(),
        }
    }

    pub fn try_consume(&mut self, tokens: u64) -> bool {
        self.refill();

        let tokens_needed = tokens as f64;
        if self.tokens >= tokens_needed {
            self.tokens -= tokens_needed;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        let tokens_to_add = elapsed.as_secs_f64() * self.refill_rate;

        self.tokens = (self.tokens + tokens_to_add).min(self.capacity as f64);
        self.last_refill = now;
    }

    pub fn available_tokens(&mut self) -> u64 {
        self.refill();
        self.tokens.max(0.0) as u64
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    pub fn utilization(&mut self) -> f64 {
        self.refill();
        1.0 - (self.tokens / self.capacity as f64)
    }

    pub fn time_until_available(&mut self, tokens: u64) -> Duration {
        self.refill();
        let tokens_needed = tokens as f64;
        if self.tokens >= tokens_needed {
            Duration::from_secs(0)
        } else if self.refill_rate == 0.0 {
            Duration::from_secs(u64::MAX)
        } else {
            let tokens_deficit = tokens_needed - self.tokens;
            let seconds = tokens_deficit / self.refill_rate;
            Duration::from_secs_f64(seconds)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientBackoff {
    consecutive_rejections: u32,
    last_rejection: Instant,
}

impl Default for ClientBackoff {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBackoff {
    pub fn new() -> Self {
        Self {
            consecutive_rejections: 0,
            last_rejection: Instant::now(),
        }
    }

    pub fn record_rejection(&mut self) {
        self.consecutive_rejections += 1;
        self.last_rejection = Instant::now();
    }

    pub fn record_success(&mut self) {
        self.consecutive_rejections = 0;
    }

    pub fn calculate_backoff(&self, config: &RateLimitConfig) -> Duration {
        if self.consecutive_rejections == 0 {
            return Duration::from_secs(0);
        }

        let base = config.backoff_base_seconds;
        let max = config.backoff_max_seconds;

        let exponential =
            base.saturating_mul(1 << self.consecutive_rejections.saturating_sub(1).min(10));
        let capped = exponential.min(max);

        Duration::from_secs(capped)
    }

    pub fn should_allow_after_backoff(&self, config: &RateLimitConfig) -> bool {
        if self.consecutive_rejections == 0 {
            return true;
        }

        let backoff = self.calculate_backoff(config);
        Instant::now().duration_since(self.last_rejection) >= backoff
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    pub limit: u64,
    pub remaining: u64,
    pub reset_after: Duration,
    pub retry_after: Option<Duration>,
    pub backpressure: bool,
}

#[derive(Debug)]
pub enum RateLimitError {
    GlobalLimitExceeded { retry_after: Duration },
    UserLimitExceeded { retry_after: Duration },
    IpLimitExceeded { retry_after: Duration },
    InvalidApiKey,
    BackoffRequired { retry_after: Duration },
}

pub struct RateLimiter {
    config: RateLimitConfig,
    global_bucket: RwLock<TokenBucket>,
    user_buckets: RwLock<HashMap<String, TokenBucket>>,
    ip_buckets: RwLock<HashMap<String, TokenBucket>>,
    client_backoffs: RwLock<HashMap<String, ClientBackoff>>,
    api_key_store: Arc<ApiKeyStore>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig, api_key_store: Arc<ApiKeyStore>) -> Self {
        let global_bucket = TokenBucket::new(
            config.effective_global_burst(),
            config.effective_global_rate(),
        );

        Self {
            global_bucket: RwLock::new(global_bucket),
            user_buckets: RwLock::new(HashMap::new()),
            ip_buckets: RwLock::new(HashMap::new()),
            client_backoffs: RwLock::new(HashMap::new()),
            api_key_store,
            config,
        }
    }

    pub async fn check_rate_limit(
        &self,
        api_key: Option<&str>,
        client_ip: &str,
    ) -> Result<RateLimitInfo, RateLimitError> {
        if !self.config.enabled {
            return Ok(RateLimitInfo {
                limit: 0,
                remaining: 0,
                reset_after: Duration::from_secs(0),
                retry_after: None,
                backpressure: false,
            });
        }

        let client_id = api_key.unwrap_or(client_ip);
        let mut backoffs = self.client_backoffs.write().await;

        let backoff = backoffs
            .entry(client_id.to_string())
            .or_insert_with(ClientBackoff::default);

        if !backoff.should_allow_after_backoff(&self.config) {
            let retry_after = backoff.calculate_backoff(&self.config);
            return Err(RateLimitError::BackoffRequired {
                retry_after: self.add_jitter(retry_after),
            });
        }

        drop(backoffs);

        {
            let mut global = self.global_bucket.write().await;
            let utilization = global.utilization();

            if utilization > self.config.backpressure_threshold {
                if !global.try_consume(1) {
                    let retry_after = global.time_until_available(1);
                    self.record_rejection(client_id).await;
                    return Err(RateLimitError::GlobalLimitExceeded {
                        retry_after: self.add_jitter(retry_after),
                    });
                }

                return Ok(RateLimitInfo {
                    limit: global.capacity(),
                    remaining: global.available_tokens(),
                    reset_after: Duration::from_secs(0),
                    retry_after: Some(Duration::from_secs(1)),
                    backpressure: true,
                });
            }

            if !global.try_consume(1) {
                let retry_after = global.time_until_available(1);
                self.record_rejection(client_id).await;
                return Err(RateLimitError::GlobalLimitExceeded {
                    retry_after: self.add_jitter(retry_after),
                });
            }
        }

        if let Some(key) = api_key {
            self.check_user_limit(key, client_id).await
        } else {
            self.check_ip_limit(client_ip, client_id).await
        }
    }

    async fn check_user_limit(
        &self,
        api_key: &str,
        client_id: &str,
    ) -> Result<RateLimitInfo, RateLimitError> {
        let key_info = self
            .api_key_store
            .get(api_key)
            .await
            .ok_or(RateLimitError::InvalidApiKey)?;

        if !key_info.is_active {
            return Err(RateLimitError::InvalidApiKey);
        }

        let mut buckets = self.user_buckets.write().await;

        let bucket = buckets.entry(api_key.to_string()).or_insert_with(|| {
            TokenBucket::new(
                self.config.effective_authenticated_burst(),
                self.config.effective_authenticated_rate(),
            )
        });

        if bucket.try_consume(1) {
            self.record_success(client_id).await;
            Ok(RateLimitInfo {
                limit: bucket.capacity(),
                remaining: bucket.available_tokens(),
                reset_after: Duration::from_secs(0),
                retry_after: None,
                backpressure: false,
            })
        } else {
            let retry_after = bucket.time_until_available(1);
            self.record_rejection(client_id).await;
            Err(RateLimitError::UserLimitExceeded {
                retry_after: self.add_jitter(retry_after),
            })
        }
    }

    async fn check_ip_limit(
        &self,
        client_ip: &str,
        client_id: &str,
    ) -> Result<RateLimitInfo, RateLimitError> {
        let mut buckets = self.ip_buckets.write().await;

        let bucket = buckets.entry(client_ip.to_string()).or_insert_with(|| {
            TokenBucket::new(
                self.config.effective_anonymous_burst(),
                self.config.effective_anonymous_rate(),
            )
        });

        if bucket.try_consume(1) {
            self.record_success(client_id).await;
            Ok(RateLimitInfo {
                limit: bucket.capacity(),
                remaining: bucket.available_tokens(),
                reset_after: Duration::from_secs(0),
                retry_after: None,
                backpressure: false,
            })
        } else {
            let retry_after = bucket.time_until_available(1);
            self.record_rejection(client_id).await;
            Err(RateLimitError::IpLimitExceeded {
                retry_after: self.add_jitter(retry_after),
            })
        }
    }

    fn add_jitter(&self, duration: Duration) -> Duration {
        if self.config.jitter_max_seconds == 0 {
            return duration;
        }

        let mut rng = rand::thread_rng();
        let jitter_secs = rng.gen_range(0..=self.config.jitter_max_seconds);
        duration + Duration::from_secs(jitter_secs)
    }

    async fn record_rejection(&self, client_id: &str) {
        let mut backoffs = self.client_backoffs.write().await;
        if let Some(backoff) = backoffs.get_mut(client_id) {
            backoff.record_rejection();
        } else {
            let mut backoff = ClientBackoff::new();
            backoff.record_rejection();
            backoffs.insert(client_id.to_string(), backoff);
        }
    }

    async fn record_success(&self, client_id: &str) {
        let mut backoffs = self.client_backoffs.write().await;
        if let Some(backoff) = backoffs.get_mut(client_id) {
            backoff.record_success();
        }
    }

    pub async fn get_quota_info(&self, api_key: &str) -> Option<RateLimitInfo> {
        let key_info = self.api_key_store.get(api_key).await?;

        if !key_info.is_active {
            return None;
        }

        let mut buckets = self.user_buckets.write().await;

        let bucket = buckets.entry(api_key.to_string()).or_insert_with(|| {
            TokenBucket::new(
                self.config.effective_authenticated_burst(),
                self.config.effective_authenticated_rate(),
            )
        });

        Some(RateLimitInfo {
            limit: bucket.capacity(),
            remaining: bucket.available_tokens(),
            reset_after: bucket.time_until_available(1),
            retry_after: None,
            backpressure: false,
        })
    }

    pub async fn cleanup_stale_entries(&self, max_age: Duration) {
        let now = Instant::now();

        let mut ip_buckets = self.ip_buckets.write().await;
        ip_buckets.retain(|_, bucket| now.duration_since(bucket.last_refill) < max_age);

        let mut backoffs = self.client_backoffs.write().await;
        backoffs.retain(|_, backoff| now.duration_since(backoff.last_rejection) < max_age);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_token_bucket_creation() {
        let bucket = TokenBucket::new(100, 10);
        assert_eq!(bucket.capacity(), 100);
    }

    #[test]
    fn test_token_refill() {
        let mut bucket = TokenBucket::new(100, 100);
        bucket.try_consume(100);
        assert_eq!(bucket.available_tokens(), 0);
        sleep(Duration::from_millis(250));
        let tokens = bucket.available_tokens();
        assert!(
            (20..=35).contains(&tokens),
            "Expected 20-35 tokens after 250ms at 100/s refill rate, got {tokens}"
        );
    }

    #[test]
    fn test_consume_more_than_available() {
        let mut bucket = TokenBucket::new(100, 10);
        assert!(bucket.try_consume(50));
        assert!(!bucket.try_consume(60));
    }

    #[test]
    fn test_effective_rates() {
        let config = RateLimitConfig::default();
        assert_eq!(config.effective_global_rate(), 250);
        assert_eq!(config.effective_authenticated_rate(), 25);
        assert_eq!(config.effective_anonymous_rate(), 5);
    }

    #[test]
    fn test_backoff_calculation() {
        let config = RateLimitConfig::default();
        let mut backoff = ClientBackoff::new();

        assert_eq!(backoff.calculate_backoff(&config), Duration::from_secs(0));

        backoff.record_rejection();
        assert_eq!(backoff.calculate_backoff(&config), Duration::from_secs(1));

        backoff.record_rejection();
        assert_eq!(backoff.calculate_backoff(&config), Duration::from_secs(2));

        backoff.record_success();
        assert_eq!(backoff.calculate_backoff(&config), Duration::from_secs(0));
    }

    #[test]
    fn test_utilization() {
        let mut bucket = TokenBucket::new(100, 10);
        assert!((bucket.utilization() - 0.0).abs() < 0.01);

        bucket.try_consume(50);
        assert!((bucket.utilization() - 0.5).abs() < 0.01);

        bucket.try_consume(50);
        assert!((bucket.utilization() - 1.0).abs() < 0.01);
    }
}
