use std::collections::HashMap;

use automerge::transaction::Transactable;
use automerge::{AutoCommit, ObjType, ReadDoc, ScalarValue, Value};
use chrono::{DateTime, Utc};

use crate::models::{RateCollection, Source};
use crate::sync::{SyncError, SyncResult};

const RATES_KEY: &str = "rates";
const METADATA_KEY: &str = "metadata";
const TIMESTAMP_KEY: &str = "timestamp";
const BASE_CODE_KEY: &str = "base_code";
const SOURCE_KEY: &str = "source";
const RATE_KEY: &str = "rate";

#[derive(Debug, Clone)]
pub struct RateEntry {
    pub base_code: String,
    pub target_code: String,
    pub rate: f64,
    pub timestamp: DateTime<Utc>,
    pub source: String,
}

impl RateEntry {
    pub fn new(
        base_code: String,
        target_code: String,
        rate: f64,
        timestamp: DateTime<Utc>,
        source: String,
    ) -> Self {
        Self {
            base_code,
            target_code,
            rate,
            timestamp,
            source,
        }
    }

    pub fn key(&self) -> String {
        format!("{}_{}", self.base_code, self.target_code)
    }
}

pub struct CrdtDocument {
    doc: AutoCommit,
    peer_id: String,
    saved_bytes: Vec<u8>,
}

impl CrdtDocument {
    pub fn new(peer_id: String) -> Self {
        let mut doc = AutoCommit::new();

        let rates = doc
            .put_object(automerge::ROOT, RATES_KEY, ObjType::Map)
            .unwrap();
        let _metadata = doc
            .put_object(automerge::ROOT, METADATA_KEY, ObjType::Map)
            .unwrap();
        doc.put(&rates, TIMESTAMP_KEY, ScalarValue::Int(0)).unwrap();
        doc.put(&rates, BASE_CODE_KEY, ScalarValue::Str("USD".into()))
            .unwrap();

        let saved_bytes = doc.save();

        Self {
            doc,
            peer_id,
            saved_bytes,
        }
    }

