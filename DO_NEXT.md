# Do Next

## Phase 6: Cache Layer

### Goal

Implement in-memory and SQLite caching with two-tier architecture.

### Tasks

| # | Task | Files | Status |
|---|------|-------|--------|
| 6.1 | Add moka cache dependency | `Cargo.toml` | Pending |
| 6.2 | Create cache module | `src/cache/mod.rs` | Pending |
| 6.3 | Define Cache trait | `src/cache/mod.rs` | Pending |
| 6.4 | Implement memory cache | `src/cache/memory.rs` | Pending |
| 6.5 | Implement SQLite cache | `src/cache/sqlite.rs` | Pending |
| 6.6 | Implement tiered cache | `src/cache/tiered.rs` | Pending |
| 6.7 | Add cache metrics | `src/cache/metrics.rs` | Pending |
| 6.8 | Integrate with AppState | `src/server/state.rs` | Pending |
| 6.9 | Test cache operations | `tests/cache.rs` | Pending |

### Task Details

#### 6.1 - Add moka Cache Dependency
Add to `Cargo.toml`:
```toml
moka = { version = "0.12", features = ["sync"] }
```

#### 6.2 - Create Cache Module
Create `src/cache/mod.rs` with module structure.

#### 6.3 - Define Cache Trait
```rust
#[async_trait]
pub trait Cache<K, V>: Send + Sync {
    async fn get(&self, key: &K) -> Result<Option<V>>;
    async fn set(&self, key: K, value: V, ttl: Option<Duration>) -> Result<()>;
    async fn delete(&self, key: &K) -> Result<()>;
    async fn clear(&self) -> Result<()>;
}
```

#### 6.4 - Implement Memory Cache
Create `src/cache/memory.rs`:
- Use moka for in-memory caching
- TTL support
- Size-based eviction

#### 6.5 - Implement SQLite Cache
Create `src/cache/sqlite.rs`:
- Use existing SQLite connection pool
- TTL via timestamp column
- Periodic cleanup of expired entries

#### 6.6 - Implement Tiered Cache
Create `src/cache/tiered.rs`:
- L1: Memory cache (fast, limited)
- L2: SQLite cache (slower, persistent)
- Read-through, write-through

#### 6.7 - Add Cache Metrics
Create `src/cache/metrics.rs`:
- Cache hits/misses
- Cache evictions
- Cache size
- Average latency

#### 6.8 - Integrate with AppState
Update `src/server/state.rs`:
- Add cache to AppState
- Configure from settings

#### 6.9 - Test Cache Operations
Create `tests/cache.rs`:
- Test memory cache
- Test SQLite cache
- Test tiered cache
- Test TTL expiration
- Test cache eviction

### Deliverables

- Two-tier caching (memory → SQLite)
- TTL management
- Cache metrics

### Acceptance Criteria

- [ ] Cache trait defined
- [ ] Memory cache implemented with moka
- [ ] SQLite cache implemented
- [ ] Tiered cache coordinates both
- [ ] Cache metrics exposed
- [ ] Tests pass
- [ ] Clippy passes with no warnings
- [ ] Format check passes
- [ ] CI passes

### Verification Commands

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

# Run and verify metrics include cache stats
cargo run &
curl http://localhost:8080/metrics | grep slowpokeapi_cache
```

### After Completion

1. Update PLAN.md - Mark Phase 6 complete
2. Update STATUS.md - Move to Phase 7
3. Update WHAT_WE_DID.md - Document Phase 6
4. Update DO_NEXT.md - Set up Phase 7 tasks
5. Move `tasks/phase6/*.md` to `tasks/done/phase6/`
6. Create feature branch for Phase 7
7. Create PR
8. Ensure CI passes
