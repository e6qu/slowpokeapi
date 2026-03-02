-- Historical rates table for storing historical exchange rates
CREATE TABLE IF NOT EXISTS historical_rates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL,
    base_currency TEXT NOT NULL,
    target_currency TEXT NOT NULL,
    rate REAL NOT NULL,
    source TEXT NOT NULL DEFAULT 'frankfurter',
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    UNIQUE(date, base_currency, target_currency)
);

CREATE INDEX IF NOT EXISTS idx_historical_date ON historical_rates(date);
CREATE INDEX IF NOT EXISTS idx_historical_base ON historical_rates(base_currency);
CREATE INDEX IF NOT EXISTS idx_historical_date_base ON historical_rates(date, base_currency);
