# Микросервисы GameVerse Framework

## Описание
Данный каталог содержит микросервисы, которые обеспечивают модульность и масштабируемость системы. Каждый микросервис отвечает за отдельную функциональность и может быть развернут независимо.

## Структура микросервисов
- **authentication/** - Сервис аутентификации и авторизации
- **logging/** - Централизованное логирование
- **inventory/** - Сервис управления инвентарем
- **chat/** - Сервис чата и коммуникаций
- **player-data/** - Сервис хранения и управления данными игроков
- **world-state/** - Сервис синхронизации состояния мира
- **resource-manager/** - Сервис управления игровыми ресурсами
- **metrics/** - Сервис сбора метрик и мониторинга

## Технологии
- Rust для высокопроизводительных сервисов
- gRPC/Protocol Buffers для внутреннего взаимодействия
- Docker для контейнеризации
- PostgreSQL/Redis для хранения данных

## Архитектура микросервисов
Каждый микросервис следует архитектуре чистого домена (Clean Architecture) с четким разделением слоев:
- **Domain** - ядро бизнес-логики, модели данных и интерфейсы репозиториев
- **Infrastructure** - реализация репозиториев, работа с базой данных и кешем
- **API** - REST и gRPC интерфейсы
- **Config** - конфигурация и настройки
- **Utils** - утилиты и вспомогательные функции

## Межсервисное взаимодействие
Микросервисы взаимодействуют друг с другом через:
1. **gRPC** - для синхронных запросов между сервисами
2. **Централизованная система логирования** - для отладки и мониторинга
3. **Общие библиотеки** - для переиспользования кода

## Прогресс разработки
- [x] Настройка базовой структуры микросервисов
- [x] Определение API для взаимодействия между сервисами
- [x] Настройка общей системы логирования
- [x] Начало реализации сервиса аутентификации
- [x] Создание библиотеки логирования
- [ ] Реализация сервиса сбора логов
- [ ] Завершение микросервиса аутентификации
- [ ] Настройка мониторинга сервисов

## Текущие задачи
1. Завершение микросервиса аутентификации:
   - Реализация бизнес-логики
   - Настройка базы данных и миграций
   - Имплементация REST и gRPC API
2. Развитие микросервиса логирования:
   - Имплементация gRPC сервера для сбора логов
   - Настройка ElasticSearch и Kibana
   - Интеграция с другими микросервисами 