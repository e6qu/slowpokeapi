use std::sync::Arc;

use crate::cache::Cache;
use crate::models::RateCollection;
use crate::sync::{CrdtDocument, SyncError, SyncResult};

pub struct Reconciler {
    document: Arc<tokio::sync::RwLock<CrdtDocument>>,
    cache: Arc<dyn Cache<String, RateCollection>>,
}

impl Reconciler {
    pub fn new(
        document: Arc<tokio::sync::RwLock<CrdtDocument>>,
        cache: Arc<dyn Cache<String, RateCollection>>,
    ) -> Self {
        Self { document, cache }
    }

    pub async fn reconcile(&self, base_code: &str) -> SyncResult<()> {
        let cache_key = format!("latest:{base_code}");

        let cached_rates = self
            .cache
            .get(&cache_key)
            .await
            .map_err(|e| SyncError::PersistenceError(e.to_string()))?;

        let doc = self.document.read().await;
        let crdt_rates = doc.get_rates()?;

        if crdt_rates.base_code != base_code {
            return Ok(());
        }

        match cached_rates {
            Some(cached) => {
                if crdt_rates.timestamp > cached.timestamp {
                    self.cache
                        .set(cache_key, crdt_rates, None)
                        .await
                        .map_err(|e| SyncError::PersistenceError(e.to_string()))?;
                }
            }
            None => {
                self.cache
                    .set(cache_key, crdt_rates, None)
                    .await
                    .map_err(|e| SyncError::PersistenceError(e.to_string()))?;
            }
        }

        Ok(())
    }

    pub async fn reconcile_all(&self, base_codes: &[&str]) -> SyncResult<Vec<SyncError>> {
        let mut errors = Vec::new();

        for base_code in base_codes {
            if let Err(e) = self.reconcile(base_code).await {
                errors.push(e);
            }
        }

        Ok(errors)
    }
}
