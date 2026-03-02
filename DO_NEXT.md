# Do Next

## Phase 3: Prometheus Metrics

### Goal

Add Prometheus metrics endpoint with standard HTTP request metrics.

### Tasks

| # | Task | Files | Status |
|---|------|-------|--------|
| 3.1 | Add prometheus and axum-prometheus dependencies | `Cargo.toml` | Pending |
| 3.2 | Create metrics module | `src/metrics/mod.rs` | Pending |
| 3.3 | Define custom metrics | `src/metrics/definitions.rs` | Pending |
| 3.4 | Add Prometheus middleware layer | `src/server/router.rs` | Pending |
| 3.5 | Implement /metrics endpoint | `src/handlers/metrics.rs` | Pending |
| 3.6 | Add HTTP request metrics | `src/metrics/definitions.rs` | Pending |
| 3.7 | Test metrics output | `tests/metrics.rs` | Pending |

### Task Details

#### 3.1 - Add Dependencies
Add to `Cargo.toml`:
- `prometheus` - Prometheus metrics library
- `axum-prometheus` - Axum integration for Prometheus metrics

#### 3.2 - Create Metrics Module
Create `src/metrics/mod.rs`:
```rust
pub mod definitions;

pub use definitions::*;
```

#### 3.3 - Define Custom Metrics
Create `src/metrics/definitions.rs`:
- HTTP request duration histogram
- HTTP request counter
- In-flight requests gauge

#### 3.4 - Add Middleware Layer
Update `src/server/router.rs`:
- Add PrometheusMiddlewareLayer
- Configure metrics prefix

#### 3.5 - Implement /metrics Endpoint
Create `src/handlers/metrics.rs`:
- Serve Prometheus text format metrics
- Add OpenAPI annotation

#### 3.6 - HTTP Request Metrics
Configure standard metrics:
- `http_requests_total` - Counter
- `http_request_duration_seconds` - Histogram
- `http_requests_in_flight` - Gauge

#### 3.7 - Tests
- Test that `/metrics` returns Prometheus format
- Verify metric names are present
- Test metrics are updated after requests

### Deliverables

- `/metrics` endpoint in Prometheus text format
- HTTP request count, latency, and in-flight metrics
- Standard Prometheus naming conventions

### Acceptance Criteria

- [ ] `/metrics` endpoint returns valid Prometheus format
- [ ] HTTP request metrics are collected
- [ ] Tests pass
- [ ] Clippy passes with no warnings
- [ ] Format check passes
- [ ] CI passes

### Verification Commands

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

cargo run &
curl http://localhost:8080/metrics
curl http://localhost:8080/healthz
curl http://localhost:8080/metrics | grep http_requests
```

### After Completion

1. Update PLAN.md - Mark Phase 3 complete
2. Update STATUS.md - Move to Phase 4
3. Update WHAT_WE_DID.md - Document Phase 3
4. Update DO_NEXT.md - Set up Phase 4 tasks
5. Move `tasks/phase3/*.md` to `tasks/done/phase3/`
6. Create feature branch for Phase 4
7. Create PR
8. Ensure CI passes
