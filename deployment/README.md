# GameVerse Framework Server Bootstrap v0.2

Полная инфраструктура для развертывания GameVerse Framework в production окружении с использованием Docker, Kubernetes и облачных провайдеров.

## 🚀 Быстрый старт

### Docker Compose (Локальная разработка)

```bash
# Клонирование репозитория
git clone https://github.com/gameverse/GameVerseFramework.git
cd GameVerseFramework

# Запуск полного стека
cd deployment/docker
docker-compose up -d

# Проверка статуса
docker-compose ps

# Просмотр логов
docker-compose logs -f gameverse-server
```

Сервисы будут доступны по адресам:
- **GameVerse Server**: http://localhost:8080
- **Admin API**: http://localhost:30121
- **Grafana Dashboard**: http://localhost:3000 (admin/admin)
- **Prometheus**: http://localhost:9091
- **Jaeger Tracing**: http://localhost:16686

### Kubernetes (Production)

```bash
# Добавление Helm репозитория
helm repo add gameverse https://charts.gameverse.dev
helm repo update

# Установка с default значениями
helm install gameverse gameverse/gameverse

# Или с custom values
helm install gameverse gameverse/gameverse -f custom-values.yaml

# Проверка статуса
kubectl get pods -l app.kubernetes.io/name=gameverse
kubectl get services gameverse
```

### Terraform (Облачная инфраструктура)

#### AWS

```bash
cd deployment/terraform/aws

# Инициализация
terraform init

# Планирование
terraform plan -var="db_password=your-secure-password"

# Применение
terraform apply -var="db_password=your-secure-password"

# Получение outputs
terraform output server_endpoint
terraform output admin_endpoint
```

#### GCP

```bash
cd deployment/terraform/gcp
terraform init
terraform plan -var="project_id=your-gcp-project"
terraform apply -var="project_id=your-gcp-project"
```

#### Azure

```bash
cd deployment/terraform/azure
terraform init
terraform plan -var="resource_group_name=gameverse-rg"
terraform apply -var="resource_group_name=gameverse-rg"
```

## 📁 Структура проекта

```
deployment/
├── docker/                    # Docker Compose stack
│   ├── Dockerfile             # Multi-stage GameVerse server build
│   ├── docker-compose.yml     # Полный stack с мониторингом
│   ├── nginx/                 # Reverse proxy конфигурация
│   ├── monitoring/            # Prometheus + Grafana конфигурация
│   └── init-scripts/          # Database initialization scripts
├── kubernetes/                # Kubernetes Helm Charts
│   └── gameverse-helm/        # Helm chart для GameVerse
│       ├── Chart.yaml         # Chart metadata
│       ├── values.yaml        # Default конфигурация
│       └── templates/         # Kubernetes manifests
└── terraform/                 # Infrastructure as Code
    ├── aws/                   # AWS модуль
    ├── gcp/                   # Google Cloud модуль
    └── azure/                 # Azure модуль
```

## 🐳 Docker

### Multi-stage Build

Dockerfile оптимизирован для production с тремя stages:

1. **Dependencies**: Сборка и кеширование зависимостей
2. **Builder**: Компиляция GameVerse server
3. **Runtime**: Минимальный Alpine Linux образ (< 50MB)

### Особенности

- **Non-root execution**: Безопасность через dedicated пользователя
- **Health checks**: Автоматическая проверка состояния
- **Security scanning**: Интеграция с Trivy
- **Resource limits**: CPU/Memory constraints

### Сервисы в stack

| Сервис | Порт | Описание |
|--------|------|----------|
| gameverse-server | 8080, 30121, 9090 | Основной сервер + Admin API + Metrics |
| postgres | 5432 | PostgreSQL database |
| redis | 6379 | Redis cache |
| prometheus | 9091 | Metrics collection |
| grafana | 3000 | Monitoring dashboard |
| jaeger | 16686 | Distributed tracing |
| nginx | 80, 443 | Reverse proxy + SSL termination |

## ⚓ Kubernetes

### Helm Chart Features

- **Auto-scaling**: HorizontalPodAutoscaler based on CPU/Memory
- **High Availability**: Multi-replica deployment с anti-affinity
- **Security**: Non-root containers, read-only filesystem
- **Monitoring**: Prometheus ServiceMonitor integration
- **Secrets Management**: Kubernetes secrets для credentials
- **Resource Management**: Requests/limits для всех containers

### Конфигурация

```yaml
# values.yaml пример
gameverse:
  replicaCount: 3
  autoscaling:
    enabled: true
    minReplicas: 2
    maxReplicas: 10
  resources:
    requests:
      cpu: 100m
      memory: 128Mi
    limits:
      cpu: 500m
      memory: 512Mi
```

### Мониторинг

- **Prometheus**: Автоматический scraping metrics
- **Grafana**: Pre-configured dashboards
- **Jaeger**: Distributed tracing
- **Alerts**: Critical alerts через Alertmanager

## 🏗️ Terraform

### AWS Module

Создает полную инфраструктуру:

- **VPC**: Multi-AZ setup с public/private subnets
- **Auto Scaling Group**: Elastic server capacity
- **Application Load Balancer**: High availability + health checks
- **RDS PostgreSQL**: Managed database с backups
- **ElastiCache Redis**: Managed cache cluster
- **Security Groups**: Minimal required permissions

### Переменные

```hcl
# terraform.tfvars пример
project_name = "gameverse-prod"
environment = "production"
instance_type = "t3.large"
min_servers = 2
max_servers = 10
db_instance_class = "db.t3.small"
```