    pub fn from_bytes(peer_id: String, data: &[u8]) -> SyncResult<Self> {
        let mut doc = AutoCommit::load(data)
            .map_err(|e| SyncError::CrdtError(format!("Failed to load document: {e}")))?;

        let saved_bytes = doc.save();

        Ok(Self {
            doc,
            peer_id,
            saved_bytes,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.saved_bytes.clone()
    }

    pub fn apply_changes(&mut self, changes: &[u8]) -> SyncResult<()> {
        self.doc
            .load_incremental(changes)
            .map_err(|e| SyncError::CrdtError(format!("Failed to apply changes: {e}")))?;
        self.saved_bytes = self.doc.save();
        Ok(())
    }

    pub fn get_changes_since(&mut self) -> Vec<u8> {
        let new_save = self.doc.save();
        let diff = if new_save.len() > self.saved_bytes.len() {
            new_save[self.saved_bytes.len()..].to_vec()
        } else {
            Vec::new()
        };
        self.saved_bytes = new_save;
        diff
    }

    pub fn update_rate(&mut self, entry: &RateEntry) -> SyncResult<()> {
        let key = entry.key();
        let timestamp_ms = entry.timestamp.timestamp_millis();

        let rates = match self.doc.get(automerge::ROOT, RATES_KEY) {
            Ok(Some((Value::Object(ObjType::Map), obj_id))) => obj_id,
            _ => self
                .doc
                .put_object(automerge::ROOT, RATES_KEY, ObjType::Map)
                .map_err(|e| SyncError::CrdtError(format!("Failed to create rates: {e}")))?,
        };

        let rate_entry = match self.doc.get(&rates, &key) {
            Ok(Some((Value::Object(ObjType::Map), obj_id))) => obj_id,
            _ => self
                .doc
                .put_object(&rates, &key, ObjType::Map)
                .map_err(|e| SyncError::CrdtError(format!("Failed to create entry: {e}")))?,
        };

        self.doc
            .put(&rate_entry, RATE_KEY, ScalarValue::F64(entry.rate))
            .map_err(|e| SyncError::CrdtError(format!("Failed to put rate: {e}")))?;
        self.doc
            .put(&rate_entry, TIMESTAMP_KEY, ScalarValue::Int(timestamp_ms))
            .map_err(|e| SyncError::CrdtError(format!("Failed to put timestamp: {e}")))?;
        self.doc
            .put(
                &rate_entry,
                BASE_CODE_KEY,
                ScalarValue::Str(entry.base_code.clone().into()),
            )
            .map_err(|e| SyncError::CrdtError(format!("Failed to put base_code: {e}")))?;
        self.doc
            .put(
                &rate_entry,
                "target_code",
                ScalarValue::Str(entry.target_code.clone().into()),
            )
            .map_err(|e| SyncError::CrdtError(format!("Failed to put target_code: {e}")))?;
        self.doc
            .put(
                &rate_entry,
                SOURCE_KEY,
                ScalarValue::Str(entry.source.clone().into()),
            )
            .map_err(|e| SyncError::CrdtError(format!("Failed to put source: {e}")))?;

        self.doc
            .put(
                &rates,
                BASE_CODE_KEY,
                ScalarValue::Str(entry.base_code.clone().into()),
            )
            .map_err(|e| SyncError::CrdtError(format!("Failed to put rates base_code: {e}")))?;
        self.doc
            .put(&rates, TIMESTAMP_KEY, ScalarValue::Int(timestamp_ms))
            .map_err(|e| SyncError::CrdtError(format!("Failed to put rates timestamp: {e}")))?;

        self.saved_bytes = self.doc.save();

        Ok(())
    }

    pub fn update_rates(&mut self, rates: &RateCollection) -> SyncResult<()> {
        for (target_code, rate) in &rates.rates {
            let entry = RateEntry::new(
                rates.base_code.clone(),
                target_code.clone(),
                *rate,
                rates.timestamp,
                rates.source.to_string(),
            );
            self.update_rate(&entry)?;
        }
        Ok(())
    }

    pub fn get_rate(&self, base_code: &str, target_code: &str) -> SyncResult<Option<RateEntry>> {
        let key = format!("{base_code}_{target_code}");

        let rates = match self.doc.get(automerge::ROOT, RATES_KEY) {
            Ok(Some((Value::Object(ObjType::Map), obj_id))) => obj_id,
            _ => return Ok(None),
        };

        let entry = match self.doc.get(&rates, &key) {
            Ok(Some((Value::Object(ObjType::Map), obj_id))) => obj_id,
            _ => return Ok(None),
        };

        let rate = match self.doc.get(&entry, RATE_KEY) {
            Ok(Some((Value::Scalar(s), _))) => match s.as_ref() {
                ScalarValue::F64(v) => *v,
                ScalarValue::Int(v) => *v as f64,
                _ => 0.0,
            },
            _ => 0.0,
        };

        let timestamp_ms = match self.doc.get(&entry, TIMESTAMP_KEY) {
            Ok(Some((Value::Scalar(s), _))) => match s.as_ref() {
                ScalarValue::Int(v) => *v,
                ScalarValue::Uint(v) => *v as i64,
                _ => 0,
            },
            _ => 0,
        };

        let source = match self.doc.get(&entry, SOURCE_KEY) {
            Ok(Some((Value::Scalar(s), _))) => match s.as_ref() {
                ScalarValue::Str(v) => v.to_string(),
                _ => "unknown".to_string(),
            },
            _ => "unknown".to_string(),
        };

        let timestamp = DateTime::from_timestamp_millis(timestamp_ms).unwrap_or_else(Utc::now);

        Ok(Some(RateEntry {
            base_code: base_code.to_string(),
            target_code: target_code.to_string(),
            rate,
            timestamp,
            source,
        }))
    }

    pub fn get_rates(&self) -> SyncResult<RateCollection> {
        let rates_obj = match self.doc.get(automerge::ROOT, RATES_KEY) {
            Ok(Some((Value::Object(ObjType::Map), obj_id))) => obj_id,
            _ => {
                return Ok(RateCollection {
                    base_code: "USD".to_string(),
                    rates: HashMap::new(),
                    timestamp: Utc::now(),
                    source: Source::Cached,
                });
            }
        };

        let base_code = match self.doc.get(&rates_obj, BASE_CODE_KEY) {
            Ok(Some((Value::Scalar(s), _))) => match s.as_ref() {
                ScalarValue::Str(v) => v.to_string(),
                _ => "USD".to_string(),
            },
            _ => "USD".to_string(),
        };

        let timestamp_ms = match self.doc.get(&rates_obj, TIMESTAMP_KEY) {
            Ok(Some((Value::Scalar(s), _))) => match s.as_ref() {
                ScalarValue::Int(v) => *v,
                ScalarValue::Uint(v) => *v as i64,
                _ => 0,
            },
            _ => 0,
        };

        let timestamp = DateTime::from_timestamp_millis(timestamp_ms).unwrap_or_else(Utc::now);

        let mut rates = HashMap::new();

        for key in self.doc.keys(&rates_obj) {
            if key == BASE_CODE_KEY || key == TIMESTAMP_KEY {
                continue;
            }

            if let Ok(Some((Value::Object(ObjType::Map), entry_id))) =
                self.doc.get(&rates_obj, &key)
            {
                let target_code = key.split('_').nth(1).unwrap_or(&key).to_string();

                let rate = match self.doc.get(&entry_id, RATE_KEY) {
                    Ok(Some((Value::Scalar(s), _))) => match s.as_ref() {
                        ScalarValue::F64(v) => *v,
                        ScalarValue::Int(v) => *v as f64,
                        _ => continue,
                    },
                    _ => continue,
                };

                rates.insert(target_code, rate);
            }
        }

        Ok(RateCollection {
            base_code,
            rates,
            timestamp,
            source: Source::Cached,
        })
    }

    pub fn get_all_rate_entries(&self) -> SyncResult<Vec<RateEntry>> {
        let rates = self.get_rates()?;
        let mut entries = Vec::new();

        for target_code in rates.rates.keys() {
            if let Some(entry) = self.get_rate(&rates.base_code, target_code)? {
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    pub fn merge(&mut self, other: &CrdtDocument) -> SyncResult<()> {
        let other_bytes = other.to_bytes();
        self.apply_changes(&other_bytes)
    }

    pub fn get_peer_id(&self) -> &str {
        &self.peer_id
    }

    pub fn document_size(&self) -> usize {
        self.saved_bytes.len()
    }
}

impl Clone for CrdtDocument {
    fn clone(&self) -> Self {
        Self::from_bytes(self.peer_id.clone(), &self.saved_bytes).expect("Failed to clone document")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_rate_collection() -> RateCollection {
        let mut rates = HashMap::new();
        rates.insert("EUR".to_string(), 0.92);
        rates.insert("GBP".to_string(), 0.79);

        RateCollection {
            base_code: "USD".to_string(),
            rates,
            timestamp: Utc::now(),
            source: Source::Frankfurter,
        }
    }

    #[test]
    fn test_crdt_document_new() {
        let doc = CrdtDocument::new("peer1".to_string());
        assert_eq!(doc.get_peer_id(), "peer1");
        assert!(doc.document_size() > 0);
    }

    #[test]
    fn test_crdt_update_and_get_rates() {
        let mut doc = CrdtDocument::new("peer1".to_string());
        let rates = create_test_rate_collection();

        doc.update_rates(&rates).unwrap();

        let retrieved = doc.get_rates().unwrap();
        assert_eq!(retrieved.base_code, "USD");
        assert_eq!(retrieved.rates.get("EUR"), Some(&0.92));
        assert_eq!(retrieved.rates.get("GBP"), Some(&0.79));
    }

    #[test]
    fn test_crdt_get_single_rate() {
        let mut doc = CrdtDocument::new("peer1".to_string());
        let rates = create_test_rate_collection();

        doc.update_rates(&rates).unwrap();

        let entry = doc.get_rate("USD", "EUR").unwrap();
        assert!(entry.is_some());
        let entry = entry.unwrap();
        assert_eq!(entry.base_code, "USD");
        assert_eq!(entry.target_code, "EUR");
        assert_eq!(entry.rate, 0.92);
    }

    #[test]
    fn test_crdt_clone() {
        let mut doc1 = CrdtDocument::new("peer1".to_string());
        let rates = create_test_rate_collection();
        doc1.update_rates(&rates).unwrap();

        let doc2 = doc1.clone();

        let rates1 = doc1.get_rates().unwrap();
        let rates2 = doc2.get_rates().unwrap();

        assert_eq!(rates1.rates.len(), rates2.rates.len());
        assert_eq!(rates1.rates.get("EUR"), rates2.rates.get("EUR"));
    }

    #[test]
    fn test_crdt_serialization_roundtrip() {
        let mut doc1 = CrdtDocument::new("peer1".to_string());
        let rates = create_test_rate_collection();
        doc1.update_rates(&rates).unwrap();

        let bytes = doc1.to_bytes();
        let doc2 = CrdtDocument::from_bytes("peer2".to_string(), &bytes).unwrap();

        let rates1 = doc1.get_rates().unwrap();
        let rates2 = doc2.get_rates().unwrap();

        assert_eq!(rates1.rates.len(), rates2.rates.len());
        assert_eq!(rates1.rates.get("EUR"), rates2.rates.get("EUR"));
        assert_eq!(rates1.rates.get("GBP"), rates2.rates.get("GBP"));
    }

    #[test]
    fn test_crdt_last_writer_wins() {
        let mut doc = CrdtDocument::new("peer1".to_string());

        let entry1 = RateEntry::new(
            "USD".to_string(),
            "EUR".to_string(),
            0.90,
            Utc::now(),
            "source1".to_string(),
        );

        std::thread::sleep(std::time::Duration::from_millis(10));

        let entry2 = RateEntry::new(
            "USD".to_string(),
            "EUR".to_string(),
            0.92,
            Utc::now(),
            "source2".to_string(),
        );

        doc.update_rate(&entry1).unwrap();
        doc.update_rate(&entry2).unwrap();

        let retrieved = doc.get_rate("USD", "EUR").unwrap().unwrap();
        assert_eq!(retrieved.rate, 0.92);
    }
}
