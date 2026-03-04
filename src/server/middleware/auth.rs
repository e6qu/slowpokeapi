use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::auth::{validate_api_key, AuthConfig};
use crate::storage::ApiKeyStore;

pub struct AuthMiddleware {
    config: AuthConfig,
    api_key_store: Arc<ApiKeyStore>,
}

impl AuthMiddleware {
    pub fn new(config: AuthConfig, api_key_store: Arc<ApiKeyStore>) -> Self {
        Self {
            config,
            api_key_store,
        }
    }

    pub async fn validate<B>(&self, request: &Request<B>) -> Result<String, AuthError> {
        if !self.config.enabled {
            return Ok("anonymous".to_string());
        }

        let path = request.uri().path();

        if self.is_public_path(path) {
            return Ok("public".to_string());
        }

        if !self.config.require_api_key {
            return Ok("anonymous".to_string());
        }

        let api_key = validate_api_key(request).ok_or(AuthError::MissingApiKey)?;

        let key_info = self
            .api_key_store
            .get(&api_key)
            .await
            .ok_or(AuthError::InvalidApiKey)?;

        if !key_info.is_active {
            return Err(AuthError::InvalidApiKey);
        }

        Ok(api_key)
    }

    fn is_public_path(&self, path: &str) -> bool {
        self.config
            .public_paths
            .iter()
            .any(|public_path| path == public_path || path.starts_with(public_path))
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingApiKey,
    InvalidApiKey,
}

pub async fn auth_middleware(
    State(auth_middleware): State<Arc<AuthMiddleware>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, AuthMiddlewareError> {
    let api_key_result = auth_middleware.validate(&request).await;
    let is_authenticated = api_key_result.is_ok();

    let _api_key = api_key_result.map_err(AuthMiddlewareError::from)?;

    let mut response = next.run(request).await;

    if is_authenticated {
        if let Ok(value) = "true".parse() {
            response.headers_mut().insert("X-API-Key-Valid", value);
        }
    }

    Ok(response)
}

#[derive(Debug)]
pub enum AuthMiddlewareError {
    Auth(AuthError),
}

impl From<AuthError> for AuthMiddlewareError {
    fn from(err: AuthError) -> Self {
        Self::Auth(err)
    }
}

impl IntoResponse for AuthMiddlewareError {
    fn into_response(self) -> Response {
        match self {
            Self::Auth(AuthError::MissingApiKey) => {
                (StatusCode::UNAUTHORIZED, "Missing API key").into_response()
            }
            Self::Auth(AuthError::InvalidApiKey) => {
                (StatusCode::UNAUTHORIZED, "Invalid API key").into_response()
            }
        }
    }
}
