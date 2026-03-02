# Prometheus Metrics Specification

## Overview

SlowPokeAPI exposes a `/metrics` endpoint in Prometheus text format for monitoring.

## Endpoint

**Path:** `/metrics`

**Method:** `GET`

**Content-Type:** `text/plain; version=0.0.4; charset=utf-8`

## Metric Categories

### 1. HTTP Server Metrics

```
# HELP slowpokeapi_http_requests_total Total number of HTTP requests
# TYPE slowpokeapi_http_requests_total counter
slowpokeapi_http_requests_total{method="GET",path="/v1/latest/{base}",status="200"} 12345

# HELP slowpokeapi_http_request_duration_seconds HTTP request latency
# TYPE slowpokeapi_http_request_duration_seconds histogram
slowpokeapi_http_request_duration_seconds_bucket{method="GET",path="/v1/latest/{base}",le="0.005"} 100
slowpokeapi_http_request_duration_seconds_bucket{method="GET",path="/v1/latest/{base}",le="0.01"} 500
slowpokeapi_http_request_duration_seconds_bucket{method="GET",path="/v1/latest/{base}",le="0.025"} 2000
slowpokeapi_http_request_duration_seconds_bucket{method="GET",path="/v1/latest/{base}",le="0.05"} 5000
slowpokeapi_http_request_duration_seconds_bucket{method="GET",path="/v1/latest/{base}",le="0.1"} 9000
slowpokeapi_http_request_duration_seconds_bucket{method="GET",path="/v1/latest/{base}",le="0.25"} 11000
slowpokeapi_http_request_duration_seconds_bucket{method="GET",path="/v1/latest/{base}",le="0.5"} 12000
slowpokeapi_http_request_duration_seconds_bucket{method="GET",path="/v1/latest/{base}",le="1"} 12200
slowpokeapi_http_request_duration_seconds_bucket{method="GET",path="/v1/latest/{base}",le="+Inf"} 12345
slowpokeapi_http_request_duration_seconds_sum{method="GET",path="/v1/latest/{base}"} 1234.5
slowpokeapi_http_request_duration_seconds_count{method="GET",path="/v1/latest/{base}"} 12345

# HELP slowpokeapi_http_requests_in_flight Current number of HTTP requests being processed
# TYPE slowpokeapi_http_requests_in_flight gauge
slowpokeapi_http_requests_in_flight 5

# HELP slowpokeapi_http_response_size_bytes HTTP response size
# TYPE slowpokeapi_http_response_size_bytes histogram
slowpokeapi_http_response_size_bytes_bucket{le="100"} 5000
slowpokeapi_http_response_size_bytes_bucket{le="1000"} 10000
slowpokeapi_http_response_size_bytes_bucket{le="10000"} 12300
slowpokeapi_http_response_size_bytes_bucket{le="+Inf"} 12345
```

### 2. Cache Metrics

```
# HELP slowpokeapi_cache_entries_total Number of entries in cache
# TYPE slowpokeapi_cache_entries_total gauge
slowpokeapi_cache_entries_total{cache="memory"} 1523
slowpokeapi_cache_entries_total{cache="sqlite"} 45678

# HELP slowpokeapi_cache_hits_total Total cache hits
# TYPE slowpokeapi_cache_hits_total counter
slowpokeapi_cache_hits_total{cache="memory"} 50000
slowpokeapi_cache_hits_total{cache="sqlite"} 3000

# HELP slowpokeapi_cache_misses_total Total cache misses
# TYPE slowpokeapi_cache_misses_total counter
slowpokeapi_cache_misses_total{cache="memory"} 5000

# HELP slowpokeapi_cache_evictions_total Total cache evictions
# TYPE slowpokeapi_cache_evictions_total counter
slowpokeapi_cache_evictions_total{cache="memory"} 200

# HELP slowpokeapi_cache_hit_ratio Cache hit ratio (0-1)
# TYPE slowpokeapi_cache_hit_ratio gauge
slowpokeapi_cache_hit_ratio{cache="memory"} 0.91
```

### 3. Upstream API Metrics

