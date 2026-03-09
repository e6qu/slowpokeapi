# SlowPokeAPI

A high-performance currency exchange rate API built in Rust, compatible with RapidAPI's exchange rate API specification.

[![CI](https://github.com/e6qu/slowpokeapi/actions/workflows/ci.yml/badge.svg)](https://github.com/e6qu/slowpokeapi/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-AGPL--3.0%20or%20MIT-blue.svg)](LICENSE)

## Features

- **Multi-Source Data**: Aggregates rates from Frankfurter, fawazahmed0, CoinGecko, and CoinCap
- **Cryptocurrency Support**: BTC, ETH, and 13+ other cryptocurrencies
- **Precious Metals**: XAU (gold), XAG (silver), XPT (platinum), XPD (palladium)
- **Distributed Sync**: CRDT-based SQLite synchronization across instances
- **Production Ready**: Prometheus metrics, health checks, rate limiting, authentication
- **Multiple Deployments**: Binary, Docker, Kubernetes (Helm), AWS ECS (Terraform)

## Quick Start

### Using Docker

```bash
docker run -p 8081:8081 -p 8082:8082 ghcr.io/e6qu/slowpokeapi:latest
```

### Using Docker Compose

```bash
docker-compose up -d
```

### Building from Source

```bash
git clone https://github.com/e6qu/slowpokeapi.git
cd slowpokeapi
cargo build --release
./target/release/slowpokeapi
```

## API Overview

| Endpoint | Description |
|----------|-------------|
| `GET /v1/currencies` | List all supported currencies |
| `GET /v1/latest/{base_code}` | Latest rates for base currency |
| `GET /v1/pair/{base}/{target}` | Convert between two currencies |
| `GET /v1/history/{base}/{y}/{m}/{d}` | Historical rates for date |
| `GET /v1/enriched/{base_code}` | Rates with metadata |
| `GET /health` | Health check |
| `GET /metrics` | Prometheus metrics |

### Example Request

```bash
curl http://localhost:8081/v1/latest/USD
```

### Example Response

```json
{
  "base_code": "USD",
  "conversion_rates": {
    "EUR": 0.85,
    "GBP": 0.73,
    "JPY": 110.0
  },
  "documentation": "https://github.com/e6qu/slowpokeapi",
  "result": "success",
  "time_last_update_unix": 1640995200,
  "time_next_update_unix": 1641081600
}
```

## Configuration

Configuration is via environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `API_PORT` | 8081 | HTTP API port |
| `SYNC_PORT` | 8082 | Sync protocol port |
| `DATABASE_URL` | sqlite:data/rates.db | SQLite database path |
| `RATE_LIMIT_ENABLED` | true | Enable rate limiting |
| `AUTH_ENABLED` | false | Enable API key authentication |

See [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) for complete configuration reference.

## Deployment Options

- **Binary**: Single static binary with embedded migrations
- **Docker**: Multi-platform images (linux/amd64, linux/arm64)
- **Docker Compose**: Complete stack with monitoring
- **Kubernetes**: Production Helm chart with HPA, PDB, ServiceMonitor
- **AWS ECS**: Terraform modules with Fargate, ALB, EFS, auto-scaling

See [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) for detailed deployment guides.

## Development

### Prerequisites

- Rust 1.75+
- SQLite 3
- Docker (optional)

### Running Tests

```bash
cargo test --all-features
```

### Running Lints

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Client    │────▶│  Axum HTTP  │────▶│   Handler   │
└─────────────┘     └─────────────┘     └──────┬──────┘
                                               │
                         ┌─────────────────────┼─────────────────────┐
                         ▼                     ▼                     ▼
                   ┌──────────┐         ┌──────────┐          ┌──────────┐
                   │  Cache   │         │   Auth   │          │ Metrics  │
                   │ (SQLite) │         │Middleware│          │(Prometheus)
                   └────┬─────┘         └──────────┘          └──────────┘
                        │
                        ▼
                 ┌────────────┐
                 │   CRDT     │◀────▶ Peer sync (port 8082)
                 │   Sync     │
                 └─────┬──────┘
                       ▼
              ┌─────────────────┐
              │ Upstream Manager │
              └────────┬────────┘
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
   ┌─────────┐   ┌──────────┐   ┌──────────┐
   │Frankfurter│   │CoinGecko │   │ CoinCap  │
   │ fawazahmed0│   │          │   │          │
   └─────────┘   └──────────┘   └──────────┘
```

## Documentation

- [Deployment Guide](docs/DEPLOYMENT.md) - Installation and deployment options
- [API Reference](docs/API.md) - Complete API documentation
- [CHANGELOG.md](CHANGELOG.md) - Version history

## Monitoring

- **Metrics**: Prometheus metrics at `/metrics`
- **Health**: Health check at `/health`
- **Grafana**: Dashboard template in `deploy/grafana/`
- **Alerts**: Prometheus alert rules in `deploy/prometheus/`

## License

Dual-licensed under AGPL-3.0 or MIT at your option. See [LICENSE](LICENSE) for details.

## About

This project is a clean-room reimplementation developed independently based solely on publicly available information and specifications. It is not affiliated with or endorsed by RapidAPI.
