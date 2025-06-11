# GameVerse CLI Tools

## Описание
Комплексный инструмент командной строки для разработки плагинов в экосистеме GameVerse Framework. Предоставляет современный developer experience, превосходящий возможности FiveM инструментов.

## Ключевые возможности

### 🚀 Современная архитектура
- **Async-first**: Полностью асинхронная архитектура на базе Tokio
- **Type Safety**: Строгая типизация на Rust для надежности
- **Cross-platform**: Поддержка Windows, Linux, macOS
- **Interactive UX**: Прогресс-бары, диалоги, rich output

### 🛠️ Управление плагинами
- **Template Engine**: Handlebars шаблоны с custom helpers
- **Multi-language**: Поддержка Rust, TypeScript, Lua
- **Hot Reload**: Автоматическая перезагрузка при изменениях
- **Cross-compilation**: Автоматическая сборка для разных платформ

### 📦 Marketplace интеграция
- **Plugin Discovery**: Поиск и установка плагинов
- **Version Management**: Семантическое версионирование
- **Publishing**: Публикация в marketplace
- **Dependencies**: Автоматическое управление зависимостями

## Установка

### Из исходного кода
```bash
cd GameVerseFramework/sdk/cli-tools
cargo build --release
cp target/release/gameverse /usr/local/bin/  # Linux/macOS
```

### Установка shell completions
```bash
# Bash
gameverse completions bash > ~/.bash_completions/gameverse

# Zsh
gameverse completions zsh > ~/.zsh_completions/_gameverse

# Fish
gameverse completions fish > ~/.config/fish/completions/gameverse.fish
```

## Использование

### Создание нового плагина

#### Интерактивный режим
```bash
gameverse plugin new my-economy
# CLI предложит выбрать шаблон и язык программирования
```

#### Быстрое создание
```bash
# Rust плагин с economy шаблоном
gameverse plugin new my-economy --template economy --language rust

# TypeScript плагин с basic шаблоном
gameverse plugin new my-admin --template admin --language typescript

# Lua плагин
gameverse plugin new my-roleplay --template roleplay --language lua
```

### Сборка плагина

#### Локальная сборка
```bash
cd my-economy
gameverse plugin build
```

#### Cross-compilation
```bash
# Сборка для всех платформ
gameverse plugin build --cross-compile all

# Сборка для конкретных платформ
gameverse plugin build --cross-compile windows,linux

# Release сборка с оптимизацией
gameverse plugin build --target release --optimize
```

### Тестирование
```bash
# Юнит-тесты
gameverse plugin test

# Интеграционные тесты
gameverse plugin test --integration

# Performance тесты
gameverse plugin test --performance
```

### Развертывание
```bash
# На dev сервер
gameverse plugin deploy --server localhost:8080 --environment dev

# На production с подтверждением
gameverse plugin deploy --server prod.gameverse.dev --environment prod

# Форсированное развертывание
gameverse plugin deploy --force
```

### Упаковка и публикация
```bash
# Создание пакета
gameverse plugin package --output my-economy-v1.0.0.gvp

# Публикация в marketplace
gameverse marketplace publish

# Установка из marketplace
gameverse marketplace install economy-advanced
```

## Шаблоны плагинов

### Доступные шаблоны
- **basic**: Минимальный плагин для начинающих
- **economy**: Экономическая система с валютой и банками
- **roleplay**: Ролевые возможности (работы, организации)
- **admin**: Административные команды и панель
- **custom**: Пустой шаблон для продвинутых разработчиков

### Поддерживаемые языки
- **Rust**: Максимальная производительность и безопасность
- **TypeScript**: Быстрая разработка с типизацией
- **Lua**: Простота и совместимость с FiveM

## Конфигурация

### Файл конфигурации (gameverse.toml)
```toml
[plugin]
default_template = "basic"
default_language = "rust"
auto_reload = true
default_author = "Your Name"
default_license = "MIT"

[server]
default_host = "localhost"
default_port = 8080
use_tls = false
timeout = 30

[marketplace]
url = "https://marketplace.gameverse.dev"
auto_update_check = true

[build]
default_targets = ["windows", "linux", "macos"]
optimization = "release"
parallel = true

[templates]
repository = "https://github.com/gameverse/plugin-templates"
cache_duration = 24
```

### Environment Variables
```bash
# Server settings
export GAMEVERSE_SERVER_HOST=localhost
export GAMEVERSE_SERVER_PORT=8080
export GAMEVERSE_API_KEY=your-api-key

# Plugin settings
export GAMEVERSE_PLUGIN_AUTHOR="Your Name"
export GAMEVERSE_PLUGIN_LICENSE=MIT

# Marketplace
export GAMEVERSE_MARKETPLACE_TOKEN=your-token
```

