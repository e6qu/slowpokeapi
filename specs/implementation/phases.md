# Implementation Phases & Tasks

## Overview

SlowPokeAPI will be implemented in phases, with each phase resulting in a pull request. Phases are sized to fit within ~100k tokens of LLM context.

---

## Phase 1: Project Foundation

**Goal:** Set up the Rust project structure with basic Axum server, configuration, and health endpoints.

### Tasks

| # | Task | Files |
|---|------|-------|
| 1.1 | Initialize Cargo project with dependencies | `Cargo.toml` |
| 1.2 | Create directory structure | `src/{main,config,server,handlers,models}/` |
| 1.3 | Implement configuration loading (env + file) | `src/config/mod.rs`, `src/config/settings.rs` |
| 1.4 | Set up tracing/logging | `src/logging.rs` |
| 1.5 | Create basic Axum router | `src/server/mod.rs`, `src/server/router.rs` |
| 1.6 | Implement AppState | `src/server/state.rs` |
| 1.7 | Add health endpoints (`/healthz`, `/readyz`, `/livez`) | `src/handlers/health.rs` |
| 1.8 | Add deep health check (`/health`) | `src/handlers/health.rs` |
| 1.9 | Create error types and responses | `src/models/error.rs` |
| 1.10 | Add basic tests | `src/handlers/health.rs` |

### Deliverables
- Binary that starts and responds to health checks
- Configuration via environment variables
- Basic logging

### Commands
```bash
cargo run
curl http://localhost:8080/healthz
curl http://localhost:8080/health
```

---

## Phase 2: OpenAPI & Swagger UI

**Goal:** Add OpenAPI schema generation and Swagger UI documentation.

### Tasks

| # | Task | Files |
|---|------|-------|
| 2.1 | Add utoipa dependencies | `Cargo.toml` |
| 2.2 | Create API response models with schemas | `src/models/api/response.rs` |
| 2.3 | Add OpenAPI annotations to health handlers | `src/handlers/health.rs` |
| 2.4 | Create OpenAPI router configuration | `src/server/router.rs` |
| 2.5 | Add Swagger UI endpoint | `src/server/router.rs` |
| 2.6 | Add OpenAPI spec JSON endpoint | `src/server/router.rs` |
| 2.7 | Test Swagger UI | `tests/openapi.rs` |

### Deliverables
- `/swagger-ui/` endpoint with interactive documentation
- `/api-docs/openapi.json` with full schema
- Health endpoints documented

### Commands
```bash
cargo run
open http://localhost:8080/swagger-ui/
curl http://localhost:8080/api-docs/openapi.json
```

---

## Phase 3: Prometheus Metrics

**Goal:** Add Prometheus metrics endpoint with standard metrics.

### Tasks

| # | Task | Files |
|---|------|-------|
| 3.1 | Add prometheus and axum-prometheus dependencies | `Cargo.toml` |
| 3.2 | Create metrics module | `src/metrics/mod.rs` |
| 3.3 | Define custom metrics | `src/metrics/definitions.rs` |
| 3.4 | Add Prometheus middleware layer | `src/server/router.rs` |
| 3.5 | Implement `/metrics` endpoint | `src/handlers/metrics.rs` |
| 3.6 | Add HTTP request metrics | `src/metrics/definitions.rs` |
| 3.7 | Test metrics output | `tests/metrics.rs` |

### Deliverables
- `/metrics` endpoint in Prometheus text format
- HTTP request count, latency, and in-flight metrics

### Commands
```bash
cargo run
curl http://localhost:8080/metrics
```

---

## Phase 4: SQLite Storage Layer

**Goal:** Implement SQLite database with migrations and repository pattern.

### Tasks

