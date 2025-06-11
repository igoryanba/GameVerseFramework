//! # Система конфигурации плагинов GameVerse
//!
//! TOML-based конфигурация плагинов превосходящая FiveM fxmanifest.lua:
//! - Type-safe конфигурация через serde
//! - Автоматическая валидация
//! - Семантическое версионирование
//! - Поддержка разных типов плагинов (Rust, Lua, WASM, TypeScript)

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

/// Манифест плагина в формате TOML
/// 
/// Превосходство над FiveM fxmanifest.lua:
/// - Строгая типизация вместо динамического Lua
/// - Автоматическая валидация при загрузке
/// - Поддержка семантического версионирования
/// - Расширенные метаданные для маркетплейса
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Основная информация о плагине
    pub plugin: PluginMetadata,
    
    /// Зависимости плагина
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
    
    /// Скрипты плагина
    #[serde(default)]
    pub scripts: PluginScripts,
    
    /// Ресурсы плагина (файлы, ассеты)
    #[serde(default)]
    pub resources: PluginResources,
    
    /// Права доступа плагина
    #[serde(default)]
    pub permissions: PluginPermissions,
    
    /// Конфигурация плагина
    #[serde(default)]
    pub config: HashMap<String, toml::Value>,
    
    /// API эндпоинты, предоставляемые плагином
    #[serde(default)]
    pub api: Vec<ApiEndpointSpec>,
    
    /// События, на которые подписывается плагин
    #[serde(default)]
    pub events: Vec<String>,
    
    /// Команды, регистрируемые плагином
    #[serde(default)]
    pub commands: Vec<CommandSpec>,
}

/// Основные метаданные плагина
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Уникальный идентификатор плагина
    pub id: String,
    
    /// Человеко-читаемое имя
    pub name: String,
    
    /// Версия (семантическое версионирование)
    pub version: String,
    
    /// Автор плагина
    pub author: String,
    
    /// Описание плагина
    pub description: String,
    
    /// Веб-сайт или репозиторий
    pub website: Option<String>,
    
    /// Лицензия (SPDX identifier)
    pub license: Option<String>,
    
    /// Категория плагина
    pub category: PluginCategory,
    
    /// Теги для поиска
    #[serde(default)]
    pub tags: Vec<String>,
    
    /// Минимальная версия GameVerse
    pub min_gameverse_version: String,
    
    /// Максимальная версия GameVerse (опционально)
    pub max_gameverse_version: Option<String>,
    
    /// Поддерживаемые игры
    #[serde(default)]
    pub supported_games: Vec<String>,
    
    /// Иконка плагина (путь к файлу)
    pub icon: Option<String>,
    
    /// Скриншоты для маркетплейса
    #[serde(default)]
    pub screenshots: Vec<String>,
}

/// Категория плагина для маркетплейса
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PluginCategory {
    /// Игровые моды
    Gamemode,
    /// Экономика
    Economy,
    /// Транспорт
    Vehicles,
    /// Работы
    Jobs,
    /// Интерфейс
    Interface,
    /// Утилиты
    Utilities,
    /// Администрирование
    Administration,
    /// Безопасность
    Security,
    /// Развлечения
    Entertainment,
    /// Библиотеки
    Library,
    /// Другое
    Other,
}

/// Спецификация зависимости
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencySpec {
    /// Простая версия
    Simple(String),
    /// Расширенная спецификация
    Extended {
        /// Версия (поддерживает semver ranges)
        version: String,
        /// Обязательная ли зависимость
        #[serde(default = "default_true")]
        required: bool,
        /// URL для загрузки если не найдена
        url: Option<String>,
        /// Описание зависимости
        description: Option<String>,
    },
}

fn default_true() -> bool { true }

/// Скрипты плагина
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginScripts {
    /// Серверные скрипты
    #[serde(default)]
    pub server: Vec<String>,
    
    /// Клиентские скрипты
    #[serde(default)]
    pub client: Vec<String>,
    
    /// Общие скрипты
    #[serde(default)]
    pub shared: Vec<String>,
    
    /// WebAssembly модули
    #[serde(default)]
    pub wasm: Vec<String>,
    
    /// TypeScript модули
    #[serde(default)]
    pub typescript: Vec<String>,
    
    /// Lua скрипты
    #[serde(default)]
    pub lua: Vec<String>,
}

