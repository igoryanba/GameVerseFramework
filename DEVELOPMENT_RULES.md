# Правила разработки GameVerse Framework

## Общие принципы

### Ключевые ценности
1. **Производительность** - фреймворк должен обеспечивать максимальную производительность для игрового опыта
2. **Стабильность** - избегаем критических сбоев и потери данных
3. **Удобство использования** - интуитивно понятные API и интерфейсы
4. **Безопасность** - защита от вредоносных скриптов, читов и атак
5. **Масштабируемость** - возможность расширения и адаптации под разные игры
6. **Совместимость и Миграция** - разработка инструментов и подходов для облегчения перехода разработчиков с существующих платформ (например, FiveM).
7. **Расширенная поддержка игр** - обеспечение полноценной поддержки для Grand Theft Auto V и Red Dead Redemption 2, включая специфичные для игр функции и нативные API.

### Методология разработки
- **Agile/Scrum** с двухнедельными спринтами
- Ежедневные стендапы (15 минут) для синхронизации
- Планирование спринта в начале каждого цикла
- Ретроспектива в конце спринта
- Непрерывная интеграция и доставка (CI/CD)

## Стандарты кодирования

### Общие стандарты
- **Именование**: CamelCase для типов и функций, snake_case для переменных и модулей
- **Документация**: документируйте все публичные API и сложные алгоритмы
- **Комментарии**: объясняйте "почему", а не "что" делает код
- **Тесты**: обязательны для всех новых функций
- **Рефакторинг**: регулярное улучшение существующего кода
- **Учет специфики игры**: При разработке модулей и функций всегда учитывать специфику целевой игры (GTA V / RDR2), если применимо.

### Rust
- Строгое соблюдение стандартов Rust (используем rustfmt и clippy)
- Избегаем использования `unsafe` кода без веской причины
- Явно указываем ошибки (Result) вместо паники
- Избегаем блокирующих операций в асинхронном контексте
- Используем `#[derive]` для стандартных трейтов

```rust
// Пример правильного кода
pub struct Player {
    id: u64,
    name: String,
    position: Vector3<f32>,
}

impl Player {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            position: Vector3::zeros(),
        }
    }
    
    pub fn teleport(&mut self, position: Vector3<f32>) -> Result<(), GameError> {
        // Валидация перед изменением состояния
        if !is_valid_position(&position) {
            return Err(GameError::InvalidPosition);
        }
        
        self.position = position;
        Ok(())
    }
}
```

### C++
- Следуем стандарту C++20
- Предпочитаем использование STL вместо собственных реализаций
- Используем умные указатели для управления памятью
- Следуем правилу "Rule of Zero/Five"
- Делаем константами все, что не должно изменяться

```cpp
// Пример правильного кода
class GameClient {
public:
    explicit GameClient(std::string_view name);
    
    // Деструктор может быть виртуальным, если класс используется полиморфно
    virtual ~GameClient() = default;
    
    // Делаем методы константными, где это возможно
    [[nodiscard]] const std::string& getName() const noexcept { return name_; }
    
    // Явно запрещаем копирование, если это необходимо
    GameClient(const GameClient&) = delete;
    GameClient& operator=(const GameClient&) = delete;
    
    // Разрешаем перемещение
    GameClient(GameClient&&) noexcept = default;
    GameClient& operator=(GameClient&&) noexcept = default;
    
private:
    std::string name_;
    std::unique_ptr<Connection> connection_;
};
```

### TypeScript
- Используем строгую типизацию (`strict: true` в tsconfig.json)
- Предпочитаем интерфейсы для API и типы для данных
- Используем асинхронность через async/await
- Следуем принципам функционального программирования где возможно

```typescript
// Пример правильного кода
interface PlayerData {
  id: string;
  name: string;
  health: number;
  position: [number, number, number];
}

async function getPlayerData(id: string): Promise<PlayerData> {
  try {
    const response = await api.get(`/players/${id}`);
    return response.data;
  } catch (error) {
    console.error(`Failed to fetch player data: ${error}`);
    throw new Error(`Could not retrieve player data for ID: ${id}`);
  }
}
```

### Lua
- Используем локальные переменные где возможно
- Организуем код в модули
- Избегаем глобальных переменных
- Комментируем сложные части для новичков

```lua
-- Пример правильного кода
local Vector3 = require("math.vector3")

local Player = {}
Player.__index = Player

function Player.new(id, name)
    local self = setmetatable({}, Player)
    self.id = id
    self.name = name
    self.position = Vector3.new(0, 0, 0)
    return self
end

function Player:teleport(x, y, z)
    -- Валидация позиции
    if not isValidPosition(x, y, z) then
        return false, "Invalid position"
    end
    
    self.position = Vector3.new(x, y, z)
    return true
end

return Player
```

### 🔥 Dynamic Plugin Loading (Новый стандарт)

#### Общие принципы безопасности
- **FFI Safety First**: Все операции с dynamic loading должны быть обернуты в unsafe блоки с проверками
- **Memory Management**: Используем RAII принципы для автоматического освобождения ресурсов
- **ABI Compatibility**: Всегда проверяем совместимость перед загрузкой библиотеки
- **Thread Safety**: Все операции должны быть Send + Sync compliant

#### Стандарты для динамических плагинов

```rust
// Правильная реализация плагина
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn GameVersePlugin {
    let plugin = Box::new(MyPlugin::new());
    Box::into_raw(plugin)
}

#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut dyn GameVersePlugin) {
    if !plugin.is_null() {
        unsafe {
            let _ = Box::from_raw(plugin);
        }
    }
}

impl GameVersePlugin for MyPlugin {
    fn initialize(&mut self, context: &PluginContext) -> PluginResult<()> {
        // Инициализация с проверкой контекста
        if context.version != expected_version() {
            return Err(PluginError::IncompatibleVersion);
        }
        Ok(())
    }
    
    fn finalize(&mut self) -> PluginResult<()> {
        // Освобождение ресурсов
        self.cleanup_resources()?;
        Ok(())
    }
}
```

#### Cross-Platform совместимость
- **Windows**: `.dll` библиотеки с `__declspec(dllexport)`
- **Linux**: `.so` библиотеки с `-fPIC` флагом
- **macOS**: `.dylib` библиотеки с правильными rpath

