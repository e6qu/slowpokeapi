# Do Next

## Phase 2: OpenAPI & Swagger UI

### Goal

Add OpenAPI schema generation and Swagger UI documentation for the API.

### Tasks

| # | Task | Files | Status |
|---|------|-------|--------|
| 2.1 | Add utoipa dependencies | `Cargo.toml` | Pending |
| 2.2 | Create API response models with schemas | `src/models/api/response.rs` | Pending |
| 2.3 | Add OpenAPI annotations to health handlers | `src/handlers/health.rs` | Pending |
| 2.4 | Create OpenAPI router configuration | `src/server/router.rs` | Pending |
| 2.5 | Add Swagger UI endpoint | `src/server/router.rs` | Pending |
| 2.6 | Add OpenAPI spec JSON endpoint | `src/server/router.rs` | Pending |
| 2.7 | Test Swagger UI | `tests/openapi.rs` | Pending |

### Task Details

#### 2.1 - Add utoipa Dependencies
Add to `Cargo.toml`:
- `utoipa` - OpenAPI generation
- `utoipa-swagger-ui` - Swagger UI integration
- `utoipa-rapidoc` - Alternative API docs UI (optional)

#### 2.2 - API Response Models
Create response types with OpenAPI schemas:
```rust
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub components: ComponentHealth,
}
```

#### 2.3 - OpenAPI Annotations
Add `#[utoipa::path]` annotations to health handlers:
```rust
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn health(...) -> Json<HealthResponse> { ... }
```

#### 2.4 - OpenAPI Router Configuration
Create OpenAPI spec builder:
```rust
pub fn build_openapi() -> OpenApi {
    OpenApiBuilder::new()
        .info(Info::new("SlowPokeAPI", "1.0.0"))
        .paths(...)
        .build()
}
```

#### 2.5 - Swagger UI Endpoint
Mount Swagger UI at `/swagger-ui/`:
```rust
Router::new()
    .merge(SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi()))
```

#### 2.6 - OpenAPI Spec JSON Endpoint
Serve raw OpenAPI JSON at `/api-docs/openapi.json`

#### 2.7 - Tests
- Test that `/swagger-ui/` returns HTML
- Test that `/api-docs/openapi.json` returns valid JSON
- Verify health endpoints are documented

### Deliverables

- `/swagger-ui/` - Interactive API documentation
- `/api-docs/openapi.json` - Raw OpenAPI 3.0.3 schema
- Health endpoints fully documented

### Acceptance Criteria

- [ ] Swagger UI loads at `/swagger-ui/`
- [ ] OpenAPI JSON available at `/api-docs/openapi.json`
- [ ] All health endpoints documented
- [ ] Tests pass
- [ ] Clippy passes with no warnings
- [ ] Format check passes
- [ ] CI passes

### Verification Commands

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

cargo run &
curl http://localhost:8080/api-docs/openapi.json | jq
open http://localhost:8080/swagger-ui/
```

### After Completion

1. Update PLAN.md - Mark Phase 2 complete
2. Update STATUS.md - Move to Phase 3
3. Update WHAT_WE_DID.md - Document Phase 2
4. Update DO_NEXT.md - Set up Phase 3 tasks
5. Move `tasks/phase2/*.md` to `tasks/done/phase2/`
6. Create feature branch for Phase 3
7. Create PR
8. Ensure CI passes
