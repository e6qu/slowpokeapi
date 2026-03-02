# Do Next

## Phase 7: Upstream API Clients

### Goal

Implement HTTP clients for Frankfurter and fawazahmed0 APIs with fallback and circuit breaker.

### Tasks

| # | Task | Files | Status |
|---|------|-------|--------|
| 7.1 | Add reqwest dependency | `Cargo.toml` | Pending |
| 7.2 | Create upstream module | `src/upstream/mod.rs` | Pending |
| 7.3 | Define Upstream trait | `src/upstream/mod.rs` | Pending |
| 7.4 | Create shared HTTP client | `src/upstream/client.rs` | Pending |
| 7.5 | Implement Frankfurter client | `src/upstream/frankfurter.rs` | Pending |
| 7.6 | Implement fawazahmed0 client | `src/upstream/fawaz.rs` | Pending |
| 7.7 | Create upstream manager with fallback | `src/upstream/manager.rs` | Pending |
| 7.8 | Add circuit breaker | `src/upstream/circuit_breaker.rs` | Pending |
| 7.9 | Add upstream metrics | `src/upstream/metrics.rs` | Pending |
| 7.10 | Test upstream clients | `tests/upstream.rs` | Pending |

### Task Details

#### 7.1 - Add reqwest Dependency
Add to `Cargo.toml` (already in dev-dependencies, move to dependencies):
```toml
reqwest = { version = "0.12", features = ["json"] }
```

#### 7.2 - Create Upstream Module
Create `src/upstream/mod.rs` with module structure.

#### 7.3 - Define Upstream Trait
```rust
#[async_trait]
pub trait Upstream: Send + Sync {
    async fn get_latest_rates(&self, base: &str) -> Result<RateCollection>;
    async fn get_historical_rates(&self, base: &str, date: NaiveDate) -> Result<HistoricalRate>;
    fn name(&self) -> &str;
    fn is_healthy(&self) -> bool;
}
```

#### 7.4 - Create Shared HTTP Client
Create `src/upstream/client.rs`:
- Shared reqwest client with connection pooling
- Configurable timeout
- Retry configuration

#### 7.5 - Implement Frankfurter Client
Create `src/upstream/frankfurter.rs`:
- Base URL: https://api.frankfurter.app
- Latest rates: `/latest?from={base}`
- Historical: `/{date}..{date}?from={base}`

#### 7.6 - Implement fawazahmed0 Client
Create `src/upstream/fawaz.rs`:
- Base URL: https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1
- Latest rates: `/currencies/{base}.json`
- Extensive currency support including crypto

#### 7.7 - Create Upstream Manager
Create `src/upstream/manager.rs`:
- Fallback chain: Frankfurter → fawazahmed0
- Automatic failover on errors
- Health tracking

#### 7.8 - Add Circuit Breaker
Create `src/upstream/circuit_breaker.rs`:
- Open after N consecutive failures
- Half-open after timeout
- Close on success

#### 7.9 - Add Upstream Metrics
Create `src/upstream/metrics.rs`:
- `slowpokeapi_upstream_requests_total`
- `slowpokeapi_upstream_errors_total`
- `slowpokeapi_upstream_latency_seconds`
- `slowpokeapi_upstream_circuit_breaker_state`

#### 7.10 - Test Upstream Clients
Create `tests/upstream.rs`:
- Mock HTTP responses for unit tests
- Test fallback behavior
- Test circuit breaker

### Deliverables

- HTTP clients for fiat currency APIs
- Fallback chain (Frankfurter → fawaz)
- Circuit breaker for fault tolerance
- Upstream metrics

### Acceptance Criteria

- [ ] Upstream trait defined
- [ ] Frankfurter client implemented
- [ ] fawazahmed0 client implemented
- [ ] Fallback manager implemented
- [ ] Circuit breaker implemented
- [ ] Metrics exposed
- [ ] Tests pass
- [ ] Clippy passes with no warnings
- [ ] Format check passes
- [ ] CI passes

### Verification Commands

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

# Run and verify upstream metrics
cargo run &
curl http://localhost:8080/metrics | grep slowpokeapi_upstream
```

### After Completion

1. Update PLAN.md - Mark Phase 7 complete
2. Update STATUS.md - Move to Phase 8
3. Update WHAT_WE_DID.md - Document Phase 7
4. Update DO_NEXT.md - Set up Phase 8 tasks
5. Move `tasks/phase7/*.md` to `tasks/done/phase7/`
6. Create feature branch for Phase 8
7. Create PR
8. Ensure CI passes