/// Ресурсы плагина
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginResources {
    /// Файлы для загрузки на клиент
    #[serde(default)]
    pub files: Vec<String>,
    
    /// Ассеты (модели, текстуры, звуки)
    #[serde(default)]
    pub assets: Vec<AssetSpec>,
    
    /// Базы данных и миграции
    #[serde(default)]
    pub database: DatabaseSpec,
    
    /// Конфигурационные файлы
    #[serde(default)]
    pub configs: Vec<String>,
}

/// Спецификация ассета
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetSpec {
    /// Путь к файлу
    pub path: String,
    /// Тип ассета
    pub asset_type: AssetType,
    /// Сжатие
    #[serde(default)]
    pub compression: CompressionType,
    /// Приоритет загрузки
    #[serde(default)]
    pub priority: u8,
}

/// Тип ассета
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    Model,
    Texture,
    Sound,
    Animation,
    Font,
    Shader,
    Other,
}

/// Тип сжатия
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    None,
    Gzip,
    Lz4,
    Zstd,
}

impl Default for CompressionType {
    fn default() -> Self {
        CompressionType::None
    }
}

/// Спецификация базы данных
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseSpec {
    /// Миграции
    #[serde(default)]
    pub migrations: Vec<String>,
    
    /// Схемы
    #[serde(default)]
    pub schemas: Vec<String>,
    
    /// Тестовые данные
    #[serde(default)]
    pub fixtures: Vec<String>,
}

/// Права доступа плагина
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginPermissions {
    /// Доступ к файловой системе
    #[serde(default)]
    pub filesystem: FilesystemPermissions,
    
    /// Доступ к сети
    #[serde(default)]
    pub network: NetworkPermissions,
    
    /// Доступ к базе данных
    #[serde(default)]
    pub database: DatabasePermissions,
    
    /// Системные вызовы
    #[serde(default)]
    pub syscalls: Vec<String>,
    
    /// Доступ к другим плагинам
    #[serde(default)]
    pub plugins: Vec<String>,
}

/// Права доступа к файловой системе
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilesystemPermissions {
    /// Директории для чтения
    #[serde(default)]
    pub read: Vec<String>,
    
    /// Директории для записи
    #[serde(default)]
    pub write: Vec<String>,
    
    /// Максимальный размер файлов (байты)
    pub max_file_size: Option<u64>,
}

/// Права доступа к сети
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkPermissions {
    /// Разрешенные хосты для исходящих запросов
    #[serde(default)]
    pub outbound_hosts: Vec<String>,
    
    /// Разрешенные порты для прослушивания
    #[serde(default)]
    pub listen_ports: Vec<u16>,
    
    /// Максимальное количество соединений
    pub max_connections: Option<u32>,
}

/// Права доступа к базе данных
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabasePermissions {
    /// Разрешенные таблицы для чтения
    #[serde(default)]
    pub read_tables: Vec<String>,
    
    /// Разрешенные таблицы для записи
    #[serde(default)]
    pub write_tables: Vec<String>,
    
    /// Разрешение на создание таблиц
    #[serde(default)]
    pub create_tables: bool,
    
    /// Максимальное количество запросов в секунду
    pub query_rate_limit: Option<u32>,
}

/// Спецификация API эндпоинта
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpointSpec {
    /// HTTP метод
    pub method: String,
    
    /// Путь эндпоинта
    pub path: String,
    
    /// Описание эндпоинта
    pub description: String,
    
    /// Требуется ли аутентификация
    #[serde(default)]
    pub requires_auth: bool,
    
    /// Необходимые права доступа
    #[serde(default)]
    pub permissions: Vec<String>,
    
    /// Лимит запросов в секунду
    pub rate_limit: Option<u32>,
}

/// Спецификация команды
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSpec {
    /// Название команды
    pub name: String,
    
    /// Описание команды
    pub description: String,
    
    /// Права доступа
    #[serde(default)]
    pub permissions: Vec<String>,
    
    /// Параметры команды
    #[serde(default)]
    pub parameters: Vec<CommandParameter>,
}

/// Параметр команды
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandParameter {
    /// Название параметра
    pub name: String,
    
    /// Тип параметра
    pub param_type: ParameterType,
    
    /// Обязательный ли параметр
    #[serde(default)]
    pub required: bool,
    
    /// Описание параметра
    pub description: String,
}

