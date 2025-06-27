# Network Outputs
output "vpc_id" {
  description = "ID of the VPC"
  value       = aws_vpc.gameverse.id
}

output "vpc_cidr_block" {
  description = "CIDR block of the VPC"
  value       = aws_vpc.gameverse.cidr_block
}

output "public_subnet_ids" {
  description = "IDs of the public subnets"
  value       = aws_subnet.public[*].id
}

output "private_subnet_ids" {
  description = "IDs of the private subnets"
  value       = aws_subnet.private[*].id
}

# Load Balancer Outputs
output "load_balancer_arn" {
  description = "ARN of the load balancer"
  value       = aws_lb.gameverse.arn
}

output "load_balancer_dns_name" {
  description = "DNS name of the load balancer"
  value       = aws_lb.gameverse.dns_name
}

output "load_balancer_zone_id" {
  description = "Zone ID of the load balancer"
  value       = aws_lb.gameverse.zone_id
}

output "server_endpoint" {
  description = "GameVerse server public endpoint"
  value       = "http://${aws_lb.gameverse.dns_name}"
}

output "admin_endpoint" {
  description = "GameVerse admin API endpoint"
  value       = "http://${aws_lb.gameverse.dns_name}/admin"
}

# Auto Scaling Group Outputs
output "autoscaling_group_arn" {
  description = "ARN of the Auto Scaling Group"
  value       = aws_autoscaling_group.gameverse.arn
}

output "autoscaling_group_name" {
  description = "Name of the Auto Scaling Group"
  value       = aws_autoscaling_group.gameverse.name
}

# Database Outputs
output "database_endpoint" {
  description = "RDS instance endpoint"
  value       = aws_db_instance.gameverse.endpoint
  sensitive   = true
}

output "database_port" {
  description = "RDS instance port"
  value       = aws_db_instance.gameverse.port
}

output "database_name" {
  description = "Database name"
  value       = aws_db_instance.gameverse.db_name
}

output "database_username" {
  description = "Database username"
  value       = aws_db_instance.gameverse.username
  sensitive   = true
}

output "database_connection_string" {
  description = "Database connection string"
  value       = "postgresql://${aws_db_instance.gameverse.username}:${var.db_password}@${aws_db_instance.gameverse.endpoint}/${aws_db_instance.gameverse.db_name}"
  sensitive   = true
}

# Redis Outputs
output "redis_endpoint" {
  description = "Redis primary endpoint"
  value       = aws_elasticache_replication_group.gameverse.primary_endpoint_address
  sensitive   = true
}

output "redis_port" {
  description = "Redis port"
  value       = aws_elasticache_replication_group.gameverse.port
}

output "redis_connection_string" {
  description = "Redis connection string"
  value       = "redis://${aws_elasticache_replication_group.gameverse.primary_endpoint_address}:${aws_elasticache_replication_group.gameverse.port}"
  sensitive   = true
}

# Security Group Outputs
output "server_security_group_id" {
  description = "ID of the server security group"
  value       = aws_security_group.gameverse_server.id
}

output "database_security_group_id" {
  description = "ID of the database security group"
  value       = aws_security_group.database.id
}

# Instance Outputs
output "launch_template_id" {
  description = "ID of the launch template"
  value       = aws_launch_template.gameverse.id
}

output "launch_template_latest_version" {
  description = "Latest version of the launch template"
  value       = aws_launch_template.gameverse.latest_version
}

# Monitoring and Health Check Outputs
output "health_check_url" {
  description = "Health check URL"
  value       = "http://${aws_lb.gameverse.dns_name}/api/health"
}

output "metrics_endpoint" {
  description = "Prometheus metrics endpoint"
  value       = "http://${aws_lb.gameverse.dns_name}/metrics"
}

# Environment Information
output "environment" {
  description = "Environment name"
  value       = var.environment
}

output "region" {
  description = "AWS region"
  value       = var.aws_region
}

output "project_name" {
  description = "Project name"
  value       = var.project_name
}

# Resource ARNs for monitoring and management
output "resource_arns" {
  description = "ARNs of all major resources"
  value = {
    vpc                = aws_vpc.gameverse.arn
    load_balancer     = aws_lb.gameverse.arn
    autoscaling_group = aws_autoscaling_group.gameverse.arn
    database          = aws_db_instance.gameverse.arn
    redis             = aws_elasticache_replication_group.gameverse.arn
  }
}

# Quick Connect Information
output "quick_connect_info" {
  description = "Quick connection information for administrators"
  value = {
    game_server_url = "http://${aws_lb.gameverse.dns_name}:8080"
    admin_panel_url = "http://${aws_lb.gameverse.dns_name}/admin"
    health_check    = "http://${aws_lb.gameverse.dns_name}/api/health"
    metrics_url     = "http://${aws_lb.gameverse.dns_name}/metrics"
  }
} 