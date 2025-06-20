# Микросервис Управления Данными Игроков GameVerse Framework

## Статус
�� **В РАЗРАБОТКЕ** - Этап 1 завершен ✅

## Описание
Комплексный микросервис для управления всеми аспектами данных игроков в масштабируемых многопользовательских играх уровня **Rockstar Games (GTA Online)**. Включает расширенные социальные функции, рейтинговые системы, модерацию и безопасность.

## Архитектура

### 🎮 **Основные доменные модели**

#### **Базовые данные игрока**
- **Player** - основная модель игрока с профилем, статистикой, валютами
- **PlayerSession** - управление игровыми сессиями и онлайн статусом
- **Achievement** - система достижений и прогресса

#### **🤝 Социальные системы** (НОВОЕ)
- **Friendship** - система друзей с запросами и статусами
- **Clan/Organization** - кланы, команды, криминальные организации
  - Иерархия ролей (Leader, Officer, Veteran, Member, Recruit)
  - Типы кланов (Casual, Competitive, Roleplay, Criminal, Business)
- **PlayerBlock** - блок-лист токсичных игроков
- **PlayerPresence** - онлайн статус и текущая активность

#### **🏆 Рейтинговые системы** (НОВОЕ)
- **PlayerRating** - рейтинги по 12+ категориям игрового процесса
  - PvP: Deathmatch, Racing, LastTeamStanding, Adversary
  - Кооп: Heists, Missions, Survival
  - Специальные: Arena, GunRunning, Import/Export
- **Leaderboard** - таблицы лидеров (Global, Regional, Friends, Clan)
- **Tournament** - турниры и соревновательные события
- **Season** - сезонная система с наградами

#### **🛡️ Модерация и безопасность** (НОВОЕ)
- **PlayerPunishment** - система наказаний (ban, mute, kick, warning)
- **PlayerReport** - отчеты о нарушениях и токсичном поведении
- **PunishmentAppeal** - апелляции на наказания
- **PlayerTrustScore** - система доверия и антифрод
- **AntiCheatInfo** - интеграция с системами античитов
- **ChatFilter** - фильтрация сообщений

#### **⭐ Репутация и карма** (НОВОЕ)  
- **PlayerReputation** - система кармы и репутации по категориям
- **ReputationTitle** - титулы различной редкости
- **PlayerVoting** - голосование игроков друг за друга
- **PlayerInfluence** - система влияния в сообществе
- **CommunityLeader** - лидеры сообщества

### 🔧 **Сервисы и репозитории**

#### **Основные сервисы**
- **PlayerService** - CRUD операции с игроками
- **SocialService** - управление друзьями, кланами, блок-листом
- **RatingService** - рейтинги, лидерборды, турниры
- **ModerationService** - наказания, отчеты, система доверия
- **ReputationService** - карма, титулы, голосование

#### **Репозитории**
- **PlayerRepository** - базовые данные игроков
- **SocialRepository** - друзья, кланы, присутствие
- **RatingRepository** - рейтинги и турниры
- **ModerationRepository** - наказания и отчеты
- **ReputationRepository** - репутация и титулы

### 🌐 **API Эндпоинты**

#### **REST API**

**👤 Players Management**
- `GET /players/{id}` - профиль игрока
- `PUT /players/{id}` - обновление профиля
- `POST /players/{id}/experience` - добавление опыта
- `POST /players/{id}/currency` - управление валютами

**🤝 Social Features**
- `POST /players/{id}/friends/request` - запрос дружбы
- `PUT /friends/{friendship_id}/accept` - принять дружбу
- `GET /players/{id}/friends` - список друзей
- `POST /clans` - создание клана
- `POST /clans/{id}/invite` - приглашение в клан
- `GET /clans/{id}/members` - участники клана
- `POST /players/{id}/block` - заблокировать игрока

**🏆 Ratings & Competitions**
- `GET /players/{id}/ratings` - рейтинги игрока
- `POST /ratings/update` - обновление рейтинга
- `GET /leaderboards/{category}` - таблица лидеров
- `GET /tournaments` - список турниров
- `POST /tournaments/{id}/register` - регистрация в турнире

**🛡️ Moderation & Safety**
- `POST /reports` - создание отчета о нарушении
- `GET /players/{id}/punishments` - наказания игрока
- `POST /appeals` - создание апелляции
- `GET /players/{id}/trust-score` - рейтинг доверия

**⭐ Reputation System**
- `GET /players/{id}/reputation` - репутация игрока
- `POST /reputation/vote` - голосование за игрока
- `GET /players/{id}/titles` - титулы игрока
- `GET /community/leaders` - лидеры сообщества

