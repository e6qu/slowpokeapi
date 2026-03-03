# SlowPokeAPI Development Plan

## Project Goal

Build a Rust-based currency exchange rate API (RapidAPI-compatible) with:
- Distributed SQLite storage with CRDT-based synchronization
- Public upstream APIs only (Frankfurter, fawazahmed0, CoinGecko, CoinCap)
- Full observability (OpenAPI, Prometheus metrics, health checks)
- Multiple deployment targets (binary, container, Helm, Terraform ECS)

## Implementation Strategy

Implement in **22 phases**, each resulting in a pull request. Each phase must:
1. Fit within ~100k tokens of LLM context
2. Update the 4 crucial files (PLAN.md, STATUS.md, WHAT_WE_DID.md, DO_NEXT.md)
3. Pass all CI checks
4. Match the specification in `specs/`

## Phase Overview

| Phase | Name | Status |
|-------|------|--------|
| 0 | Specs & Workflow Setup | ✅ Complete |
| 1 | Project Foundation | ✅ Complete |
| 2 | OpenAPI & Swagger UI | ✅ Complete |
| 3 | Prometheus Metrics | ✅ Complete |
| 4 | SQLite Storage Layer | ✅ Complete |
| 5 | Data Models | ✅ Complete |
| 6 | Cache Layer | ✅ Complete |
| 7 | Upstream API Clients | ✅ Complete |
| 8 | Currencies Endpoint | ✅ Complete |
| 9 | Latest Rates Endpoint | ✅ Complete |
| 10 | Pair Conversion Endpoint | ✅ Complete |
| 11 | Historical Rates Endpoint | 🔄 In Progress |
| 12 | Enriched Endpoint | Pending |
| 13 | Cryptocurrency Support | Pending |
| 14 | CRDT Sync Engine | Pending |
| 15 | Sync Integration | Pending |
| 16 | Rate Limiting & Quota | Pending |
| 17 | Authentication | Pending |
| 18 | Docker & Container | Pending |
| 19 | Helm Chart | Pending |
| 20 | Terraform ECS | Pending |
| 21 | CI/CD Pipeline | Pending |
| 22 | Documentation & Final Polish | Pending |

## Current Phase

**Phase 11: Historical Rates Endpoint** (In Progress)

See `DO_NEXT.md` for tasks.
