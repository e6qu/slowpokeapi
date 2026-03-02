# Component Design

## Core Components

### 1. HTTP Server (`src/server/`)

```
src/server/
├── mod.rs
├── router.rs          # Route definitions
├── middleware/
│   ├── mod.rs
│   ├── auth.rs        # API key validation
│   ├── logging.rs     # Request/response logging
│   ├── cors.rs        # CORS handling
│   └── ratelimit.rs   # Rate limiting
└── state.rs           # AppState definition
```

**Responsibilities:**
- Configure Axum router with all endpoints
- Apply middleware chain
- Manage shared application state
- Graceful shutdown handling

**Dependencies:**
- `axum`: Web framework
- `tower`: Middleware utilities
- `tower-http`: HTTP middleware

### 2. API Handlers (`src/handlers/`)

```
src/handlers/
├── mod.rs
├── currencies.rs      # GET /v1/currencies
├── latest.rs          # GET /v1/latest/{base}
├── pair.rs            # GET /v1/pair/{base}/{target}/{amount}
├── history.rs         # GET /v1/history/{base}/{year}/{month}/{day}
├── enriched.rs        # GET /v1/enriched/{base}/{target}
├── quota.rs           # GET /v1/quota
├── health.rs          # /healthz, /readyz, /livez, /health
└── metrics.rs         # GET /metrics
```

**Responsibilities:**
- Parse and validate request parameters
- Call appropriate service methods
- Format responses per API spec
- Handle errors consistently

**Dependencies:**
- `serde`: JSON serialization
- `utoipa`: OpenAPI annotations

### 3. Cache Layer (`src/cache/`)

```
src/cache/
├── mod.rs
├── memory.rs          # In-memory LRU cache (moka)
├── sqlite.rs          # SQLite persistent cache
└── tiered.rs          # Tiered cache (memory → SQLite)
```

**Responsibilities:**
- In-memory caching for hot data
- SQLite persistence for durability
- TTL management
- Cache invalidation
- Cache hit/miss metrics

**Dependencies:**
- `moka`: In-memory cache
- `rusqlite` or `sqlx`: SQLite access

### 4. Upstream Fetcher (`src/upstream/`)

```
src/upstream/
├── mod.rs
├── client.rs          # Shared HTTP client
├── frankfurter.rs     # Frankfurter API client
├── fawaz.rs           # fawazahmed0 API client
├── coingecko.rs       # CoinGecko API client
└── coincap.rs         # CoinCap API client
```

**Responsibilities:**
- Make HTTP requests to upstream APIs
- Parse various response formats (JSON, XML)
- Handle rate limits and backoff
- Fallback chain logic
- Circuit breaker pattern

**Dependencies:**
- `reqwest`: HTTP client
- `quick-xml`: XML parsing (ECB)

### 5. Sync Engine (`src/sync/`)

```
src/sync/
├── mod.rs
├── crdt.rs            # Automerge CRDT wrapper
├── gossip.rs          # Gossip protocol implementation
├── peer.rs            # Peer discovery and management
├── broadcast.rs       # Change broadcast
└── merge.rs           # State merge logic
```

**Responsibilities:**
- Manage CRDT document state
- Discover peers via DNS or config
- Broadcast local changes
- Receive and merge remote changes
- Conflict resolution

**Dependencies:**
- `automerge`: CRDT implementation
- `tokio`: Async networking
- `serde`: Serialization

### 6. Storage Layer (`src/storage/`)

```
src/storage/
├── mod.rs
├── sqlite.rs          # SQLite connection pool
├── migrations/        # Database migrations
│   ├── 001_init.sql
│   └── 002_rates.sql
└── models.rs          # Database models
```

**Responsibilities:**
- Connection pool management
- Migration execution
- CRUD operations
- Transaction handling

**Dependencies:**
- `rusqlite` or `sqlx`: SQLite driver
- `include_dir`: Embedded migrations

### 7. Models (`src/models/`)

```
src/models/
├── mod.rs
├── currency.rs        # Currency code and metadata
├── rate.rs            # Exchange rate
├── historical.rs      # Historical rate
├── error.rs           # Error types
└── api/               # API request/response types
    ├── mod.rs
    ├── request.rs
    └── response.rs
```

