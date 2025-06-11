# test-basic-plugin - GameVerse Plugin

A test-basic-plugin plugin for GameVerse Framework

## 🚀 Описание

Это базовый плагин для GameVerse Framework, созданный с использованием шаблона `basic` на языке Rust. Плагин демонстрирует основные возможности GameVerse Plugin System с поддержкой dynamic loading и hot reload.

## ⚡ Особенности

- ✅ **Dynamic Loading** - загрузка как динамическая библиотека (.dll/.so/.dylib)
- ✅ **Hot Reload** - обновление без перезапуска сервера
- ✅ **FFI Interface** - безопасный C ABI для интеграции с GameVerse Core
- ✅ **Event Handling** - обработка игровых событий (подключение/отключение игроков)
- ✅ **Command System** - система команд с аргументами
- ✅ **Async Support** - полная поддержка асинхронного программирования
- ✅ **Memory Safety** - автоматическое управление памятью через Rust

## 🛠️ Установка и сборка

### Быстрый старт
```bash
# Сборка плагина
cargo build --release

# Запуск тестов
cargo test

# Проверка качества кода
cargo clippy

# Форматирование кода
cargo fmt
```

### Cross-platform сборка
```bash
# Windows
cargo build --release --target x86_64-pc-windows-msvc

# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# macOS
cargo build --release --target x86_64-apple-darwin
```

## 🎯 Структура проекта

```
test-basic-plugin/
├── Cargo.toml              # Конфигурация Rust проекта
├── gameverse.toml          # Конфигурация GameVerse плагина
├── src/
│   └── lib.rs              # Основной код плагина
├── tests/                  # Интеграционные тесты
└── README.md               # Документация
```

## 📋 API Reference

### GameVerse Plugin Trait

```rust
pub trait GameVersePlugin: Send + Sync {
    fn initialize(&mut self) -> Result<()>;
    fn on_player_connect(&self, player_id: u32) -> Result<()>;
    fn on_player_disconnect(&self, player_id: u32) -> Result<()>;
    fn handle_command(&self, command: &str, args: &[&str]) -> Result<String>;
    fn shutdown(&mut self) -> Result<()>;
}
```

### FFI Functions

- `create_plugin()` - создание экземпляра плагина
- `initialize_plugin()` - инициализация плагина
- `on_player_connect(player_id)` - обработка подключения игрока
- `destroy_plugin()` - завершение работы плагина
- `get_plugin_version()` - получение версии плагина
- `get_plugin_name()` - получение названия плагина

## 🎮 Использование

### Команды плагина

- `/hello` - приветствие от плагина
- `/info` - информация о плагине

### Пример расширения функциональности

```rust
impl GameVersePlugin for Plugin {
    fn handle_command(&self, command: &str, args: &[&str]) -> Result<String> {
        match command {
            "hello" => Ok("Привет из плагина!".to_string()),
            "stats" => self.get_player_stats(args),
            "teleport" => self.teleport_player(args),
            _ => Ok(format!("Неизвестная команда: {}", command)),
        }
    }
}
```

## 🔧 Конфигурация

Плагин настраивается через файл `gameverse.toml`:

```toml
[plugin]
name = "test-basic-plugin"
version = "1.0.0"
description = "A test-basic-plugin plugin for GameVerse Framework"
abi_version = "0.2.0"

[plugin.permissions]
required = ["basic_access"]

[build.rust]
crate_type = ["cdylib"]
edition = "2021"
```

## 🧪 Тестирование

```bash
# Единичные тесты
cargo test

# Интеграционные тесты с GameVerse
cargo test --features integration

# Тесты производительности
cargo test --release --features performance
```

## 📈 Performance

Благодаря использованию Rust и GameVerse Dynamic Plugin Loading:

- ⚡ **Native Performance** - без overhead интерпретации
- 🔥 **Hot Reload** - обновление за < 200ms
- 💾 **Memory Safety** - отсутствие утечек памяти
- 🔒 **Thread Safety** - безопасная многопоточность

## 🤝 Contributing

1. Форкните репозиторий
2. Создайте feature ветку
3. Внесите изменения
4. Добавьте тесты
5. Создайте Pull Request

## 📄 Лицензия

[Укажите лицензию]

---

**Создано с помощью GameVerse CLI Tools v0.2.0** 🚀 