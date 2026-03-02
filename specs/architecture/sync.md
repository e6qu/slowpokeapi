# CRDT Synchronization Specification

## Overview

SlowPokeAPI uses **automerge-rs** for CRDT-based synchronization between SQLite replicas. This provides eventual consistency without requiring a central coordinator.

## Why CRDT?

| Requirement | CRDT Solution |
|-------------|---------------|
| Multiple replicas | Each replica maintains local state |
| No central coordinator | Fully distributed |
| Eventual consistency | Convergence guarantees |
| Network partitions | Partitions heal automatically |
| Conflict resolution | Mathematically correct merges |

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

## Data Model

### Exchange Rates in CRDT

```rust
struct RateEntry {
    base_code: String,
    target_code: String,
    rate: f64,
    timestamp: i64,          // Unix timestamp (milliseconds)
    source: String,          // "frankfurter" | "fawaz" | "coingecko"
}

struct SyncState {
    rates: HashMap<String, RateEntry>,  // Key: "{base}_{target}"
    metadata: HashMap<String, String>,  // Last sync time, version, etc.
    peers: HashSet<String>,              // Known peer addresses
}
```

### Automerge Document Structure

```javascript
{
  rates: {
    "USD_EUR": {
      rate: 0.92,
      timestamp: 1709337600000,
      source: "frankfurter"
    },
    "EUR_GBP": {
      rate: 0.86,
      timestamp: 1709337600000,
      source: "frankfurter"
    }
  },
  metadata: {
    lastUpdate: 1709337600000,
    version: 42
  }
}
```

## Conflict Resolution

### Last-Writer-Wins (LWW)

For exchange rates, we use LWW based on timestamp:

```
Replica A: rate = 0.92, timestamp = 1000
Replica B: rate = 0.93, timestamp = 1001

Merged: rate = 0.93 (higher timestamp wins)
```

### Automerge Semantics

Automerge provides:
- **Counters**: Increment-only (for metrics)
- **Registers**: LWW for single values
- **Maps**: CRDT-based key-value stores
- **Lists**: Sequence CRDTs for ordered data

For our use case, we use **Maps with LWW registers** for rates.

## Gossip Protocol

### Peer Discovery

```rust
enum PeerDiscovery {
    DNS(String),           // DNS A record lookup: "slowpokeapi-headless"
    Static(Vec<String>),   // Static list: ["10.0.0.1:8080", "10.0.0.2:8080"]
    Kubernetes,            // Kubernetes API (watch endpoints)
}
```

### Gossip Messages

```rust
enum GossipMessage {
    Syn {
        from: PeerId,
        vector_clock: VectorClock,
        digest: StateDigest,
    },
    SynAck {
        from: PeerId,
        missing_changes: Vec<ChangeHash>,
    },
    Ack {
        from: PeerId,
        changes: Vec<Change>,
    },
    Heartbeat {
        from: PeerId,
        timestamp: i64,
    },
}
```

### Gossip Flow

```
Replica A                    Replica B
    │                            │
    │──── Syn (digest) ─────────►│
    │                            │
    │◄─── SynAck (missing) ──────│
    │                            │
    │──── Ack (changes) ────────►│
    │                            │
    │◄─── Ack (ack) ─────────────│
    │                            │
```

### Gossip Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `gossip_interval` | 5s | Time between gossip rounds |
| `fanout` | 3 | Number of peers to gossip with per round |
| `heartbeat_interval` | 10s | Heartbeat frequency |
| `peer_timeout` | 60s | Time before peer considered dead |
| `max_message_size` | 1MB | Maximum gossip message size |

## Sync API

### Internal Sync Service

