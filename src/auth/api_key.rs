use axum::http::Request;

pub fn validate_api_key<B>(request: &Request<B>) -> Option<String> {
    extract_api_key_from_header(request).or_else(|| extract_api_key_from_query(request))
}

fn extract_api_key_from_header<B>(request: &Request<B>) -> Option<String> {
    request
        .headers()
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| {
            request
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "))
                .map(|s| s.to_string())
        })
}

fn extract_api_key_from_query<B>(request: &Request<B>) -> Option<String> {
    request.uri().query().and_then(|query| {
        query.split('&').find_map(|pair| {
            let parts: Vec<&str> = pair.splitn(2, '=').collect();
            if parts.len() == 2 && parts[0] == "api_key" {
                return Some(parts[1].to_string());
            }
            None
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;

    #[test]
    fn test_extract_from_header() {
        let request = Request::builder()
            .header("X-API-Key", "test-key")
            .body(())
            .unwrap();

        let result = validate_api_key(&request);
        assert_eq!(result, Some("test-key".to_string()));
    }

    #[test]
    fn test_extract_from_bearer() {
        let request = Request::builder()
            .header("Authorization", "Bearer bearer-token")
            .body(())
            .unwrap();

        let result = validate_api_key(&request);
        assert_eq!(result, Some("bearer-token".to_string()));
    }

    #[test]
    fn test_extract_from_query() {
        let request = Request::builder()
            .uri("/test?api_key=query-key")
            .body(())
            .unwrap();

        let result = validate_api_key(&request);
        assert_eq!(result, Some("query-key".to_string()));
    }

    #[test]
    fn test_header_takes_precedence() {
        let request = Request::builder()
            .header("X-API-Key", "header-key")
            .uri("/test?api_key=query-key")
            .body(())
            .unwrap();

        let result = validate_api_key(&request);
        assert_eq!(result, Some("header-key".to_string()));
    }

    #[test]
    fn test_no_api_key() {
        let request = Request::builder().uri("/test").body(()).unwrap();

        let result = validate_api_key(&request);
        assert_eq!(result, None);
    }

    #[test]
    fn test_bearer_without_prefix() {
        let request = Request::builder()
            .header("Authorization", "Basic not-a-bearer")
            .body(())
            .unwrap();

        let result = validate_api_key(&request);
        assert_eq!(result, None);
    }
}
