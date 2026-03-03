# Do Next
## Phase 12: Enriched Endpoint
### Goal

Implement `/v1/enriched/{base}/{target}` endpoint with currency metadata.

### Tasks

| #  | Task | Files | Status |
|---|------|-------|--------|
| 12.1 | Create enriched handler | `src/handlers/enriched.rs` | Pending |
| 12.2 | Create currency metadata database | `src/storage/metadata.rs` | Pending |
| 12.3 | Seed metadata on startup | `src/bootstrap.rs` | Pending |
| 12.4 | Implement enriched response | `src/handlers/enriched.rs` | Pending |
| 12.5 | Add OpenAPI annotations | `src/handlers/enriched.rs` | Pending |
| 12.6 | Add route to router | `src/server/router.rs` | Pending |
| 12.7 | Test endpoint | `tests/enriched.rs` | Pending |

### Task Details

#### 12.1 - Create enriched handler
Create `src/handlers/enriched.rs`:
- Handler for enriched endpoint
- Path parameters for base and target currencies
- Fetch rate and metadata
- Return `EnrichedResponse`

#### 12.2 - Create currency metadata database
- Define metadata structure
- Static metadata for common currencies
- Include: name, symbol, locale, country code, flag URL

#### 12.3 - Seed metadata on startup
- Initialize metadata on app startup
- Store in memory for fast access
- Consider SQLite for persistence (optional)

#### 12.4 - Implement enriched response
- Combine rate data with metadata
- Format as `EnrichedResponse`
- Include target currency details

#### 12.5 - Add OpenAPI annotations
- Add `#[utoipa::path]` annotations
- Document response schemas
- Add tags for grouping

#### 12.6 - Add route to router
Update `src/server/router.rs`:
- Mount `GET /v1/enriched/:base_code/:target_code`

#### 12.7 - Test endpoint
Create `tests/enriched.rs`:
- Test enriched response structure
- Test metadata inclusion
- Test invalid currency codes

### Deliverables

- `GET /v1/enriched/{base}/{target}` - Rate with target currency metadata
- Response formatted as RapidAPI-compatible JSON
- Metadata includes: name, symbol, locale, etc.

### Acceptance Criteria
- [ ] Enriched handler created
- [ ] Metadata database created
- [ ] Metadata seeding implemented
- [ ] Response formatting correct
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
curl http://localhost:8080/v1/enriched/USD/EUR
```

### After completion
1. Update PLAN.md - Mark Phase 12 complete
2. Update STATUS.md - Move to Phase 13
3. Update WHAT_WE_DID.md - document Phase 12
4. Update DO_NEXT.md - set up Phase 13 tasks
5. Create feature branch for Phase 13
6. Create PR
7. Ensure CI passes
