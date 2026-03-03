# Do Next
## Phase 19: Helm Chart
### Goal

Create Helm chart for Kubernetes deployment.

### Tasks

| #  | Task | Files | Status |
|---|------|-------|--------|
| 19.1 | Create Helm chart structure | `deploy/helm/slowpokeapi/` | Pending |
| 19.2 | Create Chart.yaml | `deploy/helm/slowpokeapi/Chart.yaml` | Pending |
| 19.3 | Create values.yaml | `deploy/helm/slowpokeapi/values.yaml` | Pending |
| 19.4 | Create deployment template | `deploy/helm/slowpokeapi/templates/deployment.yaml` | Pending |
| 19.5 | Create service template | `deploy/helm/slowpokeapi/templates/service.yaml` | Pending |
| 19.6 | Create configmap template | `deploy/helm/slowpokeapi/templates/configmap.yaml` | Pending |
| 19.7 | Create ingress template | `deploy/helm/slowpokeapi/templates/ingress.yaml` | Pending |
| 19.8 | Create statefulset template | `deploy/helm/slowpokeapi/templates/statefulset.yaml` | Pending |
| 19.9 | Create HPA template | `deploy/helm/slowpokeapi/templates/hpa.yaml` | Pending |
| 19.10 | Create ServiceMonitor template | `deploy/helm/slowpokeapi/templates/servicemonitor.yaml` | Pending |
| 19.11 | Create values-prod.yaml | `deploy/helm/slowpokeapi/values-prod.yaml` | Pending |
| 19.12 | Test helm template rendering | - | Pending |

### Deliverables

- Complete Helm chart
- Production values file
- StatefulSet support with PVC

### Acceptance Criteria
- [ ] Helm chart structure created
- [ ] Chart.yaml with metadata
- [ ] values.yaml with defaults
- [ ] Deployment template
- [ ] Service template
- [ ] ConfigMap template
- [ ] Ingress template
- [ ] StatefulSet template
- [ ] HPA template
- [ ] ServiceMonitor template
- [ ] values-prod.yaml
- [ ] Tests pass
- [ ] Clippy passes with no warnings
- [ ] Format check passes
- [ ] CI passes

### Verification commands
```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
helm template slowpokeapi deploy/helm/slowpokeapi
```

### After completion
1. Update PLAN.md - Mark Phase 19 complete
2. Update STATUS.md - Move to Phase 20
3. Update WHAT_WE_DID.md - document Phase 19
4. Update DO_NEXT.md - set up Phase 20 tasks
5. Create feature branch for Phase 20
6. Create PR
7. Ensure CI passes
