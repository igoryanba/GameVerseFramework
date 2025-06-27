terraform {
  required_version = ">= 1.0"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

# Placeholder: Create GKE autopilot cluster and Cloud SQL + Memorystore

module "network" {
  source  = "terraform-google-modules/network/google"
  version = "6.0.0"

  project_id   = var.project_id
  network_name = "${var.project_name}-vpc"
  subnets = [
    {
      subnet_name   = "subnet-01"
      subnet_ip     = "10.10.0.0/24"
      subnet_region = var.region
    },
  ]
}

# ------------------- GKE Autopilot Cluster -------------------
resource "google_container_cluster" "gameverse" {
  name     = "${var.project_name}-gke"
  location = var.region
  enable_autopilot = true

  release_channel {
    channel = "REGULAR"
  }

  network    = module.network.network_name
  subnetwork = module.network.subnets[0].subnet_name

  ip_allocation_policy {}

  workload_identity_config {
    identity_namespace = "${var.project_id}.svc.id.goog"
  }

  # autopilot uses default node pools managed by Google
}

# Grant cluster-admin to current user for Helm provisioning
resource "google_container_cluster_iam_member" "cluster_admin" {
  project  = var.project_id
  location = google_container_cluster.gameverse.location
  cluster  = google_container_cluster.gameverse.name
  role     = "roles/container.admin"
  member   = "user:${var.gcp_admin_email}"
}

# ------------------- Cloud SQL PostgreSQL -------------------
resource "google_sql_database_instance" "postgres" {
  name             = "${var.project_name}-db"
  database_version = "POSTGRES_15"
  region           = var.region
  deletion_protection = false

  settings {
    tier = "db-custom-1-3840" # shared-core equivalent
    ip_configuration {
      ipv4_enabled    = false
      private_network = module.network.network_self_link
    }
    backup_configuration {
      enabled = true
      point_in_time_recovery_enabled = true
    }
  }
}

resource "google_sql_user" "app" {
  name     = var.db_username
  instance = google_sql_database_instance.postgres.name
  password = var.db_password
}

# ------------------- Memorystore Redis -------------------
resource "google_redis_instance" "redis" {
  name           = "${var.project_name}-redis"
  tier           = "BASIC"
  memory_size_gb = 1
  region         = var.region
  transit_encryption_mode = "SERVER_AUTHENTICATION"
  authorized_network = module.network.network_self_link
}

# ------------------- Outputs -------------------
output "gke_endpoint" {
  value = google_container_cluster.gameverse.endpoint
}

output "postgres_private_ip" {
  value = google_sql_database_instance.postgres.private_ip_address
}

output "redis_host" {
  value = google_redis_instance.redis.host
} 