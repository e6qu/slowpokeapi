# Task: Create Error Types

## Status
[ ] Pending

## Description

Create error types for the application with proper HTTP response mapping.

## Requirements

1. Central Error enum with thiserror
2. Implement IntoResponse for HTTP responses
3. JSON error response format

## Structure

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>;
```

## Files
- `src/models/error.rs`
- `src/error.rs`

## Notes
- Match error types to OpenAPI spec later
