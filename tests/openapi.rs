use reqwest::StatusCode;

mod common;

#[tokio::test]
async fn swagger_ui_returns_html() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/swagger-ui/"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::OK, response.status());
    let body = response.text().await.expect("Failed to read body");
    assert!(body.contains("Swagger UI") || body.contains("swagger"));
}

#[tokio::test]
async fn openapi_json_returns_valid_spec() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/api-docs/openapi.json"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::OK, response.status());

    let body = response.text().await.expect("Failed to read body");
    let spec: serde_json::Value = serde_json::from_str(&body).expect("Invalid JSON");

    assert!(spec.get("openapi").is_some());
    assert!(spec.get("info").is_some());
    assert!(spec.get("paths").is_some());
}

#[tokio::test]
async fn health_endpoints_in_openapi_spec() {
    let addr = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{addr}/api-docs/openapi.json"))
        .send()
        .await
        .expect("Failed to execute request");

    let body = response.text().await.expect("Failed to read body");
    let spec: serde_json::Value = serde_json::from_str(&body).expect("Invalid JSON");

    let paths = spec.get("paths").expect("paths not found in OpenAPI spec");
    assert!(paths.get("/healthz").is_some());
    assert!(paths.get("/readyz").is_some());
    assert!(paths.get("/livez").is_some());
    assert!(paths.get("/health").is_some());
}
