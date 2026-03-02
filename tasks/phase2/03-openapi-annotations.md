# Task: Add OpenAPI Annotations

## Status
[x] Done

## Description

Add `#[utoipa::path]` annotations to health handlers.

## Files
- `src/handlers/health.rs`

## Details

- Annotate `healthz`, `readyz`, `livez`, `health` handlers
- Define responses with status codes
- Link to schema types
