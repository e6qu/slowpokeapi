pub mod token_bucket;

pub use token_bucket::TokenBucket;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_second: u64,
    pub burst_capacity: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 10,
            burst_capacity: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub name: String,
    pub rate_limit: RateLimitConfig,
    pub is_active: bool,
}

impl ApiKey {
    pub fn new(key: String, name: String, rate_limit: RateLimitConfig) -> Self {
        Self {
            key,
            name,
            rate_limit,
            is_active: true,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct QuotaInfo {
    pub api_key: String,
    pub limit: u64,
    pub remaining: u64,
    pub reset_seconds: u64,
}
