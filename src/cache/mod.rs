pub mod memory;
pub mod metrics;
pub mod sqlite;
pub mod tiered;

use std::time::Duration;

use async_trait::async_trait;

pub use memory::MemoryCache;
pub use metrics::CacheMetrics;
pub use sqlite::SqliteCache;
pub use tiered::TieredCache;

use crate::Error;

pub type RateCache = TieredCache<String, crate::models::RateCollection>;

#[async_trait]
pub trait Cache<K, V>: Send + Sync
where
    K: Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    async fn get(&self, key: &K) -> Result<Option<V>, Error>;
    async fn set(&self, key: K, value: V, ttl: Option<Duration>) -> Result<(), Error>;
    async fn delete(&self, key: &K) -> Result<(), Error>;
    async fn clear(&self) -> Result<(), Error>;
}

pub fn create_rate_cache(config: &crate::config::CacheConfig, pool: sqlx::SqlitePool) -> RateCache {
    let l1 = MemoryCache::new(config.max_capacity, Duration::from_secs(config.ttl_seconds));
    let l2 = SqliteCache::new(pool);
    TieredCache::new(l1, l2)
}
