# Микросервис Чата GameVerse Framework

Микросервис чата обеспечивает современную систему коммуникации для игровых серверов с поддержкой текстовых сообщений, голосового чата и расширенных функций мессенджера.

## Особенности

### Текстовый чат
- **Современные функции мессенджера**: Forward, Edit, Reply, Delete сообщений
- **Групповые чаты**: Создание и управление группами
- **Цитирование сообщений**: Ответы на конкретные сообщения
- **История сообщений**: Полная история с поиском и фильтрацией
- **Статусы сообщений**: "Отправлено", "Доставлено", "Прочитано"
- **Типизация**: Индикатор набора текста в реальном времени

### Голосовой чат
- **Proximity Voice**: 3D позиционный звук с настраиваемыми дистанциями
- **Режимы голоса**: Шепот, нормальная речь, крик
- **Радиосвязь**: Система частот с авторизацией доступа
- **Телефонные звонки**: Качественная голосовая связь
- **Push-to-Talk**: Настраиваемые клавиши активации
- **Voice Activity Detection**: Автоматическое определение речи

### Roleplay функции
- **Команды действий**: /me, /do, /try, /dice
- **OOC коммуникация**: /ooc, /looc с цветовым кодированием
- **Анонимность**: Поддержка масок и псевдонимов
- **Система каналов**: Разделение IC/OOC чатов
- **Административные команды**: Модерация и управление

### Расширенные возможности
- **Богатый контент**: Поддержка эмодзи, GIF, изображений
- **Система уведомлений**: Push-уведомления и звуковые оповещения
- **Блокировка пользователей**: Ignore-лист и модерация
- **Интеграция**: API для других микросервисов
- **Логирование**: Полное логирование для администрации

## Архитектура

### Компоненты
```
chat/
├── src/
│   ├── domain/           # Доменные модели
│   ├── infrastructure/   # Репозитории и внешние сервисы
│   ├── application/      # Сервисы и бизнес-логика
│   ├── interfaces/       # API handlers (HTTP/gRPC)
│   └── config/          # Конфигурация
├── migrations/          # Миграции базы данных
├── tests/              # Тесты
├── proto/              # Protobuf схемы
├── examples/           # Примеры использования
└── docs/               # Документация
```

### Технологический стек
- **Язык**: Rust
- **База данных**: PostgreSQL (основная), Redis (кеш, real-time)
- **Протоколы**: HTTP REST, gRPC, WebSocket
- **Голосовой чат**: Интеграция с FiveM Mumble/pma-voice
- **Очереди**: Redis Pub/Sub для real-time обмена
- **Мониторинг**: Structured logging с tracing

## Модели данных

### ChatMessage
```rust
pub struct ChatMessage {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub message_type: MessageType,
    pub reply_to: Option<Uuid>,
    pub edited_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub metadata: ChatMessageMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessageMetadata {
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    pub character_name: Option<String>,
    pub is_anonymous: bool,
    pub location: Option<GameLocation>,
}
```

### ChatChannel
```rust
pub struct ChatChannel {
    pub id: Uuid,
    pub name: String,
    pub channel_type: ChannelType,
    pub created_by: Uuid,
    pub is_private: bool,
    pub max_participants: Option<i32>,
    pub settings: ChannelSettings,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub enum ChannelType {
    Global,
    Local,
    Radio { frequency: String },
    Phone { call_id: Uuid },
    Group,
    Direct,
    OOC,
    Admin,
}
```

### VoiceSession
```rust
pub struct VoiceSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_type: VoiceSessionType,
    pub channel_id: Option<Uuid>,
    pub voice_range: VoiceRange,
    pub is_muted: bool,
    pub position: Option<GamePosition>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub enum VoiceSessionType {
    Proximity,
    Radio { frequency: String },
    Phone { call_id: Uuid },
    Group { channel_id: Uuid },
}
```

## API Спецификация

### REST API

#### Сообщения
- `POST /channels/{id}/messages` - Отправить сообщение
- `GET /channels/{id}/messages` - Получить историю сообщений
- `PUT /messages/{id}` - Редактировать сообщение
- `DELETE /messages/{id}` - Удалить сообщение
- `POST /messages/{id}/reply` - Ответить на сообщение
- `POST /messages/{id}/forward` - Переслать сообщение

