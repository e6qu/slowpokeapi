-- Rates table for storing exchange rates
CREATE TABLE IF NOT EXISTS rates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    base_currency TEXT NOT NULL,
    target_currency TEXT NOT NULL,
    rate REAL NOT NULL,
    timestamp INTEGER NOT NULL,
    source TEXT NOT NULL DEFAULT 'frankfurter',
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    UNIQUE(base_currency, target_currency, timestamp)
);

CREATE INDEX IF NOT EXISTS idx_rates_base ON rates(base_currency);
CREATE INDEX IF NOT EXISTS idx_rates_timestamp ON rates(timestamp);
CREATE INDEX IF NOT EXISTS idx_rates_base_target ON rates(base_currency, target_currency);
