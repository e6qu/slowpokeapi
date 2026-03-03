use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use crate::ratelimit::TokenBucket;
use crate::storage::ApiKeyStore;

pub struct RateLimiter {
    buckets: Arc<RwLock<HashMap<String, TokenBucket>>>,
    api_key_store: Arc<ApiKeyStore>,
}

impl RateLimiter {
    pub fn new(api_key_store: Arc<ApiKeyStore>) -> Self {
        Self {
            buckets: Arc::new(RwLock::new(HashMap::new())),
            api_key_store,
        }
    }

    pub async fn check_rate_limit(&self, api_key: &str) -> Result<RateLimitInfo, RateLimitError> {
        let key_info = self
            .api_key_store
            .get(api_key)
            .await
            .ok_or(RateLimitError::InvalidApiKey)?;

        if !key_info.is_active {
            return Err(RateLimitError::InvalidApiKey);
        }

        let mut buckets = self.buckets.write().await;

        let bucket = buckets.entry(api_key.to_string()).or_insert_with(|| {
            TokenBucket::new(
                key_info.rate_limit.burst_capacity,
                key_info.rate_limit.requests_per_second,
            )
        });

        if bucket.try_consume(1) {
            Ok(RateLimitInfo {
                limit: bucket.capacity(),
                remaining: bucket.available_tokens(),
                reset_after: Duration::from_secs(0),
            })
        } else {
            Err(RateLimitError::RateExceeded {
                retry_after: bucket.time_until_available(1),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    pub limit: u64,
    pub remaining: u64,
    pub reset_after: Duration,
}

#[derive(Debug)]
pub enum RateLimitError {
    InvalidApiKey,
    RateExceeded { retry_after: Duration },
}

pub async fn rate_limit_middleware(
    State(rate_limiter): State<Arc<RateLimiter>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, RateLimitMiddlewareError> {
    let api_key = extract_api_key(&request);

    let rate_limit_info = match api_key {
        Some(key) => rate_limiter.check_rate_limit(&key).await,
        None => {
            return Err(RateLimitMiddlewareError::MissingApiKey);
        }
    };

    match rate_limit_info {
        Ok(info) => {
            let mut response = next.run(request).await;
            add_rate_limit_headers(&mut response, &info);
            Ok(response)
        }
        Err(RateLimitError::InvalidApiKey) => Err(RateLimitMiddlewareError::InvalidApiKey),
        Err(RateLimitError::RateExceeded { retry_after }) => {
            Err(RateLimitMiddlewareError::RateExceeded { retry_after })
        }
    }
}

fn extract_api_key<B>(request: &Request<B>) -> Option<String> {
    request
        .headers()
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

fn add_rate_limit_headers(response: &mut Response, info: &RateLimitInfo) {
    let headers = response.headers_mut();
    headers.insert("X-RateLimit-Limit", info.limit.to_string().parse().unwrap());
    headers.insert(
        "X-RateLimit-Remaining",
        info.remaining.to_string().parse().unwrap(),
    );
    headers.insert(
        "X-RateLimit-Reset",
        info.reset_after.as_secs().to_string().parse().unwrap(),
    );
}

#[derive(Debug)]
pub enum RateLimitMiddlewareError {
    MissingApiKey,
    InvalidApiKey,
    RateExceeded { retry_after: Duration },
}

impl IntoResponse for RateLimitMiddlewareError {
    fn into_response(self) -> Response {
        match self {
            Self::MissingApiKey => (StatusCode::UNAUTHORIZED, "Missing API key").into_response(),
            Self::InvalidApiKey => (StatusCode::UNAUTHORIZED, "Invalid API key").into_response(),
            Self::RateExceeded { retry_after } => {
                let mut response = Response::new(Body::from("Rate limit exceeded"));
                *response.status_mut() = StatusCode::TOO_MANY_REQUESTS;
                response.headers_mut().insert(
                    "Retry-After",
                    retry_after.as_secs().to_string().parse().unwrap(),
                );
                response
            }
        }
    }
}