| # | Task | Files |
|---|------|-------|
| 4.1 | Add sqlx and dependencies | `Cargo.toml` |
| 4.2 | Create migrations directory | `migrations/001_initial.sql` |
| 4.3 | Create rates table migration | `migrations/002_rates.sql` |
| 4.4 | Create historical rates migration | `migrations/003_historical.sql` |
| 4.5 | Create sync state migration | `migrations/004_sync_state.sql` |
| 4.6 | Implement storage module | `src/storage/mod.rs` |
| 4.7 | Implement SQLite connection pool | `src/storage/sqlite.rs` |
| 4.8 | Create rates repository | `src/storage/repositories/rates.rs` |
| 4.9 | Create historical repository | `src/storage/repositories/historical.rs` |
| 4.10 | Add database health check | `src/storage/health.rs` |
| 4.11 | Test storage operations | `tests/storage.rs` |

### Deliverables
- SQLite database with migrations
- Repository traits and implementations
- Database health check integration

---

## Phase 5: Data Models

**Goal:** Implement all domain models and API request/response types.

### Tasks

| # | Task | Files |
|---|------|-------|
| 5.1 | Create Currency model | `src/models/currency.rs` |
| 5.2 | Create ExchangeRate model | `src/models/rate.rs` |
| 5.3 | Create HistoricalRate model | `src/models/historical.rs` |
| 5.4 | Create CurrencyMetadata model | `src/models/metadata.rs` |
| 5.5 | Create API response types | `src/models/api/response.rs` |
| 5.6 | Create error types with API mapping | `src/models/error.rs` |
| 5.7 | Add validation logic | `src/models/validation.rs` |
| 5.8 | Add OpenAPI schemas to all models | Various |
| 5.9 | Test model serialization | `tests/models.rs` |

### Deliverables
- All domain models with validation
- API request/response types
- OpenAPI schema annotations

---

## Phase 6: Cache Layer

**Goal:** Implement in-memory and SQLite caching.

### Tasks

| # | Task | Files |
|---|------|-------|
| 6.1 | Add moka cache dependency | `Cargo.toml` |
| 6.2 | Create cache module | `src/cache/mod.rs` |
| 6.3 | Define Cache trait | `src/cache/mod.rs` |
| 6.4 | Implement memory cache | `src/cache/memory.rs` |
| 6.5 | Implement SQLite cache | `src/cache/sqlite.rs` |
| 6.6 | Implement tiered cache | `src/cache/tiered.rs` |
| 6.7 | Add cache metrics | `src/cache/metrics.rs` |
| 6.8 | Integrate with AppState | `src/server/state.rs` |
| 6.9 | Test cache operations | `tests/cache.rs` |

### Deliverables
- Two-tier caching (memory → SQLite)
- TTL management
- Cache metrics

---

## Phase 7: Upstream API Clients

**Goal:** Implement HTTP clients for Frankfurter and fawazahmed0 APIs.

### Tasks

| # | Task | Files |
|---|------|-------|
| 7.1 | Add reqwest dependency | `Cargo.toml` |
| 7.2 | Create upstream module | `src/upstream/mod.rs` |
| 7.3 | Define Upstream trait | `src/upstream/mod.rs` |
| 7.4 | Create shared HTTP client | `src/upstream/client.rs` |
| 7.5 | Implement Frankfurter client | `src/upstream/frankfurter.rs` |
| 7.6 | Implement fawazahmed0 client | `src/upstream/fawaz.rs` |
| 7.7 | Create upstream manager with fallback | `src/upstream/manager.rs` |
| 7.8 | Add circuit breaker | `src/upstream/circuit_breaker.rs` |
| 7.9 | Add upstream metrics | `src/upstream/metrics.rs` |
| 7.10 | Test upstream clients | `tests/upstream.rs` |

### Deliverables
- HTTP clients for fiat currency APIs
- Fallback chain (Frankfurter → fawaz)
- Circuit breaker for fault tolerance

---

## Phase 8: Currencies Endpoint

**Goal:** Implement `/v1/currencies` and `/v1/currencies.min` endpoints.

### Tasks

