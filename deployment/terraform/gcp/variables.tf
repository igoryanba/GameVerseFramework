variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "region" {
  description = "GCP region"
  type        = string
  default     = "us-central1"
}

variable "project_name" {
  description = "Project name prefix"
  type        = string
  default     = "gameverse"
}

variable "gcp_admin_email" {
  description = "Google account e-mail to grant cluster-admin"
  type        = string
}

variable "db_username" {
  description = "Postgres username"
  type        = string
  default     = "gameverse"
}

variable "db_password" {
  description = "Postgres password"
  type        = string
  sensitive   = true
} 