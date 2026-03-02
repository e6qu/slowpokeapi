# Container/Docker Specification

## Overview

SlowPokeAPI is packaged as a minimal OCI-compliant container image using Distroless base.

## Image Details

### Base Images

| Stage | Base Image |
|-------|------------|
| Build | `rust:1.75-slim` |
| Runtime | `gcr.io/distroless/cc-debian12:latest` |

### Image Tags

```
ghcr.io/e6qu/slowpokeapi:latest
ghcr.io/e6qu/slowpokeapi:1.0.0
ghcr.io/e6qu/slowpokeapi:1.0.0-debug
```

## Dockerfile

### Multi-Stage Build

```dockerfile
# Build stage
FROM rust:1.75-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Build application
COPY src ./src
COPY migrations ./migrations
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM gcr.io/distroless/cc-debian12:latest

COPY --from=builder /app/target/release/slowpokeapi /slowpokeapi
COPY --from=builder /app/migrations /migrations

EXPOSE 8080
EXPOSE 8081

USER nonroot:nonroot

ENTRYPOINT ["/slowpokeapi"]
```

### Debug Image (with shell)

```dockerfile
# Debug stage for troubleshooting
FROM debian:bookworm-slim AS debug

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/slowpokeapi /slowpokeapi
COPY --from=builder /app/migrations /migrations

EXPOSE 8080
EXPOSE 8081

ENTRYPOINT ["/slowpokeapi"]
```

## Build Commands

### Standard Build

```bash
docker build -t slowpokeapi:latest .
```

### Build with Arguments

```bash
docker build \
    --build-arg RUST_VERSION=1.75 \
    --build-arg TARGET=x86_64-unknown-linux-gnu \
    -t slowpokeapi:latest .
```

### Multi-Architecture Build

```bash
# Setup buildx
docker buildx create --use

# Build for multiple platforms
docker buildx build \
    --platform linux/amd64,linux/arm64 \
    -t ghcr.io/e6qu/slowpokeapi:latest \
    --push .
```

## Container Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SLOWPOKEAPI_SERVER_HOST` | `0.0.0.0` | Server bind address |
| `SLOWPOKEAPI_SERVER_PORT` | `8080` | HTTP server port |
| `SLOWPOKEAPI_SYNC_PORT` | `8081` | Sync protocol port |
| `SLOWPOKEAPI_STORAGE_PATH` | `/data/slowpokeapi.db` | SQLite database path |
| `SLOWPOKEAPI_LOG_LEVEL` | `info` | Log level (trace/debug/info/warn/error) |
| `SLOWPOKEAPI_SYNC_ENABLED` | `true` | Enable CRDT sync |
| `SLOWPOKEAPI_SYNC_DISCOVERY` | `dns` | Peer discovery method |
| `SLOWPOKEAPI_SYNC_DNS_NAME` | - | DNS name for peer discovery |
| `SLOWPOKEAPI_CACHE_TTL_SECONDS` | `3600` | Cache TTL |
| `SLOWPOKEAPI_UPSTREAM_TIMEOUT_SECONDS` | `10` | Upstream API timeout |

### Volume Mounts

| Container Path | Purpose |
|----------------|---------|
| `/data` | SQLite database storage |
| `/config` | Configuration files |
| `/migrations` | Database migrations |

## Running the Container

### Basic Run

```bash
docker run -d \
    --name slowpokeapi \
    -p 8080:8080 \
    -p 8081:8081 \
    -v slowpokeapi-data:/data \
    ghcr.io/e6qu/slowpokeapi:latest
```

### With Environment Variables

```bash
docker run -d \
    --name slowpokeapi \
    -p 8080:8080 \
    -p 8081:8081 \
    -v slowpokeapi-data:/data \
    -e SLOWPOKEAPI_LOG_LEVEL=debug \
    -e SLOWPOKEAPI_CACHE_TTL_SECONDS=7200 \
    ghcr.io/e6qu/slowpokeapi:latest
```

### With Configuration File

```bash
docker run -d \
    --name slowpokeapi \
    -p 8080:8080 \
    -p 8081:8081 \
    -v slowpokeapi-data:/data \
    -v ./config.toml:/config/config.toml:ro \
    ghcr.io/e6qu/slowpokeapi:latest \
    --config /config/config.toml
```

### Development Mode

```bash
docker run -d \
    --name slowpokeapi \
    -p 8080:8080 \
    -p 8081:8081 \
    -v $(pwd)/data:/data \
    -e SLOWPOKEAPI_LOG_LEVEL=debug \
    ghcr.io/e6qu/slowpokeapi:debug
```

## Docker Compose

### Single Instance