#### Каналы
- `POST /channels` - Создать канал
- `GET /channels` - Список каналов пользователя
- `PUT /channels/{id}` - Обновить настройки канала
- `POST /channels/{id}/members` - Добавить участника
- `DELETE /channels/{id}/members/{user_id}` - Удалить участника

#### Голосовой чат
- `POST /voice/sessions` - Начать голосовую сессию
- `PUT /voice/sessions/{id}` - Обновить настройки сессии
- `DELETE /voice/sessions/{id}` - Завершить сессию
- `GET /voice/sessions/active` - Активные сессии

### gRPC API
```protobuf
service ChatService {
    rpc SendMessage(SendMessageRequest) returns (ChatMessage);
    rpc GetMessages(GetMessagesRequest) returns (stream ChatMessage);
    rpc EditMessage(EditMessageRequest) returns (ChatMessage);
    rpc DeleteMessage(DeleteMessageRequest) returns (Empty);
    
    rpc CreateChannel(CreateChannelRequest) returns (ChatChannel);
    rpc JoinChannel(JoinChannelRequest) returns (Empty);
    rpc LeaveChannel(LeaveChannelRequest) returns (Empty);
}

service VoiceService {
    rpc StartVoiceSession(StartVoiceSessionRequest) returns (VoiceSession);
    rpc UpdateVoicePosition(UpdateVoicePositionRequest) returns (Empty);
    rpc EndVoiceSession(EndVoiceSessionRequest) returns (Empty);
    rpc GetActiveVoiceSessions(Empty) returns (stream VoiceSession);
}
```

### WebSocket API
```typescript
interface WebSocketMessage {
    type: 'message' | 'typing' | 'voice_status' | 'channel_update';
    data: any;
    timestamp: string;
}

// Клиентские события
type ClientEvent = 
    | { type: 'send_message', data: SendMessageData }
    | { type: 'start_typing', data: { channel_id: string } }
    | { type: 'stop_typing', data: { channel_id: string } }
    | { type: 'join_channel', data: { channel_id: string } }
    | { type: 'voice_range_change', data: { range: VoiceRange } };

// Серверные события
type ServerEvent = 
    | { type: 'new_message', data: ChatMessage }
    | { type: 'message_edited', data: ChatMessage }
    | { type: 'message_deleted', data: { message_id: string } }
    | { type: 'user_typing', data: { user_id: string, channel_id: string } }
    | { type: 'voice_session_update', data: VoiceSession };
```

## Интеграции

### Микросервис аутентификации
- Проверка JWT токенов
- Получение информации о пользователе
- Валидация прав доступа

### Микросервис данных игроков
- Получение игровой позиции
- Статус персонажа (живой/мертвый)
- Информация о персонаже

### Микросервис логирования
- Отправка всех сообщений для аудита
- Логирование голосовых сессий
- Метрики использования

### Внешние сервисы
- **pma-voice**: Интеграция с голосовым чатом FiveM
- **Redis**: Real-time сообщения и кеширование
- **ElasticSearch**: Полнотекстовый поиск по сообщениям

## Этапы разработки

### Этап 1: Базовая функциональность ✅ ЗАВЕРШЕН
- [x] Планирование архитектуры
- [x] Создание README и спецификаций
- [x] Базовые модели данных
- [x] Настройка проекта Rust
- [x] Система ошибок и конфигурации

### Этап 2: Базовая реализация ✅ ЗАВЕРШЕН (Декабрь 2024)
- [x] PostgreSQL репозитории (MessageRepository, ChannelRepository)
- [x] Redis сервис для кеширования и real-time функций
- [x] Полная бизнес-логика через сервисы
- [x] HTTP REST API со всеми эндпоинтами
- [x] Event-driven архитектура через mpsc каналы
- [x] Telegram Bot интеграция
- [x] Система конфигурации и graceful shutdown
- [x] Успешная компиляция всех модулей

### Этап 3: Инфраструктура и тестирование ✅ ЗАВЕРШЕН (Декабрь 2024)
- [x] Создание миграций PostgreSQL для схемы чата
- [x] Реализация gRPC API сервера
- [x] WebSocket сервер для real-time коммуникации
- [x] Полное интеграционное тестирование с БД
- [x] Нагрузочное тестирование API

