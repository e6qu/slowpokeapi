use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Peer {
    pub id: String,
    pub address: SocketAddr,
    pub last_seen: Instant,
    pub is_healthy: bool,
}

impl Peer {
    pub fn new(id: String, address: SocketAddr) -> Self {
        Self {
            id,
            address,
            last_seen: Instant::now(),
            is_healthy: true,
        }
    }

    pub fn is_stale(&self, timeout: Duration) -> bool {
        self.last_seen.elapsed() > timeout
    }
}

pub struct PeerManager {
    peers: Arc<RwLock<HashMap<String, Peer>>>,
    peer_timeout: Duration,
}

impl PeerManager {
    pub fn new(peer_timeout: Duration) -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            peer_timeout,
        }
    }

    pub async fn add_peer(&self, peer: Peer) {
        let mut peers = self.peers.write().await;
        peers.insert(peer.id.clone(), peer);
    }

    pub async fn remove_peer(&self, peer_id: &str) {
        let mut peers = self.peers.write().await;
        peers.remove(peer_id);
    }

    pub async fn get_all_peers(&self) -> Vec<Peer> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }

    pub async fn get_healthy_peers(&self) -> Vec<Peer> {
        let peers = self.peers.read().await;
        peers
            .values()
            .filter(|p| p.is_healthy && !p.is_stale(self.peer_timeout))
            .cloned()
            .collect()
    }

    pub async fn count(&self) -> usize {
        let peers = self.peers.read().await;
        peers.len()
    }
}

impl Default for PeerManager {
    fn default() -> Self {
        Self::new(Duration::from_secs(60))
    }
}
