# Task: Create API Response Models

## Status
[x] Done

## Description

Create API response types with OpenAPI schemas for health endpoint.

## Files
- `src/models/api/mod.rs`
- `src/models/api/response.rs`

## Details

- Create `HealthResponse` with `#[derive(ToSchema)]`
- Create `ComponentHealth` struct
- Add proper serde attributes
