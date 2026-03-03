# Do Next
## Phase 14: CRDT Sync Engine
### Goal

Implement automerge-based CRDT sync between replicas.

### Tasks

| #  | Task | Files | Status |
|---|------|-------|--------|
| 14.1 | Add automerge dependency | `Cargo.toml` | Pending |
| 14.2 | Create sync module | `src/sync/mod.rs` | Pending |
| 14.3 | Define SyncEngine trait | `src/sync/mod.rs` | Pending |
| 14.4 | Implement CRDT document wrapper | `src/sync/crdt.rs` | Pending |
| 14.5 | Implement gossip protocol | `src/sync/gossip.rs` | Pending |
| 14.6 | Implement peer discovery | `src/sync/peer.rs` | Pending |
| 14.7 | Implement WebSocket transport | `src/sync/transport.rs` | Pending |
| 14.8 | Integrate with storage layer | `src/sync/storage.rs` | Pending |
| 14.9 | Add sync metrics | `src/sync/metrics.rs` | Pending |
| 14.10 | Test sync between replicas | `tests/sync.rs` | Pending |

### Deliverables

- CRDT-based state management
- Gossip protocol for peer sync
- WebSocket transport
- Tests for sync functionality

### Acceptance Criteria
- [ ] Automerge dependency added
- [ ] Sync module created
- [ ] SyncEngine trait defined
- [ ] CRDT document wrapper implemented
- [ ] Gossip protocol implemented
- [ ] Peer discovery implemented
- [ ] WebSocket transport implemented
- [ ] Storage integration complete
- [ ] Sync metrics added
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
1. Update PLAN.md - Mark Phase 14 complete
2. Update STATUS.md - Move to Phase 15
3. Update WHAT_WE_DID.md - document Phase 14
4. Update DO_NEXT.md - set up Phase 15 tasks
5. Create feature branch for Phase 15
6. Create PR
7. Ensure CI passes
