# Do Next
## Phase 18: Docker & Container
### Goal

Create multi-stage Dockerfile and container configuration.

### Tasks

| #  | Task | Files | Status |
|---|------|-------|--------|
| 18.1 | Create multi-stage Dockerfile | `Dockerfile` | Pending |
| 18.2 | Create debug Dockerfile | `Dockerfile.debug` | Pending |
| 18.3 | Create docker-compose.yml | `docker-compose.yml` | Pending |
| 18.4 | Create docker-compose cluster | `docker-compose.cluster.yml` | Pending |
| 18.5 | Add .dockerignore | `.dockerignore` | Pending |
| 18.6 | Create container entrypoint script | `docker/entrypoint.sh` | Pending |
| 18.7 | Test container build and run | - | Pending |

### Deliverables

- Production-ready container image
- Docker Compose for local development
- Multi-replica cluster setup

### Acceptance Criteria
- [ ] Multi-stage Dockerfile created
- [ ] Docker Compose files created
- [ ] Container builds successfully
- [ ] Container runs correctly
- [ ] OpenAPI documentation updated
- [ ] Tests pass
- [ ] Clippy passes with no warnings
- [ ] Format check passes
- [ ] CI passes

### Verification commands
```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
docker build -t slowpokeapi:test .
docker run -p 8080:8080 slowpokeapi:test
```

### After completion
1. Update PLAN.md - Mark Phase 18 complete
2. Update STATUS.md - Move to Phase 19
3. Update WHAT_WE_DID.md - document Phase 18
4. Update DO_NEXT.md - set up Phase 19 tasks
5. Create feature branch for Phase 19
6. Create PR
7. Ensure CI passes
