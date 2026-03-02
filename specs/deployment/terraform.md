# Terraform ECS Specification

## Overview

Terraform configuration for deploying SlowPokeAPI to AWS ECS Fargate with ALB, optional EFS for persistence, and CloudWatch monitoring.

## Architecture

```
                                   ┌─────────────────────────────────┐
                                   │           Route53               │
                                   │    api.slowpokeapi.io           │
                                   └────────────────┬────────────────┘
                                                    │
                                   ┌────────────────▼────────────────┐
                                   │              ACM                │
                                   │         (TLS Certificate)       │
                                   └────────────────┬────────────────┘
                                                    │
                                   ┌────────────────▼────────────────┐
                                   │           ALB                   │
                                   │    (Application Load Balancer)  │
                                   └────────────────┬────────────────┘
                                                    │
                      ┌─────────────────────────────┼─────────────────────────────┐
                      │                             │                             │
                      ▼                             ▼                             ▼
           ┌─────────────────────┐     ┌─────────────────────┐     ┌─────────────────────┐
           │    ECS Service      │     │    ECS Service      │     │    ECS Service      │
           │                     │     │                     │     │                     │
           │  ┌───────────────┐  │     │  ┌───────────────┐  │     │  ┌───────────────┐  │
           │  │  Task (Fargate│  │     │  │  Task (Fargate│  │     │  │  Task (Fargate│  │
           │  │    1 vCPU     │  │     │  │    1 vCPU     │  │     │  │    1 vCPU     │  │
           │  │    512 MB     │  │     │  │    512 MB     │  │     │  │    512 MB     │  │
           │  └───────────────┘  │     │  └───────────────┘  │     │  └───────────────┘  │
           │                     │     │                     │     │                     │
           └──────────┬──────────┘     └──────────┬──────────┘     └──────────┬──────────┘
                      │                             │                             │
                      └─────────────────────────────┼─────────────────────────────┘
                                                    │
                                   ┌────────────────▼────────────────┐
                                   │            EFS                  │
                                   │   (Optional Persistent Storage) │
                                   └─────────────────────────────────┘
```

## File Structure

```
deploy/terraform/
├── main.tf
├── variables.tf
├── outputs.tf
├── providers.tf
├── vpc.tf
├── security.tf
├── alb.tf
├── ecs.tf
├── efs.tf
├── cloudwatch.tf
├── route53.tf
├── versions.tf
└── environments/
    ├── dev.tfvars
    └── prod.tfvars
```

## versions.tf

```hcl
terraform {
  required_version = ">= 1.5.0"
  
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
  
  backend "s3" {
    bucket         = "slowpokeapi-terraform-state"
    key            = "terraform.tfstate"
    region         = "us-east-1"
    encrypt        = true
    dynamodb_table = "slowpokeapi-terraform-locks"
  }
}
```

## variables.tf

```hcl
variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "dev"
}

variable "project_name" {
  description = "Project name for resource naming"
  type        = string
  default     = "slowpokeapi"
}

variable "image_tag" {
  description = "Container image tag"
  type        = string
  default     = "latest"
}

variable "desired_count" {
  description = "Number of ECS tasks"
  type        = number
  default     = 2
}

variable "cpu" {
  description = "CPU units per task"
  type        = number
  default     = 256
}

variable "memory" {
  description = "Memory per task (MB)"
  type        = number
  default     = 512
}

variable "enable_persistence" {
  description = "Enable EFS for persistent storage"
  type        = bool
  default     = false
}

variable "domain_name" {
  description = "Domain name for the API"
  type        = string
  default     = ""
}

variable "enable_autoscaling" {
  description = "Enable ECS auto scaling"
  type        = bool
  default     = true
}

variable "min_capacity" {
  description = "Minimum number of tasks for autoscaling"
  type        = number
  default     = 2
}

variable "max_capacity" {
  description = "Maximum number of tasks for autoscaling"
  type        = number
  default     = 10
}

variable "sync_enabled" {
  description = "Enable CRDT sync between replicas"
  type        = bool
  default     = true
}

variable "log_level" {
  description = "Log level"
  type        = string
  default     = "info"
}

variable "cache_ttl_seconds" {
  description = "Cache TTL in seconds"
  type        = number
  default     = 3600
}

variable "tags" {
  description = "Tags to apply to resources"
  type        = map(string)
  default     = {}
}
```

