# Architecture Overview

## High-Level Architecture

```
                                    ┌─────────────────────────────────────────┐
                                    │           Load Balancer                 │
                                    │         (ALB / kube-proxy)              │
                                    └────────────────┬────────────────────────┘
                                                     │
                    ┌────────────────────────────────┼────────────────────────────────┐
                    │                                │                                │
                    ▼                                ▼                                ▼
           ┌────────────────┐              ┌────────────────┐              ┌────────────────┐
           │   Replica 1    │              │   Replica 2    │              │   Replica N    │
           │                │              │                │              │                │
           │  ┌──────────┐  │              │  ┌──────────┐  │              │  ┌──────────┐  │
           │  │  Axum    │  │              │  │  Axum    │  │              │  │  Axum    │  │
           │  │  Server  │  │              │  │  Server  │  │              │  │  Server  │  │
           │  └────┬─────┘  │              │  └────┬─────┘  │              │  └────┬─────┘  │
           │       │        │              │       │        │              │       │        │
           │  ┌────▼─────┐  │              │  ┌────▼─────┐  │              │  ┌────▼─────┐  │
           │  │  Cache   │  │              │  │  Cache   │  │              │  │  Cache   │  │
           │  │  Layer   │  │              │  │  Layer   │  │              │  │  Layer   │  │
           │  └────┬─────┘  │              │  └────┬─────┘  │              │  └────┬─────┘  │
           │       │        │              │       │        │              │       │        │
           │  ┌────▼─────┐  │              │  ┌────▼─────┐  │              │  ┌────▼─────┐  │
           │  │  Sync    │◄─┼──────────────┼─►│  Sync    │◄─┼──────────────┼─►│  Sync    │  │
           │  │  Engine  │  │   CRDT        │  │  Engine  │  │   CRDT        │  │  Engine  │  │
           │  └────┬─────┘  │   Gossip      │  └────┬─────┘  │   Gossip      │  └────┬─────┘  │
           │       │        │               │       │        │               │       │        │
           │  ┌────▼─────┐  │               │  ┌────▼─────┐  │               │  ┌────▼─────┐  │
           │  │  SQLite  │  │               │  │  SQLite  │  │               │  │  SQLite  │  │
           │  │ (Local)  │  │               │  │ (Local)  │  │               │  │ (Local)  │  │
           │  └──────────┘  │               │  └──────────┘  │               │  └──────────┘  │
           └───────┬────────┘               └───────┬────────┘               └───────┬────────┘
                   │                                │                                │
                   │         Background Sync via CRDT (Eventual Consistency)        │
                   └────────────────────────────────────────────────────────────────┘
```

## Request Flow

```
1. Client Request
   │
   ▼
2. Load Balancer → Route to healthy replica
   │
   ▼
3. Axum Handler
   │
   ├─► Auth Middleware (optional API key validation)
   │
   ├─► Rate Limit Check (in-memory or Redis)
   │
   ▼
4. Cache Layer
   │
   ├─► Cache HIT → Return cached data
   │
   └─► Cache MISS → Continue
       │
       ▼
5. Upstream Fetcher
   │
   ├─► Try Frankfurter API
   │   └─► Success → Cache & return
   │   └─► Fail → Try fallback
   │
   ├─► Try fawazahmed0 API
   │   └─► Success → Cache & return
   │   └─► Fail → Return cached stale or error
   │
   ▼
6. Response
   │
   ├─► Update local SQLite
   │
   ├─► Sync Engine broadcasts change via CRDT
   │
   └─► Return response to client
```

## Component Layers

### Layer 1: HTTP Server (Axum)
- Request routing
- Middleware chain (auth, logging, CORS)
- OpenAPI spec generation
- Swagger UI serving

### Layer 2: API Handlers
- Business logic for each endpoint
- Input validation
- Response formatting
- Error handling

### Layer 3: Cache Layer
- In-memory LRU cache (moka)
- SQLite persistent cache
- TTL management
- Cache invalidation

### Layer 4: Upstream Fetcher
- HTTP client pool
- Source selection (primary/fallback)
- Response parsing
- Rate limit handling

### Layer 5: Sync Engine
- CRDT state management (automerge)
- Gossip protocol for peer discovery
- Change broadcast
- Conflict resolution

### Layer 6: Storage
- SQLite for persistence
- Connection pooling
- Migration management

## Deployment Modes

### Mode 1: Single Replica (Development)
```
┌─────────────────┐
│   Single Pod    │
│                 │
│  Axum + SQLite  │
│                 │
└─────────────────┘
```
- No sync needed
- Simple deployment
- Good for dev/testing

### Mode 2: Multiple Replicas (Production)
```
┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│   Replica   │  │   Replica   │  │   Replica   │
│             │  │             │  │             │
│ Axum+SQLite │◄─┼─►Axum+SQLite│◄─┼─►Axum+SQLite│
│             │  │             │  │             │
└─────────────┘  └─────────────┘  └─────────────┘
      │                │                │
      └────────────────┴────────────────┘
                 CRDT Sync
```
- Multiple replicas
- Gossip-based sync
- Eventual consistency

### Mode 3: StatefulSet with PVC (Persistent Storage)
```
┌─────────────────┐
│   StatefulSet   │
│                 │
│  ┌───────────┐  │
│  │  Replica  │  │
│  └─────┬─────┘  │
│        │        │
│  ┌─────▼─────┐  │
│  │    PVC    │  │
│  │ (EBS/EFS) │  │
│  └───────────┘  │
└─────────────────┘
```
- Persistent volume per replica
- Data survives pod restart
- Optional for production

## Consistency Model

### Eventual Consistency Guarantees

1. **Read Your Writes**: A replica always sees its own writes immediately
2. **Monotonic Reads**: A client sees monotonically increasing state
3. **Causal Consistency**: Related updates are ordered correctly
4. **Conflict Resolution**: Last-Writer-Wins (LWW) with timestamp

### Sync Latency

- Target: < 5 seconds for 95% of updates
- Worst case: < 30 seconds for all replicas
- Gossip interval: 1-5 seconds (configurable)

## Failure Modes

### Upstream API Failure
```
Frankfurter DOWN → Fallback to fawazahmed0
All upstreams DOWN → Return cached data with stale warning
No cache available → Return 503 Service Unavailable
```

### Replica Failure
```
Replica DOWN → Load balancer removes from pool
New replica joins → Syncs state from peers via CRDT
Network partition → Partitions operate independently, merge on heal
```

### SQLite Corruption
```
Corruption detected → Rebuild from peer sync
No peers available → Rebuild from upstream
```