/// Тип параметра команды
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Player,
    Vehicle,
    Object,
    Coordinates,
}

/// Конфигурация плагина
#[derive(Debug, Clone)]
pub struct PluginConfig {
    /// Путь к файлу плагина
    pub path: PathBuf,
    
    /// Манифест плагина
    pub manifest: PluginManifest,
    
    /// Загруженные конфигурации
    pub loaded_configs: HashMap<String, toml::Value>,
}

impl PluginConfig {
    /// Загрузить конфигурацию плагина из файла
    pub async fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = tokio::fs::read_to_string(path).await
            .with_context(|| format!("Failed to read plugin manifest: {}", path.display()))?;
        
        let manifest: PluginManifest = toml::from_str(&content)
            .with_context(|| format!("Failed to parse plugin manifest: {}", path.display()))?;
        
        // Валидация манифеста
        Self::validate_manifest(&manifest)?;
        
        Ok(Self {
            path: path.to_path_buf(),
            manifest,
            loaded_configs: HashMap::new(),
        })
    }
    
    /// Валидация манифеста плагина
    fn validate_manifest(manifest: &PluginManifest) -> Result<()> {
        // Проверка версии
        if !Self::is_valid_semver(&manifest.plugin.version) {
            anyhow::bail!("Invalid version format: {}", manifest.plugin.version);
        }
        
        if !Self::is_valid_semver(&manifest.plugin.min_gameverse_version) {
            anyhow::bail!("Invalid min_gameverse_version format: {}", manifest.plugin.min_gameverse_version);
        }
        
        // Проверка ID плагина
        if manifest.plugin.id.is_empty() || !manifest.plugin.id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            anyhow::bail!("Invalid plugin ID: {}", manifest.plugin.id);
        }
        
        Ok(())
    }
    
    /// Проверка семантического версионирования
    fn is_valid_semver(version: &str) -> bool {
        // Упрощенная проверка semver
        let parts: Vec<&str> = version.split('.').collect();
        parts.len() == 3 && parts.iter().all(|p| p.parse::<u32>().is_ok())
    }
    
    /// Получить значение конфигурации
    pub fn get_config_value(&self, key: &str) -> Option<&toml::Value> {
        self.manifest.config.get(key)
    }
    
    /// Проверить совместимость с версией GameVerse
    pub fn is_compatible_with(&self, _gameverse_version: &str) -> bool {
        // TODO: Реализовать полноценную проверку semver ranges
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_manifest_serialization() {
        let manifest = PluginManifest {
            plugin: PluginMetadata {
                id: "test-plugin".to_string(),
                name: "Test Plugin".to_string(),
                version: "1.0.0".to_string(),
                author: "GameVerse Team".to_string(),
                description: "A test plugin".to_string(),
                website: None,
                license: Some("MIT".to_string()),
                category: PluginCategory::Library,
                tags: vec!["test".to_string()],
                min_gameverse_version: "0.1.0".to_string(),
                max_gameverse_version: None,
                supported_games: vec!["gta5".to_string()],
                icon: None,
                screenshots: vec![],
            },
            dependencies: HashMap::new(),
            scripts: PluginScripts::default(),
            resources: PluginResources::default(),
            permissions: PluginPermissions::default(),
            config: HashMap::new(),
            api: vec![],
            events: vec![],
            commands: vec![],
        };
        
        let toml_str = toml::to_string(&manifest).unwrap();
        let deserialized: PluginManifest = toml::from_str(&toml_str).unwrap();
        
        assert_eq!(manifest.plugin.id, deserialized.plugin.id);
        assert_eq!(manifest.plugin.version, deserialized.plugin.version);
    }
    
    #[test]
    fn test_semver_validation() {
        assert!(PluginConfig::is_valid_semver("1.0.0"));
        assert!(PluginConfig::is_valid_semver("0.1.0"));
        assert!(PluginConfig::is_valid_semver("10.20.30"));
        assert!(!PluginConfig::is_valid_semver("1.0"));
        assert!(!PluginConfig::is_valid_semver("1.0.0-alpha"));
        assert!(!PluginConfig::is_valid_semver("invalid"));
    }
} 