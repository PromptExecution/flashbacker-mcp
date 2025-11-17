# 🥾 b00t k0s Infrastructure (OpenTofu/Terraform)
# Lightweight Kubernetes cluster for multi-agent hive

terraform {
  required_version = ">= 1.6.0"
  
  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "~> 3.0"
    }
  }
}

provider "docker" {
  host = "unix:///var/run/docker.sock"
}

# 🤓 k0s controller node (lightweight k8s)
resource "docker_container" "k0s_controller" {
  name  = "k0s-controller"
  image = docker_image.k0s.image_id
  
  command = [
    "k0s",
    "controller",
    "--enable-worker"
  ]
  
  privileged = true
  
  ports {
    internal = 6443
    external = 6443
    protocol = "tcp"
  }
  
  ports {
    internal = 8080
    external = 8080
    protocol = "tcp"
  }
  
  volumes {
    host_path      = "/var/lib/k0s"
    container_path = "/var/lib/k0s"
  }
  
  volumes {
    host_path      = "/etc/k0s"
    container_path = "/etc/k0s"
  }
  
  restart = "unless-stopped"
  
  labels {
    label = "b00t.role"
    value = "k0s-controller"
  }
}

resource "docker_image" "k0s" {
  name = "k0sproject/k0s:latest"
}

# 🦀 CommerceRack Rust API container
resource "docker_container" "commercerack_api" {
  name  = "commercerack-api"
  image = docker_image.commercerack.image_id
  
  depends_on = [docker_container.k0s_controller]
  
  ports {
    internal = 8000
    external = 8000
    protocol = "tcp"
  }
  
  env = [
    "DATABASE_URL=postgresql://postgres:postgres@postgres:5432/commercerack",
    "REDIS_URL=redis://redis:6379",
    "RUST_LOG=info"
  ]
  
  restart = "unless-stopped"
  
  labels {
    label = "b00t.service"
    value = "commercerack-api"
  }
}

resource "docker_image" "commercerack" {
  name = "commercerack-rust:latest"
  
  build {
    context    = "../../commercerack-rust"
    dockerfile = "Dockerfile"
  }
}

# 🗄️ PostgreSQL database
resource "docker_container" "postgres" {
  name  = "postgres"
  image = docker_image.postgres.image_id
  
  ports {
    internal = 5432
    external = 5432
    protocol = "tcp"
  }
  
  env = [
    "POSTGRES_DB=commercerack",
    "POSTGRES_USER=postgres",
    "POSTGRES_PASSWORD=postgres"
  ]
  
  volumes {
    host_path      = "${path.cwd}/data/postgres"
    container_path = "/var/lib/postgresql/data"
  }
  
  restart = "unless-stopped"
}

resource "docker_image" "postgres" {
  name = "postgres:16-alpine"
}

# 🔐 Redis cache
resource "docker_container" "redis" {
  name  = "redis"
  image = docker_image.redis.image_id
  
  ports {
    internal = 6379
    external = 6379
    protocol = "tcp"
  }
  
  restart = "unless-stopped"
}

resource "docker_image" "redis" {
  name = "redis:7-alpine"
}

output "k0s_api_endpoint" {
  value = "https://localhost:6443"
  description = "k0s Kubernetes API endpoint"
}

output "commercerack_api_endpoint" {
  value = "http://localhost:8000"
  description = "CommerceRack API endpoint"
}

output "postgres_connection" {
  value = "postgresql://postgres:postgres@localhost:5432/commercerack"
  description = "PostgreSQL connection string"
  sensitive = true
}
