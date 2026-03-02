use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid currency code: {0}")]
    InvalidCurrencyCode(String),

    #[error("Invalid date format: {0}")]
    InvalidDate(String),

    #[error("Invalid rate value: {0}")]
    InvalidRate(String),

    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    #[error("Missing required parameter: {0}")]
    MissingParameter(String),
}
