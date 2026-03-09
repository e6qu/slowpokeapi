# Deployment Guide

This guide covers all deployment options for SlowPokeAPI.

## Table of Contents

- [Configuration](#configuration)
- [Binary Deployment](#binary-deployment)
- [Docker Deployment](#docker-deployment)
- [Docker Compose](#docker-compose)
- [Kubernetes with Helm](#kubernetes-with-helm)
- [AWS ECS with Terraform](#aws-ecs-with-terraform)
- [Monitoring Setup](#monitoring-setup)

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `API_HOST` | 0.0.0.0 | HTTP API bind address |
| `API_PORT` | 8081 | HTTP API port |
| `SYNC_HOST` | 0.0.0.0 | Sync protocol bind address |
| `SYNC_PORT` | 8082 | Sync protocol port |
| `DATABASE_URL` | sqlite:data/rates.db | SQLite database path |
| `LOG_LEVEL` | info | Logging level (error, warn, info, debug, trace) |
| `LOG_FORMAT` | json | Log format (json, pretty) |
| `CACHE_MAX_CAPACITY` | 10000 | Maximum cache entries |
| `CACHE_TTL_SECONDS` | 3600 | Cache TTL in seconds |
| `SYNC_ENABLED` | false | Enable CRDT sync |
| `SYNC_PEER_ID` | (random UUID) | Unique peer identifier |
| `SYNC_INTERVAL_MS` | 5000 | Sync interval in milliseconds |
| `SYNC_PEER_TIMEOUT_MS` | 60000 | Peer timeout in milliseconds |
| `RATE_LIMIT_ENABLED` | true | Enable rate limiting |
| `RATE_LIMIT_GLOBAL_RPS` | 500 | Global requests per second |
| `RATE_LIMIT_GLOBAL_BURST` | 1000 | Global burst capacity |
| `RATE_LIMIT_AUTH_RPS` | 50 | Authenticated requests per second |
| `RATE_LIMIT_AUTH_BURST` | 100 | Authenticated burst capacity |
| `RATE_LIMIT_ANON_RPS` | 10 | Anonymous requests per second |
| `RATE_LIMIT_ANON_BURST` | 20 | Anonymous burst capacity |
| `AUTH_ENABLED` | false | Enable API key authentication |
| `AUTH_REQUIRE_KEY` | false | Require API key for all requests |

### Configuration File

You can also use a YAML configuration file:

```yaml
server:
  host: 0.0.0.0
  port: 8081

logging:
  level: info
  format: json

database:
  url: sqlite:data/rates.db

cache:
  max_capacity: 10000
  ttl_seconds: 3600

sync:
  enabled: false
  peer_id: "my-peer-001"
  sync_interval_ms: 5000
  peer_timeout_ms: 60000

rate_limit:
  enabled: true
  global_requests_per_second: 500
  global_burst_capacity: 1000
  authenticated_requests_per_second: 50
  authenticated_burst_capacity: 100
  anonymous_requests_per_second: 10
  anonymous_burst_capacity: 20

auth:
  enabled: false
  require_api_key: false
  public_paths:
    - /health
    - /metrics
```

## Binary Deployment

### Prerequisites

- Rust 1.75+ 
- SQLite 3
- Linux, macOS, or Windows

### Build from Source

```bash
# Clone repository
git clone https://github.com/e6qu/slowpokeapi.git
cd slowpokeapi

# Build release binary
cargo build --release

# Binary location
./target/release/slowpokeapi
```

### Run Binary

```bash
# With environment variables
export DATABASE_URL=sqlite:/var/lib/slowpokeapi/rates.db
export API_PORT=8081
./target/release/slowpokeapi

# Or with config file
./target/release/slowpokeapi --config /etc/slowpokeapi/config.yaml
```

### Systemd Service

Create `/etc/systemd/system/slowpokeapi.service`:

```ini
[Unit]
Description=SlowPokeAPI Currency Exchange Service
After=network.target

[Service]
Type=simple
User=slowpokeapi
Group=slowpokeapi
WorkingDirectory=/var/lib/slowpokeapi
Environment="DATABASE_URL=sqlite:/var/lib/slowpokeapi/rates.db"
Environment="API_PORT=8081"
Environment="LOG_LEVEL=info"
ExecStart=/usr/local/bin/slowpokeapi
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable slowpokeapi
sudo systemctl start slowpokeapi
```

## Docker Deployment

### Quick Start

```bash
docker run -d \
  --name slowpokeapi \
  -p 8081:8081 \
  -p 8082:8082 \
  -v slowpokeapi-data:/data \
  ghcr.io/e6qu/slowpokeapi:latest
```

### With Custom Configuration

```bash
docker run -d \
  --name slowpokeapi \
  -p 8081:8081 \
  -p 8082:8082 \
  -v $(pwd)/data:/data \
  -e DATABASE_URL=sqlite:/data/rates.db \
  -e LOG_LEVEL=debug \
  -e RATE_LIMIT_ENABLED=true \
  ghcr.io/e6qu/slowpokeapi:latest
```

### Build Local Image

```bash
docker build -t slowpokeapi:local .
docker run -p 8081:8081 slowpokeapi:local
```

## Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  slowpokeapi:
    image: ghcr.io/e6qu/slowpokeapi:latest
    ports:
      - "8081:8081"
      - "8082:8082"
    volumes:
      - slowpokeapi-data:/data
    environment:
      - DATABASE_URL=sqlite:/data/rates.db
      - LOG_LEVEL=info
      - RATE_LIMIT_ENABLED=true
      - AUTH_ENABLED=false
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8081/health"]
      interval: 10s
      timeout: 5s
      retries: 3
    restart: unless-stopped

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./deploy/prometheus:/etc/prometheus
      - prometheus-data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    volumes:
      - grafana-data:/var/lib/grafana
      - ./deploy/grafana/dashboard.json:/var/lib/grafana/dashboards/slowpokeapi.json
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin

volumes:
  slowpokeapi-data:
  prometheus-data:
  grafana-data:
```

Run:

```bash
docker-compose up -d
```

## Kubernetes with Helm

### Prerequisites

- Kubernetes 1.24+
- Helm 3.12+

### Install Chart

```bash
# Add Helm repository (if published)
helm repo add slowpokeapi https://e6qu.github.io/slowpokeapi
helm repo update

# Install
helm install slowpokeapi slowpokeapi/slowpokeapi

# Or install from local chart
helm install slowpokeapi ./deploy/helm/slowpokeapi
```

### Basic Deployment

```bash
# Default deployment (stateless)
helm install slowpokeapi ./deploy/helm/slowpokeapi

# With persistence (StatefulSet)
helm install slowpokeapi ./deploy/helm/slowpokeapi \
  --set persistence.enabled=true \
  --set persistence.size=10Gi
```

### Production Deployment

```bash
# Use production values
helm install slowpokeapi ./deploy/helm/slowpokeapi \
  -f ./deploy/helm/slowpokeapi/values-prod.yaml \
  --set ingress.enabled=true \
  --set ingress.hosts[0].host=api.example.com
```

### Key Configuration Options

| Parameter | Description | Default |
|-----------|-------------|---------|
| `replicaCount` | Number of replicas | 1 |
| `image.tag` | Image tag | latest |
| `service.type` | Service type | ClusterIP |
| `persistence.enabled` | Enable PVC | false |
| `persistence.size` | PVC size | 10Gi |
| `ingress.enabled` | Enable ingress | false |
| `autoscaling.enabled` | Enable HPA | false |
| `serviceMonitor.enabled` | Enable ServiceMonitor | false |

### Upgrade

```bash
helm upgrade slowpokeapi ./deploy/helm/slowpokeapi
```

### Uninstall

```bash
helm uninstall slowpokeapi
```

## AWS ECS with Terraform

### Prerequisites

- AWS CLI configured
- Terraform 1.5+

### Deploy

```bash
cd deploy/terraform

# Initialize
terraform init

# Plan
terraform plan -var-file=prod.tfvars

# Apply
terraform apply -var-file=prod.tfvars
```

### Configuration

Edit `prod.tfvars`:

```hcl
aws_region = "us-east-1"
environment = "production"

# VPC
vpc_cidr = "10.0.0.0/16"
azs = ["us-east-1a", "us-east-1b", "us-east-1c"]

# ECS
desired_count = 3
min_count = 3
max_count = 20

# Resources
task_cpu = 1024
task_memory = 2048

# Domain
domain_name = "api.example.com"
route53_zone_id = "Z1234567890ABC"

# EFS
enable_efs = true
```

### Outputs

After deployment, Terraform outputs:

- `alb_dns_name` - Load balancer URL
- `api_url` - Full API URL (if domain configured)
- `ecs_cluster_name` - ECS cluster name
- `cloudwatch_log_group` - Log group for debugging

### Cleanup

```bash
terraform destroy -var-file=prod.tfvars
```

## Monitoring Setup

### Prometheus Scraping

Add to `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'slowpokeapi'
    static_configs:
      - targets: ['localhost:8081']
    metrics_path: /metrics
```

### Grafana Dashboard

Import `deploy/grafana/dashboard.json`:

1. Go to Grafana → Create → Import
2. Upload `dashboard.json` or paste contents
3. Select Prometheus datasource
4. Import

### Health Checks

| Endpoint | Description |
|----------|-------------|
| `GET /health` | Combined health status |
| `GET /healthz` | Kubernetes liveness probe |
| `GET /readyz` | Kubernetes readiness probe |
| `GET /livez` | Kubernetes liveness probe |

### Key Metrics

| Metric | Description |
|--------|-------------|
| `slowpokeapi_requests_total` | Total HTTP requests |
| `slowpokeapi_request_duration_seconds` | Request latency |
| `slowpokeapi_sync_operations_total` | CRDT sync operations |
| `slowpokeapi_peers_connected` | Connected peers |

## Troubleshooting

### Database Permissions

```bash
# Ensure data directory is writable
chmod 755 /var/lib/slowpokeapi
chown -R slowpokeapi:slowpokeapi /var/lib/slowpokeapi
```

### Port Conflicts

```bash
# Check if port is in use
lsof -i :8081

# Use different port
export API_PORT=9090
```

### Sync Issues

Ensure port 8082 is open between instances for CRDT synchronization.

### Rate Limiting

Check rate limit headers in responses:
- `X-RateLimit-Limit`
- `X-RateLimit-Remaining`
- `X-RateLimit-Reset`
