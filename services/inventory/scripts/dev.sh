#!/bin/bash
set -e

# Проверяем, запущен ли Docker
if ! docker info > /dev/null 2>&1; then
  echo "Ошибка: Docker не запущен. Пожалуйста, запустите Docker и повторите попытку."
  exit 1
fi

# Переходим в директорию сервиса инвентаря
cd "$(dirname "$0")/.."

# Запускаем Docker Compose с базой данных
echo "Запускаем инфраструктуру для разработки..."
docker-compose up -d

# Ждем, пока база данных будет готова
echo "Ожидание готовности PostgreSQL..."
for i in {1..30}; do
  if docker-compose exec postgres pg_isready -U gameverse -d gameverse_inventory > /dev/null 2>&1; then
    echo "PostgreSQL готов!"
    break
  fi
  echo -n "."
  sleep 1
done

if ! docker-compose exec postgres pg_isready -U gameverse -d gameverse_inventory > /dev/null 2>&1; then
  echo "Не удалось дождаться готовности PostgreSQL. Проверьте логи Docker."
  exit 1
fi

# Экспортируем переменные окружения для локальной разработки
export DATABASE_URL="postgres://gameverse:gameverse@localhost:5432/gameverse_inventory"
export REDIS_URL="redis://localhost:6379"
export GRPC_HOST="0.0.0.0"
export GRPC_PORT="50052"
export HTTP_HOST="0.0.0.0"
export HTTP_PORT="8081"
export LOGGING_SERVICE_ADDRESS="http://localhost:50051"
export LOG_LEVEL="debug"

# Компилируем и запускаем сервис
echo "Запускаем сервис инвентаря в режиме разработки..."
cargo watch -x "run --bin gameverse-inventory" 