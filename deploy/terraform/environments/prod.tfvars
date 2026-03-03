aws_region    = "us-east-1"
environment   = "prod"
project_name  = "slowpokeapi"
image_tag     = "latest"
desired_count = 3
cpu           = 512
memory        = 1024

enable_persistence = true
enable_autoscaling = true
min_capacity       = 3
max_capacity       = 20
sync_enabled       = true

log_level         = "info"
cache_ttl_seconds = 3600

domain_name = ""

tags = {
  Project     = "SlowPokeAPI"
  Environment = "prod"
  ManagedBy   = "terraform"
  Team        = "platform"
}
