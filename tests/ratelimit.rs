use slowpokeapi::ratelimit::{ApiKey, ClientBackoff, RateLimitConfig, TokenBucket};
use std::time::Duration;

#[test]
fn test_token_bucket_basic() {
    let mut bucket = TokenBucket::new(100, 10);

    assert_eq!(bucket.capacity(), 100);
    assert_eq!(bucket.available_tokens(), 100);

    assert!(bucket.try_consume(50));
    assert_eq!(bucket.available_tokens(), 50);

    assert!(bucket.try_consume(50));
    assert_eq!(bucket.available_tokens(), 0);

    assert!(!bucket.try_consume(1));
}

#[tokio::test]
async fn test_token_bucket_refill() {
    let mut bucket = TokenBucket::new(100, 10000);

    assert!(
        bucket.try_consume(100),
        "Should be able to consume all tokens initially"
    );

    tokio::time::sleep(Duration::from_millis(200)).await;

    let tokens = bucket.available_tokens();
    assert!(
        tokens > 0,
        "Expected tokens to be refilled after 200ms with rate 10000/sec, got {tokens}"
    );
    assert!(tokens <= 100);
}

#[test]
fn test_rate_limit_config_defaults() {
    let config = RateLimitConfig::default();

    assert!(config.enabled);
    assert_eq!(config.global_requests_per_second, 500);
    assert_eq!(config.backpressure_threshold, 0.8);
}

#[test]
fn test_effective_rates() {
    let config = RateLimitConfig::default();

    assert_eq!(config.effective_global_rate(), 250);
    assert_eq!(config.effective_authenticated_rate(), 25);
    assert_eq!(config.effective_anonymous_rate(), 5);

    assert_eq!(config.effective_global_burst(), 500);
    assert_eq!(config.effective_authenticated_burst(), 50);
    assert_eq!(config.effective_anonymous_burst(), 10);
}

#[test]
fn test_backoff_calculation() {
    let config = RateLimitConfig::default();
    let mut backoff = ClientBackoff::new();

    assert_eq!(backoff.calculate_backoff(&config), Duration::from_secs(0));

    backoff.record_rejection();
    assert_eq!(backoff.calculate_backoff(&config), Duration::from_secs(1));

    backoff.record_rejection();
    assert_eq!(backoff.calculate_backoff(&config), Duration::from_secs(2));

    backoff.record_success();
    assert_eq!(backoff.calculate_backoff(&config), Duration::from_secs(0));
}

#[test]
fn test_utilization() {
    let mut bucket = TokenBucket::new(100, 10);
    assert!((bucket.utilization() - 0.0).abs() < 0.01);

    bucket.try_consume(50);
    assert!((bucket.utilization() - 0.5).abs() < 0.01);

    bucket.try_consume(50);
    assert!((bucket.utilization() - 1.0).abs() < 0.01);
}

#[test]
fn test_api_key_creation() {
    let key = ApiKey::new("test-key-123".to_string(), "Test Key".to_string());

    assert_eq!(key.key, "test-key-123");
    assert_eq!(key.name, "Test Key");
    assert!(key.is_active);
}
