use reqwest::StatusCode;

mod common;

#[tokio::test]
async fn metrics_endpoint_returns_prometheus_format() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/metrics"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::OK, response.status());

    let content_type = response
        .headers()
        .get("content-type")
        .expect("Missing content-type header")
        .to_str()
        .expect("Invalid content-type header");
    assert!(content_type.starts_with("text/plain"));
}

#[tokio::test]
async fn metrics_contains_http_request_metrics() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();

    // Check metrics
    let response = client
        .get(format!("{addr}/metrics"))
        .send()
        .await
        .expect("Failed to execute request");

    let body = response.text().await.expect("Failed to read body");

    // Check that we have HTTP metrics (with slowpokeapi prefix)
    assert!(
        body.contains("slowpokeapi_http_requests") || body.contains("http_requests"),
        "Metrics should contain HTTP request metrics, got: {body}"
    );
}
