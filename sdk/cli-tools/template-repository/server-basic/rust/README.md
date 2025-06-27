# {{plugin_name_camel}} (Server Plugin)

Демонстрационный серверный плагин на Rust, использующий GameVerse Compatibility Layer (FCL) для работы с событиями FiveM.

## Возможности
* Регистрирует событие `chatMessage` и выводит его аргументы в лог.
* Показывает интеграцию с `FiveMCompat`.

## Сборка
```bash
cargo build --release
```

## Установка
Собранная библиотека (`lib{{plugin_name}}.so` / `.dll` / `.dylib`) помещается в папку `plugins/` сервера GameVerse и указывается в `server-config.toml`. 