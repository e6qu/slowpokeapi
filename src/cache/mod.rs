pub mod memory;
pub mod metrics;
pub mod sqlite;
pub mod tiered;

use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use memory::MemoryCache;
pub use metrics::CacheMetrics;
pub use sqlite::SqliteCache;
pub use tiered::TieredCache;

use crate::Error;

/// Cache entry with metadata for transparency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<V> {
    /// The cached value
    pub value: V,
    /// When the data was originally fetched from upstream
    pub source_timestamp: DateTime<Utc>,
}

/// Result from cache get operation with metadata
#[derive(Debug, Clone)]
pub struct CacheResult<V> {
    /// The cached value
    pub value: V,
    /// When the data was originally fetched from upstream
    pub source_timestamp: DateTime<Utc>,
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
    async fn set(&self, key: K, value: V, ttl: Option<Duration>) -> Result<(), Error>;
    async fn delete(&self, key: &K) -> Result<(), Error>;
    async fn clear(&self) -> Result<(), Error>;
}

pub fn create_rate_cache(config: &crate::config::CacheConfig, pool: sqlx::SqlitePool) -> RateCache {
    let l1 = MemoryCache::new(config.max_capacity, Duration::from_secs(config.ttl_seconds));
    let l2 = SqliteCache::new(pool);
    TieredCache::new(l1, l2)
}
