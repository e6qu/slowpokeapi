# Task: Implement Health Endpoints

## Status
[ ] Pending

## Description

Implement all four health check endpoints.

## Requirements

### /healthz (Liveness)
- Always return 200 "ok" if process is alive
- No checks, just alive confirmation

### /readyz (Readiness)
- Return 200 if ready to serve traffic
- Return 503 if not ready
- For now, always ready

### /livez (Startup)
- Return 200 after startup complete
- Return 503 during startup
- Use atomic bool to track startup

### /health (Deep Check)
- Return JSON with detailed status
- Include uptime, version
- For now, return "healthy" status

## Files
- `src/handlers/health.rs`

## Notes
- See specs/api/health.md for detailed spec
