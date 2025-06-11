# GameVerse Plugin Templates

Официальный репозиторий шаблонов для разработки плагинов GameVerse Framework.

## Описание

Этот репозиторий содержит готовые шаблоны для быстрого создания плагинов различных типов. Шаблоны интегрированы с GameVerse CLI Tools и поддерживают все основные языки программирования.

## Доступные шаблоны

### 📦 Basic Plugin Template
**Путь:** `basic/`
**Языки:** Rust, TypeScript, Lua
**Описание:** Минимальный функциональный плагин для изучения API

**Возможности:**
- Базовая структура плагина
- Примеры основных API calls
- Конфигурационные файлы
- Документация и примеры

### 💰 Economy Plugin Template 
**Путь:** `economy/`
**Языки:** Rust, TypeScript, Lua
**Описание:** Полноценная система экономики с банковскими операциями

**Возможности:**
- Банковские счета игроков
- Система транзакций
- ATM интерфейсы
- Интеграция с базой данных
- REST API для external integration

### 🎭 Roleplay Plugin Template
**Путь:** `roleplay/`  
**Языки:** Rust, TypeScript, Lua
**Описание:** Комплексная система ролевых игр

**Возможности:**
- Система персонажей
- Профессии и навыки
- Интерактивные объекты
- Chat commands и emotes
- Inventory integration

### 🛡️ Admin Plugin Template
**Путь:** `admin/`
**Языки:** Rust, TypeScript
**Описание:** Административные инструменты для сервера

**Возможности:**
- Player management (kick, ban, teleport)
- Server monitoring
- Performance metrics
- Log analysis
- Anti-cheat integration

### 🔌 API Integration Template
**Путь:** `api-integration/`
**Языки:** Rust, TypeScript
**Описание:** Интеграция с external services и APIs

**Возможности:**
- REST API client/server
- gRPC integration
- WebSocket connections
- Database operations
- Microservices communication

## Использование

### С GameVerse CLI

```bash
# Создание плагина из шаблона
gameverse plugin new my-economy --template economy --language rust

# Список доступных шаблонов
gameverse plugin templates list

# Обновление шаблонов
gameverse plugin templates update
```

### Ручная загрузка

```bash
# Клонирование репозитория
git clone https://github.com/gameverse/plugin-templates.git

# Использование specific шаблона
cp -r plugin-templates/basic/rust/ ./my-plugin/
```

## Структура шаблона

Каждый шаблон имеет следующую структуру:

```
template-name/
├── rust/                    # Rust версия шаблона
│   ├── gameverse.toml       # Конфигурация плагина
│   ├── Cargo.toml           # Rust dependencies
│   ├── src/
│   │   ├── lib.rs           # Основной entry point
│   │   └── plugin.rs        # Plugin implementation
│   └── README.md            # Документация
├── typescript/              # TypeScript версия
│   ├── gameverse.toml
│   ├── package.json
│   ├── tsconfig.json
│   ├── src/
│   │   ├── index.ts
│   │   └── plugin.ts
│   └── README.md
├── lua/                     # Lua версия (если доступна)
│   ├── gameverse.toml
│   ├── fxmanifest.lua       # FiveM compatibility
│   ├── server/
│   │   └── main.lua
│   ├── client/
│   │   └── main.lua
│   └── README.md
└── template.toml            # Метаданные шаблона
```

## Template Configuration (`template.toml`)

The `template.toml` file is crucial for defining how your template behaves. Here's an example structure:

```toml
[template]
name = "my-custom-template"
description = "A brief description of what this template does."
author = "Your Name <your.email@example.com>"
version = "1.0.0"
gameverse_version = ">=0.2.0" # Compatible GameVerse CLI version
category = "gameplay" # e.g., gameplay, utility, admin, economy
tags = ["tag1", "tag2"]

# Defines which languages this template supports and their specific configurations
[template.languages]
rust = { min_version = "1.70", features = ["database-integration", "web-api"] }
typescript = { min_version = "5.0", frameworks = ["react", "vue"] }
lua = { compatibility = "fivem", version = "5.4" }

# Defines external dependencies or services this template might rely on
[template.dependencies]
database = "postgresql" # Example: "postgresql", "mysql", "mongodb"
cache = "redis"         # Example: "redis", "memcached"
ui_framework = "react"  # Example: "react", "vue", "svelte", "none"
api_style = "grpc"      # Example: "grpc", "rest", "none"

# Defines variables that the user will be prompted for when using the template
[template.variables.project_author_name]
description = "Author's name for the new project"
type = "string" # string, integer, boolean
default = "My GameVerse Project"
validation = "^[a-zA-Z0-9_\s]+$" # Optional regex validation

[template.variables.starting_cash]
description = "Initial cash for players"
type = "integer"
default = 1000
min = 0
max = 1000000

[template.variables.enable_database_feature]
description = "Enable database integration?"
type = "boolean"
default = true
```

Please refer to the GameVerse CLI documentation for the most up-to-date specifications on `template.toml`.

## Разработка новых шаблонов

### Требования

1. **Multi-language support**: Rust и TypeScript обязательны
2. **GameVerse integration**: Использование GameVerse API
3. **Documentation**: Comprehensive README и code comments  
4. **Hot reload support**: Корректная работа с hot reload системой
5. **Best practices**: Следование GameVerse coding standards

### Процесс создания

1. Создайте директорию для шаблона
2. Реализуйте версии для каждого языка
3. Добавьте `template.toml` с метаданными
4. Протестируйте с GameVerse CLI
5. Создайте Pull Request

### Handlebars Variables

Все шаблоны поддерживают Handlebars переменные:

```rust
// В template файлах
pub struct {{upper plugin_name}}Plugin {
    name: String,
}

impl GameVersePlugin for {{upper plugin_name}}Plugin {
    fn initialize(&mut self, context: &PluginContext) -> PluginResult<()> {
        info!("Initializing {{plugin_name}} plugin v{{plugin_version}}");
        // Implementation
    }
}
```

**Доступные переменные:**
- `{{plugin_name}}` - название плагина
- `{{plugin_author}}` - автор плагина  
- `{{plugin_version}}` - версия плагина
- `{{plugin_description}}` - описание плагина
- `{{current_year}}` - текущий год
- `{{language}}` - выбранный язык программирования

**Helpers:**
- `{{upper text}}` - UPPERCASE conversion
- `{{lower text}}` - lowercase conversion
- `{{snake_case text}}` - snake_case conversion
- `{{kebab-case text}}` - kebab-case conversion

## Contributing

Мы приветствуем контрибуции от сообщества! См. [CONTRIBUTING.md](CONTRIBUTING.md) для guidelines.

### Типы контрибуций

- **Новые шаблоны**: Создание template для новых use cases
- **Language support**: Добавление поддержки новых языков
- **Bug fixes**: Исправление ошибок в существующих шаблонах
- **Documentation**: Улучшение документации и примеров
- **Performance**: Оптимизация template processing

## Лицензия

MIT License - см. [LICENSE](LICENSE) для подробностей.

## Поддержка

- **Discord**: [GameVerse Community](https://discord.gg/gameverse)
- **GitHub Issues**: [Template Repository Issues](https://github.com/gameverse/plugin-templates/issues)
- **Documentation**: [gameverse.dev/templates](https://gameverse.dev/templates)

---

**Часть экосистемы GameVerse Framework для современной разработки игровых серверов** 🚀