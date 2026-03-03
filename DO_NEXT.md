# Do Next
## Phase 9: Latest Rates Endpoint
### Goal

Implement `/v1/latest` endpoint to fetch current exchange rates.

### Tasks

| #  | Task | Files | Status |
|---|------|-------|--------|
| 9.1 | Create latest handler | `src/handlers/latest.rs` | Pending |
| 9.2 | Implement rate fetching logic | `src/services/rates.rs` | Pending |
| 9.3 | Add cache integration | `src/services/rates.rs` | Pending |
| 9.4 | Add upstream fallback | `src/services/rates.rs` | Pending |
| 9.5 | Implement response formatting | `src/handlers/latest.rs` | Pending |
| 9.6 | Add input validation | `src/handlers/latest.rs` | Pending |
| 9.7 | Add openapi annotations | `src/handlers/latest.rs` | Pending |
| 9.8 | Add route to router | `src/server/router.rs` | Pending |
| 9.9 | Test endpoint | `tests/latest.rs` | Pending |

### Task Details

#### 9.1 - Create latest handler
Create `src/handlers/latest.rs`:
- Handler for latest rates endpoint
- Use upstream manager to fetch rates
- Return `LatestRatesResponse`

#### 9.2 - implement rate fetching logic
Create `src/services/rates.rs`:
- Service to fetch latest rates
- Check cache first
- Try upstream APIs on miss
- Update cache on success

#### 9.3 - add cache integration
- Inject cache into rates service
- Use TieredCache for storage

#### 9.4 - add upstream fallback
- Use upstream manager to try primary then fallback
- Return error if all fail

#### 9.5 - implement response formatting
- Format as `LatestRatesResponse`
- Include base currency, date, rates
- Include source information

#### 9.6 - add input validation
- Validate base currency (3-letter ISO code)
- Validate target currencies (comma-separated or list)
- Return 400 for invalid currencies

#### 9.7 - add openapi annotations
- Add `#[utoipa::path]` annotations
- Document response schemas
- Add tags for grouping

#### 9.8 - add route to router
Update `src/server/router.rs`:
- Mount `GET /v1/latest`

#### 9.9 - test endpoint
Create `tests/latest.rs`:
- Test latest rates endpoint
- Test response format
- Test caching behavior
- Test upstream fallback

### Deliverables

- `GET /v1/latest` - Current exchange rates
- Response formatted as RapidAPI-compatible JSON

### Acceptance Criteria
- [ ] Latest handler created
- [ ] Rate fetching logic implemented
- [ ] Cache integration working
- [ ] Upstream fallback working
- [ ] Response formatting correct
- [ ] Input validation working
- [ ] OpenAPI annotations added
- [ ] Routes mounted
- [ ] Tests pass
- [ ] Clippy passes with no warnings
- [ ] Format check passes
- [ ] CI passes

### Verification commands
```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

# Run and test endpoint
cargo run &
curl http://localhost:8080/v1/latest?base=USD
```

### after completion
1. Update PLAN.md - Mark Phase 9 complete
2. Update STATUS.md - Move to Phase 10
3. Update what_we_did.md - document Phase 9
4. Update do_next.md - set up Phase 10 tasks
5. Create feature branch for Phase 10
6. Create PR
7. ensure ci passes
