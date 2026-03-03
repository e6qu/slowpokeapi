-- Add document_state column to sync_state table for CRDT document storage
ALTER TABLE sync_state ADD COLUMN document_state BLOB;

-- Add peer_list column to sync_state table for peer management
ALTER TABLE sync_state ADD COLUMN peer_list TEXT DEFAULT '[]';
