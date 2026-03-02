# Do Next

## Phase 4: SQLite Storage Layer

### Goal

Implement SQLite database with migrations and repository pattern.

### Tasks

| # | Task | Files | Status |
|---|------|-------|--------|
| 4.1 | Add sqlx and dependencies | `Cargo.toml` | Pending |
| 4.2 | Create migrations directory | `migrations/001_initial.sql` | Pending |
| 4.3 | Create rates table migration | `migrations/002_rates.sql` | Pending |
| 4.4 | Create historical rates migration | `migrations/003_historical.sql` | Pending |
| 4.5 | Create sync state migration | `migrations/004_sync_state.sql` | Pending |
| 4.6 | Implement storage module | `src/storage/mod.rs` | Pending |
| 4.7 | Implement SQLite connection pool | `src/storage/sqlite.rs` | Pending |
| 4.8 | Create rates repository | `src/storage/repositories/rates.rs` | Pending |
| 4.9 | Create historical repository | `src/storage/repositories/historical.rs` | Pending |
| 4.10 | Add database health check | `src/storage/health.rs` | Pending |
| 4.11 | Test storage operations | `tests/storage.rs` | Pending |

### Task Details

#### 4.1 - Add sqlx Dependencies
Add to `Cargo.toml`:
- `sqlx` with sqlite runtime-tokio features
- Enable offline mode for CI

#### 4.2 - Create Migrations Directory
Create `migrations/` directory with initial schema.

#### 4.3 - Create Rates Table Migration
Create `migrations/002_rates.sql`:
```sql
CREATE TABLE rates (
    base_currency TEXT NOT NULL,
    target_currency TEXT NOT NULL,
    rate REAL NOT NULL,
    timestamp INTEGER NOT NULL,
    PRIMARY KEY (base_currency, target_currency)
);
```

#### 4.4 - Create Historical Rates Migration
Create `migrations/003_historical.sql`:
```sql
CREATE TABLE historical_rates (
    date TEXT NOT NULL,
    base_currency TEXT NOT NULL,
    target_currency TEXT NOT NULL,
    rate REAL NOT NULL,
    PRIMARY KEY (date, base_currency, target_currency)
);
```

#### 4.5 - Create Sync State Migration
Create `migrations/004_sync_state.sql`:
```sql
CREATE TABLE sync_state (
    id INTEGER PRIMARY KEY,
    last_sync INTEGER NOT NULL,
    node_id TEXT NOT NULL
);
```

#### 4.6 - Implement Storage Module
Create `src/storage/mod.rs` with repository traits.

#### 4.7 - Implement SQLite Connection Pool
Create `src/storage/sqlite.rs`:
- Connection pool management
- Migration runner

#### 4.8 - Create Rates Repository
Create `src/storage/repositories/rates.rs`:
- CRUD operations for rates

#### 4.9 - Create Historical Repository
Create `src/storage/repositories/historical.rs`:
- CRUD operations for historical rates

#### 4.10 - Add Database Health Check
Update health check to verify database connectivity.

#### 4.11 - Test Storage Operations
Create integration tests for storage layer.

### Deliverables

- SQLite database with migrations
- Repository traits and implementations
- Database health check integration

### Acceptance Criteria

- [ ] Migrations run successfully
- [ ] Repository operations work
- [ ] Health check includes database
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
curl http://localhost:8080/health | jq .checks.database
```

### After Completion

1. Update PLAN.md - Mark Phase 4 complete
2. Update STATUS.md - Move to Phase 5
3. Update WHAT_WE_DID.md - Document Phase 4
4. Update DO_NEXT.md - Set up Phase 5 tasks
5. Move `tasks/phase4/*.md` to `tasks/done/phase4/`
6. Create feature branch for Phase 5
7. Create PR
8. Ensure CI passes