```toml
# Правильная конфигурация для cross-platform плагина
[plugin]
name = "cross-platform-example"
version = "1.0.0"

[build]
windows = { library = "target/release/plugin.dll" }
linux = { library = "target/release/libplugin.so" }  
macos = { library = "target/release/libplugin.dylib" }

[dependencies.abi]
version = "1.0.0"  # Обязательная ABI версия
symbols = ["create_plugin", "destroy_plugin"]  # Требуемые символы
```

#### Тестирование динамических плагинов

```rust
#[cfg(test)]
mod plugin_tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_plugin_lifecycle() {
        let temp_dir = TempDir::new().unwrap();
        let plugin_path = temp_dir.path().join("test_plugin.dll");
        
        // Создаем тестовую библиотеку
        create_test_library(&plugin_path).unwrap();
        
        // Тестируем загрузку
        let library = unsafe { Library::new(&plugin_path).unwrap() };
        let create_fn: Symbol<extern "C" fn() -> *mut dyn GameVersePlugin> = 
            unsafe { library.get(b"create_plugin").unwrap() };
            
        let plugin_ptr = create_fn();
        assert!(!plugin_ptr.is_null());
        
        // Безопасная очистка
        unsafe { let _ = Box::from_raw(plugin_ptr); }
    }
}
```

#### Performance Guidelines
- **Lazy Loading**: Загружаем библиотеки только при необходимости
- **Symbol Caching**: Кешируем часто используемые символы
- **Memory Pooling**: Используем memory pools для часто создаваемых объектов
- **Profiling**: Обязательный profiling для production плагинов

### 🛠️ CLI Tools Development Standards (v0.2.0) ✅ **ЗАВЕРШЕНО**

### ⚙️ Native Generator Development Standards (v0.1.0 - В ПРОЦЕССЕ)

#### Текущий прогресс
- **Парсинг**:
    - ✅ Успешный парсинг локальных Markdown-файлов нативов FiveM (из `natives-master`).
    - ⚠️ Присутствуют незначительные предупреждения от Markdown-парсера (например, `Unrecognized YAML_directive "returns"`), требующие анализа и возможного устранения для полной чистоты.
- **Генерация кода (Rust)**:
    - ✅ Улучшенная генерация Rust-оболочек для нативных функций. **Проект успешно собирается без предупреждений.**
    - ✅ Корректная обработка строковых типов (Rust `String` для `char*`).
    - ✅ Корректная обработка указателей (например, `*mut Vector3` -> `&mut Vector3`, `Any*` -> `*mut std::ffi::c_void`).
    - ✅ Корректная обработка `Vector3` как по значению, так и по указателю.
    - ✅ Реализованы хелперы Handlebars для генерации специфичных для Rust типов и вызовов FFI.
    - ✅ Устранены основные предупреждения `dead_code` и проблемы с неиспользуемыми переменными/импортами в генераторе и генерируемом коде.
- **Типы**:
    - ✅ Базовая структура `NativeType` для представления типов из FiveM.
    - ✅ **Улучшенная обработка `NativeType::Any`**: Внедрены `FunctionCallback` и `Opaque(Option<String>)` для более точного представления и обработки ранее неизвестных типов. ✅
    - ✅ **Реализована базовая поддержка `NativeType::Array`**: параметры в Rust видны как `Vec<T>`, FFI получает указатель; возвращаемые значения FFI типа массив (`*mut InnerRawType`) транслируются в `NativeResult<*mut InnerRawType>`. ✅
    - ✅ **Улучшенная обработка массивов с динамической длиной**: Добавлена возможность указывать out-параметр, содержащий длину возвращаемого массива, через внешний конфигурационный файл (`native_configs.toml`). Это позволяет корректно обрабатывать функции вида `int* GET_SOME_ARRAY(int* length_output)`. Поле `return_array_length_out_param` добавлено в `NativeFunction`. ✅
    - ✅ Реализована базовая генерация кода для `FunctionCallback` и `Opaque`:
        - Автоматическое добавление типа `FunctionCallback` и генерация псевдотипов для `Opaque` с использованием имени.
        - Продвинутая поддержка специфичных сигнатур `FunctionCallback` вынесена в отдельную задачу.
    - 🚧 Требуется улучшение обработки `NativeType::Array` для специфичных случаев (например, `char[]`, `const char*[]`) и автоматического определения размера, где это возможно, помимо уже реализованной конфигурации через out-параметр.

#### Планы на ближайшую итерацию
- **Парсер**:
    - [ ] Исследовать и устранить оставшиеся предупреждения Markdown-парсера для повышения надежности извлечения данных.
    - [ ] Расширить возможности парсера для чтения и применения конфигураций из `native_configs.toml` для переопределения поведения генерации нативов. Это включает:
        - **Переопределение типов**: Возможность указать конкретный Rust-тип для возвращаемого значения или параметров функции, если автоматически определенный тип неточен или требует уточнения (например, `EntityHandle`, `MyCustomStruct`).
        - **Переименование параметров**: Задание кастомных имен для параметров в генерируемых Rust-обертках для улучшения читаемости или избежания конфликтов с ключевыми словами Rust.
        - **Управление поведением функции**: Опции для принудительной маркировки безопасной обертки как `unsafe fn`, вставки пользовательского кода в пролог/эпилог функции.
        - **Подсказки для `NativeType::Any`**: Предоставление более конкретных указаний для типов, которые парсер определил как `Any` (например, `any_type_hint = "PointerToPlayerData"`).
        - **Пользовательские трансформации**: Возможность указать вспомогательные функции для кастомной обработки (маршалинга/анмаршалинга) параметров или возвращаемых значений.
        - **Значения по умолчанию**: Определение значений по умолчанию для опциональных параметров в Rust-обертках.
        - Конфигурация должна также поддерживать уже реализованный `return_array_length_out_param` для массивов.
        - **Пример структуры в TOML для одной функции:**
          ```toml
          [NATIVES.YOUR_NATIVE_FUNCTION_NAME]
          return_type_override = "SpecificReturnType"
          mark_safe_wrapper_unsafe = false
          prologue_code = "/* custom setup code */"
          # ... другие опции уровня функции

          [[NATIVES.YOUR_NATIVE_FUNCTION_NAME.parameters]]
          original_name = "p0" # Имя параметра из документации или авто-генерации
          new_rust_name = "meaningful_name"
          type_override = "SpecificParamType"
          any_type_hint = "HintForAnyType"
          # transform_input_with = "custom_input_transformer_fn"
          # default_value_for_optional = "42"
          ```
