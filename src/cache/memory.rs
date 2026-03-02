use std::time::Duration;

use async_trait::async_trait;
use moka::sync::Cache as MokaCache;

use super::Cache;
use crate::Error;

pub struct MemoryCache<K, V>
where
    K: std::hash::Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    inner: MokaCache<K, V>,
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
        Ok(self.inner.get(key))
    }

    async fn set(&self, key: K, value: V, _ttl: Option<Duration>) -> Result<(), Error> {
        self.inner.insert(key, value);
        Ok(())
    }

    async fn delete(&self, key: &K) -> Result<(), Error> {
        self.inner.invalidate(key);
        Ok(())
    }

    async fn clear(&self) -> Result<(), Error> {
        self.inner.invalidate_all();
        Ok(())
    }
}