| # | Task | Files |
|---|------|-------|
| 8.1 | Create currencies handler | `src/handlers/currencies.rs` |
| 8.2 | Implement list currencies | `src/handlers/currencies.rs` |
| 8.3 | Implement minimal currencies | `src/handlers/currencies.rs` |
| 8.4 | Add OpenAPI annotations | `src/handlers/currencies.rs` |
| 8.5 | Add route to router | `src/server/router.rs` |
| 8.6 | Seed currency data on startup | `src/bootstrap.rs` |
| 8.7 | Test endpoints | `tests/currencies.rs` |

### Deliverables
- `GET /v1/currencies` - Full currency list with names
- `GET /v1/currencies.min` - Currency codes only

---

## Phase 9: Latest Rates Endpoint

**Goal:** Implement `/v1/latest/{base_code}` endpoint.

### Tasks

| # | Task | Files |
|---|------|-------|
| 9.1 | Create latest handler | `src/handlers/latest.rs` |
| 9.2 | Implement rate fetching logic | `src/services/rates.rs` |
| 9.3 | Add cache integration | `src/services/rates.rs` |
| 9.4 | Add upstream fallback | `src/services/rates.rs` |
| 9.5 | Implement response formatting | `src/handlers/latest.rs` |
| 9.6 | Add input validation | `src/handlers/latest.rs` |
| 9.7 | Add OpenAPI annotations | `src/handlers/latest.rs` |
| 9.8 | Add route to router | `src/server/router.rs` |
| 9.9 | Test endpoint | `tests/latest.rs` |

### Deliverables
- `GET /v1/latest/{base_code}` - Latest exchange rates
- Cache-first with upstream fallback

---

## Phase 10: Pair Conversion Endpoint

**Goal:** Implement `/v1/pair/{base}/{target}/{amount}` endpoint.

### Tasks

| # | Task | Files |
|---|------|-------|
| 10.1 | Create pair handler | `src/handlers/pair.rs` |
| 10.2 | Implement rate lookup | `src/services/conversion.rs` |
| 10.3 | Implement conversion calculation | `src/services/conversion.rs` |
| 10.4 | Add response formatting | `src/handlers/pair.rs` |
| 10.5 | Add input validation | `src/handlers/pair.rs` |
| 10.6 | Add OpenAPI annotations | `src/handlers/pair.rs` |
| 10.7 | Add route to router | `src/server/router.rs` |
| 10.8 | Test endpoint | `tests/pair.rs` |

### Deliverables
- `GET /v1/pair/{base}/{target}` - Rate for pair
- `GET /v1/pair/{base}/{target}/{amount}` - Converted amount

---

## Phase 11: Historical Rates Endpoint

**Goal:** Implement `/v1/history/{base}/{year}/{month}/{day}` endpoint.

### Tasks

| # | Task | Files |
|---|------|-------|
| 11.1 | Create history handler | `src/handlers/history.rs` |
| 11.2 | Implement historical fetch from Frankfurter | `src/upstream/frankfurter.rs` |
| 11.3 | Add SQLite caching for historical data | `src/cache/sqlite.rs` |
| 11.4 | Implement response formatting | `src/handlers/history.rs` |
| 11.5 | Add date validation | `src/handlers/history.rs` |
| 11.6 | Add OpenAPI annotations | `src/handlers/history.rs` |
| 11.7 | Add route to router | `src/server/router.rs` |
| 11.8 | Test endpoint | `tests/history.rs` |

### Deliverables
- `GET /v1/history/{base}/{year}/{month}/{day}` - Historical rates
- Caching of historical data

---

## Phase 12: Enriched Endpoint

**Goal:** Implement `/v1/enriched/{base}/{target}` endpoint with metadata.

### Tasks

