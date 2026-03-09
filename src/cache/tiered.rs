use std::time::Duration;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use super::{Cache, CacheResult, MemoryCache, SqliteCache, UpstreamRequestDetails};
use crate::Error;

pub struct TieredCache<K, V>
where
    K: std::hash::Hash + Eq + Clone + ToString + Send + Sync + 'static,
    V: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    l1: MemoryCache<K, V>,
    l2: SqliteCache,
}

impl<K, V> TieredCache<K, V>
where
    K: std::hash::Hash + Eq + Clone + ToString + Send + Sync + 'static,
    V: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    pub fn new(l1: MemoryCache<K, V>, l2: SqliteCache) -> Self {
        Self { l1, l2 }
    }
}

#[async_trait]
impl<K, V> Cache<K, V> for TieredCache<K, V>
where
    K: std::hash::Hash + Eq + Clone + ToString + Send + Sync + 'static,
    V: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    async fn get(&self, key: &K) -> Result<Option<V>, Error> {
        if let Some(value) = self.l1.get(key).await? {
            return Ok(Some(value));
        }

        if let Some(value) = <SqliteCache as Cache<K, V>>::get(&self.l2, key).await? {
            self.l1.set(key.clone(), value.clone(), None).await?;
            return Ok(Some(value));
        }

        Ok(None)
    }

    async fn get_with_metadata(&self, key: &K) -> Result<Option<CacheResult<V>>, Error> {
        // Try L1 (memory cache) first
        if let Some(result) = self.l1.get_with_metadata(key).await? {
            return Ok(Some(result));
        }

        // Try L2 (SQLite cache)
        if let Some(result) = <SqliteCache as Cache<K, V>>::get_with_metadata(&self.l2, key).await?
        {
            // Populate L1 cache with the value
            self.l1
                .set_with_metadata(
                    key.clone(),
                    result.value.clone(),
                    result.upstream_request.clone(),
                    None,
                )
                .await?;
            return Ok(Some(result));
        }

        Ok(None)
    }

    async fn set(&self, key: K, value: V, ttl: Option<Duration>) -> Result<(), Error> {
        self.l1.set(key.clone(), value.clone(), ttl).await?;
        <SqliteCache as Cache<K, V>>::set(&self.l2, key, value, ttl).await?;
        Ok(())
    }

    async fn set_with_metadata(
        &self,
        key: K,
        value: V,
        upstream_request: UpstreamRequestDetails,
        ttl: Option<Duration>,
    ) -> Result<(), Error> {
        self.l1
            .set_with_metadata(key.clone(), value.clone(), upstream_request.clone(), ttl)
            .await?;
        <SqliteCache as Cache<K, V>>::set_with_metadata(
            &self.l2,
            key,
            value,
            upstream_request,
            ttl,
        )
        .await?;
        Ok(())
    }

    async fn delete(&self, key: &K) -> Result<(), Error> {
        self.l1.delete(key).await?;
        <SqliteCache as Cache<K, V>>::delete(&self.l2, key).await?;
        Ok(())
    }

    async fn clear(&self) -> Result<(), Error> {
        self.l1.clear().await?;
        <SqliteCache as Cache<K, V>>::clear(&self.l2).await?;
        Ok(())
    }
}
