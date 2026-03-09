use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::sync::SyncError;

pub type PeerId = String;
pub type VectorClock = HashMap<PeerId, u64>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDigest {
    pub checksum: u64,
    pub entry_count: usize,
    pub vector_clock: VectorClock,
}

impl StateDigest {
    pub fn new(checksum: u64, entry_count: usize, vector_clock: VectorClock) -> Self {
        Self {
            checksum,
            entry_count,
            vector_clock,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GossipMessage {
    Syn {
        from: PeerId,
        vector_clock: VectorClock,
        digest: StateDigest,
    },
    SynAck {
        from: PeerId,
        missing_changes: Vec<u8>,
        their_digest: StateDigest,
    },
    Ack {
        from: PeerId,
        changes: Vec<u8>,
    },
    Heartbeat {
        from: PeerId,
        timestamp: i64,
    },
}

impl GossipMessage {
    pub fn to_bytes(&self) -> Result<Vec<u8>, SyncError> {
        bincode::serialize(self)
            .map_err(|e| SyncError::TransportError(format!("Failed to serialize: {e}")))
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, SyncError> {
        bincode::deserialize(data)
            .map_err(|e| SyncError::TransportError(format!("Failed to deserialize: {e}")))
    }

    pub fn peer_id(&self) -> &PeerId {
        match self {
            GossipMessage::Syn { from, .. } => from,
            GossipMessage::SynAck { from, .. } => from,
            GossipMessage::Ack { from, .. } => from,
            GossipMessage::Heartbeat { from, .. } => from,
        }
    }
}

pub struct GossipState {
    pub local_clock: VectorClock,
    pub last_gossip: Instant,
    pub gossip_interval: Duration,
    pub fanout: usize,
}

impl GossipState {
    pub fn new(peer_id: PeerId, gossip_interval: Duration, fanout: usize) -> Self {
        let mut local_clock = HashMap::new();
        local_clock.insert(peer_id, 1);

        Self {
            local_clock,
            last_gossip: Instant::now(),
            gossip_interval,
            fanout,
        }
    }

    pub fn should_gossip(&self) -> bool {
        self.last_gossip.elapsed() >= self.gossip_interval
    }

    pub fn mark_gossip(&mut self) {
        self.last_gossip = Instant::now();
    }

    pub fn increment_clock(&mut self, peer_id: &PeerId) {
        let counter = self.local_clock.entry(peer_id.clone()).or_insert(0);
        *counter += 1;
    }

    pub fn merge_vector_clocks(&mut self, other: &VectorClock) {
        for (peer_id, counter) in other {
            let local_counter = self.local_clock.entry(peer_id.clone()).or_insert(0);
            *local_counter = (*local_counter).max(*counter);
        }
    }
}

pub fn compute_digest(data: &[u8]) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

pub use compute_digest as digest;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gossip_message_serialization() {
        let msg = GossipMessage::Syn {
            from: "peer1".to_string(),
            vector_clock: HashMap::new(),
            digest: StateDigest::new(123, 5, HashMap::new()),
        };

        let bytes = msg.to_bytes().unwrap();
        let decoded = GossipMessage::from_bytes(&bytes).unwrap();

        assert_eq!(msg.peer_id(), decoded.peer_id());
    }

    #[test]
    fn test_vector_clock_merge() {
        let mut state = GossipState::new("peer1".to_string(), Duration::from_secs(5), 3);
        state.increment_clock(&"peer1".to_string());

        let mut other = HashMap::new();
        other.insert("peer1".to_string(), 5u64);
        other.insert("peer2".to_string(), 3u64);

        state.merge_vector_clocks(&other);

        assert_eq!(state.local_clock.get("peer1"), Some(&5));
        assert_eq!(state.local_clock.get("peer2"), Some(&3));
    }

    #[test]
    fn test_compute_digest() {
        let data1 = b"hello world";
        let data2 = b"hello world";
        let data3 = b"different data";

        let digest1 = compute_digest(data1);
        let digest2 = compute_digest(data2);
        let digest3 = compute_digest(data3);

        assert_eq!(digest1, digest2);
        assert_ne!(digest1, digest3);
    }
}
