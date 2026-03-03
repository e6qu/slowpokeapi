# Do Next
## Phase 16: Rate Limiting & Quota
### Goal

Implement per-API-key rate limiting and quota tracking.

### Tasks

| #  | Task | Files | Status |
|---|------|-------|--------|
| 16.1 | Create rate limit module | `src/ratelimit/mod.rs` | Done (from Phase 17 prep) |
| 16.2 | Implement token bucket | `src/ratelimit/token_bucket.rs` | Done (from Phase 17 prep) |
| 16.3 | Implement API key store | `src/storage/api_keys.rs` | Done (from Phase 17 prep) |
| 16.4 | Create rate limit middleware | `src/server/middleware/ratelimit.rs` | Pending |
| 16.5 | Implement `/v1/quota` endpoint | `src/handlers/quota.rs` | Pending |
| 16.6 | Add rate limit headers | `src/server/middleware/ratelimit.rs` | Pending |
| 16.7 | Add OpenAPI annotations | `src/handlers/quota.rs` | Pending |
| 16.8 | Test rate limiting | `tests/ratelimit.rs` | Done (partial) |

### Deliverables

- Rate limiting middleware
- `GET /v1/quota` endpoint
- Rate limit headers (X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset)

### Acceptance Criteria
- [ ] Token bucket rate limiting implemented
- [ ] Per-API-key rate limiting
- [ ] Rate limit middleware integrated
- [ ] Quota endpoint returns usage info
- [ ] Rate limit headers included in responses
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
2. Update STATUS.md - Move to Phase 18
3. Update WHAT_WE_DID.md - document Phase 16
4. Update DO_NEXT.md - set up Phase 18 tasks
5. Create feature branch for Phase 18
6. Create PR
7. Ensure CI passes
