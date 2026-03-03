use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::ratelimit::ApiKey;
use crate::Error;

pub struct ApiKeyStore {
    pool: SqlitePool,
    cache: Arc<RwLock<HashMap<String, ApiKey>>>,
}

impl ApiKeyStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn initialize(&self) -> Result<(), Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS api_keys (
                key TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                is_active INTEGER NOT NULL DEFAULT 1
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        self.load_cache().await?;

        Ok(())
    }

    async fn load_cache(&self) -> Result<(), Error> {
        let rows: Vec<(String, String, i64)> =
            sqlx::query_as("SELECT key, name, is_active FROM api_keys WHERE is_active = 1")
                .fetch_all(&self.pool)
                .await?;

        let mut cache = self.cache.write().await;
        cache.clear();

        for (key, name, is_active) in rows {
            let api_key = ApiKey {
                key: key.clone(),
                name,
                is_active: is_active != 0,
            };
            cache.insert(key, api_key);
        }

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Option<ApiKey> {
        let cache = self.cache.read().await;
        cache.get(key).cloned()
    }

    pub async fn create(&self, api_key: ApiKey) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO api_keys (key, name, is_active)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(&api_key.key)
        .bind(&api_key.name)
        .bind(api_key.is_active as i64)
        .execute(&self.pool)
        .await?;

        let mut cache = self.cache.write().await;
        cache.insert(api_key.key.clone(), api_key);

        Ok(())
    }

    pub async fn deactivate(&self, key: &str) -> Result<(), Error> {
        sqlx::query("UPDATE api_keys SET is_active = 0 WHERE key = ?")
            .bind(key)
            .execute(&self.pool)
            .await?;

        let mut cache = self.cache.write().await;
        cache.remove(key);

        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<ApiKey>, Error> {
        let cache = self.cache.read().await;
        Ok(cache.values().cloned().collect())
    }
}
