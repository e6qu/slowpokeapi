use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::net::SocketAddr;
use std::time::Duration;

use crate::ratelimit::{RateLimitError, RateLimitInfo};
use crate::server::AppState;

#[derive(Debug)]
pub enum RateLimitMiddlewareError {
    MissingApiKey,
    InvalidApiKey,
    RateExceeded { retry_after: Duration },
    BackoffRequired { retry_after: Duration },
    ServiceUnavailable { retry_after: Duration },
}

impl IntoResponse for RateLimitMiddlewareError {
    fn into_response(self) -> Response {
        match self {
            Self::MissingApiKey => (StatusCode::UNAUTHORIZED, "Missing API key").into_response(),
            Self::InvalidApiKey => (StatusCode::UNAUTHORIZED, "Invalid API key").into_response(),
            Self::RateExceeded { retry_after } => {
                let mut response = Response::new(Body::from("Rate limit exceeded"));
                *response.status_mut() = StatusCode::TOO_MANY_REQUESTS;
                if let Ok(value) = retry_after.as_secs().to_string().parse() {
                    response.headers_mut().insert("Retry-After", value);
                }
                response
            }
            Self::BackoffRequired { retry_after } => {
                let mut response = Response::new(Body::from("Too many requests, please back off"));
                *response.status_mut() = StatusCode::TOO_MANY_REQUESTS;
                if let Ok(value) = retry_after.as_secs().to_string().parse() {
                    response.headers_mut().insert("Retry-After", value);
                }
                response
            }
            Self::ServiceUnavailable { retry_after } => {
                let mut response = Response::new(Body::from("Service temporarily unavailable"));
                *response.status_mut() = StatusCode::SERVICE_UNAVAILABLE;
                if let Ok(value) = retry_after.as_secs().to_string().parse() {
                    response.headers_mut().insert("Retry-After", value);
                }
                response
            }
        }
    }
}

pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    ConnectInfo(client_addr): ConnectInfo<SocketAddr>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, RateLimitMiddlewareError> {
    let rate_limiter = state.rate_limiter.as_ref().ok_or_else(|| {
        RateLimitMiddlewareError::ServiceUnavailable {
            retry_after: Duration::from_secs(1),
        }
    })?;

    let api_key = extract_api_key(&request);
    let client_ip = client_addr.ip().to_string();

    let rate_limit_result = rate_limiter
        .check_rate_limit(api_key.as_deref(), &client_ip)
        .await;

    match rate_limit_result {
        Ok(info) => {
            let mut response = next.run(request).await;
            add_rate_limit_headers(&mut response, &info);
            Ok(response)
        }
        Err(RateLimitError::InvalidApiKey) => Err(RateLimitMiddlewareError::InvalidApiKey),
        Err(RateLimitError::GlobalLimitExceeded { retry_after }) => {
            Err(RateLimitMiddlewareError::ServiceUnavailable { retry_after })
        }
        Err(RateLimitError::UserLimitExceeded { retry_after }) => {
            Err(RateLimitMiddlewareError::RateExceeded { retry_after })
        }
        Err(RateLimitError::IpLimitExceeded { retry_after }) => {
            Err(RateLimitMiddlewareError::RateExceeded { retry_after })
        }
        Err(RateLimitError::BackoffRequired { retry_after }) => {
            Err(RateLimitMiddlewareError::BackoffRequired { retry_after })
        }
    }
}

fn extract_api_key<B>(request: &Request<B>) -> Option<String> {
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

fn add_rate_limit_headers(response: &mut Response, info: &RateLimitInfo) {
    let headers = response.headers_mut();
    if let Ok(value) = info.limit.to_string().parse() {
        headers.insert("X-RateLimit-Limit", value);
    }
    if let Ok(value) = info.remaining.to_string().parse() {
        headers.insert("X-RateLimit-Remaining", value);
    }
    if let Ok(value) = info.reset_after.as_secs().to_string().parse() {
        headers.insert("X-RateLimit-Reset", value);
    }

    if info.backpressure {
        if let Ok(value) = "true".parse() {
            headers.insert("X-RateLimit-Backpressure", value);
        }
    }

    if let Some(retry_after) = info.retry_after {
        if let Ok(value) = retry_after.as_secs().to_string().parse() {
            headers.insert("X-RateLimit-Retry-After", value);
        }
    }
}
