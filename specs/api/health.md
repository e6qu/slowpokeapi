# Health Endpoints Specification

## Overview

SlowPokeAPI implements Kubernetes-style health probes and a deep health check.

## Endpoints

### `/healthz` - Liveness Probe

**Purpose:** Determines if the container is running. If this fails, Kubernetes restarts the container.

**Method:** `GET`

**Response:**
- `200 OK` - Service is alive
- Body: `ok`

**Implementation:**
```rust
async fn healthz() -> &'static str {
    "ok"
}
```

**Kubernetes Config:**
```yaml
livenessProbe:
  httpGet:
    path: /healthz
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 10
  failureThreshold: 3
```

---

### `/readyz` - Readiness Probe

**Purpose:** Determines if the container is ready to accept traffic. If this fails, Kubernetes removes the pod from service endpoints.

**Method:** `GET`

**Response:**
- `200 OK` - Service is ready
- `503 Service Unavailable` - Service not ready

**Checks:**
1. SQLite connection is available
2. At least one upstream API is reachable (optional, configurable)
3. Sync engine is initialized (if enabled)

**Implementation:**
```rust
async fn readyz(State(state): State<AppState>) -> impl IntoResponse {
    let mut checks = Vec::new();
    
    // Check SQLite
    if state.db.ping().await.is_err() {
        checks.push(("database", "failed"));
    }
    
    // Check if at least one upstream is healthy
    if !state.upstreams.any_healthy() {
        checks.push(("upstream", "no healthy upstream"));
    }
    
    if checks.is_empty() {
        (StatusCode::OK, "ok")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "not ready")
    }
}
```

**Kubernetes Config:**
```yaml
readinessProbe:
  httpGet:
    path: /readyz
    port: 8080
  initialDelaySeconds: 10
  periodSeconds: 5
  failureThreshold: 3
```

---

### `/livez` - Startup Probe

**Purpose:** Determines if the container has started successfully. Used for slow-starting containers to avoid being killed before ready.

**Method:** `GET`

**Response:**
- `200 OK` - Service has started
- Body: `ok`

**Implementation:**
```rust
static STARTED: AtomicBool = AtomicBool::new(false);

async fn livez() -> impl IntoResponse {
    if STARTED.load(Ordering::Relaxed) {
        (StatusCode::OK, "ok")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "starting")
    }
}
```

**Kubernetes Config:**
```yaml
startupProbe:
  httpGet:
    path: /livez
    port: 8080
  initialDelaySeconds: 0
  periodSeconds: 1
  failureThreshold: 30  # Allow up to 30s for startup
```

---

### `/health` - Deep Health Check

**Purpose:** Returns detailed health status of all components. Useful for debugging and monitoring.

**Method:** `GET`

**Response:**
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "uptime_seconds": 3600,
  "timestamp": "2026-03-02T12:00:00Z",
  "checks": {
    "database": {
      "status": "pass",
      "latency_ms": 2,
      "message": "SQLite connection healthy"
    },
    "upstream_frankfurter": {
      "status": "pass",
      "latency_ms": 45,
      "message": "Last successful request: 30s ago"
    },
    "upstream_fawaz": {
      "status": "warn",
      "latency_ms": 5000,
      "message": "High latency detected"
    },
    "upstream_coingecko": {
      "status": "pass",
      "latency_ms": 120
    },
    "sync_engine": {
      "status": "pass",
      "latency_ms": 1,
      "message": "3 peers connected, last sync 2s ago",
      "details": {
        "peers_total": 3,
        "peers_connected": 3,
        "last_sync_ago_seconds": 2,
        "pending_changes": 0
      }
    },
    "cache": {
      "status": "pass",
      "details": {
        "memory_entries": 1523,
        "hit_rate": 0.94
      }
    },
    "disk": {
      "status": "pass",
      "details": {
        "used_percent": 45,
        "available_mb": 5120
      }
    }
  }
}
```

**Status Values:**
- `healthy` - All checks pass
- `degraded` - Some checks warn but service is functional
- `unhealthy` - Critical checks failing

**Check Status Values:**
- `pass` - Check passed
- `warn` - Check passed with warnings
- `fail` - Check failed

**Implementation:**
```rust
#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    uptime_seconds: u64,
    timestamp: String,
    checks: HashMap<String, HealthCheck>,
}

#[derive(Serialize)]
struct HealthCheck {
    status: String,
    latency_ms: Option<u64>,
    message: Option<String>,
    details: Option<serde_json::Value>,
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let mut checks = HashMap::new();
    let mut overall_status = "healthy";
    
    // Database check
    let start = Instant::now();
    match state.db.ping().await {
        Ok(_) => {
            checks.insert("database".to_string(), HealthCheck {
                status: "pass".to_string(),
                latency_ms: Some(start.elapsed().as_millis() as u64),
                message: Some("SQLite connection healthy".to_string()),
                details: None,
            });
        }
        Err(e) => {
            overall_status = "unhealthy";
            checks.insert("database".to_string(), HealthCheck {
                status: "fail".to_string(),
                latency_ms: None,
                message: Some(format!("Database error: {}", e)),
                details: None,
            });
        }
    }
    
    // Upstream checks
    for upstream in &state.upstreams {
        let check = check_upstream(upstream).await;
        if check.status == "fail" && upstream.is_critical {
            overall_status = "unhealthy";
        } else if check.status == "warn" && overall_status == "healthy" {
            overall_status = "degraded";
        }
        checks.insert(format!("upstream_{}", upstream.name()), check);
    }
    
    // Sync engine check
    if state.config.sync.enabled {
        let check = check_sync(&state.sync).await;
        if check.status == "fail" {
            overall_status = "degraded"; // Sync failure is degraded, not unhealthy
        }
        checks.insert("sync_engine".to_string(), check);
    }
    
    // Cache check
    checks.insert("cache".to_string(), check_cache(&state.cache));
    
    // Disk check
    checks.insert("disk".to_string(), check_disk().await);
    
    HealthResponse {
        status: overall_status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: state.start_time.elapsed().as_secs(),
        timestamp: Utc::now().to_rfc3339(),
        checks,
    }
}
```

## Response Codes

| Endpoint | 200 | 503 |
|----------|-----|-----|
| `/healthz` | Always (if process alive) | Never |
| `/readyz` | All checks pass | Any check fails |
| `/livez` | After startup complete | During startup |
| `/health` | Always (check body for status) | Never |

## Configuration

```toml
[health]
# Threshold for marking upstream as degraded (ms)
upstream_latency_warn_ms = 1000
upstream_latency_fail_ms = 5000

# How long since last success before marking upstream as failed
upstream_stale_seconds = 300

# Disk usage thresholds
disk_warn_percent = 80
disk_fail_percent = 95

# Memory usage thresholds
memory_warn_percent = 80
memory_fail_percent = 95
```

## Metrics

Health checks also update Prometheus metrics:

```
slowpokeapi_health_status{check="database"} 1
slowpokeapi_health_status{check="upstream_frankfurter"} 1
slowpokeapi_health_check_duration_seconds{check="database"} 0.002
```
