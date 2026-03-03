# Do Next

## Phase 8: Currencies Endpoint

### Goal

Implement `/v1/currencies` and `/v1/currencies.min` endpoints to list supported currencies.

### Tasks

| # | Task | Files | Status |
|---|------|-------|--------|
| 8.1 | Create currencies handler | `src/handlers/currencies.rs` | Pending |
| 8.2 | Implement list currencies | `src/handlers/currencies.rs` | Pending |
| 8.3 | Implement minimal currencies | `src/handlers/currencies.rs` | Pending |
| 8.4 | Add OpenAPI annotations | `src/handlers/currencies.rs` | Pending |
| 8.5 | Add routes to router | `src/server/router.rs` | Pending |
| 8.6 | Seed currency data on startup | `src/bootstrap.rs` | Pending |
| 8.7 | Test endpoints | `tests/currencies.rs` | Pending |

### Task Details

#### 8.1 - Create Currencies Handler
Create `src/handlers/currencies.rs`:
- Handler for full currencies list
- Handler for minimal currencies list

#### 8.2 - Implement List Currencies
- Return all supported currencies with metadata
- Include currency code, name, symbol
- Return as `CurrenciesResponse` model (flattened HashMap)

#### 8.3 - Implement Minimal Currencies
- Return only currency codes (no metadata)
- Use same `CurrenciesResponse` type
- Flat structure with codes as keys

#### 8.4 - Add OpenAPI Annotations
- Add `#[utoipa::path]` annotations
- Document response schemas
- Add tags for grouping

#### 8.5 - Add Routes to Router
Update `src/server/router.rs`:
- Mount `GET /v1/currencies`
- Mount `GET /v1/currencies.min`

#### 8.6 - Seed Currency Data
Create `src/bootstrap.rs`:
- Define list of supported fiat currencies
- Currency metadata (code, name, symbol)
- Load on startup
- At least 10 currencies: USD, EUR, GBP, JPY, CAD, AUD, CHF, CNY, SEK, NZD

#### 8.7 - Test Endpoints
Create `tests/currencies.rs`:
- Test full currencies endpoint
- Test minimal currencies endpoint
- Test response format
- Test at least 10 currencies returned

### Deliverables

- `GET /v1/currencies` - Full currency list with names and symbols
- `GET /v1/currencies.min` - Currency codes only

### Acceptance Criteria

- [ ] Currencies handler created
- [ ] Full list returns all currencies with metadata
- [ ] Minimal list returns just codes
- [ ] OpenAPI annotations added
- [ ] Routes mounted
- [ ] Currency data seeded
- [ ] Tests pass
- [ ] Clippy passes with no warnings
- [ ] Format check passes
- [ ] CI passes
- [ ] At least 10 currencies available
- [ ] Each currency has name

### Verification Commands

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

# Run and test endpoints
cargo run &
curl http://localhost:8080/v1/currencies
curl http://localhost:8080/v1/currencies.min
```

### After Completion

1. Update PLAN.md - Mark Phase 8 complete
2. Update STATUS.md - Move to Phase 9
3. Update WHAT_WE_DID.md - Document Phase 8
4. Update DO_NEXT.md - Set up Phase 9 tasks
5. Create feature branch for Phase 9
6. Create PR
7. Ensure CI passes
