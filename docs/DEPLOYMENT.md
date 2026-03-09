# Deployment Guide

Deployment options for SlowPokeAPI.

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

Environment variables use the prefix `SLOWPOKEAPI__` with `__` as the separator for nested config:

| Variable | Default | Description |
|----------|---------|-------------|
| `SLOWPOKEAPI__SERVER__HOST` | 0.0.0.0 | HTTP bind address |
| `SLOWPOKEAPI__SERVER__PORT` | 8080 | HTTP port |
| `SLOWPOKEAPI__DATABASE__URL` | sqlite::memory: | SQLite database path |
| `SLOWPOKEAPI__LOGGING__LEVEL` | info | Log level (error, warn, info, debug, trace) |
| `SLOWPOKEAPI__LOGGING__FORMAT` | json | Log format (json, pretty) |
| `SLOWPOKEAPI__CACHE__MAX_CAPACITY` | 10000 | Maximum cache entries |
| `SLOWPOKEAPI__CACHE__TTL_SECONDS` | 3600 | Cache TTL in seconds |
| `SLOWPOKEAPI__SYNC__ENABLED` | false | Enable CRDT sync |
| `SLOWPOKEAPI__SYNC__PEER_ID` | (random UUID) | Unique peer identifier |
| `SLOWPOKEAPI__SYNC__SYNC_INTERVAL_MS` | 5000 | Sync interval in milliseconds |
| `SLOWPOKEAPI__SYNC__PEER_TIMEOUT_MS` | 60000 | Peer timeout in milliseconds |
| `SLOWPOKEAPI__RATE_LIMIT__ENABLED` | true | Enable rate limiting |
| `SLOWPOKEAPI__AUTH__ENABLED` | false | Enable API key authentication |

### Configuration File

Configuration can also be loaded from a YAML file:

```yaml
server:
  host: 0.0.0.0
  port: 8080

logging:
  level: info
  format: json

database:
  url: sqlite::memory:

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

auth:
  enabled: false
```

Load with: `./slowpokeapi --config /etc/slowpokeapi/config.yaml`

## Binary Deployment

### Prerequisites

- Rust 1.75+
- SQLite 3

### Build from Source

```bash
git clone https://github.com/e6qu/slowpokeapi.git
cd slowpokeapi
cargo build --release
```

Binary location: `./target/release/slowpokeapi`

### Run Binary

```bash
# With environment variables
export SLOWPOKEAPI__DATABASE__URL=sqlite:/var/lib/slowpokeapi/rates.db
export SLOWPOKEAPI__SERVER__PORT=8080
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
Environment="SLOWPOKEAPI__DATABASE__URL=sqlite:/var/lib/slowpokeapi/rates.db"
Environment="SLOWPOKEAPI__SERVER__PORT=8080"
Environment="SLOWPOKEAPI__LOGGING__LEVEL=info"
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
  -p 8080:8080 \
  -v slowpokeapi-data:/data \
  ghcr.io/e6qu/slowpokeapi:latest
```

### With Custom Configuration

```bash
docker run -d \
  --name slowpokeapi \
  -p 8080:8080 \
  -v $(pwd)/data:/data \
  -e SLOWPOKEAPI__DATABASE__URL=sqlite:/data/rates.db \
  -e SLOWPOKEAPI__LOGGING__LEVEL=debug \
  ghcr.io/e6qu/slowpokeapi:latest
```

### Build Local Image

```bash
docker build -t slowpokeapi:local .
docker run -p 8080:8080 slowpokeapi:local
```

## Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  slowpokeapi:
    image: ghcr.io/e6qu/slowpokeapi:latest
    ports:
      - "8080:8080"
    volumes:
      - slowpokeapi-data:/data
    environment:
      - SLOWPOKEAPI__DATABASE__URL=sqlite:/data/rates.db
      - SLOWPOKEAPI__LOGGING__LEVEL=info
      - SLOWPOKEAPI__RATE_LIMIT__ENABLED=true
      - SLOWPOKEAPI__AUTH__ENABLED=false
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/healthz"]
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
# Install from local chart
helm install slowpokeapi ./deploy/helm/slowpokeapi
```

### Basic Deployment

```bash
# Default deployment
helm install slowpokeapi ./deploy/helm/slowpokeapi

# With persistence
helm install slowpokeapi ./deploy/helm/slowpokeapi \
  --set persistence.enabled=true \
  --set persistence.size=10Gi
```

### Production Deployment

```bash
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
      - targets: ['localhost:8080']
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
| `GET /health` | Deep health check with database status |
| `GET /healthz` | Liveness probe (returns "ok") |
| `GET /readyz` | Readiness probe (checks database) |
| `GET /livez` | Liveness probe (returns "ok") |

### Key Metrics

| Metric | Description |
|--------|-------------|
| `slowpokeapi_requests_total` | Total HTTP requests |
| `slowpokeapi_request_duration_seconds` | Request latency |
| `slowpokeapi_cache_hits_total` | Cache hits |
| `slowpokeapi_cache_misses_total` | Cache misses |

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
lsof -i :8080

# Use different port
export SLOWPOKEAPI__SERVER__PORT=9090
```

### Rate Limiting

Check rate limit headers in responses:
- `X-RateLimit-Limit`
- `X-RateLimit-Remaining`
- `X-RateLimit-Reset`
