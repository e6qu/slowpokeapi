# Do Next
## Phase 11: Historical Rates Endpoint
### Goal

Implement `/v1/history/{base}/{year}/{month}/{day}` endpoint.

### Tasks

| #  | Task | Files | Status |
|---|------|-------|--------|
| 11.1 | Create history handler | `src/handlers/history.rs` | Pending |
| 11.2 | Implement historical fetch from upstream | `src/upstream/*.rs` | Pending |
| 11.3 | Add SQLite caching for historical data | `src/cache/sqlite.rs` | Pending |
| 11.4 | Implement response formatting | `src/handlers/history.rs` | Pending |
| 11.5 | Add date validation | `src/handlers/history.rs` | Pending |
| 11.6 | Add OpenAPI annotations | `src/handlers/history.rs` | Pending |
| 11.7 | Add route to router | `src/server/router.rs` | Pending |
| 11.8 | Test endpoint | `tests/history.rs` | Pending |

### Task Details

#### 11.1 - Create history handler
Create `src/handlers/history.rs`:
- Handler for historical rates endpoint
- Path parameters: base, year, month, day
- Return `HistoricalResponse`

#### 11.2 - Implement historical fetch
- Use upstream manager's `get_historical_rates` method
- Check cache first
- Fallback chain on miss

#### 11.3 - Add caching
- Cache key: `history:{base}:{year}:{month}:{day}`
- Longer TTL for historical data (doesn't change)

#### 11.4 - Response formatting
- Format as `HistoricalResponse`
- Include base_code, year, month, day
- Include conversion_rates

#### 11.5 - Date validation
- Validate year (reasonable range, e.g., 1999+)
- Validate month (1-12)
- Validate day (1-31, consider month)
- Return 400 for invalid dates

#### 11.6 - Add OpenAPI annotations
- Add `#[utoipa::path]` annotations
- Document response schemas
- Add tags for grouping

#### 11.7 - Add route to router
Update `src/server/router.rs`:
- Mount `GET /v1/history/:base_code/:year/:month/:day`

#### 11.8 - Test endpoint
Create `tests/history.rs`:
- Test valid historical date
- Test invalid date formats
- Test future date rejection

### Deliverables

- `GET /v1/history/{base}/{year}/{month}/{day}` - Historical rates
- Response formatted as RapidAPI-compatible JSON
- Caching of historical data

### Acceptance Criteria
- [ ] History handler created
- [ ] Historical fetch working
- [ ] Caching working
- [ ] Response formatting correct
- [ ] Date validation working
- [ ] OpenAPI annotations added
- [ ] Route mounted
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
curl http://localhost:8080/v1/history/USD/2024/01/15
```

### After completion
1. Update PLAN.md - Mark Phase 11 complete
2. Update STATUS.md - Move to Phase 12
3. Update WHAT_WE_DID.md - document Phase 11
4. Update DO_NEXT.md - set up Phase 12 tasks
5. Create feature branch for Phase 12
6. Create PR
7. Ensure CI passes
