use reqwest::StatusCode;
use slowpokeapi::models::CurrenciesResponse;

mod common;

#[tokio::test]
async fn currencies_endpoint_returns_list() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/currencies"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::OK, response.status());

    let body: CurrenciesResponse = response.json().await.expect("Failed to parse JSON");

    assert!(body.currencies.contains_key("USD"));
    assert!(body.currencies.contains_key("EUR"));
    assert!(body.currencies.contains_key("GBP"));
    assert!(body.currencies.len() >= 10);
}

#[tokio::test]
async fn currencies_minimal_endpoint_returns_codes_only() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/currencies.min"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::OK, response.status());

    let body: CurrenciesResponse = response.json().await.expect("Failed to parse JSON");

    assert!(body.currencies.contains_key("USD"));
    assert!(body.currencies.contains_key("EUR"));

    for name in body.currencies.values() {
        assert!(
            name.is_empty(),
            "Minimal currencies should have empty names"
        );
    }
}