| # | Task | Files |
|---|------|-------|
| 12.1 | Create enriched handler | `src/handlers/enriched.rs` |
| 12.2 | Create currency metadata database | `src/storage/metadata.rs` |
| 12.3 | Seed metadata on startup | `src/bootstrap.rs` |
| 12.4 | Implement enriched response | `src/handlers/enriched.rs` |
| 12.5 | Add OpenAPI annotations | `src/handlers/enriched.rs` |
| 12.6 | Add route to router | `src/server/router.rs` |
| 12.7 | Test endpoint | `tests/enriched.rs` |

### Deliverables
- `GET /v1/enriched/{base}/{target}` - Rate with target currency metadata

---

## Phase 13: Cryptocurrency Support

**Goal:** Add CoinGecko and CoinCap upstream clients for crypto/metal rates.

### Tasks

| # | Task | Files |
|---|------|-------|
| 13.1 | Implement CoinGecko client | `src/upstream/coingecko.rs` |
| 13.2 | Implement CoinCap client | `src/upstream/coincap.rs` |
| 13.3 | Add crypto/metal currency codes | `src/models/currency.rs` |
| 13.4 | Implement cross-rate calculation | `src/services/conversion.rs` |
| 13.5 | Update upstream manager | `src/upstream/manager.rs` |
| 13.6 | Add crypto support to latest endpoint | `src/handlers/latest.rs` |
| 13.7 | Test crypto conversions | `tests/crypto.rs` |

### Deliverables
- BTC, ETH, and other crypto support
- XAU, XAG precious metal support
- Cross-rate calculation (fiat ↔ crypto)

---

## Phase 14: CRDT Sync Engine

**Goal:** Implement automerge-based sync between replicas.

### Tasks

| # | Task | Files |
|---|------|-------|
| 14.1 | Add automerge dependency | `Cargo.toml` |
| 14.2 | Create sync module | `src/sync/mod.rs` |
| 14.3 | Define SyncEngine trait | `src/sync/mod.rs` |
| 14.4 | Implement CRDT document wrapper | `src/sync/crdt.rs` |
| 14.5 | Implement gossip protocol | `src/sync/gossip.rs` |
| 14.6 | Implement peer discovery | `src/sync/peer.rs` |
| 14.7 | Implement WebSocket transport | `src/sync/transport.rs` |
| 14.8 | Integrate with storage layer | `src/sync/storage.rs` |
| 14.9 | Add sync metrics | `src/sync/metrics.rs` |
| 14.10 | Test sync between replicas | `tests/sync.rs` |

### Deliverables
- CRDT-based state management
- Gossip protocol for peer sync
- WebSocket transport

---

## Phase 15: Sync Integration

**Goal:** Integrate sync engine with cache and storage layers.

### Tasks

| # | Task | Files |
|---|------|-------|
| 15.1 | Hook cache updates to sync | `src/cache/tiered.rs` |
| 15.2 | Hook sync updates to cache | `src/sync/integration.rs` |
| 15.3 | Implement reconciliation | `src/sync/reconciliation.rs` |
| 15.4 | Add sync configuration | `src/config/settings.rs` |
| 15.5 | Update health check for sync | `src/handlers/health.rs` |
| 15.6 | Test full sync flow | `tests/integration.rs` |

### Deliverables
- Automatic sync on rate updates
- Reconciliation between SQLite and CRDT

---

## Phase 16: Rate Limiting & Quota

**Goal:** Implement per-API-key rate limiting and quota tracking.

### Tasks

| # | Task | Files |
|---|------|-------|
| 16.1 | Create rate limit module | `src/ratelimit/mod.rs` |
| 16.2 | Implement token bucket | `src/ratelimit/token_bucket.rs` |
| 16.3 | Implement API key store | `src/storage/api_keys.rs` |
| 16.4 | Create rate limit middleware | `src/server/middleware/ratelimit.rs` |
| 16.5 | Implement `/v1/quota` endpoint | `src/handlers/quota.rs` |
| 16.6 | Add rate limit headers | `src/server/middleware/ratelimit.rs` |
| 16.7 | Add OpenAPI annotations | `src/handlers/quota.rs` |
| 16.8 | Test rate limiting | `tests/ratelimit.rs` |

