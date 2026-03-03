use std::time::Duration;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use super::{Cache, MemoryCache, SqliteCache};
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

    async fn set(&self, key: K, value: V, ttl: Option<Duration>) -> Result<(), Error> {
        self.l1.set(key.clone(), value.clone(), ttl).await?;
        <SqliteCache as Cache<K, V>>::set(&self.l2, key, value, ttl).await?;
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