## main.tf

```hcl
locals {
  name_prefix = "${var.project_name}-${var.environment}"
  
  common_tags = merge(var.tags, {
    Project     = var.project_name
    Environment = var.environment
    ManagedBy   = "terraform"
  })
}
```

## vpc.tf

```hcl
data "aws_availability_zones" "available" {}

module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "~> 5.0"

  name = "${local.name_prefix}-vpc"
  cidr = "10.0.0.0/16"

  azs             = slice(data.aws_availability_zones.available.names, 0, 3)
  private_subnets = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  public_subnets  = ["10.0.101.0/24", "10.0.102.0/24", "10.0.103.0/24"]

  enable_nat_gateway = true
  single_nat_gateway = var.environment == "dev"

  enable_vpn_gateway = false

  tags = local.common_tags
}
```

## security.tf

```hcl
resource "aws_security_group" "alb" {
  name        = "${local.name_prefix}-alb-sg"
  description = "ALB security group"
  vpc_id      = module.vpc.vpc_id

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = local.common_tags
}

resource "aws_security_group" "ecs" {
  name        = "${local.name_prefix}-ecs-sg"
  description = "ECS tasks security group"
  vpc_id      = module.vpc.vpc_id

  ingress {
    from_port       = 8080
    to_port         = 8080
    protocol        = "tcp"
    security_groups = [aws_security_group.alb.id]
  }

  # Sync port (internal communication)
  ingress {
    from_port = 8081
    to_port   = 8081
    protocol  = "tcp"
    self      = true
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = local.common_tags
}

resource "aws_security_group" "efs" {
  count = var.enable_persistence ? 1 : 0

  name        = "${local.name_prefix}-efs-sg"
  description = "EFS security group"
  vpc_id      = module.vpc.vpc_id

  ingress {
    from_port       = 2049
    to_port         = 2049
    protocol        = "tcp"
    security_groups = [aws_security_group.ecs.id]
  }

  tags = local.common_tags
}
```

## alb.tf

```hcl
resource "aws_lb" "main" {
  name               = "${local.name_prefix}-alb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [aws_security_group.alb.id]
  subnets            = module.vpc.public_subnets

  enable_deletion_protection = var.environment == "prod"

  tags = local.common_tags
}

resource "aws_lb_target_group" "http" {
  name        = "${local.name_prefix}-http"
  port        = 8080
  protocol    = "HTTP"
  vpc_id      = module.vpc.vpc_id
  target_type = "ip"

  health_check {
    enabled             = true
    healthy_threshold   = 2
    interval            = 30
    matcher             = "200"
    path                = "/healthz"
    port                = "traffic-port"
    protocol            = "HTTP"
    timeout             = 5
    unhealthy_threshold = 3
  }

  tags = local.common_tags
}

resource "aws_lb_listener" "http" {
  load_balancer_arn = aws_lb.main.arn
  port              = "80"
  protocol          = "HTTP"

  default_action {
    type = "redirect"
    redirect {
      port        = "443"
      protocol    = "HTTPS"
      status_code = "HTTP_301"
    }
  }
}

resource "aws_lb_listener" "https" {
  count = var.domain_name != "" ? 1 : 0

  load_balancer_arn = aws_lb.main.arn
  port              = "443"
  protocol          = "HTTPS"
  ssl_policy        = "ELBSecurityPolicy-2021-06"
  certificate_arn   = aws_acm_certificate.main[0].arn

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.http.arn
  }
}
```

## ecs.tf

