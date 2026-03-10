pub mod crdt;
pub mod discovery;
pub mod gossip;
pub mod integration;
pub mod metrics;
pub mod peer;
pub mod reconciliation;
pub mod service;

use std::time::Duration;
use thiserror::Error;

pub use crdt::{CrdtDocument, RateEntry};
pub use discovery::PeerDiscovery;
pub use gossip::{compute_digest, GossipMessage, GossipState, StateDigest};
pub use integration::SyncIntegration;
pub use metrics::SYNC_METRICS;
pub use peer::{Peer, PeerManager};
pub use reconciliation::Reconciler;
pub use service::{SyncService, SyncStatus};

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
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Discovery error: {0}")]
    DiscoveryError(String),
}

#[derive(Debug, Clone)]
pub struct SyncConfig {
    pub peer_id: String,
    pub enabled: bool,
    pub gossip_interval: Duration,
    pub fanout: usize,
    pub heartbeat_interval: Duration,
    pub peer_timeout: Duration,
    pub discovery: DiscoveryConfig,
}

#[derive(Debug, Clone)]
pub enum DiscoveryConfig {
    Dns { dns_name: String },
    Static { peers: Vec<String> },
    Disabled,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            peer_id: uuid::Uuid::new_v4().to_string(),
            enabled: false,
            gossip_interval: Duration::from_secs(5),
            fanout: 3,
            heartbeat_interval: Duration::from_secs(10),
            peer_timeout: Duration::from_secs(60),
            discovery: DiscoveryConfig::Disabled,
        }
    }
}

impl SyncConfig {
    pub fn with_dns_discovery(mut self, dns_name: String) -> Self {
        self.discovery = DiscoveryConfig::Dns { dns_name };
        self
    }

    pub fn with_static_peers(mut self, peers: Vec<String>) -> Self {
        self.discovery = DiscoveryConfig::Static { peers };
        self
    }

    pub fn enabled(mut self) -> Self {
        self.enabled = true;
        self
    }
}
