//! Error types for SlowPokeAPI

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use super::api::{ErrorResponse, ErrorType, ResponseResult};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Invalid currency code: {0}")]
    InvalidCurrency(String),

    #[error("Invalid date: {0}")]
    InvalidDate(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl Error {
    pub fn to_error_response(&self) -> ErrorResponse {
        let (error_type, message) = match self {
            Error::NotFound(msg) => (ErrorType::NotFound, Some(msg.clone())),
            Error::Validation(msg) => (ErrorType::MalformedRequest, Some(msg.clone())),
            Error::InvalidCurrency(msg) => (ErrorType::InvalidCurrency, Some(msg.clone())),
            Error::InvalidDate(msg) => (ErrorType::InvalidDate, Some(msg.clone())),
            Error::Internal(msg) => (ErrorType::InvalidKey, Some(msg.clone())),
            Error::Database(e) => (ErrorType::InvalidKey, Some(e.to_string())),
        };

        ErrorResponse {
            result: ResponseResult::Error,
            error_type,
            message,
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::Validation(_) => StatusCode::BAD_REQUEST,
            Error::InvalidCurrency(_) => StatusCode::BAD_REQUEST,
            Error::InvalidDate(_) => StatusCode::BAD_REQUEST,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_response = self.to_error_response();
        (status, Json(error_response)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
