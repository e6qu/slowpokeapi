# SlowPokeAPI

Currency exchange rate API built in Rust.

[![CI](https://github.com/e6qu/slowpokeapi/actions/workflows/ci.yml/badge.svg)](https://github.com/e6qu/slowpokeapi/actions/workflows/ci.yml)
[![Test](https://img.shields.io/github/actions/workflow/status/e6qu/slowpokeapi/ci.yml?logo=github&label=test&branch=main)](https://github.com/e6qu/slowpokeapi/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/github/actions/workflow/status/e6qu/slowpokeapi/ci.yml?logo=codecov&label=coverage&branch=main)](https://github.com/e6qu/slowpokeapi/actions/workflows/ci.yml)
[![Security](https://img.shields.io/github/actions/workflow/status/e6qu/slowpokeapi/ci.yml?logo=shield&label=security&branch=main)](https://github.com/e6qu/slowpokeapi/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-AGPL--3.0%20or%20MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

**Lines of Code:** ~15,000 | **Binary Size:** ~8MB

## Features

- **Multi-Source Data**: Aggregates rates from Frankfurter, fawazahmed0, CoinGecko, and CoinCap
- **Cryptocurrency Support**: BTC, ETH, and 15 other cryptocurrencies
- **Precious Metals**: XAU (gold), XAG (silver), XPT (platinum), XPD (palladium)
- **Caching**: Tiered cache (in-memory + SQLite) with configurable TTL
- **Observability**: Prometheus metrics, health checks, distributed tracing
- **Rate Limiting**: Token bucket rate limiting per client
- **Authentication**: Optional API key authentication

## Quick Start

### Using Docker

```bash
docker run -p 8080:8080 ghcr.io/e6qu/slowpokeapi:latest
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
| `GET /v1/enriched/{base}/{target}` | Rate with target metadata |
| `GET /health` | Health check |
| `GET /metrics` | Prometheus metrics |

### Example Request

```bash
curl http://localhost:8080/v1/latest/USD
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
  "time_next_update_unix": 1641081600,
  "data_source": {
    "source": "frankfurter",
    "last_retrieved": "2024-01-01T00:00:00Z",
    "last_cached": null,
    "upstream_request": {
      "endpoint": "https://api.frankfurter.app/latest?from=USD"
    }
  }
}
```

## Configuration

Configuration is via environment variables with the prefix `SLOWPOKEAPI__`:

| Variable | Default | Description |
|----------|---------|-------------|
| `SLOWPOKEAPI__SERVER__PORT` | 8080 | HTTP port |
| `SLOWPOKEAPI__DATABASE__URL` | sqlite::memory: | SQLite database path |
| `SLOWPOKEAPI__CACHE__TTL_SECONDS` | 3600 | Cache TTL in seconds |
| `SLOWPOKEAPI__RATE_LIMIT__ENABLED` | true | Enable rate limiting |
| `SLOWPOKEAPI__AUTH__ENABLED` | false | Enable API key authentication |

See [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) for complete configuration reference.

## Deployment Options

- **Binary**: Single static binary
- **Docker**: Multi-platform images
- **Docker Compose**: Stack with Prometheus and Grafana
- **Kubernetes**: Helm chart
- **AWS ECS**: Terraform modules

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
                    ┌─────────────┐
                    │   Client    │
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │  Axum HTTP  │
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │   Handler   │
                    └──────┬──────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
        ▼                  ▼                  ▼
 ┌─────────────┐   ┌─────────────┐   ┌─────────────┐
 │  Tiered     │   │ Rate Limiter│   │ Auth/Key    │
 │ Cache       │   │             │   │ Storage     │
 │(Mem+SQLite) │   │             │   │             │
 └──────┬──────┘   └─────────────┘   └─────────────┘
        │
        ▼
 ┌─────────────┐
 │  Upstream   │
 │  Manager    │
 └──────┬──────┘
        │
   ┌────┴────┬────────┐
   │         │        │
   ▼         ▼        ▼
┌──────┐ ┌──────┐ ┌──────┐
│Frank-│ │Coin- │ │Coin- │
│furter│ │Gecko │ │Cap   │
└──────┘ └──────┘ └──────┘
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

This project is a reimplementation developed independently based on publicly available information. It is not affiliated with or endorsed by RapidAPI.