```hcl
resource "aws_ecs_cluster" "main" {
  name = "${local.name_prefix}-cluster"

  setting {
    name  = "containerInsights"
    value = "enabled"
  }

  tags = local.common_tags
}

resource "aws_cloudwatch_log_group" "ecs" {
  name              = "/ecs/${local.name_prefix}"
  retention_in_days = 30

  tags = local.common_tags
}

resource "aws_ecs_task_definition" "main" {
  family                   = local.name_prefix
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = var.cpu
  memory                   = var.memory
  
  task_role_arn      = aws_iam_role.ecs_task.arn
  execution_role_arn = aws_iam_role.ecs_execution.arn

  container_definitions = jsonencode([
    {
      name      = "slowpokeapi"
      image     = "${aws_ecr_repository.main.repository_url}:${var.image_tag}"
      essential = true

      portMappings = [
        {
          containerPort = 8080
          protocol      = "tcp"
        },
        {
          containerPort = 8081
          protocol      = "tcp"
        }
      ]

      environment = [
        { name = "SLOWPOKEAPI_LOG_LEVEL", value = var.log_level },
        { name = "SLOWPOKEAPI_CACHE_TTL_SECONDS", value = tostring(var.cache_ttl_seconds) },
        { name = "SLOWPOKEAPI_SYNC_ENABLED", value = tostring(var.sync_enabled) },
        { name = "SLOWPOKEAPI_SYNC_DISCOVERY", value = "ecs" }
      ]

      logConfiguration = {
        logDriver = "awslogs"
        options = {
          awslogs-group         = aws_cloudwatch_log_group.ecs.name
          awslogs-region        = var.aws_region
          awslogs-stream-prefix = "ecs"
        }
      }

      healthCheck = {
        command     = ["CMD-SHELL", "curl -f http://localhost:8080/healthz || exit 1"]
        interval    = 30
        timeout     = 5
        retries     = 3
        startPeriod = 10
      }
    }
  ])

  tags = local.common_tags
}

resource "aws_ecs_service" "main" {
  name            = local.name_prefix
  cluster         = aws_ecs_cluster.main.id
  task_definition = aws_ecs_task_definition.main.arn
  desired_count   = var.desired_count
  launch_type     = "FARGATE"

  network_configuration {
    subnets         = module.vpc.private_subnets
    security_groups = [aws_security_group.ecs.id]
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.http.arn
    container_name   = "slowpokeapi"
    container_port   = 8080
  }

  tags = local.common_tags
}

resource "aws_ecr_repository" "main" {
  name                 = local.name_prefix
  image_tag_mutability = "MUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }

  tags = local.common_tags
}
```

## iam.tf

```hcl
resource "aws_iam_role" "ecs_execution" {
  name = "${local.name_prefix}-ecs-execution"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action = "sts:AssumeRole"
      Effect = "Allow"
      Principal = {
        Service = "ecs-tasks.amazonaws.com"
      }
    }]
  })

  tags = local.common_tags
}

resource "aws_iam_role_policy_attachment" "ecs_execution" {
  role       = aws_iam_role.ecs_execution.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"
}

resource "aws_iam_role" "ecs_task" {
  name = "${local.name_prefix}-ecs-task"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action = "sts:AssumeRole"
      Effect = "Allow"
      Principal = {
        Service = "ecs-tasks.amazonaws.com"
      }
    }]
  })

  tags = local.common_tags
}

resource "aws_iam_role_policy" "ecs_task" {
  name = "${local.name_prefix}-ecs-task-policy"
  role = aws_iam_role.ecs_task.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "ecs:DescribeTasks",
          "ecs:ListTasks"
        ]
        Resource = "*"
        Condition = {
          ArnEquals = {
            "ecs:cluster" = aws_ecs_cluster.main.arn
          }
        }
      }
    ]
  })
}
```

## efs.tf

```hcl
resource "aws_efs_file_system" "main" {
  count = var.enable_persistence ? 1 : 0

  creation_token = "${local.name_prefix}-efs"
  encrypted      = true

  lifecycle_policy {
    transition_to_ia = "AFTER_30_DAYS"
  }

  tags = local.common_tags
}

resource "aws_efs_mount_target" "main" {
  count = var.enable_persistence ? length(module.vpc.private_subnets) : 0

  file_system_id  = aws_efs_file_system.main[0].id
  subnet_id       = module.vpc.private_subnets[count.index]
  security_groups = [aws_security_group.efs[0].id]
}
```

## cloudwatch.tf

