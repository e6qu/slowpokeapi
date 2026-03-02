# SQLite Storage Specification

## Overview

SQLite is used for persistent caching of exchange rates and configuration data. Each replica maintains its own SQLite database, synchronized via CRDT.

## Database Schema

### Tables

#### `rates` - Exchange Rates

```sql
CREATE TABLE IF NOT EXISTS rates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    base_code TEXT NOT NULL,
    target_code TEXT NOT NULL,
    rate REAL NOT NULL,
    source TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    UNIQUE(base_code, target_code)
);

CREATE INDEX idx_rates_base ON rates(base_code);
CREATE INDEX idx_rates_timestamp ON rates(timestamp);
CREATE INDEX idx_rates_source ON rates(source);
```

#### `historical_rates` - Historical Exchange Rates

```sql
CREATE TABLE IF NOT EXISTS historical_rates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    base_code TEXT NOT NULL,
    date TEXT NOT NULL,
    rates_json TEXT NOT NULL,
    source TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    UNIQUE(base_code, date)
);

CREATE INDEX idx_historical_base_date ON historical_rates(base_code, date);
```

#### `currencies` - Currency Metadata

```sql
CREATE TABLE IF NOT EXISTS currencies (
    code TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    symbol TEXT,
    currency_type TEXT NOT NULL DEFAULT 'fiat',
    locale TEXT,
    country_code TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);
```

#### `sync_state` - CRDT Sync State

```sql
CREATE TABLE IF NOT EXISTS sync_state (
    key TEXT PRIMARY KEY,
    value BLOB NOT NULL,
    version INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Stores:
-- 'crdt_document': The serialized automerge document
-- 'peer_state': Known peers and their states
-- 'last_sync': Timestamp of last successful sync
```

#### `api_keys` - API Key Management (Optional)

```sql
CREATE TABLE IF NOT EXISTS api_keys (
    key TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    plan TEXT NOT NULL DEFAULT 'free',
    quota_limit INTEGER NOT NULL DEFAULT 1500,
    quota_used INTEGER NOT NULL DEFAULT 0,
    quota_reset_date TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    last_used_at INTEGER
);

CREATE INDEX idx_api_keys_reset ON api_keys(quota_reset_date);
```

#### `metadata` - General Key-Value Storage

```sql
CREATE TABLE IF NOT EXISTS metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Stores:
-- 'schema_version': Current migration version
-- 'last_upstream_fetch': Timestamp of last upstream API fetch
-- 'bootstrap_complete': Whether initial data load is done
```

## Migrations

### Migration File Structure

```
src/storage/migrations/
├── 001_initial.sql
├── 002_rates.sql
├── 003_historical.sql
├── 004_sync_state.sql
└── 005_api_keys.sql
```

### 001_initial.sql

```sql
-- Initial schema setup
CREATE TABLE IF NOT EXISTS metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

INSERT INTO metadata (key, value) VALUES ('schema_version', '1');
```

### 002_rates.sql

```sql
-- Rates table
CREATE TABLE IF NOT EXISTS rates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    base_code TEXT NOT NULL,
    target_code TEXT NOT NULL,
    rate REAL NOT NULL,
    source TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    UNIQUE(base_code, target_code)
);

CREATE INDEX idx_rates_base ON rates(base_code);
CREATE INDEX idx_rates_timestamp ON rates(timestamp);

UPDATE metadata SET value = '2' WHERE key = 'schema_version';
```

## Connection Pool

```rust
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub async fn create_pool(db_path: &str) -> Result<SqlitePool> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite:{}?mode=rwc", db_path))
        .await
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
}
```

## Repository Pattern

### Rates Repository

```rust
#[async_trait]
pub trait RatesRepository: Send + Sync {
    async fn get_rate(&self, base: &str, target: &str) -> Result<Option<ExchangeRate>>;
    async fn get_all_rates(&self, base: &str) -> Result<Option<RateCollection>>;
    async fn upsert_rate(&self, rate: &ExchangeRate) -> Result<()>;
    async fn upsert_rates(&self, collection: &RateCollection) -> Result<()>;
    async fn delete_stale(&self, older_than: Duration) -> Result<usize>;
}

pub struct SqliteRatesRepository {
    pool: SqlitePool,
}

#[async_trait]
impl RatesRepository for SqliteRatesRepository {
    async fn get_rate(&self, base: &str, target: &str) -> Result<Option<ExchangeRate>> {
        let row = sqlx::query_as!(
            RateRow,
            r#"SELECT base_code, target_code, rate, source, timestamp 
               FROM rates WHERE base_code = ? AND target_code = ?"#,
            base, target
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row.map(|r| ExchangeRate {
            base_code: r.base_code,
            target_code: r.target_code,
            rate: r.rate,
            source: r.source.parse()?,
            timestamp: DateTime::from_timestamp(r.timestamp, 0).unwrap(),
        }))
    }
    
    async fn upsert_rate(&self, rate: &ExchangeRate) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO rates (base_code, target_code, rate, source, timestamp, updated_at)
               VALUES (?, ?, ?, ?, ?, strftime('%s', 'now'))
               ON CONFLICT(base_code, target_code) 
               DO UPDATE SET rate = excluded.rate, source = excluded.source, 
                             timestamp = excluded.timestamp, updated_at = strftime('%s', 'now')"#,
            rate.base_code, rate.target_code, rate.rate, 
            rate.source.to_string(), rate.timestamp.timestamp()
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    // ... other implementations
}
```

