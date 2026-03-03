# Do Next
## Phase 21: CI/CD Pipeline
### Goal

Set up GitHub Actions for build, test, and release.

### Tasks

| #  | Task | Files | Status |
|---|------|-------|--------|
| 21.1 | Create CI workflow | `.github/workflows/ci.yml` | Pending |
| 21.2 | Add lint job (clippy, fmt) | `.github/workflows/ci.yml` | Pending |
| 21.3 | Add test job | `.github/workflows/ci.yml` | Pending |
| 21.4 | Add security audit job | `.github/workflows/ci.yml` | Pending |
| 21.5 | Create release workflow | `.github/workflows/release.yml` | Pending |
| 21.6 | Add binary build matrix | `.github/workflows/release.yml` | Pending |
| 21.7 | Add container build and push | `.github/workflows/release.yml` | Pending |
| 21.8 | Add Helm chart publish | `.github/workflows/release.yml` | Pending |
| 21.9 | Create dependabot config | `.github/dependabot.yml` | Pending |
| 21.10 | Test CI pipeline | - | Pending |

### Deliverables

- Automated CI on PRs
- Release workflow for binaries, containers, and Helm

### Acceptance Criteria
- [ ] CI workflow created
- [ ] Lint job passes
- [ ] Test job passes
- [ ] Security audit job passes
- [ ] Release workflow created
- [ ] Binary builds work
- [ ] Container builds work
- [ ] Helm chart publishes

### Verification commands
```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

### After completion
1. Update PLAN.md - Mark Phase 21 complete
2. Update STATUS.md - Move to Phase 22
3. Update WHAT_WE_DID.md - document Phase 21
4. Update DO_NEXT.md - set up Phase 22 tasks
5. Create feature branch for Phase 22
6. Create PR
7. Ensure CI passes