```yaml
version: '3.8'

services:
  slowpokeapi:
    image: ghcr.io/e6qu/slowpokeapi:latest
    ports:
      - "8080:8080"
      - "8081:8081"
    volumes:
      - slowpokeapi-data:/data
    environment:
      - SLOWPOKEAPI_LOG_LEVEL=info
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/healthz"]
      interval: 10s
      timeout: 5s
      retries: 3
    restart: unless-stopped

volumes:
  slowpokeapi-data:
```

### Multi-Replica Cluster

```yaml
version: '3.8'

services:
  slowpokeapi-1:
    image: ghcr.io/e6qu/slowpokeapi:latest
    hostname: slowpokeapi-1
    ports:
      - "8080:8080"
      - "8081:8081"
    volumes:
      - slowpokeapi-data-1:/data
    environment:
      - SLOWPOKEAPI_SYNC_ENABLED=true
      - SLOWPOKEAPI_SYNC_STATIC_PEERS=slowpokeapi-2:8081,slowpokeapi-3:8081
    networks:
      - slowpokeapi-net

  slowpokeapi-2:
    image: ghcr.io/e6qu/slowpokeapi:latest
    hostname: slowpokeapi-2
    ports:
      - "8082:8080"
      - "8083:8081"
    volumes:
      - slowpokeapi-data-2:/data
    environment:
      - SLOWPOKEAPI_SYNC_ENABLED=true
      - SLOWPOKEAPI_SYNC_STATIC_PEERS=slowpokeapi-1:8081,slowpokeapi-3:8081
    networks:
      - slowpokeapi-net

  slowpokeapi-3:
    image: ghcr.io/e6qu/slowpokeapi:latest
    hostname: slowpokeapi-3
    ports:
      - "8084:8080"
      - "8085:8081"
    volumes:
      - slowpokeapi-data-3:/data
    environment:
      - SLOWPOKEAPI_SYNC_ENABLED=true
      - SLOWPOKEAPI_SYNC_STATIC_PEERS=slowpokeapi-1:8081,slowpokeapi-2:8081
    networks:
      - slowpokeapi-net

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
    depends_on:
      - slowpokeapi-1
      - slowpokeapi-2
      - slowpokeapi-3
    networks:
      - slowpokeapi-net

volumes:
  slowpokeapi-data-1:
  slowpokeapi-data-2:
  slowpokeapi-data-3:

networks:
  slowpokeapi-net:
    driver: bridge
```

## Health Checks

### Dockerfile Health Check

```dockerfile
HEALTHCHECK --interval=10s --timeout=5s --start-period=5s --retries=3 \
    CMD ["/slowpokeapi", "health-check"]
```

### Manual Health Check

```bash
# Check if container is healthy
docker inspect --format='{{.State.Health.Status}}' slowpokeapi

# Get health check logs
docker inspect --format='{{json .State.Health}}' slowpokeapi | jq
```

## Security

### Container Security

```dockerfile
# Run as non-root user
USER nonroot:nonroot

# Read-only root filesystem
# (applied at runtime)
```

### Docker Run Security Options

```bash
docker run -d \
    --name slowpokeapi \
    --read-only \
    --cap-drop=ALL \
    --security-opt=no-new-privileges \
    -v slowpokeapi-data:/data \
    -p 8080:8080 \
    ghcr.io/e6qu/slowpokeapi:latest
```

### Podman Alternative

```bash
podman run -d \
    --name slowpokeapi \
    --read-only \
    --cap-drop=ALL \
    --security-opt=no-new-privileges \
    -v slowpokeapi-data:/data \
    -p 8080:8080 \
    ghcr.io/e6qu/slowpokeapi:latest
```

## Image Size

| Image | Size |
|-------|------|
| Release (distroless) | ~25 MB |
| Debug (debian-slim) | ~100 MB |

## Registry Publishing

### GitHub Container Registry

```bash
# Login
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin

# Tag and push
docker tag slowpokeapi:latest ghcr.io/e6qu/slowpokeapi:latest
docker push ghcr.io/e6qu/slowpokeapi:latest

# Push with version
docker tag slowpokeapi:latest ghcr.io/e6qu/slowpokeapi:1.0.0
docker push ghcr.io/e6qu/slowpokeapi:1.0.0
```

### Docker Hub

```bash
# Login
docker login

# Tag and push
docker tag slowpokeapi:latest e6qu/slowpokeapi:latest
docker push e6qu/slowpokeapi:latest
```

## CI/CD Pipeline

```yaml
name: Container Build

on:
  push:
    branches: [main]
    tags: ['v*']

jobs:
  build:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
      
      - name: Login to GHCR
        uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}
      
      - name: Extract metadata
        id: meta
        uses: docker/metadata-action@v5
        with:
          images: ghcr.io/${{ github.repository }}
      
      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          platforms: linux/amd64,linux/arm64
          push: true
          tags: ${{ steps.meta.outputs.tags }}
          labels: ${{ steps.meta.outputs.labels }}
          cache-from: type=gha
          cache-to: type=gha,mode=max
```
