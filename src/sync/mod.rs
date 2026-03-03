pub mod crdt;
pub mod integration;
pub mod metrics;
pub mod peer;
pub mod reconciliation;

use thiserror::Error;

pub use crdt::CrdtDocument;
pub use integration::SyncIntegration;
pub use metrics::SYNC_METRICS;
pub use peer::PeerManager;
pub use reconciliation::Reconciler;

pub type SyncResult<T> = Result<T, SyncError>;

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("CRDT error: {0}")]
    CrdtError(String),
    #[error("Transport error: {0}")]
    TransportError(String),
    #[error("Persistence error: {0}")]
    PersistenceError(String),
    #[error("Invalid document: {0}")]
    InvalidDocument(String),
}

#[derive(Debug, Clone)]
pub struct SyncConfig {
    pub peer_id: String,
    pub sync_interval_ms: u64,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            peer_id: uuid::Uuid::new_v4().to_string(),
            sync_interval_ms: 5000,
        }
    }
}