### Outputs

После применения получите:

```bash
server_endpoint = "http://gameverse-alb-123456789.us-west-2.elb.amazonaws.com"
admin_endpoint = "http://gameverse-alb-123456789.us-west-2.elb.amazonaws.com/admin"
database_endpoint = "gameverse-db.xyz.us-west-2.rds.amazonaws.com:5432"
redis_endpoint = "gameverse-redis.xyz.cache.amazonaws.com:6379"
```

## 📊 Мониторинг

### Prometheus Metrics

GameVerse server экспортирует custom metrics:

```
# Game-specific metrics
gameverse_players_connected_total
gameverse_avg_tick_ms
gameverse_memory_rss_bytes
gameverse_errors_total
gameverse_request_duration_seconds

# System metrics
process_cpu_seconds_total
process_memory_bytes
http_requests_total
```

### Grafana Dashboards

Pre-configured dashboards включают:

- **Server Performance**: CPU, Memory, Tick time
- **Player Statistics**: Connected players, sessions
- **Database Health**: Connection pool, query performance
- **Error Monitoring**: Error rates, response times
- **Infrastructure**: Load balancer, auto-scaling metrics

### Alerting Rules

Critical alerts:

- Server down (> 1 minute)
- High memory usage (> 80%)
- High error rate (> 5%)
- Database connection issues
- Slow response times (> 500ms)

## 🔒 Безопасность

### Container Security

- **Non-root execution**: UID/GID 1001
- **Read-only filesystem**: Immutable containers
- **Security scanning**: Trivy integration
- **Minimal attack surface**: Alpine Linux base

### Network Security

- **TLS encryption**: HTTPS/SSL termination
- **Security headers**: HSTS, CSP, X-Frame-Options
- **Rate limiting**: API protection
- **Network policies**: Kubernetes network isolation

### Secrets Management

- **Kubernetes secrets**: Database credentials
- **Environment variables**: Runtime configuration
- **Vault integration**: Advanced secret management (optional)

## 🚀 Производительность

### Optimization Features

- **Multi-stage builds**: Faster Docker builds
- **Resource limits**: Predictable performance
- **Auto-scaling**: Automatic capacity management
- **Caching**: Redis для session data
- **Connection pooling**: Database optimization

### Benchmarks

| Metric | Target | Monitoring |
|--------|--------|------------|
| Response Time | < 100ms | Prometheus |
| Memory Usage | < 512MB | Grafana |
| CPU Usage | < 70% | Auto-scaling |
| Uptime | > 99.9% | Health checks |

## 🔧 Конфигурация

### Environment Variables

```bash
# Server configuration
RUST_LOG=info
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
ADMIN_API_PORT=30121

# Database
DATABASE_URL=postgresql://user:pass@host:5432/db
REDIS_URL=redis://host:6379

# Security
ADMIN_JWT_SECRET=your-secret-key
```

### Volume Mounts

```yaml
volumes:
  - name: config
    mountPath: /app/config
  - name: logs
    mountPath: /app/logs
  - name: resources
    mountPath: /app/resources
```

## 🔄 CI/CD Integration

### GitHub Actions

```yaml
# .github/workflows/deploy.yml
name: Deploy GameVerse
on:
  push:
    branches: [main]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build and push Docker image
        run: |
          docker build -t gameverse/server:${{ github.sha }} .
          docker push gameverse/server:${{ github.sha }}
      - name: Deploy to Kubernetes
        run: |
          helm upgrade gameverse ./deployment/kubernetes/gameverse-helm \
            --set image.tag=${{ github.sha }}
```

### GitLab CI

```yaml
# .gitlab-ci.yml
stages:
  - build
  - deploy

build:
  stage: build
  script:
    - docker build -t $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA .
    - docker push $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA

deploy:
  stage: deploy
  script:
    - helm upgrade gameverse ./deployment/kubernetes/gameverse-helm \
        --set image.tag=$CI_COMMIT_SHA
```

## 🆘 Troubleshooting

### Общие проблемы

#### Docker Compose

```bash
# Проверка логов
docker-compose logs gameverse-server

# Перезапуск сервиса
docker-compose restart gameverse-server

# Полная пересборка
docker-compose down -v
docker-compose up --build -d
```

#### Kubernetes

```bash
# Проверка pod статуса
kubectl describe pod gameverse-xxx

# Просмотр логов
kubectl logs -f deployment/gameverse

# Debug networking
kubectl exec -it gameverse-xxx -- curl localhost:30121/api/health
```

#### Terraform

```bash
# Проверка state
terraform show

# Refresh state
terraform refresh

# Destroy и recreate
terraform destroy
terraform apply
```

### Health Checks

```bash
# Server health
curl http://localhost:30121/api/health

# Database connectivity
curl http://localhost:30121/api/health/database

# Redis connectivity
curl http://localhost:30121/api/health/redis
```

## 📚 Дополнительные ресурсы

- [GameVerse Framework Documentation](../docs/)
- [Server Configuration Guide](../docs/SERVER_CONFIG.md)
- [Monitoring Setup](../docs/MONITORING.md)
- [Security Best Practices](../docs/SECURITY.md)
- [Performance Tuning](../docs/PERFORMANCE.md)

## 🤝 Поддержка

- **GitHub Issues**: Для багов и feature requests
- **Discord**: Для community поддержки
- **Documentation**: Полная документация в `/docs`

---

**GameVerse Framework Server Bootstrap v0.2** обеспечивает enterprise-grade инфраструктуру для production deployment с автоматическим масштабированием, мониторингом и безопасностью. 