```rust
pub struct SyncService {
    document: Arc<Mutex<Automerge>>,
    peers: Arc<RwLock<HashSet<Peer>>>,
    config: SyncConfig,
    shutdown: CancellationToken,
}

impl SyncService {
    pub async fn start(&self) -> Result<()>;
    pub async fn stop(&self) -> Result<()>;
    
    pub async fn update_rate(&self, entry: RateEntry) -> Result<()>;
    pub async fn get_rate(&self, base: &str, target: &str) -> Option<RateEntry>;
    pub async fn get_all_rates(&self) -> HashMap<String, RateEntry>;
    
    pub async fn force_sync(&self) -> Result<()>;
    pub async fn get_sync_status(&self) -> SyncStatus;
}
```

### Sync Status

```rust
pub struct SyncStatus {
    pub peer_count: usize,
    pub connected_peers: usize,
    pub last_sync: Option<DateTime<Utc>>,
    pub pending_changes: usize,
    pub document_version: u64,
    pub is_syncing: bool,
}
```

## Integration with SQLite

### Write Path

```
1. API handler receives rate update
2. Write to SQLite (local)
3. Update CRDT document
4. Queue change for broadcast
5. Return response to client
```

### Read Path

```
1. API handler requests rate
2. Try SQLite cache (with TTL check)
3. If stale/missing, query CRDT state
4. If still missing, fetch from upstream
5. Update SQLite and CRDT
6. Return rate
```

### Reconciliation

Periodic reconciliation ensures SQLite and CRDT stay in sync:

```rust
async fn reconcile(db: &SqlitePool, doc: &Automerge) -> Result<()> {
    let db_rates = db.get_all_rates().await?;
    let crdt_rates = doc.get_all_rates();
    
    for (key, rate) in crdt_rates {
        if !db_rates.contains_key(&key) || db_rates[&key].timestamp < rate.timestamp {
            db.upsert_rate(&rate).await?;
        }
    }
    
    Ok(())
}
```

## Transport

### WebSocket Transport (Recommended)

```
ws://peer:8080/sync
```

- Persistent connections
- Low latency
- Built-in heartbeat

### HTTP Transport (Fallback)

```
POST http://peer:8080/sync/message
```

- Stateless
- Works through load balancers
- Higher latency

## Configuration

```toml
[sync]
enabled = true
gossip_interval_secs = 5
fanout = 3
heartbeat_interval_secs = 10
peer_timeout_secs = 60

[sync.discovery]
method = "dns"  # dns | static | kubernetes
dns_name = "slowpokeapi-headless.default.svc.cluster.local"
# OR
static_peers = ["10.0.0.1:8080", "10.0.0.2:8080"]

[sync.transport]
type = "websocket"  # websocket | http
port = 8081
```

## Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `sync_peers_total` | Gauge | Number of known peers |
| `sync_connected_peers` | Gauge | Number of connected peers |
| `sync_changes_sent_total` | Counter | Changes broadcast |
| `sync_changes_received_total` | Counter | Changes received |
| `sync_merge_duration_seconds` | Histogram | Time to merge changes |
| `sync_document_size_bytes` | Gauge | CRDT document size |
| `sync_last_success_timestamp` | Gauge | Last successful sync |

## Edge Cases

### Cold Start (New Replica)

1. New replica starts with empty state
2. Discovers peers via DNS/config
3. Requests full state from random peer
4. Merges received state
5. Begins normal gossip

### Network Partition

1. Partition occurs
2. Replicas in each partition continue independently
3. Partitions heal when network restored
4. CRDT guarantees convergence
5. All replicas reach same state

### Slow Peer

1. Peer falls behind
2. Gossip includes change digests, not full changes
3. Slow peer requests missing changes on-demand
4. Eventually catches up

### Conflicting Updates

```
Replica A: USD_EUR = 0.92 @ t=1000
Replica B: USD_EUR = 0.91 @ t=1001

Conflict detected during merge
Resolution: Use t=1001 value (0.91)
Result converges to 0.91 on all replicas
```

## Dependencies

```toml
[dependencies]
automerge = "0.5"
tokio = { version = "1", features = ["full"] }
tokio-tungstenite = "0.21"
serde = { version = "1", features = ["derive"] }
bincode = "1"
tracing = "0.1"
```