```hcl
resource "aws_cloudwatch_dashboard" "main" {
  dashboard_name = "${local.name_prefix}-dashboard"

  dashboard_body = jsonencode({
    widgets = [
      {
        type   = "metric"
        x      = 0
        y      = 0
        width  = 12
        height = 6
        properties = {
          title = "Request Count"
          view  = "timeSeries"
          metrics = [
            ["AWS/ApplicationELB", "RequestCount", "LoadBalancer", aws_lb.main.arn_suffix]
          ]
          period = 60
        }
      },
      {
        type   = "metric"
        x      = 0
        y      = 6
        width  = 12
        height = 6
        properties = {
          title = "Target Response Time"
          view  = "timeSeries"
          metrics = [
            ["AWS/ApplicationELB", "TargetResponseTime", "LoadBalancer", aws_lb.main.arn_suffix]
          ]
          period = 60
        }
      }
    ]
  })
}

resource "aws_cloudwatch_metric_alarm" "high_cpu" {
  alarm_name          = "${local.name_prefix}-high-cpu"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  metric_name         = "CPUUtilization"
  namespace           = "AWS/ECS"
  period              = 60
  statistic           = "Average"
  threshold           = 80
  alarm_description   = "High CPU utilization"
  alarm_actions       = [aws_appautoscaling_policy.scale_up.arn]

  dimensions = {
    ServiceName = aws_ecs_service.main.name
    ClusterName = aws_ecs_cluster.main.name
  }

  tags = local.common_tags
}
```

## autoscaling.tf

```hcl
resource "aws_appautoscaling_target" "ecs" {
  count = var.enable_autoscaling ? 1 : 0

  max_capacity       = var.max_capacity
  min_capacity       = var.min_capacity
  resource_id        = "service/${aws_ecs_cluster.main.name}/${aws_ecs_service.main.name}"
  scalable_dimension = "ecs:service:DesiredCount"
  service_namespace  = "ecs"
}

resource "aws_appautoscaling_policy" "cpu" {
  count = var.enable_autoscaling ? 1 : 0

  name               = "${local.name_prefix}-cpu-scaling"
  policy_type        = "TargetTrackingScaling"
  resource_id        = aws_appautoscaling_target.ecs[0].resource_id
  scalable_dimension = aws_appautoscaling_target.ecs[0].scalable_dimension
  service_namespace  = aws_appautoscaling_target.ecs[0].service_namespace

  target_tracking_scaling_policy_configuration {
    predefined_metric_specification {
      predefined_metric_type = "ECSServiceAverageCPUUtilization"
    }
    target_value       = 70
    scale_in_cooldown  = 300
    scale_out_cooldown = 60
  }
}
```

## outputs.tf

```hcl
output "alb_dns_name" {
  description = "ALB DNS name"
  value       = aws_lb.main.dns_name
}

output "ecr_repository_url" {
  description = "ECR repository URL"
  value       = aws_ecr_repository.main.repository_url
}

output "ecs_cluster_name" {
  description = "ECS cluster name"
  value       = aws_ecs_cluster.main.name
}

output "ecs_service_name" {
  description = "ECS service name"
  value       = aws_ecs_service.main.name
}

output "api_url" {
  description = "API URL"
  value       = var.domain_name != "" ? "https://${var.domain_name}" : "http://${aws_lb.main.dns_name}"
}
```

## environments/prod.tfvars

```hcl
environment        = "prod"
desired_count      = 3
cpu                = 512
memory             = 1024
enable_persistence = true
enable_autoscaling = true
min_capacity       = 3
max_capacity       = 10
log_level          = "warn"
cache_ttl_seconds  = 1800

tags = {
  CostCenter = "production"
  Team       = "platform"
}
```

## Usage

### Initialize

```bash
cd deploy/terraform
terraform init
```

### Plan

```bash
terraform plan -var-file=environments/prod.tfvars
```

### Apply

```bash
terraform apply -var-file=environments/prod.tfvars
```

### Deploy New Image

```bash
# Build and push image
docker build -t $(terraform output -raw ecr_repository_url):v1.0.0 .
docker push $(terraform output -raw ecr_repository_url):v1.0.0

# Update service
terraform apply -var-file=environments/prod.tfvars -var image_tag=v1.0.0
```

### Destroy

```bash
terraform destroy -var-file=environments/prod.tfvars
```
