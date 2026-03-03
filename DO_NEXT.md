# Do Next
## Phase 16: Rate Limiting & Quota
### Goal

Implement per-API-key rate limiting and quota tracking.

### Tasks

| #  | Task | Files | Status |
|---|------|-------|--------|
| 16.1 | Create rate limit module | `src/ratelimit/mod.rs` | Pending |
| 16.2 | Implement token bucket | `src/ratelimit/token_bucket.rs` | Pending |
| 16.3 | Implement API key store | `src/storage/api_keys.rs` | Pending |
| 16.4 | Create rate limit middleware | `src/server/middleware/ratelimit.rs` | Pending |
| 16.5 | Implement `/v1/quota` endpoint | `src/handlers/quota.rs` | Pending |
| 16.6 | Add rate limit headers | `src/server/middleware/ratelimit.rs` | Pending |
| 16.7 | Add OpenAPI annotations | `src/handlers/quota.rs` | Pending |
| 16.8 | Test rate limiting | `tests/ratelimit.rs` | Pending |

### Deliverables

- Rate limiting middleware
- `GET /v1/quota` endpoint
- Rate limit headers in responses

### Acceptance Criteria
- [ ] Token bucket algorithm implemented
- [ ] API key storage in SQLite
- [ ] Rate limit middleware created
- [ ] Quota endpoint returns usage info
- [ ] Rate limit headers included
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
1. Update PLAN.md - Mark Phase 16 complete
2. Update STATUS.md - Move to Phase 17
3. Update WHAT_WE_DID.md - document Phase 16
4. Update DO_NEXT.md - set up Phase 17 tasks
5. Create feature branch for Phase 17
6. Create PR
7. Ensure CI passes
