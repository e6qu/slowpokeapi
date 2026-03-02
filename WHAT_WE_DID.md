# What We Did

## 2026-03-02: Project Setup

### Specifications Created

Created comprehensive specifications in `specs/`:

1. **Architecture** (`specs/architecture/`)
   - `README.md` - High-level architecture with ASCII diagrams
   - `components.md` - Component design, interfaces, dependency graph
   - `sync.md` - CRDT synchronization using automerge-rs with gossip protocol

2. **API** (`specs/api/`)
   - `openapi.yaml` - Full OpenAPI 3.0.3 schema for all endpoints
   - `health.md` - Kubernetes health probes specification
   - `metrics.md` - Prometheus metrics specification

3. **Data** (`specs/data/`)
   - `models.md` - Domain models and API request/response types
   - `storage.md` - SQLite schema, migrations, and repository pattern

4. **Deployment** (`specs/deployment/`)
   - `binary.md` - Binary build specification
   - `container.md` - Docker multi-stage build and compose files
   - `helm.md` - Kubernetes Helm chart specification
   - `terraform.md` - AWS ECS Fargate deployment specification

5. **Implementation** (`specs/implementation/`)
   - `phases.md` - 22 implementation phases with detailed task breakdown

### Development Workflow

- Created `AGENTS.md` with AI agent workflow instructions
- Set up task management structure (`tasks/` and `tasks/done/`)
- Created crucial tracking files: PLAN.md, STATUS.md, WHAT_WE_DID.md, DO_NEXT.md
- Added CI workflow with GitHub Actions
- Added lint configuration (rustfmt.toml, clippy.toml)
- Added .gitignore for Rust project

### Repository Setup

- Committed initial project setup
- Note: Push to origin requires SSH key configuration

### Technology Decisions

- **Framework:** Axum (web framework)
- **CRDT:** automerge-rs for distributed sync
- **Database:** SQLite with rusqlite/sqlx
- **Cache:** moka (in-memory) + SQLite (persistent)
- **Upstream APIs:** Frankfurter (primary), fawazahmed0 (fallback), CoinGecko, CoinCap
- **Metrics:** prometheus + axum-prometheus
- **OpenAPI:** utoipa + utoipa-swagger-ui