**Результаты этапа 3:**
- ✅ Успешное выполнение всех интеграционных тестов (6/6 passed)
- ✅ Полная интеграция с PostgreSQL и Redis через Docker
- ✅ Тестирование всех компонентов: репозитории, сервисы, Redis
- ✅ Проверка миграций базы данных и целостности схемы
- ✅ End-to-end тестирование chat и voice сервисов

### Этап 4: Расширенные функции чата (2-3 недели)
- [ ] Edit/Delete/Reply/Forward сообщений
- [ ] Групповые чаты
- [ ] Статусы сообщений
- [ ] Типизация в реальном времени
- [ ] Поиск и фильтрация

### Этап 5: Голосовой чат (3-4 недели)
- [ ] Интеграция с pma-voice
- [ ] Proximity voice система
- [ ] Радиосвязь
- [ ] Телефонные звонки
- [ ] gRPC API для голоса

### Этап 6: Административные функции (1-2 недели)
- [ ] Модерация сообщений
- [ ] Система банов/мутов
- [ ] Логирование для админов
- [ ] Аналитика использования

### Этап 7: Клиентская интеграция (2-3 недели)
- [ ] FiveM client scripts
- [ ] Интеграция с UI
- [ ] Команды игры
- [ ] Тестирование

## Текущее состояние реализации

### ✅ Завершенные компоненты (Этапы 1-3)

**Этап 3 - Полностью завершен ✅ (Декабрь 2024):**
- **Интеграционное тестирование:** 6/6 тестов успешно пройдены
- **Миграции PostgreSQL:** Автоматическое создание полной схемы БД
- **Интеграция с инфраструктурой:** Docker PostgreSQL + Redis
- **Тестирование компонентов:** Все репозитории, сервисы, Redis функции
- **End-to-end тестирование:** Полный цикл отправки и получения сообщений

**Этап 2 - Базовая реализация (завершен):**

**Доменные модели:**
- `ChatMessage` - сообщения с поддержкой метаданных, reply, forward
- `ChatChannel` - каналы с типами (Global, Local, Radio, Phone, Group, Direct, OOC, Admin)
- `VoiceSession` - голосовые сессии с позиционированием
- `ChatEvent` - события для event-driven архитектуры

**Infrastructure слой:**
- `MessageRepository` - CRUD операции для сообщений в PostgreSQL
- `ChannelRepository` - управление каналами и участниками
- `RedisService` - кеширование, rate limiting, typing indicators, pub/sub
- `TelegramService` - интеграция с Telegram Bot API

**Application слой:**
- `ChatService` - отправка, редактирование, удаление сообщений, typing
- `ChannelService` - создание, присоединение, покидание каналов
- `VoiceService` - управление голосовыми сессиями с позиционированием
- `TelegramIntegrationService` - интеграция игрового чата с Telegram

**HTTP API эндпоинты:**
- `POST /channels/{id}/messages` - отправка сообщений
- `GET /channels/{id}/messages` - получение истории
- `PUT /messages/{id}` - редактирование сообщений
- `DELETE /messages/{id}` - удаление сообщений
- `POST /channels/{id}/typing/start` - начало печати
- `POST /channels/{id}/typing/stop` - окончание печати
- `POST /channels` - создание каналов
- `GET /channels/{id}` - получение канала
- `GET /channels` - список каналов пользователя
- `POST /channels/{id}/join` - присоединение к каналу
- `DELETE /channels/{id}/leave` - покидание канала
- `POST /voice/sessions` - запуск голосовой сессии
- `DELETE /voice/sessions/{id}` - завершение сессии
- `PUT /voice/sessions/{id}/position` - обновление позиции

**Система конфигурации:**
- Полная конфигурация через env переменные
- Поддержка всех компонентов (database, redis, telegram, voice, security)
- Graceful shutdown и structured logging

**Статус компиляции:**
- ✅ Успешная компиляция без критических ошибок
- ⚠️ 15 предупреждений о неиспользуемых импортах (нормально для этапа разработки)
- ✅ Все основные модули интегрированы корректно

### 🔄 Следующие шаги (Этап 4)

