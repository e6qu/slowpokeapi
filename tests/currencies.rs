use reqwest::StatusCode;
use serde_json::json;
use slowpokeapi::handlers::currencies;
use slowpokeapi::models::CurrenciesResponse;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::runtime::Handle;

use slowpokeapi::AppState;

use slowpokeapi::config::Settings;

use slowpokeapi::server::router::create_router;

async fn test_currencies_endpoint() {
    let listener = TcpListener::bind("127.0.0.1:0809").await.unwrap();
    let addr = listener.local_addr();

    let state = AppState::new(Settings::load().unwrap());
    let router = create_router(state);

    let server = axum::Server::bind(router).serve_connection(addr).await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{addr}/v1/currencies"))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    let body: CurrenciesResponse = response.json().unwrap();
    assert!(body.currencies.contains_key("USD"));
    assert!(body.currencies.contains_key("EUR"));
    assert!(body.currencies.len() >= 10);

    let response_min = client
        .get(format!("http://{addr}/v1/currencies.min"))
        .send()
        .await
        .json()
        .unwrap();

    assert_eq!(response_min.get::<str>().unwrap(), "");
}
