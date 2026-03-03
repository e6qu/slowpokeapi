use std::collections::HashMap;
use std::time::Duration;

use slowpokeapi::cache::{Cache, MemoryCache, SqliteCache, TieredCache};
use slowpokeapi::models::{RateCollection, Source};

async fn create_test_pool() -> sqlx::SqlitePool {
    sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap()
}

fn create_test_rate(base: &str) -> RateCollection {
    let mut rates = HashMap::new();
    rates.insert("EUR".to_string(), 0.92);
    rates.insert("GBP".to_string(), 0.79);

    RateCollection {
        base_code: base.to_string(),
        rates,
        timestamp: chrono::Utc::now(),
        source: Source::Frankfurter,
    }
}

#[tokio::test]
async fn memory_cache_set_and_get() {
    let cache: MemoryCache<String, RateCollection> = MemoryCache::new(100, Duration::from_secs(60));
    let rate = create_test_rate("USD");

    cache
        .set("USD".to_string(), rate.clone(), None)
        .await
        .unwrap();
    let result = cache.get(&"USD".to_string()).await.unwrap();

    assert!(result.is_some());
    let retrieved = result.unwrap();
    assert_eq!(retrieved.base_code, "USD");
    assert_eq!(retrieved.rates.len(), 2);
}

#[tokio::test]
async fn memory_cache_miss() {
    let cache: MemoryCache<String, RateCollection> = MemoryCache::new(100, Duration::from_secs(60));

    let result = cache.get(&"EUR".to_string()).await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn memory_cache_delete() {
    let cache: MemoryCache<String, RateCollection> = MemoryCache::new(100, Duration::from_secs(60));
    let rate = create_test_rate("USD");

    cache
        .set("USD".to_string(), rate.clone(), None)
        .await
        .unwrap();
    cache.delete(&"USD".to_string()).await.unwrap();
    let result = cache.get(&"USD".to_string()).await.unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn memory_cache_clear() {
    let cache: MemoryCache<String, RateCollection> = MemoryCache::new(100, Duration::from_secs(60));
    let rate = create_test_rate("USD");

    cache
        .set("USD".to_string(), rate.clone(), None)
        .await
        .unwrap();
    cache.clear().await.unwrap();
    let result = cache.get(&"USD".to_string()).await.unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn sqlite_cache_set_and_get() {
    let pool = create_test_pool().await;
    sqlx::migrate!().run(&pool).await.unwrap();

    let cache: SqliteCache = SqliteCache::new(pool);
    let rate = create_test_rate("USD");

    <SqliteCache as Cache<String, RateCollection>>::set(
        &cache,
        "USD".to_string(),
        rate.clone(),
        None,
    )
    .await
    .unwrap();
    let result: Option<RateCollection> =
        <SqliteCache as Cache<String, RateCollection>>::get(&cache, &"USD".to_string())
            .await
            .unwrap();

    assert!(result.is_some());
    let retrieved = result.unwrap();
    assert_eq!(retrieved.base_code, "USD");
    assert_eq!(retrieved.rates.len(), 2);
}

#[tokio::test]
async fn sqlite_cache_miss() {
    let pool = create_test_pool().await;
    sqlx::migrate!().run(&pool).await.unwrap();

    let cache: SqliteCache = SqliteCache::new(pool);
    let result: Option<RateCollection> =
        <SqliteCache as Cache<String, RateCollection>>::get(&cache, &"EUR".to_string())
            .await
            .unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn sqlite_cache_delete() {
    let pool = create_test_pool().await;
    sqlx::migrate!().run(&pool).await.unwrap();

    let cache: SqliteCache = SqliteCache::new(pool);
    let rate = create_test_rate("USD");

    <SqliteCache as Cache<String, RateCollection>>::set(
        &cache,
        "USD".to_string(),
        rate.clone(),
        None,
    )
    .await
    .unwrap();
    <SqliteCache as Cache<String, RateCollection>>::delete(&cache, &"USD".to_string())
        .await
        .unwrap();
    let result: Option<RateCollection> =
        <SqliteCache as Cache<String, RateCollection>>::get(&cache, &"USD".to_string())
            .await
            .unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn sqlite_cache_clear() {
    let pool = create_test_pool().await;
    sqlx::migrate!().run(&pool).await.unwrap();

    let cache: SqliteCache = SqliteCache::new(pool);
    let rate = create_test_rate("USD");

    <SqliteCache as Cache<String, RateCollection>>::set(
        &cache,
        "USD".to_string(),
        rate.clone(),
        None,
    )
    .await
    .unwrap();
    <SqliteCache as Cache<String, RateCollection>>::clear(&cache)
        .await
        .unwrap();
    let result: Option<RateCollection> =
        <SqliteCache as Cache<String, RateCollection>>::get(&cache, &"USD".to_string())
            .await
            .unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn sqlite_cache_ttl_expiration() {
    let pool = create_test_pool().await;
    sqlx::migrate!().run(&pool).await.unwrap();

    let cache: SqliteCache = SqliteCache::new(pool);
    let rate = create_test_rate("USD");

    <SqliteCache as Cache<String, RateCollection>>::set(
        &cache,
        "USD".to_string(),
        rate.clone(),
        Some(Duration::from_secs(1)),
    )
    .await
    .unwrap();

    let result: Option<RateCollection> =
        <SqliteCache as Cache<String, RateCollection>>::get(&cache, &"USD".to_string())
            .await
            .unwrap();
    assert!(result.is_some());

    tokio::time::sleep(Duration::from_millis(1100)).await;

    let result: Option<RateCollection> =
        <SqliteCache as Cache<String, RateCollection>>::get(&cache, &"USD".to_string())
            .await
            .unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn tiered_cache_l1_hit() {
    let pool = create_test_pool().await;
    sqlx::migrate!().run(&pool).await.unwrap();

    let l1: MemoryCache<String, RateCollection> = MemoryCache::new(100, Duration::from_secs(60));
    let l2 = SqliteCache::new(pool);
    let cache = TieredCache::new(l1, l2);
    let rate = create_test_rate("USD");

    cache
        .set("USD".to_string(), rate.clone(), None)
        .await
        .unwrap();
    let result = cache.get(&"USD".to_string()).await.unwrap();

    assert!(result.is_some());
    assert_eq!(result.unwrap().base_code, "USD");
}

#[tokio::test]
async fn tiered_cache_l2_to_l1_promotion() {
    let pool = create_test_pool().await;
    sqlx::migrate!().run(&pool).await.unwrap();

    let l1: MemoryCache<String, RateCollection> = MemoryCache::new(100, Duration::from_secs(60));
    let l2 = SqliteCache::new(pool);

    let rate = create_test_rate("USD");
    <SqliteCache as Cache<String, RateCollection>>::set(&l2, "USD".to_string(), rate.clone(), None)
        .await
        .unwrap();

    let cache = TieredCache::new(l1, l2);

    let result = cache.get(&"USD".to_string()).await.unwrap();
    assert!(result.is_some());

    let result2 = cache.get(&"USD".to_string()).await.unwrap();
    assert!(result2.is_some());
}

#[tokio::test]
async fn tiered_cache_delete_propagates() {
    let pool = create_test_pool().await;
    sqlx::migrate!().run(&pool).await.unwrap();

    let l1: MemoryCache<String, RateCollection> = MemoryCache::new(100, Duration::from_secs(60));
    let l2 = SqliteCache::new(pool);
    let cache = TieredCache::new(l1, l2);
    let rate = create_test_rate("USD");

    cache
        .set("USD".to_string(), rate.clone(), None)
        .await
        .unwrap();
    cache.delete(&"USD".to_string()).await.unwrap();

    let result = cache.get(&"USD".to_string()).await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn tiered_cache_clear_propagates() {
    let pool = create_test_pool().await;
    sqlx::migrate!().run(&pool).await.unwrap();

    let l1: MemoryCache<String, RateCollection> = MemoryCache::new(100, Duration::from_secs(60));
    let l2 = SqliteCache::new(pool);
    let cache = TieredCache::new(l1, l2);
    let rate = create_test_rate("USD");

    cache
        .set("USD".to_string(), rate.clone(), None)
        .await
        .unwrap();
    cache.clear().await.unwrap();

    let result = cache.get(&"USD".to_string()).await.unwrap();
    assert!(result.is_none());
}
