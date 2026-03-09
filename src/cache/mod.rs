pub mod memory;
pub mod metrics;
pub mod sqlite;
pub mod tiered;

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use memory::MemoryCache;
pub use metrics::CacheMetrics;
pub use sqlite::SqliteCache;
pub use tiered::TieredCache;

use crate::Error;

/// Information about the upstream API request that fetched the data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamRequestDetails {
    /// Full endpoint URL
    pub endpoint: String,
    /// HTTP method (GET, POST, etc.)
    pub method: String,
    /// Headers used (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// Payload/body (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

impl UpstreamRequestDetails {
    /// Create a simple GET request
    pub fn get(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            method: "GET".to_string(),
            headers: None,
            payload: None,
        }
    }

    /// Create a request with method
    pub fn with_method(endpoint: impl Into<String>, method: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            method: method.into(),
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

/// Cache entry with metadata for transparency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<V> {
    /// The cached value
    pub value: V,
    /// When the data was originally fetched from upstream
    pub retrieved_at: DateTime<Utc>,
    /// Details of the upstream API request
    pub upstream_request: UpstreamRequestDetails,
}

/// Result from cache get operation with metadata
#[derive(Debug, Clone)]
pub struct CacheResult<V> {
    /// The cached value
    pub value: V,
    /// When the data was originally fetched from upstream
    pub retrieved_at: DateTime<Utc>,
    /// Details of the upstream API request
    pub upstream_request: UpstreamRequestDetails,
    /// When the data was cached (for L2 cache)
    pub cached_at: Option<DateTime<Utc>>,
    /// Which cache level served the data
    pub cache_level: CacheLevel,
}

#[derive(Debug, Clone, Copy)]
pub enum CacheLevel {
    L1, // Memory cache
    L2, // SQLite cache
}

impl CacheLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            CacheLevel::L1 => "memory",
            CacheLevel::L2 => "persistent",
        }
    }
}

pub type RateCache = TieredCache<String, crate::models::RateCollection>;

#[async_trait]
pub trait Cache<K, V>: Send + Sync
where
    K: Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    async fn get(&self, key: &K) -> Result<Option<V>, Error>;
    async fn get_with_metadata(&self, key: &K) -> Result<Option<CacheResult<V>>, Error>;
    async fn set_with_metadata(
        &self,
        key: K,
        value: V,
        upstream_request: UpstreamRequestDetails,
        ttl: Option<Duration>,
    ) -> Result<(), Error>;
    async fn set(&self, key: K, value: V, ttl: Option<Duration>) -> Result<(), Error>;
    async fn delete(&self, key: &K) -> Result<(), Error>;
    async fn clear(&self) -> Result<(), Error>;
}

pub fn create_rate_cache(config: &crate::config::CacheConfig, pool: sqlx::SqlitePool) -> RateCache {
    let l1 = MemoryCache::new(config.max_capacity, Duration::from_secs(config.ttl_seconds));
    let l2 = SqliteCache::new(pool);
    TieredCache::new(l1, l2)
}
