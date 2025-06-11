# Сервис логирования GameVerse Framework

## Описание
Централизованная система логирования для всех компонентов GameVerse Framework. Обеспечивает сбор, хранение и анализ логов со всех микросервисов и клиентов.

## Компоненты
- **Log Collector** - Сборщик логов с различных источников через gRPC
- **Log Storage** - Хранилище логов (ElasticSearch или ClickHouse)
- **Log API** - API для взаимодействия с системой логирования
- **Log UI** - Веб-интерфейс для просмотра и анализа логов (Kibana)

## Структура логов
Все логи следуют структурированному формату:
```json
{
  "timestamp": "ISO 8601 timestamp",
  "level": "INFO|WARN|ERROR|DEBUG|TRACE",
  "service": "service-name",
  "component": "component-name",
  "message": "log message",
  "context": {
    "request_id": "uuid",
    "user_id": "optional user id",
    "entity_id": "optional entity id",
    "additional_data": {}
  },
  "metrics": {
    "duration_ms": 0,
    "memory_usage": 0
  }
}
```

## Уровни логирования
- **TRACE** - Детальная трассировка для отладки
- **DEBUG** - Отладочная информация
- **INFO** - Общая информация о работе
- **WARN** - Предупреждения, не влияющие на работу
- **ERROR** - Ошибки, влияющие на работу компонента
- **FATAL** - Критические ошибки, требующие немедленного вмешательства

## Технологии
- Rust (сервис логирования)
- tokio (асинхронный рантайм)
- tonic (gRPC фреймворк)
- ElasticSearch (хранение логов)
- Kibana (визуализация)

## Архитектура

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Микросервис 1  │     │  Микросервис 2  │     │  Микросервис 3  │
└────────┬────────┘     └────────┬────────┘     └────────┬────────┘
         │                       │                       │
         │                       │                       │
         │       ┌───────────────▼───────────────┐       │
         └───────►   gRPC API (LoggingService)   ◄───────┘
                 └───────────────┬───────────────┘
                                 │
                 ┌───────────────▼───────────────┐
                 │       Сервис логирования      │
                 └───────────────┬───────────────┘
                                 │
                                 ▼
                 ┌───────────────────────────────┐
                 │         ElasticSearch         │
                 └───────────────┬───────────────┘
                                 │
                                 ▼
                 ┌───────────────────────────────┐
                 │            Kibana             │
                 └───────────────────────────────┘
```

## gRPC API

Сервис логирования предоставляет следующие gRPC методы:

- **SendLog** - отправка одного лога
- **SendLogBatch** - отправка пакета логов (для оптимизации)
- **StreamLogs** - потоковая отправка логов

Для подробной информации см. файл [proto/logging.proto](proto/logging.proto).

## Хранение логов в ElasticSearch

Для хранения логов используется ElasticSearch с настроенными индексами и политиками жизненного цикла:

- Индексы с шаблоном `gameverse-logs-*`
- Ротация индексов через ILM (Index Lifecycle Management)
- Автоматическое удаление старых логов (старше 90 дней)

Для настройки ElasticSearch используется скрипт [scripts/setup_elasticsearch.sh](scripts/setup_elasticsearch.sh).

## Конфигурация

Сервис настраивается через переменные окружения:

| Переменная | Описание | Значение по умолчанию |
|------------|----------|------------------------|
| GAMEVERSE_LOG_DIR | Директория для хранения локальных логов | logs |
| GAMEVERSE_LOG_HOST | Хост для gRPC сервера | 0.0.0.0 |
| GAMEVERSE_LOG_PORT | Порт для gRPC сервера | 50051 |
| ELASTICSEARCH_URL | URL ElasticSearch | http://localhost:9200 |
| ELASTICSEARCH_INDEX | Имя индекса в ElasticSearch | gameverse-logs |

## Запуск сервиса и хранилища

```bash
# Запуск ElasticSearch и Kibana
docker-compose up -d

# Настройка индексов ElasticSearch
cd scripts
chmod +x setup_elasticsearch.sh
./setup_elasticsearch.sh

# Запуск сервиса логирования
cd ..
cargo run
```

## Прогресс разработки
- [x] Разработка спецификации формата логов
- [x] Определение уровней логирования
- [x] Выбор технологий для системы логирования
- [x] Создание библиотеки логирования для Rust
- [x] Определение gRPC API для сервиса логирования
- [x] Реализация gRPC сервера для сбора логов
- [x] Реализация gRPC клиента для отправки логов
- [x] Настройка хранилища логов (ElasticSearch + Kibana)
- [x] Реализация отправки логов в ElasticSearch
- [ ] Интеграция с остальными микросервисами
- [ ] Создание дашбордов в Kibana для мониторинга

## Следующие шаги
1. Интеграция с микросервисом аутентификации:
   - Настройка общего формата контекста запросов
   - Добавление отслеживания запросов между сервисами
2. Визуализация логов:
   - Создание дашбордов в Kibana для мониторинга
   - Настройка алертов на основе логов

## Использование библиотеки логирования

Для использования библиотеки логирования в микросервисах:

```rust
// Пример инициализации логгера
let logger = Logger::new("auth-service");
logger.add_handler(ConsoleLogHandler).await;
logger.add_handler(FileLogHandler::new("logs/auth-service.log")).await;

// Добавление gRPC обработчика для отправки логов на центральный сервер
let grpc_handler = GrpcLogHandler::new("auth-service", "logging-service:50051").await?;
logger.add_handler(grpc_handler).await;

// Добавление ElasticSearch обработчика (опционально)
let es_handler = ElasticsearchLogHandler::new(
    "http://elasticsearch:9200", 
    "gameverse-logs", 
    Some(50), // размер пакета
    Some(5)   // интервал отправки в секундах
).await?;
logger.add_handler(es_handler).await;

// Простое логирование
logger.info("startup", "Auth service starting...").await?;
logger.debug("config", "Loaded configuration from environment").await?;

// Логирование с контекстом
let log_entry = LogEntry::new(
    LogLevel::Info, 
    "auth-service", 
    "login", 
    "User login successful"
)
.with_request_id(request_id)
.with_user_id(user_id)
.with_data("ip_address", client_ip)
.with_duration(operation_duration);

logger.log_entry(log_entry).await?;
```

## Примеры

В директории [examples](examples) содержатся примеры использования библиотеки логирования. 