### Historical Rates Repository

```rust
#[async_trait]
pub trait HistoricalRepository: Send + Sync {
    async fn get_rates(&self, base: &str, date: NaiveDate) -> Result<Option<HistoricalRate>>;
    async fn upsert_rates(&self, rates: &HistoricalRate) -> Result<()>;
    async fn get_available_dates(&self, base: &str) -> Result<Vec<NaiveDate>>;
}
```

### Sync State Repository

```rust
#[async_trait]
pub trait SyncStateRepository: Send + Sync {
    async fn get_crdt_document(&self) -> Result<Option<Vec<u8>>>;
    async fn set_crdt_document(&self, data: &[u8], version: u64) -> Result<()>;
    async fn get_last_sync(&self) -> Result<Option<DateTime<Utc>>>;
    async fn set_last_sync(&self, time: DateTime<Utc>) -> Result<()>;
}
```

## Query Patterns

### Get Latest Rates for Base Currency

```sql
SELECT base_code, target_code, rate, source, timestamp
FROM rates
WHERE base_code = ?
ORDER BY timestamp DESC;
```

### Get Rates Updated After Timestamp

```sql
SELECT base_code, target_code, rate, source, timestamp
FROM rates
WHERE updated_at > ?;
```

### Clean Up Stale Rates

```sql
DELETE FROM rates 
WHERE timestamp < strftime('%s', 'now', '-7 days');
```

### Get Historical Rates for Date Range

```sql
SELECT date, rates_json, source
FROM historical_rates
WHERE base_code = ? AND date BETWEEN ? AND ?
ORDER BY date ASC;
```

## Performance Considerations

### Connection Settings

```rust
// Enable WAL mode for better concurrent access
sqlx::query("PRAGMA journal_mode = WAL")
    .execute(&pool)
    .await?;

// Set busy timeout for concurrent access
sqlx::query("PRAGMA busy_timeout = 5000")
    .execute(&pool)
    .await?;

// Enable foreign keys
sqlx::query("PRAGMA foreign_keys = ON")
    .execute(&pool)
    .await?;
```

### Indexing Strategy

- Primary indexes on all foreign key relationships
- Composite indexes on common query patterns (base_code + date)
- Partial indexes for frequently filtered data

### Vacuum and Optimization

```sql
-- Run periodically (e.g., weekly)
VACUUM;
ANALYZE;
```

## Backup Strategy

### Automated Backups

```rust
pub async fn backup_database(db_path: &str, backup_dir: &str) -> Result<()> {
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let backup_path = format!("{}/slowpokeapi_{}.db", backup_dir, timestamp);
    
    // Use SQLite backup API
    let source = SqliteConnection::connect(&format!("sqlite:{}", db_path)).await?;
    let dest = SqliteConnection::connect(&format!("sqlite:{}", backup_path)).await?;
    
    sqlx::query("VACUUM INTO ?")
        .bind(&backup_path)
        .execute(&source)
        .await?;
    
    Ok(())
}
```

### Restore from Backup

```bash
# Stop service
systemctl stop slowpokeapi

# Restore database
cp /backups/slowpokeapi_20260302_120000.db /data/slowpokeapi.db

# Start service
systemctl start slowpokeapi
```

## File Locations

| Environment | Path |
|------------|------|
| Development | `./data/slowpokeapi.db` |
| Docker | `/data/slowpokeapi.db` |
| Kubernetes (emptyDir) | `/tmp/slowpokeapi.db` |
| Kubernetes (PVC) | `/data/slowpokeapi.db` |
| AWS ECS (EFS) | `/mnt/efs/slowpokeapi.db` |

## Configuration

```toml
[storage]
# Database file path
path = "./data/slowpokeapi.db"

# Connection pool size
pool_size = 5

# Enable WAL mode
wal_mode = true

# Busy timeout (ms)
busy_timeout_ms = 5000

# Automatic cleanup of stale data (hours)
cleanup_interval_hours = 24

# Backup settings
backup_enabled = true
backup_interval_hours = 24
backup_dir = "./backups"
backup_retention_days = 7
```
