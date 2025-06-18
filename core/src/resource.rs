//! # Система ресурсов GameVerse
//!
//! Превосходство над FiveM ресурсами:
//! - TOML конфигурация вместо fxmanifest.lua
//! - Автоматическая валидация и проверка зависимостей
//! - Hot reload без рестарта сервера
//! - Версионирование с semver
//! - Type safety и современные форматы

use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Менеджер ресурсов
#[derive(Debug, Clone)]
pub struct ResourceManager {
    /// Загруженные ресурсы
    resources: HashMap<String, Resource>,
    /// Директория ресурсов
    resources_directory: PathBuf,
    /// Состояние инициализации
    initialized: bool,
}

/// Ресурс GameVerse
#[derive(Debug, Clone)]
pub struct Resource {
    /// Уникальный идентификатор
    pub id: String,
    /// Название ресурса
    pub name: String,
    /// Версия (semver)
    pub version: String,
    /// Автор
    pub author: String,
    /// Описание
    pub description: String,
    /// Путь к ресурсу
    pub path: PathBuf,
    /// Манифест ресурса
    pub manifest: ResourceManifest,
    /// Состояние ресурса
    pub state: ResourceState,
}

/// Манифест ресурса (gameverse.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceManifest {
    /// Метаданные ресурса
    pub resource: ResourceMetadata,
    /// Скрипты ресурса
    pub scripts: Option<ResourceScripts>,
    /// Зависимости
    pub dependencies: Option<Vec<ResourceDependency>>,
    /// Конфигурация
    pub config: Option<toml::Value>,
}

/// Метаданные ресурса
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetadata {
    /// Название
    pub name: String,
    /// Версия
    pub version: String,
    /// Автор
    pub author: String,
    /// Описание
    pub description: String,
    /// Веб-сайт
    pub website: Option<String>,
    /// Лицензия
    pub license: Option<String>,
    /// Теги
    pub tags: Option<Vec<String>>,
}

/// Скрипты ресурса
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceScripts {
    /// Серверные скрипты
    pub server: Option<Vec<String>>,
    /// Клиентские скрипты
    pub client: Option<Vec<String>>,
    /// Общие скрипты
    pub shared: Option<Vec<String>>,
}

/// Зависимость ресурса
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDependency {
    /// Название зависимости
    pub name: String,
    /// Требование к версии
    pub version: String,
    /// Обязательная ли зависимость
    pub required: Option<bool>,
}

/// Состояние ресурса
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceState {
    /// Ресурс найден, но не загружен
    Found,
    /// Ресурс загружен
    Loaded,
    /// Ресурс запущен
    Started,
    /// Ресурс остановлен
    Stopped,
    /// Ошибка в ресурсе
    Error,
}

impl ResourceManager {
    /// Создать новый менеджер ресурсов
    pub fn new(resources_directory: PathBuf) -> Self {
        Self {
            resources: HashMap::new(),
            resources_directory,
            initialized: false,
        }
    }
    
    /// Инициализировать менеджер ресурсов
    pub async fn initialize(&mut self) -> Result<()> {
        use tokio_stream::wrappers::ReadDirStream;
        use tokio_stream::StreamExt;

        if !self.resources_directory.exists() {
            tracing::warn!(
                "Resources directory '{}' does not exist. Creating...",
                self.resources_directory.display()
            );
            tokio::fs::create_dir_all(&self.resources_directory).await?;
        }

        let mut dir = tokio::fs::read_dir(&self.resources_directory).await?;
        let mut stream = ReadDirStream::new(dir);

        while let Some(entry_res) = stream.next().await {
            let entry = entry_res?;
            let path = entry.path();

            if path.is_dir() {
                let manifest_path = path.join("gameverse.toml");
                if manifest_path.exists() {
                    match tokio::fs::read_to_string(&manifest_path).await {
                        Ok(contents) => {
                            match toml::from_str::<ResourceManifest>(&contents) {
                                Ok(manifest) => {
                                    let id = uuid::Uuid::new_v4().to_string();
                                    let res_name = manifest.resource.name.clone();
                                    let resource = Resource {
                                        id,
                                        name: res_name.clone(),
                                        version: manifest.resource.version.clone(),
                                        author: manifest.resource.author.clone(),
                                        description: manifest.resource.description.clone(),
                                        path: path.clone(),
                                        manifest,
                                        state: ResourceState::Loaded,
                                    };
                                    self.resources.insert(res_name, resource);
                                }
                                Err(e) => {
                                    tracing::error!(
                                        "Failed to parse manifest '{}': {}",
                                        manifest_path.display(),
                                        e
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!(
                                "Failed to read manifest '{}': {}",
                                manifest_path.display(),
                                e
                            );
                        }
                    }
                }
            }
        }

        self.initialized = true;
        tracing::info!(
            "🔥 Resource manager initialized: {} resources loaded",
            self.resources.len()
        );
        Ok(())
    }
    
    /// Загрузить ресурс
    pub async fn load_resource(&mut self, resource_name: &str) -> Result<()> {
        // TODO: Реализовать загрузку ресурса из gameverse.toml
        tracing::info!("Loading resource: {}", resource_name);
        Ok(())
    }
    
    /// Запустить ресурс
    pub async fn start_resource(&mut self, resource_name: &str) -> Result<()> {
        // TODO: Реализовать запуск ресурса
        tracing::info!("Starting resource: {}", resource_name);
        Ok(())
    }
    
    /// Остановить ресурс
    pub async fn stop_resource(&mut self, resource_name: &str) -> Result<()> {
        // TODO: Реализовать остановку ресурса
        tracing::info!("Stopping resource: {}", resource_name);
        Ok(())
    }
    
    /// Перезагрузить ресурс (hot reload)
    pub async fn reload_resource(&mut self, resource_name: &str) -> Result<()> {
        // TODO: Реализовать hot reload ресурса
        tracing::info!("Hot reloading resource: {}", resource_name);
        Ok(())
    }
    
    /// Получить список всех ресурсов
    pub fn list_resources(&self) -> Vec<&Resource> {
        self.resources.values().collect()
    }
    
    /// Получить ресурс по имени
    pub fn get_resource(&self, name: &str) -> Option<&Resource> {
        self.resources.get(name)
    }
    
    /// Проверить инициализацию
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new(PathBuf::from("resources"))
    }
}

// Пример gameverse.toml манифеста
// 
// ```toml
// [resource]
// name = "my-awesome-resource"
// version = "1.0.0"
// author = "GameVerse Developer" 
// description = "An awesome resource for GameVerse"
// website = "https://gameverse.dev"
// license = "MIT"
// tags = ["economy", "roleplay"]
//
// [scripts]
// server = ["server/main.ts", "server/events.ts"]
// client = ["client/ui.tsx", "client/events.ts"]
// shared = ["shared/config.ts"]
//
// [[dependencies]]
// name = "gameverse-core"
// version = ">=0.1.0"
// required = true
//
// [config]
// database_url = "postgresql://localhost/mydb"
// max_players = 64
// ``` 