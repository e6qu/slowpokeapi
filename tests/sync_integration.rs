use slowpokeapi::cache::{Cache, MemoryCache};
use slowpokeapi::models::{RateCollection, Source};
use slowpokeapi::sync::{CrdtDocument, Reconciler, SyncIntegration};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_sync_integration_state_roundtrip() {
    let doc = Arc::new(RwLock::new(CrdtDocument::new("peer1".to_string())));

    // Add some data
    let mut rates = HashMap::new();
    rates.insert("EUR".to_string(), 0.92);
    let rate_collection = RateCollection {
        base_code: "USD".to_string(),
        rates: rates.clone(),
        timestamp: chrono::Utc::now(),
        source: Source::Frankfurter,
    };
    doc.write().await.update_rates(&rate_collection).unwrap();

    // Serialize and deserialize
    let state = doc.read().await.to_bytes();
    let doc2 = CrdtDocument::from_bytes("peer2".to_string(), &state).unwrap();

    // Verify data is preserved (not exact bytes, but the content)
    let rates1 = doc.read().await.get_rates().unwrap();
    let rates2 = doc2.get_rates().unwrap();
    assert_eq!(rates1.rates.get("EUR"), rates2.rates.get("EUR"));
}

#[tokio::test]
async fn test_crdt_document_update_and_get() {
    let mut doc = CrdtDocument::new("peer1".to_string());

    let mut rates = HashMap::new();
    rates.insert("EUR".to_string(), 0.85);
    rates.insert("GBP".to_string(), 0.73);

    let rate_collection = RateCollection {
        base_code: "USD".to_string(),
        rates: rates.clone(),
        timestamp: chrono::Utc::now(),
        source: Source::Frankfurter,
    };

    doc.update_rates(&rate_collection).unwrap();

    let retrieved = doc.get_rates().unwrap();

    assert_eq!(retrieved.base_code, "USD");
    assert_eq!(retrieved.rates.get("EUR"), Some(&0.85));
    assert_eq!(retrieved.rates.get("GBP"), Some(&0.73));
}

#[tokio::test]
async fn test_crdt_document_serialization() {
    let mut doc = CrdtDocument::new("peer1".to_string());

    let mut rates = HashMap::new();
    rates.insert("EUR".to_string(), 0.92);

    let rate_collection = RateCollection {
        base_code: "USD".to_string(),
        rates,
        timestamp: chrono::Utc::now(),
        source: Source::Frankfurter,
    };

    doc.update_rates(&rate_collection).unwrap();

    let state = doc.to_bytes();
    assert!(!state.is_empty());

    // Load into a new document and verify data is preserved
    let doc2 = CrdtDocument::from_bytes("peer2".to_string(), &state).unwrap();

    let retrieved = doc2.get_rates().unwrap();
    assert_eq!(retrieved.base_code, "USD");
    assert_eq!(retrieved.rates.get("EUR"), Some(&0.92));
}

#[tokio::test]
async fn test_sync_integration_on_cache_update() {
    let doc = Arc::new(RwLock::new(CrdtDocument::new("peer1".to_string())));
    let cache = Arc::new(MemoryCache::new(100, Duration::from_secs(300)));

    let sync = SyncIntegration::new(doc.clone(), cache.clone());

    let mut rates = HashMap::new();
    rates.insert("EUR".to_string(), 0.92);

    let rate_collection = RateCollection {
        base_code: "USD".to_string(),
        rates: rates.clone(),
        timestamp: chrono::Utc::now(),
        source: Source::Frankfurter,
    };

    sync.on_cache_update("USD", &rate_collection).await.unwrap();

    let doc_read = doc.read().await;
    let doc_rates = doc_read.get_rates().unwrap();
    assert_eq!(doc_rates.base_code, "USD");
    assert_eq!(doc_rates.rates.get("EUR"), Some(&0.92));
}

#[tokio::test]
async fn test_sync_integration_on_sync_update() {
    let doc = Arc::new(RwLock::new(CrdtDocument::new("peer1".to_string())));
    let cache = Arc::new(MemoryCache::new(100, Duration::from_secs(300)));

    let mut rates = HashMap::new();
    rates.insert("EUR".to_string(), 0.85);

    let rate_collection = RateCollection {
        base_code: "USD".to_string(),
        rates: rates.clone(),
        timestamp: chrono::Utc::now(),
        source: Source::Cached,
    };

    doc.write().await.update_rates(&rate_collection).unwrap();

    let sync = SyncIntegration::new(doc.clone(), cache.clone());

    sync.on_sync_update("USD").await.unwrap();

    let cached = cache.get(&"latest:USD".to_string()).await.unwrap();
    assert!(cached.is_some());
    let cached = cached.unwrap();
    assert_eq!(cached.base_code, "USD");
    assert_eq!(cached.rates.get("EUR"), Some(&0.85));
}

#[tokio::test]
async fn test_reconciler_updates_stale_cache() {
    let doc = Arc::new(RwLock::new(CrdtDocument::new("peer1".to_string())));
    let cache = Arc::new(MemoryCache::new(100, Duration::from_secs(300)));

    let old_time = chrono::Utc::now() - chrono::Duration::seconds(10);
    let mut old_rates = HashMap::new();
    old_rates.insert("EUR".to_string(), 0.80);

    let old_collection = RateCollection {
        base_code: "USD".to_string(),
        rates: old_rates,
        timestamp: old_time,
        source: Source::Cached,
    };

    cache
        .set("latest:USD".to_string(), old_collection.clone(), None)
        .await
        .unwrap();

    let new_time = chrono::Utc::now();
    let mut new_rates = HashMap::new();
    new_rates.insert("EUR".to_string(), 0.92);

    let new_collection = RateCollection {
        base_code: "USD".to_string(),
        rates: new_rates,
        timestamp: new_time,
        source: Source::Frankfurter,
    };

    doc.write().await.update_rates(&new_collection).unwrap();

    let reconciler = Reconciler::new(doc, cache.clone());
    reconciler.reconcile("USD").await.unwrap();

    let cached = cache.get(&"latest:USD".to_string()).await.unwrap();
    assert!(cached.is_some());
    let cached = cached.unwrap();
    assert_eq!(cached.rates.get("EUR"), Some(&0.92));
}

#[tokio::test]
async fn test_reconciler_handles_missing_cache() {
    let doc = Arc::new(RwLock::new(CrdtDocument::new("peer1".to_string())));
    let cache = Arc::new(MemoryCache::new(100, Duration::from_secs(300)));

    let mut rates = HashMap::new();
    rates.insert("EUR".to_string(), 0.92);

    let rate_collection = RateCollection {
        base_code: "USD".to_string(),
        rates: rates.clone(),
        timestamp: chrono::Utc::now(),
        source: Source::Frankfurter,
    };

    doc.write().await.update_rates(&rate_collection).unwrap();

    let reconciler = Reconciler::new(doc, cache.clone());
    reconciler.reconcile("USD").await.unwrap();

    let cached = cache.get(&"latest:USD".to_string()).await.unwrap();
    assert!(cached.is_some());
    let cached = cached.unwrap();
    assert_eq!(cached.base_code, "USD");
    assert_eq!(cached.rates.get("EUR"), Some(&0.92));
}
