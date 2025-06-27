# ⚡️ Quick Start: первый плагин GameVerse

> 5-минутное руководство, как создать и запустить простой серверный и клиентский плагин на базе готовых шаблонов.

## Требования

* Rust ≥1.70 и `cargo`
* Node.js ≥18 (для клиентских плагинов)
* GameVerse CLI (из репозитория `sdk/cli-tools`, команда `cargo install --path .`)
* Запущенный GameVerse Server ≥0.3.0 (см. `gameverse server start`)

---

## 1. Создание серверного плагина (Rust)

```bash
# создать проект из шаблона server-basic
cd ~/projects/my-gv-plugins
gameverse plugin new hello-server \
    --template server-basic \
    --language rust

cd hello-server
# сборка (динамическая библиотека)
gameverse plugin build --release
```

Файл `src/lib.rs` содержит пример регистрации события `chatMessage` через `FiveMCompat`.

### Установка
1. Скопируйте скомпилированную библиотеку `target/release/libhello_server.(so|dll|dylib)` в папку `plugins/` вашего сервера.
2. Добавьте запись в `server-config.toml`:

```toml
[plugins]
hello-server = "plugins/libhello_server.so"
```

3. Перезапустите сервер (`gameverse server restart`).

---

## 2. Создание клиентского плагина (TypeScript)

```bash
cd ~/projects/my-gv-plugins
gameverse plugin new hello-client \
    --template client-basic \
    --language typescript

cd hello-client
npm install
npm run build # transpile TS → dist/
```

### Установка
1. Скопируйте каталог плагина в клиентскую директорию `plugins/hello-client`.
2. Убедитесь, что `gameverse.toml` находится рядом с собранным `dist/index.js`.
3. Запустите клиентскую сборку или подключитесь к серверу.

---

## 3. Проверка работы

* В игровом чате напишите сообщение — серверный плагин выведет его в лог (`chatMessage: [...]`).
* После загрузки клиента в лог клиента появится `Client plugin hello-client loaded`, а сервер получит событие `clientReady`.

---

## 4. Что почитать дальше

* `docs/FCL_LIMITATIONS_v0.1.md` — текущие возможности слоя совместимости.
* `README_ru.md#плагины` — общая структура проектов плагинов.
* `gameverse plugin --help` — весь набор подкоманд CLI. 