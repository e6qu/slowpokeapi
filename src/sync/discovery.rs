use std::collections::HashSet;
use std::net::SocketAddr;

use crate::sync::{DiscoveryConfig, SyncError, SyncResult};

pub struct PeerDiscovery {
    config: DiscoveryConfig,
}

impl PeerDiscovery {
    pub fn new(config: DiscoveryConfig) -> Self {
        Self { config }
    }

    pub async fn discover_peers(&self) -> SyncResult<Vec<SocketAddr>> {
        match &self.config {
            DiscoveryConfig::Dns { dns_name } => self.discover_dns(dns_name).await,
            DiscoveryConfig::Static { peers } => self.discover_static(peers).await,
            DiscoveryConfig::Disabled => Ok(Vec::new()),
        }
    }

    async fn discover_dns(&self, dns_name: &str) -> SyncResult<Vec<SocketAddr>> {
        let addrs = tokio::net::lookup_host(dns_name)
            .await
            .map_err(|e| SyncError::DiscoveryError(format!("DNS lookup failed: {e}")))?;

        let unique_addrs: HashSet<SocketAddr> = addrs.collect();
        Ok(unique_addrs.into_iter().collect())
    }

    async fn discover_static(&self, peers: &[String]) -> SyncResult<Vec<SocketAddr>> {
        let mut addrs = Vec::new();

        for peer in peers {
            match peer.parse::<SocketAddr>() {
                Ok(addr) => addrs.push(addr),
                Err(_) => match tokio::net::lookup_host(peer).await {
                    Ok(found) => addrs.extend(found),
                    Err(e) => {
                        tracing::warn!("Failed to resolve peer {}: {}", peer, e);
                    }
                },
            }
        }

        let unique_addrs: HashSet<SocketAddr> = addrs.into_iter().collect();
        Ok(unique_addrs.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_static_discovery() {
        let config = DiscoveryConfig::Static {
            peers: vec!["127.0.0.1:8080".to_string()],
        };

        let discovery = PeerDiscovery::new(config);
        let peers = discovery.discover_peers().await.unwrap();

        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].to_string(), "127.0.0.1:8080");
    }

    #[tokio::test]
    async fn test_disabled_discovery() {
        let discovery = PeerDiscovery::new(DiscoveryConfig::Disabled);
        let peers = discovery.discover_peers().await.unwrap();

        assert!(peers.is_empty());
    }
}