```
# HELP slowpokeapi_upstream_requests_total Total requests to upstream APIs
# TYPE slowpokeapi_upstream_requests_total counter
slowpokeapi_upstream_requests_total{source="frankfurter",status="success"} 1000
slowpokeapi_upstream_requests_total{source="frankfurter",status="error"} 5
slowpokeapi_upstream_requests_total{source="fawaz",status="success"} 50
slowpokeapi_upstream_requests_total{source="fawaz",status="error"} 2
slowpokeapi_upstream_requests_total{source="coingecko",status="success"} 200

# HELP slowpokeapi_upstream_request_duration_seconds Upstream API request latency
# TYPE slowpokeapi_upstream_request_duration_seconds histogram
slowpokeapi_upstream_request_duration_seconds_bucket{source="frankfurter",le="0.1"} 800
slowpokeapi_upstream_request_duration_seconds_bucket{source="frankfurter",le="0.25"} 950
slowpokeapi_upstream_request_duration_seconds_bucket{source="frankfurter",le="0.5"} 990
slowpokeapi_upstream_request_duration_seconds_bucket{source="frankfurter",le="1"} 999
slowpokeapi_upstream_request_duration_seconds_bucket{source="frankfurter",le="+Inf"} 1005

# HELP slowpokeapi_upstream_healthy Whether upstream API is healthy
# TYPE slowpokeapi_upstream_healthy gauge
slowpokeapi_upstream_healthy{source="frankfurter"} 1
slowpokeapi_upstream_healthy{source="fawaz"} 1
slowpokeapi_upstream_healthy{source="coingecko"} 1

# HELP slowpokeapi_upstream_last_success_timestamp Unix timestamp of last successful request
# TYPE slowpokeapi_upstream_last_success_timestamp gauge
slowpokeapi_upstream_last_success_timestamp{source="frankfurter"} 1709337600
```

### 4. Sync Engine Metrics

```
# HELP slowpokeapi_sync_peers_total Number of known peers
# TYPE slowpokeapi_sync_peers_total gauge
slowpokeapi_sync_peers_total 5

# HELP slowpokeapi_sync_peers_connected Number of currently connected peers
# TYPE slowpokeapi_sync_peers_connected gauge
slowpokeapi_sync_peers_connected 4

# HELP slowpokeapi_sync_changes_sent_total Total changes sent to peers
# TYPE slowpokeapi_sync_changes_sent_total counter
slowpokeapi_sync_changes_sent_total 1234

# HELP slowpokeapi_sync_changes_received_total Total changes received from peers
# TYPE slowpokeapi_sync_changes_received_total counter
slowpokeapi_sync_changes_received_total 5678

# HELP slowpokeapi_sync_merge_duration_seconds Time to merge received changes
# TYPE slowpokeapi_sync_merge_duration_seconds histogram
slowpokeapi_sync_merge_duration_seconds_bucket{le="0.001"} 5000
slowpokeapi_sync_merge_duration_seconds_bucket{le="0.005"} 5500
slowpokeapi_sync_merge_duration_seconds_bucket{le="0.01"} 5600
slowpokeapi_sync_merge_duration_seconds_bucket{le="+Inf"} 5678

# HELP slowpokeapi_sync_document_size_bytes Size of CRDT document
# TYPE slowpokeapi_sync_document_size_bytes gauge
slowpokeapi_sync_document_size_bytes 524288

# HELP slowpokeapi_sync_last_success_timestamp Unix timestamp of last successful sync
# TYPE slowpokeapi_sync_last_success_timestamp gauge
slowpokeapi_sync_last_success_timestamp 1709337600

# HELP slowpokeapi_sync_pending_changes Number of pending changes to broadcast
# TYPE slowpokeapi_sync_pending_changes gauge
slowpokeapi_sync_pending_changes 3
```

### 5. Database Metrics

```
# HELP slowpokeapi_db_connections_active Number of active database connections
# TYPE slowpokeapi_db_connections_active gauge
slowpokeapi_db_connections_active 3

# HELP slowpokeapi_db_connections_idle Number of idle database connections
# TYPE slowpokeapi_db_connections_idle gauge
slowpokeapi_db_connections_idle 2

# HELP slowpokeapi_db_queries_total Total database queries
# TYPE slowpokeapi_db_queries_total counter
slowpokeapi_db_queries_total{operation="select"} 10000
slowpokeapi_db_queries_total{operation="insert"} 500
slowpokeapi_db_queries_total{operation="update"} 200

# HELP slowpokeapi_db_query_duration_seconds Database query latency
# TYPE slowpokeapi_db_query_duration_seconds histogram
slowpokeapi_db_query_duration_seconds_bucket{operation="select",le="0.001"} 8000
slowpokeapi_db_query_duration_seconds_bucket{operation="select",le="0.005"} 9500
slowpokeapi_db_query_duration_seconds_bucket{operation="select",le="0.01"} 9900
slowpokeapi_db_query_duration_seconds_bucket{operation="select",le="+Inf"} 10000

# HELP slowpokeapi_db_size_bytes SQLite database file size
# TYPE slowpokeapi_db_size_bytes gauge
slowpokeapi_db_size_bytes 10485760
```

