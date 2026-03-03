use reqwest::StatusCode;
use slowpokeapi::models::EnrichedResponse;

mod common;

#[tokio::test]
async fn enriched_endpoint_returns_rate_with_metadata() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/enriched/USD/EUR"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::OK, response.status());

    let body: EnrichedResponse = response.json().await.expect("Failed to parse JSON");

    assert!(body.conversion_rate >= 0.0);
    assert_eq!(body.target_code, "EUR");
    assert_eq!(body.target_data.code, "EUR");
    assert!(!body.target_data.currency_name.is_empty());
}

#[tokio::test]
async fn enriched_endpoint_handles_lowercase_input() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/enriched/USD/eur"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::OK, response.status());

    let body: EnrichedResponse = response.json().await.expect("Failed to parse JSON");

    assert_eq!(body.target_code.to_uppercase(), "EUR");
}

#[tokio::test]
async fn enriched_endpoint_rejects_invalid_base_currency() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/enriched/US/EUR"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}

#[tokio::test]
async fn enriched_endpoint_rejects_invalid_target_currency() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/enriched/USD/EU"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}

#[tokio::test]
async fn enriched_endpoint_rejects_unknown_currency() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/enriched/USD/XYZ"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::NOT_FOUND, response.status());
}
