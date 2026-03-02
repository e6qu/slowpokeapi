# Task: Create Axum Router

## Status
[ ] Pending

## Description

Set up the basic Axum router with health endpoints.

## Requirements

1. Create router with health routes
2. Add tracing layer
3. Add CORS layer (permissive for dev)
4. Return 404 for unknown routes

## Routes

- `GET /healthz` - Liveness probe
- `GET /readyz` - Readiness probe
- `GET /livez` - Startup probe
- `GET /health` - Deep health check

## Files
- `src/server/mod.rs`
- `src/server/router.rs`

## Notes
- Use `axum::Router`
- Add layers for middleware
