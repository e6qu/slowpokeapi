# Task: Add Basic Tests

## Status
[ ] Pending

## Description

Add integration tests for health endpoints.

## Requirements

1. Test each health endpoint
2. Verify status codes
3. Verify response content

## Files
- `tests/health.rs`

## Notes
- Use tokio::test for async tests
- Spin up test server with axum::TestServer