**Responsibilities:**
- Domain model definitions
- API request/response types
- Validation logic
- OpenAPI schemas

**Dependencies:**
- `serde`: Serialization
- `utoipa`: OpenAPI schemas

### 8. Config (`src/config/`)

```
src/config/
├── mod.rs
└── settings.rs        # Configuration struct
```

**Configuration Sources:**
- Environment variables (highest priority)
- `.env` file
- Default values

**Key Settings:**
```rust
struct Settings {
    server: ServerConfig,
    cache: CacheConfig,
    upstream: UpstreamConfig,
    sync: SyncConfig,
    storage: StorageConfig,
}
```

### 9. Metrics (`src/metrics/`)

```
src/metrics/
├── mod.rs
├── definitions.rs     # Prometheus metric definitions
└── exporter.rs        # Metrics endpoint handler
```

**Metrics Exposed:**
- Request count (by endpoint, status)
- Request latency histogram
- Cache hit/miss ratio
- Upstream API latency
- Sync events count
- Active connections

**Dependencies:**
- `prometheus`: Metrics library
- `axum-prometheus`: Axum integration

### 10. Health (`src/health/`)

```
src/health/
├── mod.rs
├── checks.rs          # Individual health checks
└── reporter.rs        # Health report aggregation
```

**Health Checks:**
- SQLite connectivity
- Upstream API availability
- Sync engine status
- Memory usage
- Disk usage (if persistent storage)

## Dependency Graph

```
┌─────────────────────────────────────────────────────┐
│                    main.rs                          │
└─────────────────────┬───────────────────────────────┘
                      │
        ┌─────────────┼─────────────┐
        │             │             │
        ▼             ▼             ▼
   ┌─────────┐  ┌──────────┐  ┌─────────┐
   │ config  │  │  server  │  │ metrics │
   └─────────┘  └────┬─────┘  └─────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
   ┌─────────┐ ┌──────────┐ ┌─────────┐
   │handlers │ │  health  │ │  sync   │
   └────┬────┘ └──────────┘ └────┬────┘
        │                       │
        ▼                       ▼
   ┌─────────┐             ┌─────────┐
   │  cache  │◄────────────│  crdt   │
   └────┬────┘             └─────────┘
        │
        ▼
   ┌─────────┐
   │ storage │
   └────┬────┘
        │
        ▼
   ┌─────────┐
   │upstream │
   └─────────┘
```

## Interface Contracts

### Cache Trait

```rust
#[async_trait]
pub trait Cache: Send + Sync {
    async fn get_rate(&self, base: &str, target: &str) -> Result<Option<Rate>>;
    async fn set_rate(&self, rate: &Rate, ttl: Duration) -> Result<()>;
    async fn get_rates(&self, base: &str) -> Result<Option<HashMap<String, f64>>>;
    async fn set_rates(&self, base: &str, rates: &HashMap<String, f64>, ttl: Duration) -> Result<()>;
    async fn invalidate(&self, key: &str) -> Result<()>;
}
```

### Upstream Trait

```rust
#[async_trait]
pub trait Upstream: Send + Sync {
    async fn fetch_latest(&self, base: &str) -> Result<HashMap<String, f64>>;
    async fn fetch_historical(&self, base: &str, date: NaiveDate) -> Result<HashMap<String, f64>>;
    async fn fetch_currencies(&self) -> Result<HashMap<String, String>>;
    fn name(&self) -> &str;
    fn is_healthy(&self) -> bool;
}
```

### Sync Trait

```rust
#[async_trait]
pub trait SyncEngine: Send + Sync {
    async fn broadcast_change(&self, change: Change) -> Result<()>;
    async fn get_state(&self) -> Result<Vec<u8>>;
    async fn merge_state(&self, state: &[u8]) -> Result<()>;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
}
```

## Error Handling Strategy

All errors flow through a central error type:

```rust
pub enum Error {
    NotFound(String),
    ValidationError(String),
    UpstreamError { source: String, message: String },
    CacheError(String),
    StorageError(String),
    SyncError(String),
    Internal(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // Convert to JSON error response
    }
}
```
