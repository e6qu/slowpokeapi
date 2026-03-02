# SlowPokeAPI Specifications

## Overview

SlowPokeAPI is a Rust-based currency exchange rate API with distributed SQLite storage and CRDT-based synchronization.

## Design Principles

1. **Public APIs Only**: No proprietary, paid, or rate-limited "free tier" upstream APIs
2. **Eventual Consistency**: Accept eventual consistency for distributed SQLite replicas
3. **Stateless Service**: Service is stateless; SQLite acts as distributed cache
4. **High Availability**: Support multiple replicas with automatic synchronization
5. **Observability**: Full OpenAPI docs, Prometheus metrics, structured logging

## Specification Index

### Architecture
- [Architecture Overview](architecture/README.md)
- [Component Design](architecture/components.md)
- [CRDT Synchronization](architecture/sync.md)

### API
- [OpenAPI Schema](api/openapi.yaml)
- [Health Endpoints](api/health.md)
- [Prometheus Metrics](api/metrics.md)

### Data
- [Data Models](data/models.md)
- [SQLite Schema](data/storage.md)

### Deployment
- [Binary Build](deployment/binary.md)
- [Container/Docker](deployment/container.md)
- [Helm Chart](deployment/helm.md)
- [Terraform ECS](deployment/terraform.md)

### Implementation
- [Phases & Tasks](implementation/phases.md)

## Technology Stack

| Layer | Technology |
|-------|------------|
| Language | Rust (edition 2021) |
| Web Framework | Axum 0.7+ |
| Async Runtime | Tokio 1.x |
| Serialization | serde, serde_json |
| Database | SQLite via rusqlite/sqlx |
| CRDT | automerge-rs |
| HTTP Client | reqwest |
| OpenAPI | utoipa, utoipa-swagger-ui |
| Metrics | prometheus, axum-prometheus |
| Container | Docker (distroless) |
| Orchestration | Kubernetes / AWS ECS |

## Upstream Data Sources

| Source | Type | Auth | Notes |
|--------|------|------|-------|
| Frankfurter API | Fiat | None | Primary - ECB-backed |
| fawazahmed0/currency-api | Fiat | None | Fallback |
| CoinGecko | Crypto/Metals | None | Primary crypto |
| CoinCap | Crypto | None | Fallback |

## Non-Goals

- Real-time streaming rates (polling-based only)
- Payment processing
- User accounts with sensitive data
- High-frequency trading support