### Deliverables
- Rate limiting middleware
- `GET /v1/quota` endpoint
- Rate limit headers

---

## Phase 17: Authentication

**Goal:** Implement API key authentication.

### Tasks

| # | Task | Files |
|---|------|-------|
| 17.1 | Create auth module | `src/auth/mod.rs` |
| 17.2 | Implement API key validation | `src/auth/api_key.rs` |
| 17.3 | Create auth middleware | `src/server/middleware/auth.rs` |
| 17.4 | Support header and query auth | `src/auth/api_key.rs` |
| 17.5 | Add auth configuration | `src/config/settings.rs` |
| 17.6 | Update OpenAPI for auth | Various |
| 17.7 | Test authentication | `tests/auth.rs` |

### Deliverables
- Bearer token authentication
- Query parameter authentication
- Configurable auth requirement

---

## Phase 18: Docker & Container

**Goal:** Create multi-stage Dockerfile and container configuration.

### Tasks

| # | Task | Files |
|---|------|-------|
| 18.1 | Create multi-stage Dockerfile | `Dockerfile` |
| 18.2 | Create debug Dockerfile | `Dockerfile.debug` |
| 18.3 | Create docker-compose.yml | `docker-compose.yml` |
| 18.4 | Create docker-compose cluster | `docker-compose.cluster.yml` |
| 18.5 | Add .dockerignore | `.dockerignore` |
| 18.6 | Create container entrypoint script | `docker/entrypoint.sh` |
| 18.7 | Test container build and run | - |

### Deliverables
- Production-ready container image
- Docker Compose for local development
- Multi-replica cluster setup

---

## Phase 19: Helm Chart

**Goal:** Create Helm chart for Kubernetes deployment.

### Tasks

| # | Task | Files |
|---|------|-------|
| 19.1 | Create Helm chart structure | `deploy/helm/slowpokeapi/` |
| 19.2 | Create Chart.yaml | `deploy/helm/slowpokeapi/Chart.yaml` |
| 19.3 | Create values.yaml | `deploy/helm/slowpokeapi/values.yaml` |
| 19.4 | Create deployment template | `deploy/helm/slowpokeapi/templates/deployment.yaml` |
| 19.5 | Create service template | `deploy/helm/slowpokeapi/templates/service.yaml` |
| 19.6 | Create configmap template | `deploy/helm/slowpokeapi/templates/configmap.yaml` |
| 19.7 | Create ingress template | `deploy/helm/slowpokeapi/templates/ingress.yaml` |
| 19.8 | Create statefulset template | `deploy/helm/slowpokeapi/templates/statefulset.yaml` |
| 19.9 | Create HPA template | `deploy/helm/slowpokeapi/templates/hpa.yaml` |
| 19.10 | Create ServiceMonitor template | `deploy/helm/slowpokeapi/templates/servicemonitor.yaml` |
| 19.11 | Create values-prod.yaml | `deploy/helm/slowpokeapi/values-prod.yaml` |
| 19.12 | Test helm template rendering | - |

### Deliverables
- Complete Helm chart
- Production values file
- StatefulSet support with PVC

---

## Phase 20: Terraform ECS

**Goal:** Create Terraform configuration for AWS ECS deployment.

### Tasks

| # | Task | Files |
|---|------|-------|
| 20.1 | Create Terraform structure | `deploy/terraform/` |
| 20.2 | Create versions.tf | `deploy/terraform/versions.tf` |
| 20.3 | Create variables.tf | `deploy/terraform/variables.tf` |
| 20.4 | Create vpc.tf | `deploy/terraform/vpc.tf` |
| 20.5 | Create security.tf | `deploy/terraform/security.tf` |
| 20.6 | Create alb.tf | `deploy/terraform/alb.tf` |
| 20.7 | Create ecs.tf | `deploy/terraform/ecs.tf` |
| 20.8 | Create efs.tf | `deploy/terraform/efs.tf` |
| 20.9 | Create autoscaling.tf | `deploy/terraform/autoscaling.tf` |
| 20.10 | Create cloudwatch.tf | `deploy/terraform/cloudwatch.tf` |
| 20.11 | Create outputs.tf | `deploy/terraform/outputs.tf` |
| 20.12 | Create prod.tfvars | `deploy/terraform/environments/prod.tfvars` |
| 20.13 | Test terraform plan | - |

