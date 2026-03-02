-- Sync state table for CRDT synchronization tracking
CREATE TABLE IF NOT EXISTS sync_state (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    node_id TEXT NOT NULL,
    last_sync INTEGER NOT NULL DEFAULT 0,
    sync_count INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Insert default sync state
INSERT INTO sync_state (id, node_id, last_sync, sync_count)
VALUES (1, 'default', 0, 0);