- **Генератор кода**:
    - [ ] Улучшить обработку `NativeType::Array` в генераторе Rust, включая корректное преобразование типов и передачу параметров FFI для более сложных случаев (массивы строк, `char[]`), а также исследование возможностей по автоматическому определению размера, дополняя существующий механизм конфигурации через out-параметр.
- **Тестирование**:
    - [ ] Расширить тесты для `native-generator` для покрытия различных сценариев парсинга и генерации кода, включая все поддерживаемые `NativeType`, массивы (включая массивы с размером через out-параметр, `char[]`) и сложные случаи `Any`.
- **Документация**:
    - [ ] Обновить документацию по `native-generator` с описанием текущих возможностей, включая обработку `Any`, и ограничений.

#### Стандарты для Native Generator
- **Корректность типов**: Генерируемые оболочки должны максимально точно соответствовать семантике оригинальных нативных функций FiveM, обеспечивая безопасность типов в Rust.
- **Производительность**: Генерируемый код не должен вносить значительных накладных расходов по сравнению с прямым вызовом FFI.
- **Удобство использования**: Сгенерированные функции должны быть интуитивно понятны и просты в использовании для разработчиков плагинов.
- **Покрытие**: Стремиться к покрытию максимального количества нативных функций FiveM.
- **Качество кода**: Генерируемый код должен соответствовать общим стандартам Rust (rustfmt, clippy).

#### Новое (февраль 2025):
- ✅ **Полная поддержка массивов (`NativeType::Array`)**: параметры-массивы передаются как `Vec<T>` или `[T; N]`, возвращаемые массивы с out-параметром длины — как `NativeResult<Vec<T>>`.
- ✅ **Override-конфиги**: поддержка кастомных маршалеров, типов, имён, значений по умолчанию, трансформаций.
- ✅ **Тесты**: устойчивы к форматированию, покрывают все типы массивов, строки, edge-cases.
- 🚧 **Any/Callback/Opaque**: базовая поддержка реализована, продвинутая — в процессе доработки.

##### Пример override-конфига:
```toml
# Rust-specific overrides
[NATIVES.GET_LABEL]
return_type_override = "char[32]"

[[NATIVES.GET_LABEL.parameters]]
original_name = "buffer"
type_override = "char[32]"
marshal_with = "marshal_char_buffer"

# TypeScript-specific overrides
[[override]]
name = "GET_PLAYER_NAME" # Original native function name
typescript_name_override = "getPlayerNameAsync" # Override the function name in TypeScript
typescript_return_type_override = "Promise<string>" # Override the return type in TypeScript

parameter_typescript_overrides = [
  { original_name = "player", typescript_name_override = "playerId", typescript_type_override = "PlayerHandle" },
  { original_name = "anotherParam", typescript_type_override = "MyCustomType" } 
]

[[override]]
name = "SOME_OTHER_FUNCTION"
parameter_typescript_overrides = [
  { original_name = "dataBuffer", typescript_type_override = "ArrayBuffer" }
]
```

##### Пример Handlebars-шаблона:
```handlebars
pub fn {{snake_case function_data.name}}_safe(
    {{#each function_data.parameters as |param|}}
        {{param.name}}: {{rust_type param.param_type func_context=function_data}},
    {{/each}}
) -> NativeResult<{{rust_type function_data.return_type is_return_type_flag=true func_context=function_data}}> {
    // ...
    {{#if length_temp_var_name}}
    let len = {{length_temp_var_name}} as usize;
    let slice = unsafe { std::slice::from_raw_parts(result as *const {{array_inner_rust_type}}, len) };
    Ok(slice.iter().cloned().collect::<Vec<_>>())
    {{/if}}
}
```

##### Стандарты тестирования:
- Тесты должны проверять корректность генерации для всех поддерживаемых типов, массивов, out-параметров, кастомных override.
- Использовать debug-вывод сгенерированного кода для диагностики.
- Проверять ключевые фрагменты кода, а не только полные сигнатуры (устойчивость к форматированию).

## Поддержка массивов и строковых буферов (февраль 2025)

- **char[] с фиксированным размером**: генерируется как `[u8; N]` (или `[char; N]` при необходимости), null-terminated — как `String`.
- **const char*[N] (массив строк)**: генерируется как `[String; N]` или `Vec<String>`, маршалится как массив указателей на C-строки.
- **Vector3[8], MyStruct[COUNT] (массивы структур)**: генерируется как `[Vector3; 8]` или `Vec<Vector3>`, маршалится как массив структур.
- **Edge-cases**: пустой размер (`[]`), null-terminated, динамический размер, массивы структур с динамическим размером.
- **Динамические массивы с out-параметром длины**: поддерживаются через конфиг (см. `return_array_length_out_param`), генерируются как `Vec<T>`/`NativeResult<Vec<T>>`.
- **Override-конфиги**: поддерживаются кастомные маршалеры (`marshal_with`), переопределения типов и имён, значения по умолчанию, трансформации.
- **Тесты**: устойчивы к форматированию, проверяют ключевые фрагменты кода, покрывают все типы массивов, строки, edge-cases.
- **Any/Callback/Opaque**: базовая поддержка реализована, продвинутая — в процессе доработки.

##### Пример override-конфига:
```toml
[NATIVES.GET_LABEL]
return_type_override = "char[32]"

[[NATIVES.GET_LABEL.parameters]]
original_name = "buffer"
type_override = "char[32]"
marshal_with = "marshal_char_buffer"
```

## Рабочий процесс с Git

