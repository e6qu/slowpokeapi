use axum::http::Request;
use slowpokeapi::auth::validate_api_key;

#[tokio::test]
async fn test_auth_config_default() {
    let config = slowpokeapi::auth::AuthConfig::default();

    assert!(config.enabled);
    assert!(!config.require_api_key);
    assert!(config.public_paths.contains(&"/healthz".to_string()));
    assert!(config.public_paths.contains(&"/metrics".to_string()));
}

#[tokio::test]
async fn test_auth_config_custom() {
    let config = slowpokeapi::auth::AuthConfig {
        enabled: false,
        require_api_key: true,
        public_paths: vec!["/custom".to_string()],
    };

    assert!(!config.enabled);
    assert!(config.require_api_key);
    assert!(config.public_paths.contains(&"/custom".to_string()));
}

#[tokio::test]
async fn test_extract_api_key_from_header() {
    let request = Request::builder()
        .header("X-API-Key", "test-key-123")
        .body(())
        .unwrap();

    let result = validate_api_key(&request);
    assert_eq!(result, Some("test-key-123".to_string()));
}

#[tokio::test]
async fn test_extract_api_key_from_bearer() {
    let request = Request::builder()
        .header("Authorization", "Bearer bearer-token-456")
        .body(())
        .unwrap();

    let result = validate_api_key(&request);
    assert_eq!(result, Some("bearer-token-456".to_string()));
}

#[tokio::test]
async fn test_extract_api_key_from_query() {
    let request = Request::builder()
        .uri("/test?api_key=query-key-789")
        .body(())
        .unwrap();

    let result = validate_api_key(&request);
    assert_eq!(result, Some("query-key-789".to_string()));
}

#[tokio::test]
async fn test_header_takes_precedence() {
    let request = Request::builder()
        .header("X-API-Key", "header-key")
        .uri("/test?api_key=query-key")
        .body(())
        .unwrap();

    let result = validate_api_key(&request);
    assert_eq!(result, Some("header-key".to_string()));
}

#[tokio::test]
async fn test_no_api_key() {
    let request = Request::builder().uri("/test").body(()).unwrap();

    let result = validate_api_key(&request);
    assert_eq!(result, None);
}

#[tokio::test]
async fn test_bearer_without_prefix() {
    let request = Request::builder()
        .header("Authorization", "Basic not-a-bearer")
        .body(())
        .unwrap();

    let result = validate_api_key(&request);
    assert_eq!(result, None);
}
