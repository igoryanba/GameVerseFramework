# Примеры использования библиотеки логирования

В этой директории содержатся примеры использования библиотеки логирования GameVerse Framework.

## Базовое использование (basic_logging.rs)

Пример демонстрирует базовое использование библиотеки логирования с консольным и файловым выводом:

```bash
cargo run --example basic_logging
```

В этом примере показано:
- Создание логгера
- Добавление различных обработчиков (консоль, файл)
- Логирование сообщений разных уровней
- Использование контекста и метрик в логах

## Клиент gRPC (grpc_client.rs)

Пример демонстрирует использование gRPC клиента для отправки логов на удаленный сервер логирования:

```bash
# Запустите сервер логирования в отдельном терминале
cargo run

# В другом терминале запустите пример
cargo run --example grpc_client
```

В этом примере показано:
- Подключение к gRPC серверу логирования
- Отправка логов через gRPC
- Работа с несколькими обработчиками логов одновременно
- Использование контекста запроса для группировки логов

## Логирование в ElasticSearch (elasticsearch_logging.rs)

Пример демонстрирует отправку логов в ElasticSearch:

```bash
# Сначала запустите ElasticSearch и Kibana
cd ..
docker-compose up -d

# Настройте индексы ElasticSearch
cd scripts
chmod +x setup_elasticsearch.sh
./setup_elasticsearch.sh

# Запустите пример
cargo run --example elasticsearch_logging
```

В этом примере показано:
- Подключение к ElasticSearch
- Пакетная отправка логов через Bulk API
- Использование различных уровней логирования
- Отправка структурированных данных в логах
- Добавление контекста и метрик для аналитики

## Запуск примеров с пользовательскими настройками

Вы можете настроить работу примеров через переменные окружения:

```bash
# Для basic_logging - настройка директории логов
GAMEVERSE_LOG_DIR=./custom_logs cargo run --example basic_logging

# Для grpc_client - настройка адреса сервера логирования
LOGGING_SERVER=192.168.1.100:50051 cargo run --example grpc_client

# Для elasticsearch_logging - настройка ElasticSearch
ELASTICSEARCH_URL=http://192.168.1.100:9200 ELASTICSEARCH_INDEX=my-logs cargo run --example elasticsearch_logging
``` 