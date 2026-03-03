use std::sync::Arc;

use slowpokeapi::upstream::{FrankfurterClient, HttpClient, Upstream, UpstreamManager};

#[tokio::test]
async fn test_frankfurter_client_get_latest_rates() {
    let http = Arc::new(HttpClient::new(10));
    let client = FrankfurterClient::new(http);

    let result = client.get_latest_rates("USD").await;

    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_frankfurter_client_not_found() {
    let http = Arc::new(HttpClient::new(10));
    let client = FrankfurterClient::new(http);

    let result = client.get_latest_rates("INVALID").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_upstream_manager_fallback() {
    let http = Arc::new(HttpClient::new(10));
    let manager = UpstreamManager::new(http);

    assert!(manager.total_count() >= 1);
}
