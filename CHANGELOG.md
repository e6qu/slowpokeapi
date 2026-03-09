# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [1.0.0] - 2024-03-04

### Added

#### Core API
- **Currencies endpoint** (`GET /v1/currencies`) - List all supported currencies with names
- **Latest rates endpoint** (`GET /v1/latest/{base_code}`) - Get current exchange rates for any base currency
- **Pair conversion endpoint** (`GET /v1/pair/{base_code}/{target_code}`) - Convert between two currencies with optional amount
- **Historical rates endpoint** (`GET /v1/history/{base_code}/{year}/{month}/{day}`) - Get rates for any date back to 1999-01-04
- **Enriched endpoint** (`GET /v1/enriched/{base_code}`) - Get rates with currency metadata

#### Cryptocurrency Support
- CoinGecko API client for crypto rates
- CoinCap API client for crypto rates
- Support for 15 cryptocurrencies: BTC, ETH, BNB, XRP, ADA, SOL, DOT, DOGE, AVAX, MATIC, LINK, UNI, LTC, BCH, XLM
- Support for 4 precious metals: XAU (Gold), XAG (Silver), XPT (Platinum), XPD (Palladium)
- Automatic routing between fiat and crypto upstreams

#### Data Layer
- SQLite storage with SQLx migrations
- In-memory cache with TTL support
- Cache metrics tracking (hits, misses, sets, deletes)
- CRDT-based distributed synchronization
- Automatic peer discovery and state reconciliation

#### Resilience
- Circuit breaker pattern for upstream API protection
- Automatic fallback between multiple upstream sources
- Exponential backoff with jitter for rate-limited clients
- Health checks at `/health`, `/healthz`, `/readyz`, `/livez`

#### Rate Limiting & Quota
- Multi-tier rate limiting (global, authenticated, anonymous)
- Token bucket algorithm with automatic refill
- Backpressure detection at 80% utilization
- Configurable via environment variables
- Quota endpoint (`GET /v1/quota`) for checking limits

#### Authentication
- Optional API key authentication
- Configurable public paths
- API key management via storage layer

#### Observability
- Prometheus metrics at `/metrics`
- Structured logging with tracing
- Request duration and rate tracking
- Sync operation metrics
- OpenAPI/Swagger UI documentation

#### Deployment Options
- Single static binary
- Docker multi-platform images (linux/amd64, linux/arm64)
- Docker Compose stack
- Helm chart for Kubernetes with:
  - Deployment and StatefulSet options
  - Horizontal Pod Autoscaling
  - Pod Disruption Budgets
  - ServiceMonitor for Prometheus
  - Ingress support
- Terraform modules for AWS ECS with:
  - Fargate launch type
  - Application Load Balancer
  - EFS for persistence
  - Auto-scaling based on CPU/memory/request count
  - CloudWatch alarms and dashboard
  - Route53 DNS integration

#### CI/CD
- GitHub Actions CI workflow with:
  - Format, clippy, and doc checks
  - Test execution
  - Security audit with cargo-audit
  - Code coverage with cargo-tarpaulin
  - Container build test
  - Helm lint and template test
- Release workflow with:
  - Multi-platform binary builds
  - Docker image publishing to GHCR
  - Helm chart publishing
  - GitHub release creation
- Dependabot configuration for automated dependency updates

### Security
- Rate limiting with safety factor (0.5x published limits)
- API key validation middleware
- No secrets in container images
- Non-root container execution
- Security headers on HTTP responses

### Documentation
- Comprehensive README with quick start guide
- Deployment guide covering all deployment options
- Complete API reference with examples
- This CHANGELOG
- OpenAPI specification
- Architecture diagram

### Bug Fixes (Phase 22)
- Fixed API key exposure in quota response (now masked)
- Fixed metal currency routing with clear error message
- Added NaN/Infinity validation for amount parameter
- Removed circuit breaker Clone implementation (panic risk)
- Changed unwrap to expect for date construction
- Added logging for Frankfurter date fallback
- Changed enriched cache key prefix to avoid collision
- Fixed pair/enriched returning 404 for unknown currency
- Fixed self-to-self rate queries (now rejected with 400)
- Added minimum date validation for historical rates (1999-01-04)
- Fixed sync metrics not appearing in `/metrics`
- Fixed cache metrics integration
- Fixed duplicate code in history handler

## Upstream Sources

This project aggregates data from the following public APIs:

- **Frankfurter** (https://api.frankfurter.app/) - European Central Bank reference rates
- **fawazahmed0** (https://github.com/fawazahmed0/exchange-api) - Community exchange rate API
- **CoinGecko** (https://www.coingecko.com/en/api) - Cryptocurrency prices
- **CoinCap** (https://coincap.io/) - Cryptocurrency market data

## License

Dual-licensed under AGPL-3.0 or MIT at your option.
