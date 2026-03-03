use slowpokeapi::ratelimit::{ApiKey, RateLimitConfig, TokenBucket};
use slowpokeapi::storage::{ApiKeyStore, SqlitePool};
use sqlx::sqlite::SqlitePoolOptions;
use std::time::Duration;

async fn create_memory_pool() -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await
        .unwrap()
}

#[tokio::test]
async fn test_token_bucket_basic() {
    let mut bucket = TokenBucket::new(100, 10);

    assert_eq!(bucket.capacity(), 100);
    assert_eq!(bucket.available_tokens(), 100);

    assert!(bucket.try_consume(50));
    assert_eq!(bucket.available_tokens(), 50);

    assert!(bucket.try_consume(50));
    assert_eq!(bucket.available_tokens(), 00);

    assert!(!bucket.try_consume(1));
}

#[tokio::test]
async fn test_token_bucket_refill() {
    let mut bucket = TokenBucket::new(100, 1000);

    bucket.try_consume(100);
    assert_eq!(bucket.available_tokens(), 0);

    std::thread::sleep(Duration::from_millis(100));

    let tokens = bucket.available_tokens();
    assert!(tokens > 0);
    assert!(tokens <= 100);
}

#[tokio::test]
async fn test_api_key_store() {
    let pool = create_memory_pool().await;
    let store = ApiKeyStore::new(pool);

    store.initialize().await.unwrap();

    let api_key = ApiKey {
        key: "test-key-123".to_string(),
        name: "Test Key".to_string(),
        rate_limit: RateLimitConfig {
            requests_per_second: 10,
            burst_capacity: 100,
        },
        is_active: true,
    };

    store.create(api_key.clone()).await.unwrap();

    let retrieved = store.get("test-key-123").await;
    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.key, "test-key-123");
    assert_eq!(retrieved.name, "Test Key");

    let keys = store.list().await.unwrap();
    assert_eq!(keys.len(), 1);

    store.deactivate("test-key-123").await.unwrap();

    let retrieved = store.get("test-key-123").await;
    assert!(retrieved.is_none());
}
