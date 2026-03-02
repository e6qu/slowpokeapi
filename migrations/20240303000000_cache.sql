-- Cache entries table
CREATE TABLE IF NOT EXISTS cache_entries (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    expires_at INTEGER
);

CREATE INDEX IF NOT EXISTS idx_cache_expires_at ON cache_entries(expires_at);
