use std::sync::Arc;
use tokio::sync::RwLock;

use crate::cache::Cache;
use crate::models::RateCollection;
use crate::sync::{CrdtDocument, SyncError, SyncResult};

pub struct SyncIntegration {
    document: Arc<RwLock<CrdtDocument>>,
    cache: Arc<dyn Cache<String, RateCollection>>,
}

impl SyncIntegration {
    pub fn new(
        document: Arc<RwLock<CrdtDocument>>,
        cache: Arc<dyn Cache<String, RateCollection>>,
    ) -> Self {
        Self { document, cache }
    }

    pub async fn on_cache_update(
        &self,
        _base_code: &str,
        rates: &RateCollection,
    ) -> SyncResult<()> {
        let mut doc = self.document.write().await;
        doc.update_rates(rates)?;
        Ok(())
    }

    pub async fn on_sync_update(&self, base_code: &str) -> SyncResult<()> {
        let doc = self.document.read().await;
        let rates = doc.get_rates()?;

        if rates.base_code == base_code {
            self.cache
                .set(format!("latest:{base_code}"), rates, None)
                .await
                .map_err(|e| SyncError::PersistenceError(e.to_string()))?;
        }

        Ok(())
    }

    pub async fn get_document_changes(&self) -> Vec<u8> {
        let mut doc = self.document.write().await;
        doc.get_changes_since()
    }

    pub async fn apply_document_changes(&self, changes: &[u8]) -> SyncResult<()> {
        let mut doc = self.document.write().await;
        doc.apply_changes(changes)?;
        Ok(())
    }
}
