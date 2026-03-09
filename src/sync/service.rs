use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio::time::interval;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{accept_async, connect_async, MaybeTlsStream, WebSocketStream};

use crate::sync::{
    compute_digest, CrdtDocument, GossipMessage, GossipState, Peer, PeerDiscovery, PeerManager,
    StateDigest, SyncConfig, SyncError, SyncResult, SYNC_METRICS,
};

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct SyncService {
    config: SyncConfig,
    document: Arc<RwLock<CrdtDocument>>,
    peer_manager: Arc<PeerManager>,
    gossip_state: Arc<RwLock<GossipState>>,
    _discovery: PeerDiscovery,
}

impl SyncService {
    pub fn new(config: SyncConfig, document: Arc<RwLock<CrdtDocument>>) -> Self {
        let peer_manager = Arc::new(PeerManager::new(config.peer_timeout));
        let gossip_state = Arc::new(RwLock::new(GossipState::new(
            config.peer_id.clone(),
            config.gossip_interval,
            config.fanout,
        )));
        let discovery = PeerDiscovery::new(config.discovery.clone());

        Self {
            config,
            document,
            peer_manager,
            gossip_state,
            _discovery: discovery,
        }
    }

    pub async fn start(&self, bind_addr: SocketAddr) -> SyncResult<()> {
        if !self.config.enabled {
            tracing::info!("Sync service is disabled");
            return Ok(());
        }

        tracing::info!("Starting sync service on {}", bind_addr);

        let listener = TcpListener::bind(bind_addr)
            .await
            .map_err(|e| SyncError::TransportError(format!("Failed to bind: {e}")))?;

        self.spawn_discovery_task();
        self.spawn_gossip_task();
        self.spawn_heartbeat_task();

        loop {
            let (stream, addr) = listener
                .accept()
                .await
                .map_err(|e| SyncError::TransportError(format!("Accept failed: {e}")))?;

            let document = self.document.clone();
            let peer_manager = self.peer_manager.clone();

            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, addr, document, peer_manager).await {
                    tracing::warn!("Connection handler error: {}", e);
                }
            });
        }
    }

    fn spawn_discovery_task(&self) {
        let discovery = PeerDiscovery::new(self.config.discovery.clone());
        let peer_manager = self.peer_manager.clone();
        let interval_duration = Duration::from_secs(30);

        tokio::spawn(async move {
            let mut ticker = interval(interval_duration);

            loop {
                ticker.tick().await;

                match discovery.discover_peers().await {
                    Ok(peers) => {
                        for addr in peers {
                            let peer_id = format!("peer-{addr}");
                            let peer = Peer::new(peer_id, addr);
                            peer_manager.add_peer(peer).await;
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Discovery error: {}", e);
                    }
                }
            }
        });
    }

    fn spawn_gossip_task(&self) {
        let document = self.document.clone();
        let peer_manager = self.peer_manager.clone();
        let gossip_state = self.gossip_state.clone();
        let gossip_interval = self.config.gossip_interval;

        tokio::spawn(async move {
            let mut ticker = interval(gossip_interval);

            loop {
                ticker.tick().await;

                let mut state = gossip_state.write().await;
                if !state.should_gossip() {
                    continue;
                }

                let healthy_peers = peer_manager.get_healthy_peers().await;
                if healthy_peers.is_empty() {
                    continue;
                }

                let doc = document.read().await;
                let doc_bytes = doc.to_bytes();
                let digest = StateDigest::new(
                    compute_digest(&doc_bytes),
                    doc.document_size(),
                    state.local_clock.clone(),
                );

                drop(doc);

                let syn_msg = GossipMessage::Syn {
                    from: state.local_clock.keys().next().cloned().unwrap_or_default(),
                    vector_clock: state.local_clock.clone(),
                    digest,
                };

                for peer in healthy_peers.iter().take(3) {
                    if let Ok(_bytes) = syn_msg.to_bytes() {
                        tracing::debug!("Sending SYN to {}", peer.address);
                        SYNC_METRICS.sync_operations_total.inc();
                    }
                }

                state.mark_gossip();
            }
        });
    }

    fn spawn_heartbeat_task(&self) {
        let peer_manager = self.peer_manager.clone();
        let heartbeat_interval = self.config.heartbeat_interval;
        let peer_id = self.config.peer_id.clone();

        tokio::spawn(async move {
            let mut ticker = interval(heartbeat_interval);

            loop {
                ticker.tick().await;

                let peers = peer_manager.get_all_peers().await;
                let heartbeat = GossipMessage::Heartbeat {
                    from: peer_id.clone(),
                    timestamp: chrono::Utc::now().timestamp(),
                };

                for peer in peers {
                    if let Err(e) = send_heartbeat(&peer, &heartbeat).await {
                        tracing::debug!("Failed to send heartbeat to {}: {}", peer.id, e);
                    }
                }
            }
        });
    }

    pub async fn force_sync(&self) -> SyncResult<()> {
        let doc = self.document.read().await;
        let _ = doc.to_bytes();
        SYNC_METRICS.sync_operations_total.inc();
        Ok(())
    }

    pub async fn get_sync_status(&self) -> SyncStatus {
        let peer_count = self.peer_manager.count().await;
        let healthy_peers = self.peer_manager.get_healthy_peers().await.len();
        let doc = self.document.read().await;

        SyncStatus {
            peer_count,
            connected_peers: healthy_peers,
            last_sync: None,
            pending_changes: 0,
            document_version: 1,
            is_syncing: false,
            document_size: doc.document_size(),
        }
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    document: Arc<RwLock<CrdtDocument>>,
    peer_manager: Arc<PeerManager>,
) -> SyncResult<()> {
    let ws_stream = accept_async(MaybeTlsStream::Plain(stream))
        .await
        .map_err(|e| SyncError::TransportError(format!("WebSocket accept failed: {e}")))?;

    let peer_id = format!("ws-{addr}");
    let peer = Peer::new(peer_id.clone(), addr);
    peer_manager.add_peer(peer).await;

    handle_websocket(ws_stream, document, peer_manager, peer_id).await
}

async fn handle_websocket(
    mut ws_stream: WsStream,
    document: Arc<RwLock<CrdtDocument>>,
    _peer_manager: Arc<PeerManager>,
    peer_id: String,
) -> SyncResult<()> {
    while let Some(msg) = ws_stream.next().await {
        let msg = msg.map_err(|e| SyncError::TransportError(format!("WebSocket error: {e}")))?;

        if let Message::Binary(data) = msg {
            match GossipMessage::from_bytes(&data) {
                Ok(gossip_msg) => {
                    tracing::debug!("Received {:?} from {}", gossip_msg, peer_id);

                    match gossip_msg {
                        GossipMessage::Syn { from: _, .. } => {
                            let doc = document.read().await;
                            let changes = doc.to_bytes();

                            let syn_ack = GossipMessage::SynAck {
                                from: peer_id.clone(),
                                missing_changes: changes,
                                their_digest: StateDigest::new(0, 0, HashMap::new()),
                            };

                            if let Ok(bytes) = syn_ack.to_bytes() {
                                let _ = ws_stream.send(Message::Binary(bytes.into())).await;
                            }
                            SYNC_METRICS.sync_changes_sent_total.inc_by(1.0);
                        }
                        GossipMessage::SynAck {
                            missing_changes, ..
                        } => {
                            let mut doc = document.write().await;
                            if let Err(e) = doc.apply_changes(&missing_changes) {
                                tracing::warn!("Failed to apply changes: {}", e);
                                SYNC_METRICS.sync_errors_total.inc();
                            } else {
                                SYNC_METRICS.sync_changes_received_total.inc_by(1.0);
                            }
                        }
                        GossipMessage::Ack { changes, .. } => {
                            let mut doc = document.write().await;
                            if let Err(e) = doc.apply_changes(&changes) {
                                tracing::warn!("Failed to apply changes: {}", e);
                                SYNC_METRICS.sync_errors_total.inc();
                            } else {
                                SYNC_METRICS.sync_changes_received_total.inc_by(1.0);
                            }
                        }
                        GossipMessage::Heartbeat { from, timestamp } => {
                            tracing::debug!("Heartbeat from {} at {}", from, timestamp);
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to parse gossip message: {}", e);
                }
            }
        }
    }

    Ok(())
}

async fn send_heartbeat(peer: &Peer, heartbeat: &GossipMessage) -> SyncResult<()> {
    let addr = format!("ws://{}/sync", peer.address);

    match connect_async(&addr).await {
        Ok((mut ws_stream, _)) => {
            let bytes = heartbeat.to_bytes()?;
            ws_stream
                .send(Message::Binary(bytes.into()))
                .await
                .map_err(|e| SyncError::TransportError(format!("Send failed: {e}")))?;
            Ok(())
        }
        Err(e) => Err(SyncError::TransportError(format!("Connection failed: {e}"))),
    }
}

#[derive(Debug, Clone)]
pub struct SyncStatus {
    pub peer_count: usize,
    pub connected_peers: usize,
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    pub pending_changes: usize,
    pub document_version: u64,
    pub is_syncing: bool,
    pub document_size: usize,
}

pub async fn connect_to_peer(
    addr: SocketAddr,
    document: Arc<RwLock<CrdtDocument>>,
) -> SyncResult<()> {
    let ws_url = format!("ws://{addr}/sync");

    let (mut ws_stream, _) = connect_async(&ws_url)
        .await
        .map_err(|e| SyncError::TransportError(format!("Connection failed: {e}")))?;

    let doc = document.read().await;
    let doc_bytes = doc.to_bytes();
    let digest = StateDigest::new(compute_digest(&doc_bytes), doc_bytes.len(), HashMap::new());

    let syn_msg = GossipMessage::Syn {
        from: "local".to_string(),
        vector_clock: HashMap::new(),
        digest,
    };

    let bytes = syn_msg.to_bytes()?;
    ws_stream
        .send(Message::Binary(bytes.into()))
        .await
        .map_err(|e| SyncError::TransportError(format!("Send failed: {e}")))?;

    Ok(())
}
