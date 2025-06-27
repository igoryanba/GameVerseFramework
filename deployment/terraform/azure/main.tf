terraform {
  required_version = ">= 1.0"
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

provider "azurerm" {
  features {}
  subscription_id = var.subscription_id
}

# Placeholder: Create Resource Group, VNet, AKS, PostgreSQL Flexible Server, Redis Cache 

# ------------------- Resource Group & Network -------------------
resource "azurerm_resource_group" "rg" {
  name     = var.resource_group_name
  location = var.location
}

resource "azurerm_virtual_network" "vnet" {
  name                = "${var.project_name}-vnet"
  address_space       = ["10.50.0.0/16"]
  location            = azurerm_resource_group.rg.location
  resource_group_name = azurerm_resource_group.rg.name
}

resource "azurerm_subnet" "subnet" {
  name                 = "aks-subnet"
  resource_group_name  = azurerm_resource_group.rg.name
  virtual_network_name = azurerm_virtual_network.vnet.name
  address_prefixes     = ["10.50.1.0/24"]
}

# ------------------- AKS Cluster -------------------
resource "azurerm_kubernetes_cluster" "aks" {
  name                = "${var.project_name}-aks"
  location            = azurerm_resource_group.rg.location
  resource_group_name = azurerm_resource_group.rg.name
  dns_prefix          = "${var.project_name}-aks"

  default_node_pool {
    name       = "system"
    node_count = var.aks_node_count
    vm_size    = "Standard_B4ms"
    vnet_subnet_id = azurerm_subnet.subnet.id
    enable_auto_scaling = true
    max_count = 5
    min_count = 2
  }

  identity {
    type = "SystemAssigned"
  }

  network_profile {
    network_plugin = "azure"
    load_balancer_sku = "standard"
    outbound_type = "loadBalancer"
  }
}

# ------------------- Flexible Server (PostgreSQL) -------------------
resource "azurerm_postgresql_flexible_server" "postgres" {
  name                   = "${var.project_name}-pg"
  resource_group_name    = azurerm_resource_group.rg.name
  location               = azurerm_resource_group.rg.location
  version                = "15"
  administrator_login    = var.db_username
  administrator_password = var.db_password
  sku_name               = "Standard_B1ms"
  storage_mb             = 32768
  zone                   = "1"

  delegated_subnet_id = azurerm_subnet.subnet.id
  private_dns_zone_id = null
}

# ------------------- Azure Cache for Redis -------------------
resource "azurerm_redis_cache" "redis" {
  name                = "${var.project_name}-redis"
  location            = azurerm_resource_group.rg.location
  resource_group_name = azurerm_resource_group.rg.name

  capacity            = 1
  family              = "C"
  sku_name            = "Basic"
  enable_non_ssl_port = false
  minimum_tls_version = "1.2"
}

# ------------------- Outputs -------------------
output "aks_host" {
  value = azurerm_kubernetes_cluster.aks.kube_config.0.host
}

output "postgres_fqdn" {
  value = azurerm_postgresql_flexible_server.postgres.fqdn
}

output "redis_hostname" {
  value = azurerm_redis_cache.redis.hostname
} 