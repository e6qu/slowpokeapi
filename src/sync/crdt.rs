use std::collections::HashMap;

use crate::models::{RateCollection, Source};
use crate::sync::{SyncError, SyncResult};

pub struct CrdtDocument {
    rates: HashMap<String, f64>,
    base_code: String,
    timestamp: i64,
}

impl CrdtDocument {
    pub fn new() -> Self {
        Self {
            rates: HashMap::new(),
            base_code: "USD".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn update_rates(&mut self, rates: &RateCollection) -> SyncResult<()> {
        self.rates = rates.rates.clone();
        self.base_code = rates.base_code.clone();
        self.timestamp = rates.timestamp.timestamp();
        Ok(())
    }

    pub fn get_rates(&self) -> SyncResult<RateCollection> {
        Ok(RateCollection {
            base_code: self.base_code.clone(),
            rates: self.rates.clone(),
            timestamp: chrono::DateTime::from_timestamp(self.timestamp, 0)
                .unwrap_or_else(chrono::Utc::now),
            source: Source::Cached,
        })
    }

    pub fn get_state(&self) -> Vec<u8> {
        bincode::serialize(&(&self.rates, &self.base_code, self.timestamp)).unwrap_or_else(|e| {
            tracing::error!("CRDT serialization failed: {}", e);
            Vec::new()
        })
    }

    pub fn apply_state(&mut self, state: &[u8]) -> SyncResult<()> {
        if state.is_empty() {
            return Err(SyncError::CrdtError("Cannot apply empty state".to_string()));
        }

        let (rates, base_code, timestamp): (HashMap<String, f64>, String, i64) =
            bincode::deserialize(state)
                .map_err(|e| SyncError::CrdtError(format!("Failed to deserialize: {e}")))?;

        self.rates = rates;
        self.base_code = base_code;
        self.timestamp = timestamp;
        Ok(())
    }
}

impl Default for CrdtDocument {
    fn default() -> Self {
        Self::new()
    }
}