## Команды

### Plugin Management
- `gameverse plugin new <name>` - Создать новый плагин
- `gameverse plugin build` - Собрать плагин
- `gameverse plugin test` - Тестировать плагин
- `gameverse plugin deploy` - Развернуть плагин
- `gameverse plugin package` - Упаковать плагин
- `gameverse plugin install` - Установить плагин
- `gameverse plugin list` - Список плагинов
- `gameverse plugin validate` - Валидация плагина
- `gameverse plugin watch` - Отслеживание изменений

### Server Management
- `gameverse server start` - Запуск сервера
- `gameverse server stop` - Остановка сервера
- `gameverse server restart` - Перезапуск сервера
- `gameverse server status` - Статус сервера
- `gameverse server logs` - Логи сервера

### Marketplace
- `gameverse marketplace search <query>` - Поиск плагинов
- `gameverse marketplace info <plugin>` - Информация о плагине
- `gameverse marketplace install <plugin>` - Установка плагина
- `gameverse marketplace publish` - Публикация плагина
- `gameverse marketplace login` - Авторизация

### Utilities
- `gameverse init <name>` - Инициализация проекта
- `gameverse version` - Информация о версии
- `gameverse completions <shell>` - Shell completions

## Преимущества над FiveM

### Developer Experience
| Функция | GameVerse CLI | FiveM |
|---------|---------------|-------|
| Template Engine | ✅ Handlebars | ❌ Ручное создание |
| Type Safety | ✅ Rust/TS | ⚠️ Только Lua |
| Hot Reload | ✅ Автоматический | ❌ Ручной restart |
| Cross-compilation | ✅ Автоматическая | ❌ Ручная настройка |
| Package Manager | ✅ Встроенный | ❌ Внешние решения |
| Interactive CLI | ✅ Progress bars | ❌ Базовый вывод |
| Marketplace | ✅ Интегрированный | ⚠️ Сторонний |

### Architecture Benefits
- **Memory Safety**: Rust предотвращает segfaults и утечки памяти
- **Async Performance**: Tokio runtime для высокой производительности
- **Modern Tooling**: Cargo ecosystem vs устаревшие Lua tools
- **CI/CD Ready**: Встроенная поддержка автоматизации

## Разработка

### Структура проекта
```
sdk/cli-tools/
├── src/
│   ├── commands/          # CLI команды
│   │   ├── plugin.rs      # Управление плагинами
│   │   ├── server.rs      # Управление сервером
│   │   └── marketplace.rs # Marketplace интеграция
│   ├── templates/         # Template engine
│   ├── config.rs          # Конфигурация
│   ├── utils/            # Утилиты
│   └── main.rs           # Entry point
├── Cargo.toml            # Dependencies
└── README.md             # Документация
```

### Зависимости
- **clap**: Modern CLI framework
- **tokio**: Async runtime
- **handlebars**: Template engine
- **indicatif**: Progress bars
- **dialoguer**: Interactive prompts
- **reqwest**: HTTP client
- **serde**: Serialization
- **git2**: Git integration

### Тестирование
```bash
# Юнит-тесты
cargo test

# Интеграционные тесты
cargo test --test integration

# Performance benchmarks
cargo bench
```

## Roadmap

### v0.2.0 - Template Ecosystem
- [ ] Официальный template repository
- [ ] Template validation
- [ ] Custom helper functions
- [ ] Template versioning

### v0.3.0 - Advanced Features
- [ ] Plugin hot reload implementation
- [ ] Debugging tools integration
- [ ] Performance profiling
- [ ] Plugin dependencies resolution

### v0.4.0 - Marketplace
- [ ] Полная marketplace интеграция
- [ ] Plugin reviews и ratings
- [ ] Автоматические обновления
- [ ] Premium plugins support

### v1.0.0 - Production Ready
- [ ] Стабильный API
- [ ] Comprehensive documentation
- [ ] Plugin migration tools
- [ ] Enterprise features

## Поддержка

### Issues и Bug Reports
- GitHub Issues: [gameverse/gameverse-framework](https://github.com/gameverse/gameverse-framework/issues)
- Discord: #cli-tools канал

### Contributing
1. Fork repository
2. Create feature branch
3. Implement changes
4. Add tests
5. Submit pull request

### License
MIT License - см. [LICENSE](LICENSE) файл

---

**GameVerse CLI Tools** - современное решение для разработки игровых плагинов, превосходящее возможности устаревших FiveM инструментов. 