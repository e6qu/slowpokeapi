use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use moka::sync::Cache as MokaCache;

use super::metrics::CACHE_METRICS;
use super::{Cache, CacheEntry, CacheLevel, CacheResult, UpstreamRequestDetails};
use crate::Error;

pub struct MemoryCache<K, V>
where
    K: std::hash::Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    inner: MokaCache<K, CacheEntry<V>>,
}

impl<K, V> MemoryCache<K, V>
where
    K: std::hash::Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new(max_capacity: u64, ttl: Duration) -> Self {
        let inner = MokaCache::builder()
            .max_capacity(max_capacity)
            .time_to_live(ttl)
            .build();
        Self { inner }
    }
}

#[async_trait]
impl<K, V> Cache<K, V> for MemoryCache<K, V>
where
    K: std::hash::Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    async fn get(&self, key: &K) -> Result<Option<V>, Error> {
        let start = std::time::Instant::now();
        let result = self.inner.get(key);

        if result.is_some() {
            CACHE_METRICS.record_hit();
        } else {
            CACHE_METRICS.record_miss();
        }

        CACHE_METRICS.observe_latency("get", start.elapsed());
        Ok(result.map(|entry| entry.value))
    }

    async fn get_with_metadata(&self, key: &K) -> Result<Option<CacheResult<V>>, Error> {
        let start = std::time::Instant::now();
        let result = self.inner.get(key);

        let cache_result = result.map(|entry| CacheResult {
            value: entry.value.clone(),
            retrieved_at: entry.retrieved_at,
            upstream_request: entry.upstream_request,
            cached_at: Some(entry.retrieved_at),
            cache_level: CacheLevel::L1,
        });

        if cache_result.is_some() {
            CACHE_METRICS.record_hit();
        } else {
            CACHE_METRICS.record_miss();
        }

        CACHE_METRICS.observe_latency("get", start.elapsed());
        Ok(cache_result)
    }

    async fn set(&self, key: K, value: V, _ttl: Option<Duration>) -> Result<(), Error> {
        let start = std::time::Instant::now();
        let entry = CacheEntry {
            value,
            retrieved_at: Utc::now(),
            upstream_request: UpstreamRequestDetails::get("unknown"),
        };
        self.inner.insert(key, entry);
        CACHE_METRICS.record_set();
        CACHE_METRICS.observe_latency("set", start.elapsed());
        Ok(())
    }

    async fn set_with_metadata(
        &self,
        key: K,
        value: V,
        upstream_request: UpstreamRequestDetails,
        _ttl: Option<Duration>,
    ) -> Result<(), Error> {
        let start = std::time::Instant::now();
        let entry = CacheEntry {
            value,
            retrieved_at: Utc::now(),
            upstream_request,
        };
        self.inner.insert(key, entry);
        CACHE_METRICS.record_set();
        CACHE_METRICS.observe_latency("set", start.elapsed());
        Ok(())
    }

    async fn delete(&self, key: &K) -> Result<(), Error> {
        let start = std::time::Instant::now();
        self.inner.invalidate(key);
        CACHE_METRICS.record_delete();
        CACHE_METRICS.observe_latency("delete", start.elapsed());
        Ok(())
    }

    async fn clear(&self) -> Result<(), Error> {
        let start = std::time::Instant::now();
        self.inner.invalidate_all();
        CACHE_METRICS.observe_latency("clear", start.elapsed());
        Ok(())
    }
}
