use reqwest::StatusCode;
use slowpokeapi::models::HistoricalResponse;

mod common;

#[tokio::test]
async fn history_endpoint_returns_rates_for_valid_date() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/history/USD/2024/01/15"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::OK, response.status());

    let body: HistoricalResponse = response.json().await.expect("Failed to parse JSON");

    assert!(!body.conversion_rates.is_empty());
    assert_eq!(body.base_code, "USD");
    assert_eq!(body.year, 2024);
    assert_eq!(body.month, 1);
    assert_eq!(body.day, 15);
}

#[tokio::test]
async fn history_endpoint_handles_lowercase_currency() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/history/eur/2024/01/15"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::OK, response.status());

    let body: HistoricalResponse = response.json().await.expect("Failed to parse JSON");

    assert_eq!(body.base_code, "EUR");
}

#[tokio::test]
async fn history_endpoint_rejects_invalid_currency() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/history/US/2024/01/15"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}

#[tokio::test]
async fn history_endpoint_rejects_invalid_date() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/history/USD/2024/13/45"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}

#[tokio::test]
async fn history_endpoint_rejects_future_date() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/history/USD/2099/12/31"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}