### Ветвление
- **main** - стабильная ветка, готовая к выпуску
- **develop** - основная ветка разработки
- **feature/*** - для новых функций
- **bugfix/*** - для исправления ошибок
- **release/*** - для подготовки релиза

### Коммиты
- Коммиты должны быть атомарными (одно изменение = один коммит)
- Сообщения коммитов следуют Conventional Commits:
  - `feat:` для новых функций
  - `fix:` для исправлений
  - `docs:` для документации
  - `style:` для форматирования
  - `refactor:` для рефакторинга
  - `test:` для добавления тестов
  - `chore:` для обслуживания

Пример: `feat(core): add player teleportation function`

### Pull Requests
- PR должен включать тесты для новой функциональности
- PR должен проходить все CI/CD проверки
- PR требует как минимум одного ревью от другого разработчика
- PR должен быть связан с задачей в системе отслеживания

## Тестирование

### Типы тестов
- **Юнит-тесты**: тестирование отдельных функций и классов
- **Интеграционные тесты**: тестирование взаимодействия компонентов
- **Системные тесты**: тестирование всей системы в сборе
- **Нагрузочные тесты**: проверка производительности под нагрузкой

### Требования к тестам
- Тесты должны быть независимыми друг от друга
- Тесты должны быть детерминированными
- Тесты должны выполняться быстро
- Каждый тест должен проверять одну конкретную функциональность

```rust
// Пример теста для Rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_player_teleport_valid_position() {
        let mut player = Player::new(1, "TestPlayer".to_string());
        let position = Vector3::new(100.0, 200.0, 300.0);
        
        let result = player.teleport(position);
        
        assert!(result.is_ok());
        assert_eq!(player.position, position);
    }
    
    #[test]
    fn test_player_teleport_invalid_position() {
        let mut player = Player::new(1, "TestPlayer".to_string());
        let position = Vector3::new(f32::INFINITY, 0.0, 0.0);
        
        let result = player.teleport(position);
        
        assert!(result.is_err());
        match result {
            Err(GameError::InvalidPosition) => (),
            _ => panic!("Expected InvalidPosition error"),
        }
    }
}
```

## Безопасность

### Принципы безопасности
- Следуем принципу "наименьших привилегий"
- Валидируем все входные данные
- Используем безопасные функции и библиотеки
- Проводим регулярные аудиты безопасности
- Используем шифрование для чувствительных данных

### Проверки кода
- Статический анализ кода
- Динамический анализ при выполнении
- Проверка зависимостей на уязвимости
- Фаззинг для критических компонентов

## Организация проекта

### Структура репозитория
```
GameVerseFramework/
├── core/               # Ядро фреймворка на Rust/C++
├── scripting/          # Скриптовые движки и интеграции
├── networking/         # Сетевая инфраструктура
├── admin-panel/        # Веб-интерфейс администратора
├── client/             # Клиентская часть
├── server/             # Серверная часть
├── sdk/                # Инструменты для разработчиков
├── examples/           # Примеры использования
├── docs/               # Документация
└── tests/              # Тесты
```

### Система сборки
- Используем Cargo для Rust-компонентов
- CMake для C++ компонентов
- Yarn/npm для JavaScript/TypeScript
- Docker для контейнеризации

## Документация

### Типы документации
- **API-документация**: автоматически генерируется из комментариев в коде
- **Руководства**: пошаговые инструкции для конкретных задач
- **Концептуальная документация**: объяснение архитектуры и дизайн-решений
- **Примеры**: рабочие примеры использования компонентов

### Требования к документации
- Документация должна быть актуальной
- Обновляйте документацию вместе с кодом
- Используйте понятный язык и примеры
- Документируйте и API и внутреннюю реализацию

## Обработка ошибок

### Принципы обработки ошибок
- Используем Result/Option в Rust, исключения в C++
- Логируем все ошибки с контекстом
- Рассматриваем ошибки как часть API
- Возвращаем информативные сообщения об ошибках

### Логирование
- Используем уровни логирования (DEBUG, INFO, WARN, ERROR, FATAL)
- Логи должны содержать контекст ошибки
- Избегаем логирования чувствительной информации
- Используем структурированное логирование

## Производительность

### Оптимизация
- Профилирование перед оптимизацией
- Оптимизация критических путей
- Мониторинг производительности в реальном времени
- Бенчмарки для ключевых компонентов

### Метрики
- Время отклика сервера
- Использование CPU/RAM
- Сетевой трафик
- Количество игроков/объектов

## Коммуникация и сотрудничество

### Каналы общения
- **GitHub Issues**: для задач и ошибок
- **Discord/Slack**: для ежедневной коммуникации
- **Wiki**: для документации и гайдов
- **Встречи**: для планирования и ретроспектив

### Правила общения
- Уважайте время друг друга
- Будьте конкретными в запросах и отзывах
- Предоставляйте контекст при обсуждении проблем
- Делитесь знаниями и опытом

## Роли и ответственности

### Основные роли
- **Технический директор**: архитектурные решения, технический надзор
- **Лид-разработчики**: руководство командами, код-ревью
- **Разработчики**: реализация компонентов
- **QA-инженеры**: тестирование и обеспечение качества
- **DevOps-инженеры**: инфраструктура и развертывание
- **Технические писатели**: документация

### Матрица ответственности
| Роль | Разработка | Тестирование | Документация | Развертывание |
|------|------------|--------------|--------------|---------------|
| Тех. директор | Утверждает | Утверждает | Проверяет | Утверждает |
| Лид-разработчик | Ведет | Проверяет | Проверяет | Поддерживает |
| Разработчик | Выполняет | Поддерживает | Выполняет | Информируется |
| QA-инженер | Консультирует | Выполняет | Поддерживает | Проверяет |
| DevOps-инженер | Поддерживает | Поддерживает | Консультирует | Выполняет |
| Тех. писатель | Информируется | Информируется | Выполняет | Информируется |

## Управление версиями

### Семантическое версионирование
- **Major (X.0.0)**: несовместимые изменения API
- **Minor (0.X.0)**: новые функции с обратной совместимостью
- **Patch (0.0.X)**: исправления ошибок

### Циклы релизов
- Регулярные релизы каждые 2-3 месяца
- Исправления безопасности по мере необходимости
- Предварительные версии для тестирования новых функций
- LTS-версии для стабильной поддержки

## Заключение

Эти правила направлены на обеспечение высокого качества кода, эффективной командной работы и создание стабильного, производительного фреймворка. Правила могут эволюционировать с развитием проекта, и все участники приглашаются к обсуждению и улучшению процесса разработки. 

### 🔄 **FiveM Migration & Compatibility Standards** ✨ **НОВЫЕ СТАНДАРТЫ**

#### **QBCore Resource Conversion Standards**
- **Automatic fxmanifest.lua → gameverse.toml**: полная автоматизация с validation
- **MySQL → PostgreSQL migration**: schema + data preservation с optimization
- **CEF → WebAssembly UI**: memory-efficient conversion с performance improvements
- **Lua → TypeScript transpiler**: type-safe code generation с backwards compatibility
- **ox_lib compatibility**: автоматическая интеграция popular FiveM libraries

#### **Native Functions Compatibility**
```rust
// Стандарт для FiveM natives wrapper
pub trait FiveMNativeWrapper {
    // Полная совместимость с 7000+ GTA V natives
    fn get_player_ped(player_id: i32) -> EntityId;
    fn get_vehicle_engine_health(vehicle: EntityId) -> f32;
    fn set_ped_component_variation(ped: EntityId, component_id: i32, drawable_id: i32, texture_id: i32, palette_id: i32);
    
    // Type-safe versions для GameVerse
    fn get_player_ped_safe(player_id: PlayerId) -> Result<PedEntity, GameError>;
    fn get_vehicle_engine_health_safe(vehicle: VehicleEntity) -> Result<EngineHealth, GameError>;
}

// FiveM Event System Compatibility
pub struct FiveMEventBridge {
    // RegisterNetEvent, TriggerEvent, TriggerServerEvent совместимость
    // RegisterCommand, ExecuteCommand обертки
    // ESX/QBCore event mapping
}
```

#### **Resource Migration Pipeline**
```toml
# Automated QBCore resource conversion
[migration]
source_type = "qbcore"
target_type = "gameverse"

[migration.lua_conversion]
enabled = true
target_language = "typescript"
preserve_logic = true
add_type_safety = true

[migration.database]
source = "mysql"
target = "postgresql" 
optimize_queries = true
preserve_data = true

[migration.ui]
source = "cef"
target = "webassembly"
framework = "react"
optimize_memory = true
```

#### **FiveM API Emulation Layer**
```rust
// Совместимость с популярными FiveM функциями
mod fivem_compat {
    // ESX Framework compatibility
    pub fn get_shared_object() -> ESXObject;
    pub fn trigger_server_callback(name: &str, args: Vec<Value>) -> Result<Value>;
    
    // QBCore Framework compatibility  
    pub fn get_core_object() -> QBCoreObject;
    pub fn get_player_data() -> PlayerData;
    pub fn trigger_server_event(event: &str, args: Vec<Value>);
    
    // ox_lib compatibility
    pub fn show_context_menu(menu: ContextMenu) -> Result<()>;
    pub fn show_text_ui(text: &str, options: TextUIOptions) -> Result<()>;
}
```

#### **Performance Migration Standards**
- **Benchmark Requirements**: 5x+ improvement over original FiveM resource
- **Memory Optimization**: 80%+ reduction в UI memory usage через WebAssembly
- **Load Time**: 10x faster resource loading через modern bundling
- **Script Performance**: 10-50x improvement через WASM execution

#### **Testing Standards для Migration**
```rust
#[cfg(test)]
mod migration_tests {
    #[test]
    fn test_qbcore_resource_conversion() {
        let fivem_resource = load_fivem_resource("qb-banking");
        let converted = convert_to_gameverse(&fivem_resource).unwrap();
        
        // Валидация полной функциональности
        assert_eq!(converted.functionality_preservation, 100.0);
        assert!(converted.performance_improvement >= 5.0);
        assert!(converted.memory_reduction >= 0.8);
    }
    
    #[test] 
    fn test_native_function_compatibility() {
        // Тестирование совместимости всех natives
        for native in get_all_fivem_natives() {
            let result = gameverse_native_wrapper(&native);
            assert!(result.is_ok());
            assert_eq!(result.behavior, native.original_behavior);
        }
    }
}
```

### 🎮 **Game Integration Standards (GTA V / RDR2)**

#### **GTA V Native Integration**
```rust
// Стандарты интеграции с игровыми API
pub mod gta_v_integration {
    use game_natives::*;
    
    // Безопасная обертка для GTA V natives
    pub struct GameNativeWrapper {
        validation_enabled: bool,
        performance_monitoring: bool,
    }
    
    impl GameNativeWrapper {
        // Все 7000+ natives с type safety
        pub fn get_player_position(&self, player: PlayerId) -> Result<Vector3, NativeError> {
            self.validate_player_id(player)?;
            let pos = unsafe { GET_ENTITY_COORDS(GET_PLAYER_PED(player.0), true) };
            Ok(Vector3::new(pos.x, pos.y, pos.z))
        }
        
        pub fn spawn_vehicle(&self, model_hash: u32, position: Vector3, heading: f32) -> Result<VehicleId, NativeError> {
            self.validate_model_hash(model_hash)?;
            self.validate_position(position)?;
            
            // Safe vehicle spawning с error handling
            let vehicle = unsafe {
                REQUEST_MODEL(model_hash);
                while !HAS_MODEL_LOADED(model_hash) {
                    thread::sleep(Duration::from_millis(1));
                }
                CREATE_VEHICLE(model_hash, position.x, position.y, position.z, heading, true, false)
            };
            
            Ok(VehicleId(vehicle))
        }
    }
}
```

#### **RDR2 Support Standards**
```rust
// Red Dead Redemption 2 специфичные стандарты
pub mod rdr2_integration {
    // Native functions для RDR2
    // Horse mechanics, camp systems, etc.
    // Weather и time period systems
    // Bounty и law enforcement systems
}
```

#### **Cross-Game Compatibility**
```toml
# Конфигурация для multi-game support
[game_support]
gta_v = { enabled = true, version = "latest" }
rdr2 = { enabled = true, version = "latest" }
gta_vi = { enabled = false, planned = true } # Future support

[native_compatibility]
auto_detection = true
fallback_behavior = "error"  # or "warn" or "ignore"
performance_monitoring = true
```

### 🛡️ **Security Standards для Game Integration**

#### **Anti-Cheat Integration**
```rust
// Стандарты безопасности для игровых интеграций
pub mod security {
    pub struct AntiCheatValidator {
        // Валидация all player actions
        // Behavioral analysis для cheat detection
        // Memory protection для game data
    }
    
    impl AntiCheatValidator {
        pub fn validate_player_action(&self, action: PlayerAction) -> ValidationResult {
            // Server-side validation всех действий
            // Impossible action detection
            // Speed/teleport/aimbot detection
        }
        
        pub fn monitor_script_behavior(&self, script: &GameScript) -> BehaviorReport {
            // Script behavior analysis
            // Malicious code detection
            // Performance impact monitoring
        }
    }
}
```

#### **Game Data Protection**
```rust
// Защита игровых данных от модификации
pub mod data_protection {
    // Encrypted player data storage
    // Checksum validation для game state
    // Anti-tampering measures
    // Secure communication с game servers
}
```

### 📊 **Performance Standards для Game Integration**

#### **Benchmarking Requirements**
- **Frame Rate Impact**: < 5% FPS reduction
- **Memory Usage**: < 100MB additional per plugin
- **Network Latency**: < 50ms for all operations
- **Loading Time**: < 2 seconds for any resource

#### **Optimization Guidelines**
- **Batch Operations**: Group multiple natives calls
- **Async Processing**: Non-blocking operations где possible
- **Memory Pooling**: Reuse objects to reduce GC pressure
- **Lazy Loading**: Load resources only when needed

### 🔧 **Development Tools для Game Integration**

#### **Required Tools**
- **Game State Inspector**: Real-time monitoring игрового состояния
- **Native Function Debugger**: Step-through debugging для natives
- **Performance Profiler**: CPU/Memory/Network analysis
- **Anti-Cheat Tester**: Validation против bypass attempts

#### **IDE Integration Requirements**
- **Syntax Highlighting**: Для game-specific APIs
- **Auto-completion**: Все natives с parameter info
- **Error Detection**: Compile-time validation неправильных calls
- **Live Preview**: Real-time changes в game world

### 📋 **Quality Assurance для Game Features**

#### **Testing Matrix**
```rust
// Comprehensive testing для игровых интеграций
#[cfg(test)]
mod game_integration_tests {
    #[test]
    fn test_all_gta_v_natives() {
        // Автоматическое тестирование всех 7000+ natives
        for category in ["PLAYER", "VEHICLE", "PED", "WEAPON"] {
            test_native_category(category);
        }
    }
    
    #[test]
    fn test_performance_benchmarks() {
        // Performance testing против установленных стандартов
        let metrics = run_performance_test();
        assert!(metrics.fps_impact < 0.05);
        assert!(metrics.memory_usage < 100_000_000); // 100MB
        assert!(metrics.latency < Duration::from_millis(50));
    }
    
    #[test]
    fn test_anti_cheat_effectiveness() {
        // Тестирование против известных cheat methods
        let cheats = load_known_cheat_methods();
        for cheat in cheats {
            assert!(anti_cheat_system.detects(&cheat));
        }
    }
}
```

### 🚀 **Continuous Integration для Game Features**

#### **Automated Testing Pipeline**
- **Daily Native Validation**: Проверка всех game natives
- **Performance Regression Tests**: Monitoring производительности changes
- **Anti-Cheat Effectiveness**: Regular testing против new threats
- **Cross-Platform Validation**: Windows/Linux testing

#### **Deployment Standards**
- **Staged Rollout**: Gradual deployment для game servers
- **Rollback Capability**: Instant revert при проблемах
- **Monitoring**: Real-time health checks для game integration
- **Emergency Response**: 24/7 support для critical issues

---

**🎯 Эти стандарты обеспечивают seamless migration с FiveM и superior game integration, делая GameVerse Framework самой advanced платформой для game modding в индустрии.** 

## 🔥 CLI Tools Development Standards (v0.2.0)

Данный раздел описывает стандарты и лучшие практики для разработки с использованием и для GameVerse CLI Tools.

### 1. Система Шаблонов (Template Repository System)
- **Структура Шаблона:**
    - Каждый шаблон должен содержать `template.toml` для метаданных (имя, описание, версия, автор, поддерживаемые языки, целевые игры).
    - Четкое разделение на секции (например, `server`, `client`, `shared`, `ui`).
    - Файлы шаблонов (`*.hbs`) должны использовать Handlebars синтаксис для параметризации.
- **Официальные Шаблоны:**
    - `basic`: Минимальный шаблон для быстрого старта (Rust, TypeScript, Lua).
    - `economy`: Шаблон с базовой экономической системой.
    - Должны регулярно обновляться и тестироваться.
- **Пользовательские Шаблоны:**
    - CLI должен поддерживать загрузку шаблонов из локальных и удаленных Git-репозиториев.
    - Предоставить четкие инструкции по созданию и валидации пользовательских шаблонов.
- **Поддержка мульти-игровых шаблонов:**
    - Шаблоны должны проектироваться с возможностью адаптации под GTA V и RDR2.
    - Использование условных блоков в `template.toml` или в файлах `.hbs` для включения специфичного для игры кода или конфигураций.

### 2. Команды для Управления Плагинами (Extended Plugin Commands)
- **`gameverse plugin new <name> --template <template_name> [--game <gta5|rdr2>]`**:
    - Генерация нового плагина на основе шаблона.
    - Флаг `--game` позволяет инициализировать проект с учетом специфики игры (например, подключение соответствующего SDK нативных функций).
- **`gameverse plugin build [--release]`**: Сборка плагина.
- **`gameverse plugin test [unit|integration|performance]`**: Запуск тестов.
    - Стандартная структура директорий для тестов (`tests/unit`, `tests/integration`, `tests/performance`).
    - Интеграция с тестовыми фреймворками для Rust, TS, Lua.
- **`gameverse plugin deploy <environment>`**: Развертывание плагина.
    - Поддержка различных окружений (development, staging, production).
    - Конфигурация через `deploy.toml`.
- **`gameverse plugin package`**: Упаковка плагина для дистрибуции.
- **`gameverse plugin watch`**: Hot reload для плагинов.
- **`gameverse plugin validate`**: Проверка манифеста и структуры плагина.
- **`gameverse plugin install <name_or_url>`**: Установка зависимостей или плагинов из Marketplace.
- **`gameverse plugin list`**: Отображение списка установленных плагинов.

### 3. Конфигурация и Манифесты
- **`plugin-manifest.toml`**: Стандартизированный формат для описания плагина (имя, версия, автор, зависимости, точки входа, нативные функции).
- **Конфигурационные файлы (`.toml`)**: Приоритетный формат для конфигураций.
- **Переменные окружения**: Поддержка для переопределения настроек.

### 4. Тестирование и Качество Кода
- **Unit-тесты**: Обязательны для всех ключевых модулей и функций CLI.
- **Интеграционные тесты**: Проверка взаимодействия команд CLI с файловой системой, Git, и т.д.
- **Линтинг и форматирование**: Использование `rustfmt` и `clippy` для Rust-кода.

### 5. Обработка Ошибок и Логирование
- **Человеко-читаемые ошибки**: CLI должен выводить понятные сообщения об ошибках.
- **Уровни логирования**: Поддержка различных уровней детализации логов (`--verbose`, `-vv`).
- **Статусы выхода**: Использование стандартных кодов выхода.

## Взаимодействие с существующими платформами (FiveM)

GameVerse Framework стремится не только предложить технологически превосходящую платформу, но и обеспечить комфортный переход для разработчиков с других систем, в первую очередь FiveM.

### 1. Анализ и Обучение
- **Регулярный Мониторинг:** Постоянный анализ обновлений, функций и подходов, используемых в FiveM и его экосистеме (включая популярные фреймворки типа ESX, QBCore).
- **Выявление Лучших Практик:** Идентификация удачных решений в FiveM, которые могут быть адаптированы или улучшены в GameVerse.
- **Изучение Потребностей Сообщества:** Анализ запросов и проблем разработчиков FiveM для предложения более эффективных решений в GameVerse.

### 2. Стратегия Совместимости
- **Слой Совместимости с FiveM (FCL - FiveM Compatibility Layer):**
    - Цель: Предоставить разработчикам возможность запускать существующие FiveM ресурсы на GameVerse с минимальными изменениями или без них.
    - **Поддержка Ключевых API:** Реализация наиболее часто используемых нативных функций и серверных/клиентских событий FiveM. Приоритет отдается функциям, необходимым для работы популярных фреймворков.
    - **Форматы Ресурсов:** Обеспечение совместимости с основными форматами файлов конфигурации ресурсов FiveM (`fxmanifest.lua` или `__resource.lua`), насколько это возможно и целесообразно.
    - **Трансляция вызовов:** Где это возможно, вызовы API FiveM будут транслироваться в эквивалентные вызовы API GameVerse.
- **Ограничения Совместимости:** Четко документировать, какие аспекты FiveM не будут поддерживаться или будут иметь ограниченную поддержку, и почему. Акцент на использовании преимуществ GameVerse.

### 3. Инструменты Миграции
- **CLI Утилиты для Миграции:**
    - `gameverse migrate fivem-resource <path_to_resource> [--target-game <gta5|rdr2>`: Инструмент для анализа ресурса FiveM и автоматического преобразования его структуры и конфигурационных файлов к формату GameVerse, где это возможно.
    - Генерация отчетов о несовместимостях и необходимых ручных правках.
- **Руководства по Миграции:** Подробная документация и туториалы по переносу проектов с FiveM на GameVerse, включая типичные проблемы и их решения.
- **Конвертеры Скриптов (Экспериментально):** Исследование возможности создания инструментов для частичной автоматической конвертации Lua-скриптов в TypeScript или предоставления более безопасных Lua-окружений.

## Управление нативными функциями и поддержка игр

Эффективное управление нативными функциями и обеспечение поддержки для различных игр (GTA V, RDR2) является ключевым аспектом GameVerse Framework.

### 1. Централизованный Репозиторий и Генератор Нативов
- **`native-generator`:** Основной инструмент для сбора, обработки и предоставления нативных функций.
    - **Источники Данных:** Парсинг официальной документации (например, FiveM Natives DB, RDR2 Natives DB), локальных файлов `.meta`/`.xml` из игровых дистрибутивов, а также пользовательских определений.
    - **Поддержка GTA V и RDR2:** Генератор должен четко разделять и предоставлять наборы нативных функций для каждой игры.
    - **Формат Вывода:** Генерация кода на Rust, TypeScript, Lua для SDK, обеспечивающего типобезопасный доступ к нативам.
- **Кеширование и Обновление:** Механизмы для кеширования сгенерированных нативов и их периодического обновления.

### 2. Стандарты Описания Нативных Функций
- **Формат Определения:** Использование TOML или аналогичного структурированного формата для описания нативных функций, если они добавляются вручную или не могут быть автоматически распарсены. Включает имя, хеш, параметры (имя, тип), тип возвращаемого значения, категорию, краткое описание и целевую игру.
- **Документация:** Каждая нативная функция, доступная через SDK GameVerse, должна иметь описание, пример использования и указание на поддерживаемые игры.

### 3. Многоигровая Архитектура
- **Разделение по Играм:** В коде фреймворка и SDK четко разделять логику, специфичную для GTA V и RDR2.
- **Общий API / Специфичные Расширения:** Стремиться к созданию общего API там, где это возможно, и предоставлять специфичные для игры расширения или модули.
- **Конфигурация Проекта:** Проекты и плагины должны явно указывать целевую игру (или игры), для которой они предназначены.

### 4. Тестирование Нативных Функций
- **Автоматизированные Тесты:** Создание тестовых сценариев для проверки корректности вызова и работы ключевых нативных функций в обеих играх.
- **Интеграционные Тесты:** Тестирование взаимодействия модулей фреймворка, использующих нативные функции.
- **Обратная Связь от Сообщества:** Механизмы для сбора информации о проблемах с нативными функциями от разработчиков. 

### 🛠️ Server Runtime & IPC Standards (v0.1.0) ✅ **ЗАВЕРШЕНО**

#### Общие принципы
- **Единая точка входа**: `ServerRuntime` в `core/src/server` управляет жизненным циклом сервера.
- **IPC-слой по умолчанию**: Unix-socket `/tmp/gameverse_server.sock` (Windows — именованный канал `\\.\pipe\gameverse_server`).
- **Команды**: `shutdown`, `reload`, `reload_resources`, `status`.
- **JSON-ответы**: для `status` возвращается объект `{ status, uptime_secs, players_online, resources_loaded, avg_tick_ms }`.
- **Graceful Shutdown**: перехват `SIGINT/SIGTERM`, завершение активных задач, закрытие сокета, сохранение состояния.
- **Hot Reload**: резервное удаление ресурсов, повторная инициализация без остановки процесса.
- **Стандарты логирования**: все IPC-события логируются на уровне `INFO`, ошибки — `ERROR`.
- **REST Admin API**: Axum-сервер на порту 30121 с JWT-аутентификацией и SSE-логами.
- **Real-time SSE**: Server-Sent Events через `/api/server/logs/stream` с broadcast каналом tracing subscriber.

#### Требования к реализации
1. **PID-файл** — `~/.gameverse/server.pid`, создаётся при запуске, удаляется при корректном завершении.
2. **Код возврата** для CLI:
   | Команда | Success | Failure |
   |---------|---------|---------|
   | start   | 0       | 101     |
   | stop    | 0       | 102     |
   | reload  | 0       | 103     |
   | status  | 0       | 104     |
3. **Безопасность** — сокет/pipe создаётся с правами `0600` для предотвращения несанкционированного доступа.
4. **Расширяемость** — новые команды добавляются через `enum ServerCommand` + обработчик в `ServerRuntime::handle_command`.

---

### 🛠️ CLI Tools Development Standards (v0.3.0) – Server Management Update

#### Новые подкоманды `gameverse server`
- `start [--background]` — сборка (если требуется) и запуск `gameverse_server` в фоне.
- `stop` — отправка `shutdown` через IPC.
- `restart` — последовательный `stop` → `start`.
- `status` — вывод ответа IPC в JSON или prettified таблицу (`--json` флаг).
- `reload` — hot-reload ресурсов без остановки сервера.
- `logs [-f]` — поток логов сервера (tail), `-f` для follow.

#### Поведение CLI
1. Перед выполнением команды проверяется наличие живого PID из `server.pid`.
2. Все IPC-запросы имеют тайм-аут 3 секунды; при превышении выводится понятная ошибка.
3. `status` по умолчанию форматирует JSON в колонках, `--raw` выводит оригинальный JSON.
4. Опция `--which` автоматически ищет бинарь `gameverse_server` рядом с CLI, либо пересобирает `core/bin/server.rs`.

#### Тестирование
- Unit-тесты на формирование IPC-команд и обработку тайм-аутов.
- Интеграционные тесты: запуск test-сервера, проверка `start/stop/status` на Unix (CI) и Windows (GH Actions).
- Mock-слой для сокета, чтобы тесты не требовали реального процесса. 

### 🐳 **Server Bootstrap v0.2 Standards** ✨ **НОВЫЕ СТАНДАРТЫ**

#### **Docker Standards**
- **Multi-stage builds**: Отдельные stages для dependencies, build, runtime
- **Minimal base images**: Alpine Linux для production (< 50MB final image)
- **Security scanning**: Trivy integration в CI/CD pipeline
- **Health checks**: Proper HEALTHCHECK инструкции с endpoints
- **Non-root execution**: Dedicated user для security compliance

```dockerfile
# Стандартный Dockerfile для GameVerse Server
FROM rust:1.75-alpine AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY . .
RUN cargo build --release --bin gameverse_server

FROM alpine:3.19
RUN addgroup -g 1001 gameverse && adduser -D -s /bin/sh -u 1001 -G gameverse gameverse
COPY --from=builder /app/target/release/gameverse_server /usr/local/bin/
USER gameverse
EXPOSE 8080 30121
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:30121/api/health || exit 1
CMD ["gameverse_server"]
```

#### **Kubernetes Helm Charts Standards**
- **Values-driven configuration**: Все параметры через values.yaml
- **Resource limits**: Обязательные requests/limits для CPU/Memory
- **Probes**: liveness, readiness, startup probes
- **Security contexts**: Non-root, read-only filesystem где возможно
- **RBAC**: Minimal required permissions

```yaml
# values.yaml стандарт
gameverse:
  image:
    repository: gameverse/server
    tag: "latest"
    pullPolicy: IfNotPresent
  
  resources:
    requests:
      cpu: 100m
      memory: 128Mi
    limits:
      cpu: 500m
      memory: 512Mi
  
  autoscaling:
    enabled: true
    minReplicas: 2
    maxReplicas: 10
    targetCPUUtilizationPercentage: 70
```

#### **Terraform Standards**
- **Module structure**: Отдельные modules для разных cloud providers
- **Variable validation**: Input validation для всех параметров
- **Output consistency**: Стандартные outputs (endpoints, credentials)
- **State management**: Remote state с locking
- **Security groups**: Minimal required ports only

```hcl
# AWS module стандарт
variable "instance_type" {
  description = "EC2 instance type for GameVerse server"
  type        = string
  default     = "t3.medium"
  
  validation {
    condition = can(regex("^t3\\.", var.instance_type))
    error_message = "Instance type must be from t3 family for cost optimization."
  }
}

output "server_endpoint" {
  description = "GameVerse server public endpoint"
  value       = aws_instance.gameverse.public_dns
}
```

#### **Monitoring Stack Standards**
- **Prometheus metrics**: Custom metrics для game-specific events
- **Grafana dashboards**: Pre-built dashboards для server health
- **Jaeger tracing**: Distributed tracing для debugging
- **Alerting rules**: Critical alerts для downtime/performance
- **Log aggregation**: Centralized logging с ELK/Loki

```rust
// Prometheus metrics integration
use prometheus::{Counter, Histogram, Registry};

pub struct GameVerseMetrics {
    pub players_connected: Counter,
    pub request_duration: Histogram,
    pub errors_total: Counter,
}

impl GameVerseMetrics {
    pub fn new(registry: &Registry) -> Self {
        let players_connected = Counter::new("gameverse_players_connected_total", "Total connected players")
            .expect("metric can be created");
        registry.register(Box::new(players_connected.clone())).unwrap();
        
        Self { players_connected, /* ... */ }
    }
}
```

#### **Service Mesh Readiness**
- **Istio compatibility**: Proper service annotations
- **mTLS support**: Certificate management готовность
- **Traffic policies**: Rate limiting, circuit breakers
- **Observability**: Automatic metrics collection
- **Security policies**: Network policies enforcement

// ... existing code ... 