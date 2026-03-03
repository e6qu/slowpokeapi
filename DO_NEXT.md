# Do Next
## Phase 20: Terraform ECS
### Goal

Create Terraform configuration for AWS ECS deployment.

### Tasks

| #  | Task | Files | Status |
|---|------|-------|--------|
| 20.1 | Create Terraform structure | `deploy/terraform/` | Pending |
| 20.2 | Create versions.tf | `deploy/terraform/versions.tf` | Pending |
| 20.3 | Create variables.tf | `deploy/terraform/variables.tf` | Pending |
| 20.4 | Create vpc.tf | `deploy/terraform/vpc.tf` | Pending |
| 20.5 | Create security.tf | `deploy/terraform/security.tf` | Pending |
| 20.6 | Create alb.tf | `deploy/terraform/alb.tf` | Pending |
| 20.7 | Create ecs.tf | `deploy/terraform/ecs.tf` | Pending |
| 20.8 | Create efs.tf | `deploy/terraform/efs.tf` | Pending |
| 20.9 | Create autoscaling.tf | `deploy/terraform/autoscaling.tf` | Pending |
| 20.10 | Create cloudwatch.tf | `deploy/terraform/cloudwatch.tf` | Pending |
| 20.11 | Create outputs.tf | `deploy/terraform/outputs.tf` | Pending |
| 20.12 | Create prod.tfvars | `deploy/terraform/environments/prod.tfvars` | Pending |
| 20.13 | Test terraform plan | - | Pending |

### Deliverables

- Complete Terraform configuration
- ECS Fargate deployment
- ALB with HTTPS
- Optional EFS persistence
- Auto-scaling configuration

### Acceptance Criteria
- [ ] Terraform structure created
- [ ] VPC with public/private subnets
- [ ] Security groups configured
- [ ] ALB with target group
- [ ] ECS cluster with service and task definition
- [ ] CloudWatch alarms
- [ ] Tests pass
- [ ] Clippy passes with no warnings
- [ ] Format check passes
- [ ] CI passes

### Verification commands
```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
terraform -chdir=deploy/terraform plan
```

### After completion
1. Update PLAN.md - Mark Phase 20 complete
2. Update STATUS.md - Move to Phase 21
3. Update WHAT_WE_DID.md - document Phase 20
4. Update DO_NEXT.md - set up Phase 21 tasks
5. Create feature branch for Phase 21
6. Create PR
7. Ensure CI passes
