# CRDT Implementation Plan

## Overview

Implement a robust CRDT (Conflict-free Replicated Data Type) synchronization system for SlowPokeAPI using automerge-rs. This enables distributed SQLite replicas to achieve eventual consistency without a central coordinator.

## Goals

1. Replace basic bincode-based CRDT with proper Automerge-based CRDT
2. Implement gossip protocol for peer-to-peer synchronization
3. Add peer discovery (DNS, static configuration)
4. Implement sync service with WebSocket transport
5. Integrate with existing SQLite storage layer
6. Add comprehensive metrics and monitoring
7. Ensure proper conflict resolution (Last-Writer-Wins)

## Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                       Sync Engine                                 │
│                                                                   │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────────┐  │
│  │   Local     │    │   CRDT      │    │     Gossip          │  │
│  │   State     │◄──►│   Document  │◄──►│     Protocol        │  │
│  │  (SQLite)   │    │ (Automerge) │    │   (Peer-to-Peer)    │  │
│  └─────────────┘    └─────────────┘    └─────────────────────┘  │
│         ▲                  ▲                      ▲              │
│         │                  │                      │              │
│         ▼                  ▼                      ▼              │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────────┐  │
│  │  Read/Write │    │   Change    │    │    Peer Discovery   │  │
│  │   Hooks     │    │   Queue     │    │    (DNS/Config)     │  │
│  └─────────────┘    └─────────────┘    └─────────────────────┘  │
│                                                                   │
└──────────────────────────────────────────────────────────────────┘
```

## Implementation Tasks

### Task 1: Update Dependencies
**Status:** [x] Done - Dependencies already present in Cargo.toml

Add automerge and related dependencies to Cargo.toml.

**Files:**
- `Cargo.toml`

**Dependencies to add:**
- `automerge = "0.5"`
- `tokio-tungstenite = "0.21"` (for WebSocket transport)

---

### Task 2: Refactor CRDT Document
**Status:** [x] Done - Implemented Automerge-based CRDT with LWW semantics

Replace the basic bincode-based CrdtDocument with a proper Automerge-based implementation supporting LWW (Last-Writer-Wins) semantics.

**Files:**
- `src/sync/crdt.rs`

**Requirements:**
- Use Automerge for proper CRDT operations
- Support LWW conflict resolution based on timestamp
- Handle exchange rates as a map with metadata
- Serialize/deserialize Automerge documents

---

### Task 3: Implement Gossip Protocol
**Status:** [x] Done - Implemented gossip messages, state digest, and vector clocks

Implement the gossip protocol messages and state machine for peer-to-peer synchronization.

**Files:**
- `src/sync/gossip.rs` (new)
- `src/sync/mod.rs` (update)

**Requirements:**
- Define GossipMessage enum (Syn, SynAck, Ack, Heartbeat)
- Implement state digest for efficient change detection
- Handle message serialization

---

### Task 4: Implement Peer Discovery
**Status:** [x] Done - Implemented DNS and static peer discovery

Implement peer discovery mechanisms (DNS and static configuration).

**Files:**
- `src/sync/discovery.rs` (new)
- `src/sync/mod.rs` (update)

**Requirements:**
- Support DNS-based discovery (headless service lookup)
- Support static peer list configuration
- Implement peer health checking

---

### Task 5: Implement Sync Service
**Status:** [x] Done - Implemented sync service with gossip, discovery, and WebSocket transport

Implement the main sync service that coordinates CRDT operations, gossip protocol, and peer management.

**Files:**
- `src/sync/service.rs` (new)
- `src/sync/mod.rs` (update)

**Requirements:**
- Manage Automerge document lifecycle
- Coordinate gossip rounds
- Handle peer connections
- Expose sync status API

---

### Task 6: Implement WebSocket Transport
**Status:** [x] Done - WebSocket transport integrated into sync service

Implement WebSocket transport for peer-to-peer communication.

**Files:**
- `src/sync/transport.rs` (new)
- `src/sync/mod.rs` (update)

**Requirements:**
- WebSocket server for accepting connections
- WebSocket client for initiating connections
- Message framing and error handling

---

### Task 7: Update Configuration
**Status:** [x] Done - Added DiscoveryConfig and updated SyncConfig

Add sync configuration options to the app configuration.

**Files:**
- `src/config.rs`
- `src/sync/mod.rs`

**Requirements:**
- Sync enable/disable flag
- Gossip interval, fanout, heartbeat settings
- Peer discovery configuration
- Transport configuration

---

### Task 8: Integrate with SQLite
**Status:** [x] Done - Integration module updated for new CRDT

Update SQLite integration to work with the new CRDT system.

**Files:**
- `src/sync/integration.rs` (update)
- `src/sync/reconciliation.rs` (update)

**Requirements:**
- Write path: SQLite → CRDT → broadcast
- Read path: SQLite (cache) → CRDT → upstream
- Periodic reconciliation

---

### Task 9: Add Metrics
**Status:** [x] Done - Added sync_changes_sent_total, sync_changes_received_total, sync_merge_duration_seconds

Add comprehensive metrics for the sync system.

**Files:**
- `src/sync/metrics.rs` (update)

**Requirements:**
- Peer count metrics
- Changes sent/received counters
- Merge duration histogram
- Document size gauge

---

### Task 10: Write Tests
**Status:** [x] Done - Unit tests for CRDT, gossip, and discovery modules

Write comprehensive tests for the CRDT system.

**Files:**
- `tests/crdt_test.rs` (new)
- `tests/sync_integration_test.rs` (new)

**Requirements:**
- Unit tests for CRDT operations
- Unit tests for gossip protocol
- Integration tests for sync service

---

### Task 11: Update Documentation
**Status:** [x] Done - Updated PLAN_CRDT.md with task progress

Update all documentation to reflect the new CRDT implementation.

**Files:**
- `PLAN_CRDT.md` (this file - mark tasks complete)
- `docs/CRDT.md` (new)

**Requirements:**
- Document the CRDT architecture
- Document configuration options
- Document operational procedures

---

## Verification

After implementation, verify:

```bash
# Format check
cargo fmt --check

# Lint check
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test --all-features

# Build
cargo build --release
```

## Success Criteria

- [ ] All tasks completed and marked as done
- [ ] All tests pass
- [ ] Clippy passes with no warnings
- [ ] Format check passes
- [ ] Can run multiple replicas that sync with each other
- [ ] Conflicts are resolved correctly (LWW)
- [ ] Metrics are exposed for monitoring

## Notes

- This is a significant refactor of the existing basic CRDT implementation
- The existing `CrdtDocument` will be replaced with an Automerge-based implementation
- Backward compatibility with existing SQLite schema should be maintained
- The sync system should be optional (can be disabled via config)
