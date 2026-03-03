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
  for_each = var.enable_persistence ? toset(slice(data.aws_availability_zones.available.names, 0, 3)) : toset([])

  file_system_id  = aws_efs_file_system.main[0].id
  subnet_id       = module.vpc.private_subnets[index(slice(data.aws_availability_zones.available.names, 0, 3), each.key)]
  security_groups = [aws_security_group.efs[0].id]
}

resource "aws_efs_access_point" "main" {
  count = var.enable_persistence ? 1 : 0

  file_system_id = aws_efs_file_system.main[0].id

  posix_user {
    gid = 1000
    uid = 1000
  }

  root_directory {
    path = "/data"
    creation_info {
      owner_gid   = 1000
      owner_uid   = 1000
      permissions = "755"
    }
  }
}

resource "aws_iam_role_policy" "efs" {
  count = var.enable_persistence ? 1 : 0

  name = "${local.name_prefix}-efs-policy"
  role = aws_iam_role.ecs_task.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "elasticfilesystem:ClientMount",
          "elasticfilesystem:ClientWrite"
        ]
        Resource = aws_efs_file_system.main[0].arn
        Condition = {
          StringEquals = {
            "elasticfilesystem:AccessPointArn" = aws_efs_access_point.main[0].arn
          }
        }
      }
    ]
  })
}
