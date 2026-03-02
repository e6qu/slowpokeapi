# Do Next

## Phase 1: Project Foundation (Continuing)

### Goal

Complete the Rust project structure with proper modularity, configuration, and deep health check.

### Remaining Tasks

| # | Task | Files | Status |
|---|------|-------|--------|
| 1.2 | Create directory structure | `src/{config,server,handlers,models}/mod.rs` | Pending |
| 1.3 | Implement configuration loading | `src/config/{mod.rs, settings.rs}` | Pending |
| 1.5 | Modularize Axum router | `src/server/{mod.rs, router.rs, state.rs}` | Pending |
| 1.6 | Implement AppState | `src/server/state.rs` | Pending |
| 1.8 | Add deep health check | `src/handlers/health.rs`, `src/server/middleware/mod.rs` | Pending |
| 1.10 | Add basic tests | `tests/health_test.rs` | Pending |

### Task Details

#### 1.2 - Directory Structure
Create the modular directory structure:
```
src/
├── config/
│   ├── mod.rs
│   └── settings.rs
├── server/
│   ├── mod.rs
│   ├── router.rs
│   ├── state.rs
│   └── middleware/
│       └── mod.rs
├── handlers/
│   ├── mod.rs
│   └── health.rs
└── models/
    ├── mod.rs
    └── error.rs (move from src/error.rs)
```

#### 1.3 - Configuration Loading
- Use `config` crate for environment variables and config files
- Support `SLOWPOKEAPI_*` prefix for env vars
- Configuration struct with server, logging settings
- Default values

#### 1.5 - Modular Router
- Extract router to `src/server/router.rs`
- Register routes in modules
- Add middleware layers

#### 1.6 - AppState
- Create AppState struct with Arc for sharing
- Include config and startup time
- Make available to handlers

#### 1.8 - Deep Health Check
- Implement `/health` endpoint returning JSON
- Include component health checks
- Return status, version, uptime

#### 1.10 - Tests
- Integration test for health endpoints
- Test configuration loading
- Ensure all tests pass

### Acceptance Criteria

- [ ] All health endpoints work (/healthz, /readyz, /livez, /health)
- [ ] Configuration loads from environment
- [ ] Code is modular with proper directory structure
- [ ] All tests pass
- [ ] Clippy passes with no warnings
- [ ] Format check passes
- [ ] CI passes

### Verification Commands

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

cargo run &
curl http://localhost:8080/healthz
curl http://localhost:8080/readyz
curl http://localhost:8080/livez
curl http://localhost:8080/health | jq
```

### After Completion

1. Update PLAN.md - Mark Phase 1 complete
2. Update STATUS.md - Move to Phase 2
3. Update WHAT_WE_DID.md - Document Phase 1
4. Update DO_NEXT.md - Set up Phase 2 tasks
5. Move `tasks/phase1/*.md` to `tasks/done/phase1/`
6. Create feature branch for Phase 2
7. Create PR
8. Ensure CI passes
