variable "subscription_id" {
  description = "Azure Subscription ID"
  type        = string
}

variable "location" {
  description = "Azure location"
  type        = string
  default     = "eastus"
}

variable "resource_group_name" {
  description = "Name of resource group"
  type        = string
  default     = "gameverse-rg"
}

variable "project_name" {
  description = "Project prefix"
  type        = string
  default     = "gameverse"
}

variable "aks_node_count" {
  description = "Initial node count"
  type        = number
  default     = 2
}

variable "db_username" {
  description = "Postgres admin username"
  type        = string
  default     = "gameverse"
}

variable "db_password" {
  description = "Postgres admin password"
  type        = string
  sensitive   = true
} 