#### **gRPC API**
- **PlayerDataService** - межсервисное взаимодействие
- **SocialService** - быстрое получение социальных связей
- **RatingService** - рейтинги для матчмейкинга
- **ModerationService** - проверка наказаний
- **ReputationService** - система репутации

## 🔗 **Интеграция с другими сервисами**

### Зависимости
- **Authentication** - проверка пользователей
- **AntiCheat** - данные о подозрительной активности
- **Matchmaking** - рейтинги для подбора игроков

### Предоставляет данные для
- **Chat** - система мутов и фильтрации
- **Economy** - валюты и торговые ограничения
- **World State** - онлайн статус и присутствие
- **Analytics** - метрики поведения игроков

## 🌍 **Кроссплатформенность** (НОВОЕ)
- Связка аккаунтов разных платформ
- Синхронизация прогресса между устройствами
- Кроссплатформенные друзья и кланы
- Унифицированные лидерборды

## 🔒 **Безопасность**
- Система доверия с машинным обучением
- Автоматическое обнаружение токсичности
- Защита от накрутки рейтингов
- Энкриптированные данные наказаний

## 📊 **Масштабируемость**
Спроектировано для поддержки:
- **1,000,000+** активных игроков
- **10,000+** одновременных сессий  
- **100,000+** кланов и организаций
- **1,000,000+** записей рейтингов

## 🔧 **Технологический стек**
- **Веб-сервер**: Axum 0.6
- **База данных**: PostgreSQL + SQLX 
- **Кеширование**: Redis (горячие данные)
- **Сериализация**: Serde JSON
- **Валидация**: validator + regex
- **Логирование**: tracing

## 📈 **План разработки**

### ✅ Этап 1: Базовые модели (ЗАВЕРШЕН)
- [x] Создание Cargo.toml с зависимостями
- [x] Основная модель Player с валютами и опытом
- [x] Социальные модели (друзья, кланы, блокировка)
- [x] Рейтинговые системы и турниры
- [x] Модерация и система наказаний  
- [x] Репутация и карма
- [x] Система ошибок PlayerDataError

### ⏳ Этап 2: Репозитории и база данных
- [ ] PostgreSQL миграции для всех таблиц
- [ ] Репозитории для всех доменных моделей
- [ ] Индексы для производительности
- [ ] Redis кеширование горячих данных

### ⏳ Этап 3: Бизнес-логика сервисов
- [ ] PlayerService - основная логика игроков
- [ ] SocialService - друзья, кланы, присутствие
- [ ] RatingService - рейтинги и турниры
- [ ] ModerationService - наказания и отчеты
- [ ] ReputationService - карма и титулы

### ⏳ Этап 4: REST API handlers
- [ ] HTTP endpoints для всех операций
- [ ] Middleware аутентификации и авторизации
- [ ] Валидация запросов и ограничения скорости
- [ ] Стандартизированные ответы API

### ⏳ Этап 5: gRPC API и интеграция
- [ ] Protobuf определения
- [ ] gRPC сервер и сервисы
- [ ] Интеграция с другими микросервисами

### ⏳ Этап 6: Расширенные функции
- [ ] Машинное обучение для системы доверия
- [ ] Автоматическая модерация контента
- [ ] Кроссплатформенная синхронизация
- [ ] Аналитика и метрики

### ⏳ Этап 7: Тестирование и оптимизация
- [ ] Нагрузочные тесты (1M+ игроков)
- [ ] Интеграционные тесты
- [ ] Производительность и оптимизация
- [ ] Документация API

## 🚀 **Требования для разработки**
```bash
# База данных PostgreSQL
createdb gameverse_player_data

# Redis для кеширования
redis-server

# Переменные окружения
export DATABASE_URL="postgresql://username:password@localhost/gameverse_player_data"
export REDIS_URL="redis://localhost:6379"
export ANTI_CHEAT_API_KEY="your_anticheat_key"
```

## 🎯 **Следующие действия**
1. **Миграции БД** - создание схемы для всех новых таблиц
2. **Индексы производительности** - оптимизация для быстрых запросов
3. **Redis стратегия** - кеширование онлайн статуса и рейтингов
4. **Репозитории** - реализация CRUD операций

---
**Создан:** Декабрь 2024  
**Обновлен:** Декабрь 2024 (добавлены системы уровня Rockstar Games)  
**Ответственный:** GameVerse Team  
**Приоритет:** Критический (основа для всех социальных функций) 