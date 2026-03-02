# Do Next

## Phase 5: Data Models

### Goal

Implement all domain models and API request/response types with validation and OpenAPI schemas.

### Tasks

| # | Task | Files | Status |
|---|------|-------|--------|
| 5.1 | Create Currency model | `src/models/currency.rs` | Pending |
| 5.2 | Create ExchangeRate model | `src/models/rate.rs` | Pending |
| 5.3 | Create HistoricalRate model | `src/models/historical.rs` | Pending |
| 5.4 | Create CurrencyMetadata model | `src/models/metadata.rs` | Pending |
| 5.5 | Create API response types | `src/models/api/response.rs` | Pending |
| 5.6 | Create error types with API mapping | `src/models/error.rs` | Pending |
| 5.7 | Add validation logic | `src/models/validation.rs` | Pending |
| 5.8 | Add OpenAPI schemas to all models | Various | Pending |
| 5.9 | Test model serialization | `tests/models.rs` | Pending |

### Task Details

#### 5.1 - Create Currency Model
Create `src/models/currency.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: Option<String>,
}
```

#### 5.2 - Create ExchangeRate Model
Create `src/models/rate.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExchangeRate {
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f64>,
}
```

#### 5.3 - Create HistoricalRate Model
Create `src/models/historical.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HistoricalRate {
    pub date: String,
    pub base: String,
    pub rates: HashMap<String, f64>,
}
```

#### 5.4 - Create CurrencyMetadata Model
Create `src/models/metadata.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CurrencyMetadata {
    pub code: String,
    pub name: String,
    pub symbol: Option<String>,
    pub decimal_places: u8,
    pub country: Option<String>,
}
```

#### 5.5 - Create API Response Types
Create `src/models/api/response.rs`:
- `CurrenciesResponse` - List of supported currencies
- `LatestRatesResponse` - Latest exchange rates
- `HistoricalRatesResponse` - Historical rates
- `ConvertResponse` - Currency conversion result
- `ErrorResponse` - API error response

#### 5.6 - Create Error Types with API Mapping
Update `src/models/error.rs`:
- Map domain errors to HTTP status codes
- Add API error response generation
- Add validation errors

#### 5.7 - Add Validation Logic
Create `src/models/validation.rs`:
- Currency code validation (ISO 4217)
- Date format validation
- Rate value validation
- Request parameter validation

#### 5.8 - Add OpenAPI Schemas
Add `ToSchema` derive to all models and `utoipa::path` to response types.

#### 5.9 - Test Model Serialization
Create `tests/models.rs`:
- Test JSON serialization/deserialization
- Test validation logic
- Test edge cases

### Deliverables

- All domain models with validation
- API request/response types
- OpenAPI schema annotations
- Comprehensive test coverage

### Acceptance Criteria

- [ ] All models have ToSchema derive
- [ ] Models validate input correctly
- [ ] Error types map to HTTP status codes
- [ ] Tests pass
- [ ] Clippy passes with no warnings
- [ ] Format check passes
- [ ] CI passes

### Verification Commands

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

# Check OpenAPI schema
cargo run &
curl http://localhost:8080/api-docs/openapi.json | jq '.components.schemas'
```

### After Completion

1. Update PLAN.md - Mark Phase 5 complete
2. Update STATUS.md - Move to Phase 6
3. Update WHAT_WE_DID.md - Document Phase 5
4. Update DO_NEXT.md - Set up Phase 6 tasks
5. Move `tasks/phase5/*.md` to `tasks/done/phase5/`
6. Create feature branch for Phase 6
7. Create PR
8. Ensure CI passes
