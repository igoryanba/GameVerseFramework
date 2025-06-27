//! # Система ресурсов GameVerse
//!
//! Превосходство над FiveM ресурсами:
//! - TOML конфигурация вместо fxmanifest.lua
//! - Автоматическая валидация и проверка зависимостей
//! - Hot reload без рестарта сервера
//! - Версионирование с semver
//! - Type safety и современные форматы

use anyhow::Result;
use anyhow::Context;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use tempfile::TempDir;
use once_cell::sync::OnceCell;
use tokio::sync::broadcast;
use chrono::{Utc, DateTime};

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

/// Событие жизненного цикла ресурса
#[derive(Debug, Clone, Serialize)]
pub struct ResourceLifecycleEvent {
    /// Имя ресурса
    pub name: String,
    /// Действие
    pub action: ResourceLifecycleAction,
    /// Время события (UTC)
    pub timestamp: DateTime<Utc>,
}

/// Тип действия с ресурсом
#[derive(Debug, Clone, Serialize)]
pub enum ResourceLifecycleAction {
    Started,
    Stopped,
    Reloaded,
}

// Глобальный широковещательный канал для публикации событий ресурсов
static RESOURCE_EVENT_BUS: OnceCell<broadcast::Sender<ResourceLifecycleEvent>> = OnceCell::new();

/// Получить широковещательный отправитель событий ресурсов
fn event_sender() -> &'static broadcast::Sender<ResourceLifecycleEvent> {
    RESOURCE_EVENT_BUS.get_or_init(|| {
        let (tx, _rx) = broadcast::channel(256);
        tx
    })
}

