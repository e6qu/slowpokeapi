# Do Next
## Phase 15: Sync Integration
### Goal

Integrate sync engine with cache and storage layers.

### Tasks

| #  | Task | Files | Status |
|---|------|-------|--------|
| 15.1 | Hook cache updates to sync | `src/cache/tiered.rs` | Pending |
| 15.2 | Hook sync updates to cache | `src/sync/integration.rs` | Pending |
| 15.3 | Implement reconciliation | `src/sync/reconciliation.rs` | Pending |
| 15.4 | Add sync configuration | `src/config/settings.rs` | Pending |
| 15.5 | Update health check for sync | `src/handlers/health.rs` | Pending |
| 15.6 | Test full sync flow | `tests/integration.rs` | Pending |

### Deliverables

- Automatic sync on rate updates
- Reconciliation between SQLite and CRDT

### Acceptance Criteria
- [ ] Cache updates trigger sync
- [ ] Sync updates update cache
- [ ] Reconciliation implemented
- [ ] Sync configuration added
- [ ] Health check includes sync
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
1. Update PLAN.md - Mark Phase 15 complete
2. Update STATUS.md - Move to Phase 16
3. Update WHAT_WE_DID.md - document Phase 15
4. Update DO_NEXT.md - set up Phase 16 tasks
5. Create feature branch for Phase 16
6. Create PR
7. Ensure CI passes
