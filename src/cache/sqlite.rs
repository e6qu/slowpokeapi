use std::time::Duration;

use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::SqlitePool;

use super::{Cache, CacheEntry, CacheLevel, CacheResult, UpstreamRequestDetails};
use crate::Error;

pub struct SqliteCache {
    pool: SqlitePool,
}

impl SqliteCache {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn cleanup_expired(&self) -> Result<(), Error> {
        let now = chrono::Utc::now().timestamp();
        sqlx::query("DELETE FROM cache_entries WHERE expires_at IS NOT NULL AND expires_at < ?")
            .bind(now)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl<K, V> Cache<K, V> for SqliteCache
where
    K: ToString + Send + Sync + 'static,
    V: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    async fn get(&self, key: &K) -> Result<Option<V>, Error> {
        let key_str = key.to_string();
        let now = chrono::Utc::now().timestamp();

        let result: Option<(String,)> = sqlx::query_as(
            "SELECT value FROM cache_entries WHERE key = ? AND (expires_at IS NULL OR expires_at > ?)",
        )
        .bind(&key_str)
        .bind(now)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some((value_json,)) => {
                let entry: CacheEntry<V> = serde_json::from_str(&value_json).map_err(|e| {
                    Error::Internal(format!("Failed to deserialize cache entry: {e}"))
                })?;
                Ok(Some(entry.value))
            }
            None => Ok(None),
        }
    }

    async fn get_with_metadata(&self, key: &K) -> Result<Option<CacheResult<V>>, Error> {
        let key_str = key.to_string();
        let now = chrono::Utc::now().timestamp();

        let result: Option<(String, i64)> = sqlx::query_as(
            "SELECT value, created_at FROM cache_entries WHERE key = ? AND (expires_at IS NULL OR expires_at > ?)",
        )
        .bind(&key_str)
        .bind(now)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some((value_json, created_at)) => {
                let entry: CacheEntry<V> = serde_json::from_str(&value_json).map_err(|e| {
                    Error::Internal(format!("Failed to deserialize cache entry: {e}"))
                })?;

                let cached_at = Utc.timestamp_opt(created_at, 0).single();

                Ok(Some(CacheResult {
                    value: entry.value,
                    retrieved_at: entry.retrieved_at,
                    upstream_request: entry.upstream_request,
                    cached_at,
                    cache_level: CacheLevel::L2,
                }))
            }
            None => Ok(None),
        }
    }

    async fn set(&self, key: K, value: V, ttl: Option<Duration>) -> Result<(), Error> {
        let key_str = key.to_string();
        let entry = CacheEntry {
            value,
            retrieved_at: Utc::now(),
            upstream_request: UpstreamRequestDetails::get("unknown"),
        };
        let value_json = serde_json::to_string(&entry)
            .map_err(|e| Error::Internal(format!("Failed to serialize cache entry: {e}")))?;

        let expires_at = ttl.map(|d| {
            let secs = d.as_secs();
            let extra_second = if d.subsec_nanos() > 0 { 1 } else { 0 };
            chrono::Utc::now().timestamp() + secs as i64 + extra_second
        });

        sqlx::query(
            r#"
            INSERT INTO cache_entries (key, value, expires_at)
            VALUES (?, ?, ?)
            ON CONFLICT(key) DO UPDATE SET value = excluded.value, expires_at = excluded.expires_at, created_at = strftime('%s', 'now')
            "#,
        )
        .bind(&key_str)
        .bind(&value_json)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn set_with_metadata(
        &self,
        key: K,
        value: V,
        upstream_request: UpstreamRequestDetails,
        ttl: Option<Duration>,
    ) -> Result<(), Error> {
        let key_str = key.to_string();
        let entry = CacheEntry {
            value,
            retrieved_at: Utc::now(),
            upstream_request,
        };
        let value_json = serde_json::to_string(&entry)
            .map_err(|e| Error::Internal(format!("Failed to serialize cache entry: {e}")))?;

        let expires_at = ttl.map(|d| {
            let secs = d.as_secs();
            let extra_second = if d.subsec_nanos() > 0 { 1 } else { 0 };
            chrono::Utc::now().timestamp() + secs as i64 + extra_second
        });

        sqlx::query(
            r#"
            INSERT INTO cache_entries (key, value, expires_at)
            VALUES (?, ?, ?)
            ON CONFLICT(key) DO UPDATE SET value = excluded.value, expires_at = excluded.expires_at, created_at = strftime('%s', 'now')
            "#,
        )
        .bind(&key_str)
        .bind(&value_json)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, key: &K) -> Result<(), Error> {
        let key_str = key.to_string();
        sqlx::query("DELETE FROM cache_entries WHERE key = ?")
            .bind(&key_str)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn clear(&self) -> Result<(), Error> {
        sqlx::query("DELETE FROM cache_entries")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
