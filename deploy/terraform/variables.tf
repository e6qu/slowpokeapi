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
