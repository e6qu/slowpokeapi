# Do Next
## Phase 17: Authentication
### Goal

Implement API key authentication.

### Tasks

| #  | Task | Files | Status |
|---|------|-------|--------|
| 17.1 | Create auth module | `src/auth/mod.rs` | Pending |
| 17.2 | Implement API key validation | `src/auth/api_key.rs` | Pending |
| 17.3 | Create auth middleware | `src/server/middleware/auth.rs` | Pending |
| 17.4 | Support header and query auth | `src/auth/api_key.rs` | Pending |
| 17.5 | Add auth configuration | `src/config/settings.rs` | Pending |
| 17.6 | Update OpenAPI for auth | Various | Pending |
| 17.7 | Test authentication | `tests/auth.rs` | Pending |

### Deliverables

- Bearer token authentication
- Query parameter authentication
- Configurable auth requirement

### Acceptance Criteria
- [ ] API key validation implemented
- [ ] Authentication middleware created
- [ ] Protected routes configured
- [ ] Validation endpoint returns key info
- [ ] OpenAPI documentation updated
- [ ] Tests pass
- [ ] Clippy passes with no warnings
- [ ] Format check passes
- [ ] CI passes

### Verification commands
```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

### After completion
1. Update PLAN.md - Mark Phase 17 complete
2. Update STATUS.md - Move to Phase 18
3. Update WHAT_WE_DID.md - document Phase 17
4. Update DO_NEXT.md - set up Phase 18 tasks
5. Create feature branch for Phase 18
6. Create PR
7. Ensure CI passes
