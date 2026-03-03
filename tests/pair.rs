use reqwest::StatusCode;
use slowpokeapi::models::PairResponse;

mod common;

#[tokio::test]
async fn pair_endpoint_returns_rate_without_amount() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/v1/pair/USD/EUR"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::OK, response.status());

    let body: PairResponse = response.json().await.expect("Failed to parse JSON");

    assert!(body.conversion_rate >= 0.0);
    assert_eq!(body.target_code, "EUR");
}
