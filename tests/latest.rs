use reqwest::StatusCode;
use slowpokeapi::models::LatestRatesResponse;

mod common;

#[tokio::test]
async fn latest_endpoint_returns_rates_for_valid_currency() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/latest/USD"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::OK, response.status());

    let body: LatestRatesResponse = response.json().await.expect("Failed to parse JSON");

    assert!(!body.conversion_rates.is_empty());
    assert_eq!(body.base_code, "USD");
}

#[tokio::test]
async fn latest_endpoint_returns_rates_for_lowercase_currency() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/latest/eur"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::OK, response.status());

    let body: LatestRatesResponse = response.json().await.expect("Failed to parse JSON");

    assert_eq!(body.base_code, "EUR");
}

#[tokio::test]
async fn latest_endpoint_rejects_invalid_currency() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/latest/US"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}

#[tokio::test]
async fn latest_endpoint_rejects_numeric_currency() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/latest/US1"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}