### Deliverables
- Complete Terraform configuration
- ECS Fargate deployment
- ALB with HTTPS
- Optional EFS persistence
- Auto-scaling configuration

---

## Phase 21: CI/CD Pipeline

**Goal:** Set up GitHub Actions for build, test, and release.

### Tasks

| # | Task | Files |
|---|------|-------|
| 21.1 | Create CI workflow | `.github/workflows/ci.yml` |
| 21.2 | Add lint job (clippy, fmt) | `.github/workflows/ci.yml` |
| 21.3 | Add test job | `.github/workflows/ci.yml` |
| 21.4 | Add security audit job | `.github/workflows/ci.yml` |
| 21.5 | Create release workflow | `.github/workflows/release.yml` |
| 21.6 | Add binary build matrix | `.github/workflows/release.yml` |
| 21.7 | Add container build and push | `.github/workflows/release.yml` |
| 21.8 | Add Helm chart publish | `.github/workflows/release.yml` |
| 21.9 | Create dependabot config | `.github/dependabot.yml` |
| 21.10 | Test CI pipeline | - |

### Deliverables
- Automated CI on PRs
- Release workflow for binaries, containers, and Helm

---

## Phase 22: Documentation & Final Polish

**Goal:** Complete documentation and final integration tests.

### Tasks

| # | Task | Files |
|---|------|-------|
| 22.1 | Update README.md | `README.md` |
| 22.2 | Create DEPLOYMENT.md | `docs/DEPLOYMENT.md` |
| 22.3 | Create API.md | `docs/API.md` |
| 22.4 | Create CHANGELOG.md | `CHANGELOG.md` |
| 22.5 | Add inline code documentation | Various |
| 22.6 | Create Grafana dashboard JSON | `deploy/grafana/dashboard.json` |
| 22.7 | Create Prometheus alerts | `deploy/prometheus/alerts.yml` |
| 22.8 | Add end-to-end tests | `tests/e2e.rs` |
| 22.9 | Performance testing | `benches/` |
| 22.10 | Security review | - |

### Deliverables
- Complete documentation
- Grafana dashboard
- Prometheus alerts
- E2E tests

---

## Phase Summary

| Phase | Name | PR Scope |
|-------|------|----------|
| 1 | Project Foundation | Basic server, health, config |
| 2 | OpenAPI & Swagger | Documentation UI |
| 3 | Prometheus Metrics | Monitoring endpoint |
| 4 | SQLite Storage | Database layer |
| 5 | Data Models | Domain types |
| 6 | Cache Layer | Memory + SQLite cache |
| 7 | Upstream Clients | HTTP clients for APIs |
| 8 | Currencies Endpoint | First API endpoint |
| 9 | Latest Rates | Core functionality |
| 10 | Pair Conversion | Conversion logic |
| 11 | Historical Rates | Historical data |
| 12 | Enriched Endpoint | Metadata support |
| 13 | Cryptocurrency | Crypto/metal support |
| 14 | CRDT Sync Engine | Distributed sync |
| 15 | Sync Integration | Full sync flow |
| 16 | Rate Limiting | Quota management |
| 17 | Authentication | API keys |
| 18 | Docker & Container | Containerization |
| 19 | Helm Chart | Kubernetes deployment |
| 20 | Terraform ECS | AWS deployment |
| 21 | CI/CD Pipeline | Automation |
| 22 | Documentation | Final polish |
