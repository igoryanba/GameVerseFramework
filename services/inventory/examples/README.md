# Примеры использования сервиса инвентаря

В этой директории находятся примеры использования сервиса инвентаря GameVerse Framework.

## gRPC клиент

Пример `grpc_client.rs` демонстрирует, как использовать gRPC API сервиса инвентаря из Rust-кода:

```bash
# Запустите сервис инвентаря
cd ../
./scripts/dev.sh

# В другом терминале запустите пример
cd examples/
cargo run --example grpc_client
```

Этот пример выполняет следующие операции:
- Подключается к сервису инвентаря по gRPC
- Получает информацию об инвентаре
- Добавляет предмет в инвентарь
- Получает список предметов в инвентаре
- Обновляет предмет
- Удаляет предмет
- Получает список шаблонов предметов

## Использование через REST API

После запуска сервиса вы также можете использовать REST API:

```bash
# Получить список предметов в инвентаре
curl http://localhost:8081/inventories/{inventory_id}/items

# Добавить предмет в инвентарь
curl -X POST http://localhost:8081/inventories/{inventory_id}/items \
  -H "Content-Type: application/json" \
  -d '{"template_id": "uuid", "position": 0, "quantity": 1}'

# Создать новый инвентарь
curl -X POST http://localhost:8081/inventories \
  -H "Content-Type: application/json" \
  -d '{
    "owner_id": "player-123",
    "owner_type": "player",
    "inventory_type": "player",
    "max_weight": 100.0,
    "max_slots": 20,
    "name": "Рюкзак игрока"
  }'

# Обновить предмет
curl -X PUT http://localhost:8081/items/{item_id} \
  -H "Content-Type: application/json" \
  -d '{
    "durability": 90.0,
    "metadata": {
      "equipped": "true"
    }
  }'

# Переместить предмет между инвентарями
curl -X POST http://localhost:8081/items/{item_id}/move/{target_inventory_id} \
  -H "Content-Type: application/json" \
  -d '{
    "target_position": 5
  }'
```

Для более детального примера использования REST API вы можете запустить JavaScript клиент:

```bash
# Установите зависимости
npm install node-fetch

# Запустите сервис инвентаря
cd ../
./scripts/dev.sh

# В другом терминале запустите пример
node examples/rest_client.js
```

Этот пример демонстрирует:
- Создание шаблонов предметов
- Создание инвентаря
- Добавление предметов в инвентарь
- Обновление предметов
- Стакирование предметов
- Разделение стаков

## Шаблоны предметов

Для создания шаблонов предметов используйте API:

```bash
# Создать новый шаблон предмета
curl -X POST http://localhost:8081/templates \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Меч",
    "description": "Острый меч",
    "weight": 5.0,
    "stackable": false,
    "max_stack": 1,
    "max_durability": 100.0,
    "icon": "sword.png",
    "category": "weapons",
    "properties": {
      "damage": "10",
      "attack_speed": "1.2"
    }
  }'
``` 