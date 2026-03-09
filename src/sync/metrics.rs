use once_cell::sync::Lazy;
use prometheus::{histogram_opts, Counter, Gauge, Histogram};

pub static SYNC_METRICS: Lazy<SyncMetrics> = Lazy::new(SyncMetrics::new);

pub struct SyncMetrics {
    pub sync_operations_total: Counter,
    pub sync_errors_total: Counter,
    pub sync_changes_sent_total: Counter,
    pub sync_changes_received_total: Counter,
    pub peers_connected: Gauge,
    pub document_size_bytes: Gauge,
    pub sync_merge_duration_seconds: Histogram,
    pub sync_peers_total: Gauge,
}

impl SyncMetrics {
    fn new() -> Self {
        let sync_operations_total = Counter::new(
            "slowpokeapi_sync_operations_total",
            "Total number of sync operations",
        )
        .unwrap();

        let sync_errors_total = Counter::new(
            "slowpokeapi_sync_errors_total",
            "Total number of sync errors",
        )
        .unwrap();

        let sync_changes_sent_total = Counter::new(
            "slowpokeapi_sync_changes_sent_total",
            "Total number of changes sent",
        )
        .unwrap();

        let sync_changes_received_total = Counter::new(
            "slowpokeapi_sync_changes_received_total",
            "Total number of changes received",
        )
        .unwrap();

        let peers_connected = Gauge::new(
            "slowpokeapi_peers_connected",
            "Number of peers currently connected",
        )
        .unwrap();

        let document_size_bytes = Gauge::new(
            "slowpokeapi_document_size_bytes",
            "Size of CRDT document in bytes",
        )
        .unwrap();

        let sync_merge_duration_seconds = Histogram::with_opts(histogram_opts!(
            "slowpokeapi_sync_merge_duration_seconds",
            "Time to merge changes",
            vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]
        ))
        .unwrap();

        let sync_peers_total = Gauge::new(
            "slowpokeapi_sync_peers_total",
            "Total number of known peers",
        )
        .unwrap();

        let registry = prometheus::default_registry();
        registry
            .register(Box::new(sync_operations_total.clone()))
            .unwrap();
        registry
            .register(Box::new(sync_errors_total.clone()))
            .unwrap();
        registry
            .register(Box::new(sync_changes_sent_total.clone()))
            .unwrap();
        registry
            .register(Box::new(sync_changes_received_total.clone()))
            .unwrap();
        registry
            .register(Box::new(peers_connected.clone()))
            .unwrap();
        registry
            .register(Box::new(document_size_bytes.clone()))
            .unwrap();
        registry
            .register(Box::new(sync_merge_duration_seconds.clone()))
            .unwrap();
        registry
            .register(Box::new(sync_peers_total.clone()))
            .unwrap();

        Self {
            sync_operations_total,
            sync_errors_total,
            sync_changes_sent_total,
            sync_changes_received_total,
            peers_connected,
            document_size_bytes,
            sync_merge_duration_seconds,
            sync_peers_total,
        }
    }
}
