# Многоэтапная сборка для оптимизации размера образа
FROM rust:1.75-slim as builder

# Установка зависимостей для сборки
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Создание рабочей директории
WORKDIR /app

# Копирование файлов конфигурации
COPY Cargo.toml Cargo.lock ./

# Создание фиктивного main.rs для кеширования зависимостей
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Сборка зависимостей (кеширование)
RUN cargo build --release && rm -rf src

# Копирование исходного кода
COPY src ./src
COPY build.rs ./
COPY proto ./proto

# Финальная сборка
RUN cargo build --release

# Продакшн образ
FROM debian:bookworm-slim

# Установка runtime зависимостей
RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Создание пользователя для безопасности
RUN groupadd -r gameverse && useradd -r -g gameverse gameverse

# Создание рабочей директории
WORKDIR /app

# Копирование скомпилированного бинарника
COPY --from=builder /app/target/release/gameverse-chat /usr/local/bin/

# Изменение владельца файлов
RUN chown -R gameverse:gameverse /app

# Переключение на непривилегированного пользователя
USER gameverse

# Установка переменных окружения
ENV RUST_LOG=info
ENV HTTP_PORT=8080
ENV GRPC_PORT=50053
ENV WEBSOCKET_PORT=8081

# Открытие портов
EXPOSE 8080 50053 8081

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:${HTTP_PORT}/health || exit 1

# Запуск приложения
CMD ["gameverse-chat"] 