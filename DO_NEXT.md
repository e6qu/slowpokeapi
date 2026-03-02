# Do Next

## Immediate: Phase 1 - Project Foundation

### Goal

Set up the Rust project structure with basic Axum server, configuration, and health endpoints.

### Tasks

| # | Task | Files | Status |
|---|------|-------|--------|
| 1.1 | Initialize Cargo project with dependencies | `Cargo.toml` | Pending |
| 1.2 | Create directory structure | `src/{main,config,server,handlers,models}/` | Pending |
| 1.3 | Implement configuration loading (env + file) | `src/config/mod.rs`, `src/config/settings.rs` | Pending |
| 1.4 | Set up tracing/logging | `src/logging.rs` | Pending |
| 1.5 | Create basic Axum router | `src/server/mod.rs`, `src/server/router.rs` | Pending |
| 1.6 | Implement AppState | `src/server/state.rs` | Pending |
| 1.7 | Add health endpoints (`/healthz`, `/readyz`, `/livez`) | `src/handlers/health.rs` | Pending |
| 1.8 | Add deep health check (`/health`) | `src/handlers/health.rs` | Pending |
| 1.9 | Create error types and responses | `src/models/error.rs` | Pending |
| 1.10 | Add basic tests | `tests/health.rs` | Pending |
| 1.11 | Set up CI workflow | `.github/workflows/ci.yml` | Pending |
| 1.12 | Create lint configuration | `clippy.toml`, `rustfmt.toml` | Pending |

### Acceptance Criteria

- [ ] Binary starts and responds to health checks
- [ ] Configuration loads from environment variables
- [ ] Logging works with configurable levels
- [ ] All health endpoints return correct responses
- [ ] `cargo test` passes
- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo fmt --check` passes
- [ ] CI pipeline runs and passes

### Commands to Verify

```bash
cargo run &
curl http://localhost:8080/healthz   # => "ok"
curl http://localhost:8080/readyz    # => "ok"
curl http://localhost:8080/livez     # => "ok"
curl http://localhost:8080/health    # => JSON health report
```

### After Phase 1

Update these files:
1. `PLAN.md` - Mark Phase 1 complete
2. `STATUS.md` - Update current phase to 2
3. `WHAT_WE_DID.md` - Document Phase 1 implementation
4. `DO_NEXT.md` - Set up Phase 2 tasks

Then:
- Move `tasks/phase1/*.md` to `tasks/done/phase1/`
- Create `tasks/phase2/*.md` for Phase 2 tasks
- Push to branch, create PR, ensure CI passes