/// Подписаться на события жизненного цикла ресурсов
pub fn subscribe_resource_events() -> broadcast::Receiver<ResourceLifecycleEvent> {
    event_sender().subscribe()
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
        // Проверяем, не загружен ли ресурс уже
        if self.resources.contains_key(resource_name) {
            tracing::debug!("Resource '{}' already loaded, skipping", resource_name);
            return Ok(());
        }

        let resource_path = self.resources_directory.join(resource_name);
        if !resource_path.exists() {
            anyhow::bail!("Resource directory '{}' not found", resource_path.display());
        }

        let manifest_path = resource_path.join("gameverse.toml");
        if !manifest_path.exists() {
            anyhow::bail!("Manifest '{}' not found", manifest_path.display());
        }

        let contents = tokio::fs::read_to_string(&manifest_path).await
            .with_context(|| format!("Failed to read manifest: {}", manifest_path.display()))?;

        let manifest: ResourceManifest = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse manifest: {}", manifest_path.display()))?;

        let resource = Resource {
            id: uuid::Uuid::new_v4().to_string(),
            name: manifest.resource.name.clone(),
            version: manifest.resource.version.clone(),
            author: manifest.resource.author.clone(),
            description: manifest.resource.description.clone(),
            path: resource_path,
            manifest,
            state: ResourceState::Loaded,
        };

        self.resources.insert(resource_name.to_string(), resource);
        tracing::info!("✅ Resource '{}' successfully loaded", resource_name);
        Ok(())
    }
    
    /// Запустить ресурс
    pub async fn start_resource(&mut self, resource_name: &str) -> Result<()> {
        let res = self.resources.get_mut(resource_name)
            .ok_or_else(|| anyhow::anyhow!("Resource '{}' not found", resource_name))?;

        match res.state {
            ResourceState::Loaded | ResourceState::Stopped => {
                tracing::info!("▶️  Starting resource '{}'...", resource_name);
                // TODO: фактический запуск скриптов/плагинов
                res.state = ResourceState::Started;
                // Публикуем событие
                let _ = event_sender().send(ResourceLifecycleEvent {
                    name: resource_name.to_string(),
                    action: ResourceLifecycleAction::Started,
                    timestamp: Utc::now(),
                });
                Ok(())
            },
            ResourceState::Started => {
                tracing::warn!("Resource '{}' already started", resource_name);
                Ok(())
            },
            _ => anyhow::bail!("Resource '{}' in invalid state {:?} for start", resource_name, res.state),
        }
    }
    
    /// Остановить ресурс
    pub async fn stop_resource(&mut self, resource_name: &str) -> Result<()> {
        let res = self.resources.get_mut(resource_name)
            .ok_or_else(|| anyhow::anyhow!("Resource '{}' not found", resource_name))?;

        match res.state {
            ResourceState::Started => {
                tracing::info!("⏹️  Stopping resource '{}'...", resource_name);
                // TODO: фактическое завершение скриптов/плагинов
                res.state = ResourceState::Stopped;
                // Публикуем событие
                let _ = event_sender().send(ResourceLifecycleEvent {
                    name: resource_name.to_string(),
                    action: ResourceLifecycleAction::Stopped,
                    timestamp: Utc::now(),
                });
                Ok(())
            },
            ResourceState::Loaded | ResourceState::Stopped => {
                tracing::warn!("Resource '{}' is not running", resource_name);
                Ok(())
            },
            _ => anyhow::bail!("Resource '{}' in invalid state {:?} for stop", resource_name, res.state),
        }
    }
    
    /// Перезагрузить ресурс (hot reload)
    pub async fn reload_resource(&mut self, resource_name: &str) -> Result<()> {
        tracing::info!("♻️  Hot reloading resource '{}'...", resource_name);
        self.stop_resource(resource_name).await?;
        // Перечитываем манифест, чтобы подхватить изменения
        self.resources.remove(resource_name);
        self.load_resource(resource_name).await?;
        self.start_resource(resource_name).await?;
        // Публикуем событие
        let _ = event_sender().send(ResourceLifecycleEvent {
            name: resource_name.to_string(),
            action: ResourceLifecycleAction::Reloaded,
            timestamp: Utc::now(),
        });
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

#[cfg(test)]
mod tests {
    use tempfile::TempDir;
    use tokio::runtime::Runtime;
    use std::path::PathBuf;
    use super::*;

    fn create_test_manifest(dir: &PathBuf, name: &str) -> anyhow::Result<()> {
        let manifest_content = format!(
            r#"[resource]
name = "{name}"
version = "0.1.0"
author = "Tester"
description = "Test resource"
"#
        );
        let manifest_path = dir.join("gameverse.toml");
        let mut file = std::fs::File::create(&manifest_path)?;
        use std::io::Write as _; // bring trait for write_all
        file.write_all(manifest_content.as_bytes())?;
        Ok(())
    }

    #[test]
    fn test_load_resource() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Создаем временную директорию
            let temp_dir = tempfile::TempDir::new().expect("Failed to create temp dir");
            let resources_dir = temp_dir.path().to_path_buf();

            // Создаем директорию ресурса
            let res_name = "test_resource";
            let res_dir = resources_dir.join(res_name);
            tokio::fs::create_dir_all(&res_dir).await.expect("Failed to create resource dir");
            
            // Пишем манифест
            create_test_manifest(&res_dir, res_name).expect("Failed to create manifest");

            // Инициализируем ResourceManager
            let mut manager = ResourceManager::new(resources_dir.clone());
            manager.initialize().await.expect("Failed to initialize");
            assert_eq!(manager.list_resources().len(), 1, "ResourceManager should load initial resource");

            // Загружаем еще один ресурс динамически
            let res2_name = "dynamic_res";
            let res2_dir = resources_dir.join(res2_name);
            tokio::fs::create_dir_all(&res2_dir).await.expect("Failed to create res2 dir");
            create_test_manifest(&res2_dir, res2_name).expect("Failed to create res2 manifest");
            
            manager.load_resource(res2_name).await.expect("load_resource failed");
            assert_eq!(manager.list_resources().len(), 2, "After load_resource manager should have 2 resources");
            assert!(manager.get_resource(res2_name).is_some(), "Second resource must be accessible");
        });
    }
} 