### 6. Rate Limiting Metrics

```
# HELP slowpokeapi_ratelimit_requests_total Total rate-limited requests
# TYPE slowpokeapi_ratelimit_requests_total counter
slowpokeapi_ratelimit_requests_total{status="allowed"} 10000
slowpokeapi_ratelimit_requests_total{status="rejected"} 50

# HELP slowpokeapi_ratelimit_remaining Remaining requests for API key
# TYPE slowpokeapi_ratelimit_remaining gauge
slowpokeapi_ratelimit_remaining{api_key="abc123"} 950
```

### 7. System Metrics

```
# HELP slowpokeapi_uptime_seconds Service uptime in seconds
# TYPE slowpokeapi_uptime_seconds gauge
slowpokeapi_uptime_seconds 86400

# HELP slowpokeapi_build_info Build information
# TYPE slowpokeapi_build_info gauge
slowpokeapi_build_info{version="1.0.0",commit="abc123",rust_version="1.75"} 1

# HELP slowpokeapi_memory_usage_bytes Process memory usage
# TYPE slowpokeapi_memory_usage_bytes gauge
slowpokeapi_memory_usage_bytes 52428800
```

## Implementation

Using `axum-prometheus`:

```rust
use axum_prometheus::{PrometheusMetricLayer, PrometheusMetricLayerBuilder};

fn init_metrics() -> (PrometheusMetricLayer, PrometheusMetricExporter) {
    PrometheusMetricLayerBuilder::new()
        .with_prefix("slowpokeapi")
        .with_default_metrics()
        .build_pair()
}

// In main.rs
let (prometheus_layer, metric_exporter) = init_metrics();

let app = Router::new()
    .route("/metrics", get(metrics_handler))
    .layer(prometheus_layer);
```

Custom metrics:

```rust
use prometheus::{Counter, Gauge, HistogramVec, Registry, Opts, HistogramOpts};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    
    pub static ref CACHE_HITS: Counter = Counter::new(
        "slowpokeapi_cache_hits_total",
        "Total cache hits"
    ).unwrap();
    
    pub static ref CACHE_ENTRIES: Gauge = Gauge::new(
        "slowpokeapi_cache_entries_total",
        "Number of entries in cache"
    ).unwrap();
    
    pub static ref UPSTREAM_LATENCY: HistogramVec = HistogramVec::new(
        HistogramOpts::new(
            "slowpokeapi_upstream_request_duration_seconds",
            "Upstream API request latency"
        ).buckets(vec![0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]),
        &["source"]
    ).unwrap();
}

fn register_custom_metrics() {
    REGISTRY.register(Box::new(CACHE_HITS.clone())).unwrap();
    REGISTRY.register(Box::new(CACHE_ENTRIES.clone())).unwrap();
    REGISTRY.register(Box::new(UPSTREAM_LATENCY.clone())).unwrap();
}
```

## Prometheus Scrape Config

```yaml
scrape_configs:
  - job_name: 'slowpokeapi'
    kubernetes_sd_configs:
      - role: pod
    relabel_configs:
      - source_labels: [__meta_kubernetes_pod_annotation_prometheus_io_scrape]
        action: keep
        regex: true
      - source_labels: [__meta_kubernetes_pod_annotation_prometheus_io_path]
        action: replace
        target_label: __metrics_path__
        regex: (.+)
      - source_labels: [__address__, __meta_kubernetes_pod_annotation_prometheus_io_port]
        action: replace
        regex: ([^:]+)(?::\d+)?;(\d+)
        replacement: $1:$2
        target_label: __address__
```

## Grafana Dashboard

Key panels to include:

1. **Request Rate** - `rate(slowpokeapi_http_requests_total[5m])`
2. **Latency (p99)** - `histogram_quantile(0.99, rate(slowpokeapi_http_request_duration_seconds_bucket[5m]))`
3. **Cache Hit Ratio** - `slowpokeapi_cache_hit_ratio`
4. **Upstream Health** - `slowpokeapi_upstream_healthy`
5. **Sync Status** - `slowpokeapi_sync_peers_connected / slowpokeapi_sync_peers_total`
6. **Error Rate** - `rate(slowpokeapi_http_requests_total{status=~"5.."}[5m])`