1. **Расширенные функции чата** - добавление новых функций
2. **Голосовой чат** - завершение интеграции с pma-voice
3. **Административные функции** - добавление новых функций
4. **Клиентская интеграция** - завершение интеграции с UI

## Примеры использования

### Отправка сообщения
```bash
curl -X POST http://localhost:8080/channels/123/messages \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Привет всем!",
    "message_type": "Text",
    "metadata": {
      "character_name": "John Smith",
      "location": {"x": 100.0, "y": 200.0, "z": 30.0}
    }
  }'
```

### Начало голосовой сессии
```bash
curl -X POST http://localhost:8080/voice/sessions \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "session_type": {"Proximity": null},
    "voice_range": "Normal",
    "position": {"x": 100.0, "y": 200.0, "z": 30.0}
  }'
```

### FiveM Integration
```lua
-- client.lua
RegisterCommand("say", function(source, args, rawCommand)
    local message = table.concat(args, " ")
    TriggerServerEvent("chat:sendMessage", {
        content = message,
        channel = "local",
        messageType = "IC"
    })
end, false)

RegisterCommand("me", function(source, args, rawCommand)
    local action = table.concat(args, " ")
    TriggerServerEvent("chat:sendMessage", {
        content = action,
        channel = "local", 
        messageType = "Action"
    })
end, false)
```

## Конфигурация

### Environment Variables
```env
# База данных
DATABASE_URL=postgresql://user:password@localhost:5432/gameverse_chat
REDIS_URL=redis://localhost:6379

# Сервер
HTTP_PORT=8080
GRPC_PORT=50053
WEBSOCKET_PORT=8081

# Голосовой чат
VOICE_ENABLED=true
PMA_VOICE_INTEGRATION=true
PROXIMITY_RANGE_MAX=50.0

# Каналы
MAX_MESSAGE_LENGTH=500
MAX_CHANNEL_PARTICIPANTS=100
MESSAGE_HISTORY_DAYS=30

# Безопасность
JWT_SECRET=your-secret-key
RATE_LIMIT_MESSAGES_PER_MINUTE=30

# Интеграции
AUTH_SERVICE_URL=http://localhost:8079
PLAYER_SERVICE_URL=http://localhost:8082
LOGGING_SERVICE_URL=http://localhost:50051
```

### Chat Channels Configuration
```toml
[channels.global]
enabled = true
rate_limit = 10  # сообщений в минуту
max_length = 200
roleplay_only = false

[channels.local]
enabled = true
distance = 20.0  # метры
rate_limit = 30
max_length = 500

[channels.radio]
enabled = true
frequencies = ["100.1", "200.5", "300.0"]
require_item = "radio"
rate_limit = 60

[channels.ooc]
enabled = true
prefix = "(( "
suffix = " ))"
color = "#888888"
```

## Тестирование

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
cargo test --test integration
```

### Load Testing
```bash
# Используем Apache Bench для нагрузочного тестирования
ab -n 1000 -c 10 -H "Authorization: Bearer $TOKEN" \
   http://localhost:8080/channels/123/messages
```

## Мониторинг

### Метрики
- Количество активных пользователей
- Сообщений в секунду
- Время отклика API
- Активные голосовые сессии
- Ошибки и исключения

### Логирование
```rust
use tracing::{info, warn, error};

info!(
    user_id = %user_id,
    channel_id = %channel_id,
    message_length = message.content.len(),
    "Message sent successfully"
);
```

### Health Check
```bash
curl http://localhost:8080/health
# {"status": "ok", "database": "connected", "redis": "connected"}
```

## Безопасность

### Защита от спама
- Rate limiting на уровне пользователя
- Детекция повторяющихся сообщений
- Временные блокировки

### Модерация контента
- Фильтр нецензурной лексики
- Система репортов
- Автоматическая модерация

### Приватность
- Шифрование личных сообщений
- Контроль доступа к каналам
- Автоматическое удаление старых сообщений

## Производительность

### Оптимизации
- Кеширование в Redis
- Пагинация сообщений
- Индексы базы данных
- Connection pooling

### Масштабирование
- Горизонтальное масштабирование через Redis
- Load balancing
- Database sharding по каналам
- CDN для медиа контента

---

Этот микросервис чата обеспечивает современную систему коммуникации для игровых серверов с полной поддержкой roleplay функций и интеграцией с голосовым чатом. 