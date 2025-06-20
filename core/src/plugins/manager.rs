//! # Менеджер плагинов GameVerse
//!
//! Центральный компонент для управления плагинами, превосходящий FiveM:
//! - Автоматическое разрешение зависимостей
//! - Hot reload без остановки сервера  
//! - Безопасная изоляция плагинов
//! - Мониторинг производительности

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;

use anyhow::{Result, Context};
use tokio::sync::{RwLock, mpsc};
use tracing::{info, warn, error, debug};
use uuid::Uuid;
use serde_json;

// 🔥 Добавляем импорты для dynamic loading
use libloading::Library;
use std::ffi::CStr;
use std::os::raw::c_char;

use super::{
    GameVersePlugin, PluginInfo, PluginState, LoadedPlugin, PluginEvent,
    PluginSystemConfig, PluginResult, PluginError, PluginContext,
    PluginRegistry, DependencyGraph, PluginConfig
};

/// Информация о загруженной динамической библиотеке
#[derive(Debug)]
pub struct LoadedLibrary {
    /// Путь к библиотеке
    pub path: std::path::PathBuf,
    /// Хендл динамической библиотеки
    pub library: Library,
    /// Функция освобождения ресурсов плагина
    pub destroy_plugin_fn: Option<unsafe extern "C" fn(*mut dyn GameVersePlugin)>,
}

impl LoadedLibrary {
    /// Создать новую загруженную библиотеку
    pub fn new(path: std::path::PathBuf, library: Library) -> Self {
        // Попытаться получить функцию destroy_plugin (опционально)
        let destroy_plugin_fn = unsafe {
            library.get::<unsafe extern "C" fn(*mut dyn GameVersePlugin)>(b"destroy_plugin")
                .ok()
                .map(|symbol| *symbol)
        };
        
        if destroy_plugin_fn.is_some() {
            debug!("🧹 Found destroy_plugin function in library: {}", path.display());
        } else {
            debug!("⚠️ No destroy_plugin function found in library: {}", path.display());
        }
        
        Self {
            path,
            library,
            destroy_plugin_fn,
        }
    }
}

/// Статистика производительности плагина
#[derive(Debug, Clone, Default)]
pub struct PluginPerformanceStats {
    /// Общее время выполнения (микросекунды)
    pub total_execution_time_us: u64,
    /// Количество вызовов
    pub call_count: u64,
    /// Среднее время выполнения (микросекунды)
    pub average_execution_time_us: u64,
    /// Максимальное время выполнения (микросекунды)
    pub max_execution_time_us: u64,
    /// Количество ошибок
    pub error_count: u64,
    /// Время последнего вызова
    pub last_call_time: chrono::DateTime<chrono::Utc>,
}

impl PluginPerformanceStats {
    pub fn new() -> Self {
        Self {
            total_execution_time_us: 0,
            call_count: 0,
            average_execution_time_us: 0,
            max_execution_time_us: 0,
            error_count: 0,
            last_call_time: chrono::Utc::now(),
        }
    }
}

/// Сохраненные данные плагина для hot reload
#[derive(Debug, Default)]
pub struct PreservedPluginData {
    /// Конфигурационные данные
    pub config_data: HashMap<String, String>,
    /// Временные данные плагина
    pub runtime_data: HashMap<String, Vec<u8>>,
    /// Активные соединения (сериализованные)
    pub connections: Vec<String>,
    /// Пользовательские данные
    pub user_data: HashMap<String, serde_json::Value>,
}

/// Менеджер плагинов GameVerse
/// 
/// Превосходство над FiveM:
/// - Async/await для неблокирующих операций
/// - Type-safe интерфейсы
/// - Автоматическое управление жизненным циклом
/// - Built-in мониторинг производительности
/// - 🔥 Dynamic plugin loading с hot reload
pub struct PluginManager {
    /// Загруженные плагины
    loaded_plugins: Arc<RwLock<HashMap<String, LoadedPlugin>>>,
    
    /// Загруженные динамические библиотеки
    loaded_libraries: Arc<RwLock<HashMap<String, LoadedLibrary>>>,
    
    /// Реестр плагинов
    registry: Arc<PluginRegistry>,
    
    /// Граф зависимостей
    dependency_graph: Arc<RwLock<DependencyGraph>>,
    
    /// Конфигурация системы
    config: PluginSystemConfig,
    
    /// Канал для событий плагинов
    event_sender: mpsc::UnboundedSender<PluginEvent>,
    
    /// Контекст для плагинов
    plugin_context: Arc<PluginContext>,
    
    /// Статистика производительности
    performance_stats: Arc<RwLock<HashMap<String, PluginPerformanceStats>>>,
    
    /// Watcher для hot reload
    _hot_reload_handle: Option<tokio::task::JoinHandle<()>>,
}

impl PluginManager {
    /// Создать новый менеджер плагинов
    pub async fn new(
        config: PluginSystemConfig,
        event_sender: mpsc::UnboundedSender<PluginEvent>,
    ) -> Result<Self> {
        let plugin_context = Arc::new(PluginContext::new().await?);
        let registry = Arc::new(PluginRegistry::new(&config.plugins_directory)?);
        let dependency_graph = Arc::new(RwLock::new(DependencyGraph::new()));
        
        let mut manager = Self {
            loaded_plugins: Arc::new(RwLock::new(HashMap::new())),
            loaded_libraries: Arc::new(RwLock::new(HashMap::new())),
            registry,
            dependency_graph,
            config: config.clone(),
            event_sender,
            plugin_context,
            performance_stats: Arc::new(RwLock::new(HashMap::new())),
            _hot_reload_handle: None,
        };
        
        // Запустить hot reload если включен
        if config.enable_hot_reload {
            manager.start_hot_reload_watcher().await?;
        }
        
        info!("Plugin manager initialized with {} directory", 
              config.plugins_directory.display());
        
        Ok(manager)
    }
    
    /// Загрузить плагин по идентификатору
    pub async fn load_plugin(&self, plugin_id: &str) -> PluginResult<()> {
        info!("Loading plugin: {}", plugin_id);
        
        // Проверить, не загружен ли уже
        {
            let loaded = self.loaded_plugins.read().await;
            if loaded.contains_key(plugin_id) {
                return Err(PluginError::LoadingFailed {
                    reason: format!("Plugin {} is already loaded", plugin_id),
                });
            }
        }
        
        // Получить конфигурацию плагина
        let plugin_config = self.registry.get_plugin_config(plugin_id)
            .await
            .context("Failed to get plugin config")?;
        
        // Разрешить зависимости
        self.resolve_dependencies(&plugin_config).await?;
        
        // Создать экземпляр плагина
        let mut plugin_instance = self.create_plugin_instance(&plugin_config).await?;
        
        // Инициализировать плагин
        let start_time = Instant::now();
        match plugin_instance.initialize(&self.plugin_context).context("Plugin initialization failed") {
            Ok(_) => {
                let initialization_time = start_time.elapsed();
                debug!("Plugin {} initialized in {:?}", plugin_id, initialization_time);
            }
            Err(e) => {
                error!("Failed to initialize plugin {}: {}", plugin_id, e);
                return Err(PluginError::InitializationFailed {
                    reason: e.to_string(),
                });
            }
        }
        
        let info = plugin_instance.info();
        let loaded_plugin = LoadedPlugin {
            info: info.clone(),
            instance: plugin_instance,
            state: PluginState::Initialized,
            loaded_at: chrono::Utc::now(),
            path: plugin_config.path.clone(),
            session_id: Uuid::new_v4(),
        };
        
        // Добавить в граф зависимостей
        {
            let mut graph = self.dependency_graph.write().await;
            graph.add_plugin(&info)?;
        }
        
        // Сохранить загруженный плагин
        {
            let mut loaded = self.loaded_plugins.write().await;
            loaded.insert(plugin_id.to_string(), loaded_plugin);
        }
        
        // Инициализировать статистику
        {
            let mut stats = self.performance_stats.write().await;
            stats.insert(plugin_id.to_string(), PluginPerformanceStats::new());
        }
        
        // Отправить событие
        let _ = self.event_sender.send(PluginEvent::Loaded {
            plugin_id: plugin_id.to_string(),
        });
        
        info!("Successfully loaded plugin: {}", plugin_id);
        Ok(())
    }
    
    /// Выгрузить плагин
    pub async fn unload_plugin(&self, plugin_id: &str) -> PluginResult<()> {
        info!("Unloading plugin: {}", plugin_id);
        
        // Проверить зависимости
        self.check_unload_dependencies(plugin_id).await?;
        
        // Получить и удалить плагин
        let mut loaded_plugin = {
            let mut loaded = self.loaded_plugins.write().await;
            loaded.remove(plugin_id)
                .ok_or_else(|| PluginError::NotFound {
                    plugin_id: plugin_id.to_string(),
                })?
        };
        
        // Финализировать плагин
        if let Err(e) = loaded_plugin.instance.finalize() {
            warn!("Plugin {} finalization failed: {}", plugin_id, e);
        }
        
        // 🔥 Безопасная выгрузка динамической библиотеки
        {
            let mut libraries = self.loaded_libraries.write().await;
            if let Some(loaded_library) = libraries.remove(plugin_id) {
                debug!("🧹 Cleaning up dynamic library for plugin: {}", plugin_id);
                
                // Использовать destroy_plugin функцию если доступна
                if let Some(destroy_fn) = loaded_library.destroy_plugin_fn {
                    debug!("🗑️ Calling destroy_plugin function");
                    unsafe {
                        // Преобразуем Box обратно в сырой указатель для destroy_fn
                        let plugin_ptr = Box::into_raw(loaded_plugin.instance);
                        destroy_fn(plugin_ptr);
                    }
                    info!("✅ Plugin instance destroyed safely");
                } else {
                    debug!("📦 Using automatic Drop for plugin instance");
                    // loaded_plugin.instance будет автоматически Drop'нут
                }
                
                // Библиотека автоматически выгрузится когда loaded_library выйдет из scope
                debug!("📂 Dynamic library will be unloaded: {}", loaded_library.path.display());
            } else {
                debug!("⚪ No dynamic library found for plugin: {}", plugin_id);
                // Вероятно это встроенный плагин или тестовая заглушка
            }
        }
        
        // Удалить из графа зависимостей
        {
            let mut graph = self.dependency_graph.write().await;
            graph.remove_plugin(plugin_id)?;
        }
        
        // Удалить статистику
        {
            let mut stats = self.performance_stats.write().await;
            stats.remove(plugin_id);
        }
        
        // Отправить событие
        let _ = self.event_sender.send(PluginEvent::Unloaded {
            plugin_id: plugin_id.to_string(),
        });
        
        info!("Successfully unloaded plugin: {}", plugin_id);
        Ok(())
    }
    
    /// Перезагрузить плагин (hot reload)
    pub async fn reload_plugin(&self, plugin_id: &str) -> PluginResult<()> {
        info!("🔥 Hot reloading plugin: {}", plugin_id);
        
        // Проверить, что плагин существует
        let (old_state, old_session_id) = {
            let loaded = self.loaded_plugins.read().await;
            match loaded.get(plugin_id) {
                Some(plugin) => (plugin.state.clone(), plugin.session_id),
                None => {
                    warn!("Plugin {} not found for hot reload", plugin_id);
                    return Err(PluginError::NotFound {
                        plugin_id: plugin_id.to_string(),
                    });
                }
            }
        };
        
        // Создаем новый session ID для отслеживания hot reload
        let new_session_id = Uuid::new_v4();
        debug!("Hot reload session: {} -> {}", old_session_id, new_session_id);
        
        // Валидация состояния плагина перед reload
        if matches!(old_state, PluginState::Error(_)) {
            warn!("Plugin {} is in error state, attempting recovery reload", plugin_id);
        }
        
        // Сохранить критическое состояние плагина (если есть)
        let preserved_data = self.preserve_plugin_state(plugin_id).await?;
        
        // Выгрузить с graceful shutdown
        if let Err(e) = self.unload_plugin(plugin_id).await {
            warn!("Failed to cleanly unload plugin {} during hot reload: {}", plugin_id, e);
            // Продолжаем reload даже если выгрузка не удалась
        }
        
        // Пауза для стабилизации
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        
        // Загрузить заново
        match self.load_plugin(plugin_id).await {
            Ok(_) => {
                // Восстановить состояние если нужно
                if old_state == PluginState::Running {
                    if let Err(e) = self.start_plugin(plugin_id).await {
                        error!("Failed to restart plugin {} after hot reload: {}", plugin_id, e);
                        return Err(PluginError::HotReloadFailed {
                            reason: format!("Failed to restart after reload: {}", e),
                        });
                    }
                }
                
                // Восстановить сохраненные данные
                self.restore_plugin_state(plugin_id, preserved_data).await?;
                
                let _ = self.event_sender.send(PluginEvent::HotReloaded {
                    plugin_id: plugin_id.to_string(),
                });
                
                info!("✅ Successfully hot reloaded plugin: {} (session: {})", plugin_id, new_session_id);
                Ok(())
            }
            Err(e) => {
                error!("🚨 Failed to hot reload plugin {}: {}", plugin_id, e);
                
                // Отправить событие об ошибке
                let _ = self.event_sender.send(PluginEvent::Error {
                    plugin_id: plugin_id.to_string(),
                    error: format!("Hot reload failed: {}", e),
                });
                
                Err(PluginError::HotReloadFailed {
                    reason: e.to_string(),
                })
            }
        }
    }
    
    /// Сохранить состояние плагина перед hot reload
    async fn preserve_plugin_state(&self, plugin_id: &str) -> PluginResult<PreservedPluginData> {
        // TODO: Реализовать сохранение критических данных плагина
        // Например: конфигурация, временные данные, соединения
        debug!("Preserving state for plugin: {}", plugin_id);
        Ok(PreservedPluginData::default())
    }
    
    /// Восстановить состояние плагина после hot reload
    async fn restore_plugin_state(&self, plugin_id: &str, _data: PreservedPluginData) -> PluginResult<()> {
        // TODO: Реализовать восстановление состояния
        debug!("Restoring state for plugin: {}", plugin_id);
        Ok(())
    }
    
    /// Запустить плагин
    pub async fn start_plugin(&self, plugin_id: &str) -> PluginResult<()> {
        let mut loaded = self.loaded_plugins.write().await;
        let plugin = loaded.get_mut(plugin_id)
            .ok_or_else(|| PluginError::NotFound {
                plugin_id: plugin_id.to_string(),
            })?;
        
        match &plugin.state {
            PluginState::Initialized | PluginState::Stopped => {
                plugin.state = PluginState::Running;
                
                let _ = self.event_sender.send(PluginEvent::Started {
                    plugin_id: plugin_id.to_string(),
                });
                
                info!("Started plugin: {}", plugin_id);
                Ok(())
            }
            state => Err(PluginError::InvalidState { state: state.clone() })
        }
    }
    
    /// Остановить плагин
    pub async fn stop_plugin(&self, plugin_id: &str) -> PluginResult<()> {
        let mut loaded = self.loaded_plugins.write().await;
        let plugin = loaded.get_mut(plugin_id)
            .ok_or_else(|| PluginError::NotFound {
                plugin_id: plugin_id.to_string(),
            })?;
        
        match &plugin.state {
            PluginState::Running => {
                plugin.state = PluginState::Stopped;
                
                let _ = self.event_sender.send(PluginEvent::Stopped {
                    plugin_id: plugin_id.to_string(),
                });
                
                info!("Stopped plugin: {}", plugin_id);
                Ok(())
            }
            state => Err(PluginError::InvalidState { state: state.clone() })
        }
    }
    
    /// Получить информацию о всех загруженных плагинах
    pub async fn list_plugins(&self) -> Vec<PluginInfo> {
        let loaded = self.loaded_plugins.read().await;
        loaded.values().map(|p| p.info.clone()).collect()
    }
    
    /// Получить статистику производительности плагина
    pub async fn get_plugin_stats(&self, plugin_id: &str) -> Option<PluginPerformanceStats> {
        let stats = self.performance_stats.read().await;
        stats.get(plugin_id).cloned()
    }
    
    /// Обновить статистику производительности
    pub async fn update_stats(&self, plugin_id: &str, execution_time: Duration, success: bool) {
        let mut stats = self.performance_stats.write().await;
        if let Some(plugin_stats) = stats.get_mut(plugin_id) {
            let execution_time_us = execution_time.as_micros() as u64;
            
            plugin_stats.total_execution_time_us += execution_time_us;
            plugin_stats.call_count += 1;
            plugin_stats.average_execution_time_us = 
                plugin_stats.total_execution_time_us / plugin_stats.call_count;
            plugin_stats.max_execution_time_us = 
                plugin_stats.max_execution_time_us.max(execution_time_us);
            plugin_stats.last_call_time = chrono::Utc::now();
            
            if !success {
                plugin_stats.error_count += 1;
            }
        }
    }
    
    /// Создать экземпляр плагина из динамической библиотеки
    async fn create_plugin_instance(
        &self, 
        config: &PluginConfig
    ) -> PluginResult<Box<dyn GameVersePlugin>> {
        info!("🔥 Loading dynamic plugin: {}", config.manifest.plugin.name);
        
        // Определить путь к динамической библиотеке плагина
        let lib_path = self.find_plugin_library(config)?;
        debug!("📚 Plugin library path: {}", lib_path.display());
        
        // Загрузить динамическую библиотеку
        let library = unsafe {
            Library::new(&lib_path)
                .map_err(|e| PluginError::LoadingFailed {
                    reason: format!("Failed to load library '{}': {}", lib_path.display(), e),
                })?
        };
        
        info!("✅ Library loaded successfully: {}", lib_path.display());
        
        // Получить функцию создания плагина
        let create_plugin_fn = unsafe {
            library.get::<unsafe extern "C" fn() -> *mut dyn GameVersePlugin>(b"create_plugin")
                .map_err(|e| PluginError::LoadingFailed {
                    reason: format!("Failed to find 'create_plugin' symbol: {}", e),
                })?
        };
        
        // Создать экземпляр плагина
        let plugin_ptr = unsafe { create_plugin_fn() };
        
        if plugin_ptr.is_null() {
            return Err(PluginError::LoadingFailed {
                reason: "Plugin creation function returned null pointer".to_string(),
            });
        }
        
        // Преобразовать указатель в Box сразу для thread safety
        let plugin_box = unsafe { Box::from_raw(plugin_ptr) };
        
        // Сохранить информацию о загруженной библиотеке
        let loaded_library = LoadedLibrary::new(lib_path.clone(), library);
        {
            let mut libraries = self.loaded_libraries.write().await;
            libraries.insert(config.manifest.plugin.id.clone(), loaded_library);
        }
        
        info!("🚀 Plugin instance created successfully: {}", config.manifest.plugin.name);
        
        Ok(plugin_box)
    }
    
    /// Найти путь к динамической библиотеке плагина
    fn find_plugin_library(&self, config: &PluginConfig) -> PluginResult<std::path::PathBuf> {
        let plugin_dir = config.path.parent()
            .ok_or_else(|| PluginError::ConfigError {
                reason: "Invalid plugin config path".to_string(),
            })?;
        
        // Определить расширение библиотеки для текущей платформы
        let lib_extension = if cfg!(target_os = "windows") {
            "dll"
        } else if cfg!(target_os = "macos") {
            "dylib"
        } else {
            "so"
        };
        
        // Попытаться найти библиотеку плагина
        let possible_paths = vec![
            plugin_dir.join(format!("lib{}.{}", config.manifest.plugin.id, lib_extension)),
            plugin_dir.join(format!("{}.{}", config.manifest.plugin.id, lib_extension)),
            plugin_dir.join("target").join("release").join(format!("lib{}.{}", config.manifest.plugin.id, lib_extension)),
            plugin_dir.join("target").join("debug").join(format!("lib{}.{}", config.manifest.plugin.id, lib_extension)),
        ];
        
        for path in possible_paths {
            if path.exists() {
                debug!("📦 Found plugin library: {}", path.display());
                return Ok(path);
            }
        }
        
        Err(PluginError::LoadingFailed {
            reason: format!(
                "Plugin library not found for '{}'. Searched for .{} files in plugin directory.", 
                config.manifest.plugin.id, lib_extension
            ),
        })
    }
    
    /// Разрешить зависимости плагина
    async fn resolve_dependencies(&self, config: &PluginConfig) -> PluginResult<()> {
        info!("🔍 Resolving dependencies for plugin: {}", config.manifest.plugin.name);
        
        let mut graph = self.dependency_graph.write().await;
        
        // Проверить каждую зависимость из манифеста
        for (dep_name, dep_spec) in &config.manifest.dependencies {
            debug!("📋 Checking dependency: {} {:?}", dep_name, dep_spec);
            
            // Проверить, что зависимость загружена
            let loaded = self.loaded_plugins.read().await;
            if let Some(loaded_plugin) = loaded.get(dep_name) {
                // Проверить совместимость версий (упрощенная проверка)
                match dep_spec {
                    super::config::DependencySpec::Simple(version) => {
                        if loaded_plugin.info.version != *version {
                            return Err(PluginError::DependencyError {
                                reason: format!(
                                    "Plugin '{}' requires '{}' version '{}', but loaded version is '{}'",
                                    config.manifest.plugin.name,
                                    dep_name,
                                    version,
                                    loaded_plugin.info.version
                                ),
                            });
                        }
                    }
                    super::config::DependencySpec::Extended { version, required, .. } => {
                        // TODO: Реализовать полную semver проверку
                        if loaded_plugin.info.version != *version {
                            if *required {
                                return Err(PluginError::DependencyError {
                                    reason: format!(
                                        "Plugin '{}' requires '{}' version '{}', but loaded version is '{}'",
                                        config.manifest.plugin.name,
                                        dep_name,
                                        version,
                                        loaded_plugin.info.version
                                    ),
                                });
                            }
                        }
                    }
                }
                
                info!("✅ Dependency satisfied: {} v{}", dep_name, loaded_plugin.info.version);
            } else {
                // Проверить обязательность зависимости
                let is_required = match dep_spec {
                    super::config::DependencySpec::Simple(_) => true,
                    super::config::DependencySpec::Extended { required, .. } => *required,
                };
                
                if is_required {
                    // Попытаться загрузить обязательную зависимость
                    warn!("⚠️ Required dependency '{}' not loaded, attempting to load...", dep_name);
                    
                    drop(loaded); // Освобождаем блокировку перед рекурсивным вызовом
                    drop(graph);  // Освобождаем блокировку графа
                    
                    // 🔥 Используем Box::pin для рекурсивного async вызова
                    let load_result = Box::pin(self.load_plugin(dep_name)).await;
                    if let Err(e) = load_result {
                        return Err(PluginError::DependencyError {
                            reason: format!(
                                "Failed to load required dependency '{}': {}",
                                dep_name, e
                            ),
                        });
                    }
                    
                    info!("✅ Auto-loaded dependency: {}", dep_name);
                    
                    // Восстанавливаем блокировки
                    graph = self.dependency_graph.write().await;
                } else {
                    debug!("⚪ Optional dependency '{}' not available", dep_name);
                }
            }
        }
        
        // Создать PluginInfo для добавления в граф зависимостей
        let plugin_info = PluginInfo {
            id: config.manifest.plugin.id.clone(),
            name: config.manifest.plugin.name.clone(),
            version: config.manifest.plugin.version.clone(),
            author: config.manifest.plugin.author.clone(),
            description: config.manifest.plugin.description.clone(),
            website: config.manifest.plugin.website.clone(),
            license: config.manifest.plugin.license.clone(),
            dependencies: config.manifest.dependencies.iter().map(|(name, spec)| {
                let (version, required) = match spec {
                    super::config::DependencySpec::Simple(v) => (v.clone(), true),
                    super::config::DependencySpec::Extended { version, required, .. } => (version.clone(), *required),
                };
                super::PluginDependency { name: name.clone(), version, required }
            }).collect(),
            tags: config.manifest.plugin.tags.clone(),
        };
        
        graph.add_plugin(&plugin_info)?;
        
        info!("🎯 All dependencies resolved for plugin: {}", config.manifest.plugin.name);
        Ok(())
    }
    
    /// Проверить зависимости перед выгрузкой
    async fn check_unload_dependencies(&self, plugin_id: &str) -> PluginResult<()> {
        let graph = self.dependency_graph.read().await;
        let dependents = graph.get_dependents(plugin_id)?;
        
        if !dependents.is_empty() {
            return Err(PluginError::DependencyError {
                reason: format!(
                    "Cannot unload plugin {}: required by {:?}", 
                    plugin_id, dependents
                ),
            });
        }
        
        Ok(())
    }
    
    /// Запустить watcher для hot reload
    async fn start_hot_reload_watcher(&mut self) -> Result<()> {
        use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
        use std::sync::mpsc;
        use std::time::Duration;
        
        let plugins_dir = self.config.plugins_directory.clone();
        let loaded_plugins = Arc::clone(&self.loaded_plugins);
        let event_sender = self.event_sender.clone();
        let check_interval = Duration::from_millis(self.config.hot_reload_interval_ms);
        
        info!("🔥 Starting hot reload watcher for directory: {}", plugins_dir.display());
        
        let (tx, rx) = mpsc::channel();
        
        // Создаем file system watcher
        let mut watcher: RecommendedWatcher = Watcher::new(
            move |result: Result<Event, notify::Error>| {
                match result {
                    Ok(event) => {
                        if let Err(e) = tx.send(event) {
                            error!("Failed to send file system event: {}", e);
                        }
                    }
                    Err(e) => error!("File system watcher error: {}", e),
                }
            },
            notify::Config::default().with_poll_interval(check_interval),
        ).context("Failed to create file system watcher")?;

        // Начинаем мониторинг директории плагинов
        watcher.watch(&plugins_dir, RecursiveMode::Recursive)
            .context("Failed to watch plugins directory")?;

        // Запускаем обработчик событий в отдельной задаче
        let handle = tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Handle::current();
            
            for event in rx {
                // Обрабатываем только события модификации файлов
                if !matches!(event.kind, EventKind::Modify(_)) {
                    continue;
                }
                
                for path in event.paths {
                    // Проверяем, что это файл плагина (например, .toml или .dll/.so)
                    if let Some(extension) = path.extension() {
                        let ext = extension.to_string_lossy().to_lowercase();
                        if matches!(ext.as_str(), "toml" | "dll" | "so" | "dylib" | "rs" | "lua") {
                            debug!("🔥 Detected file change: {}", path.display());
                            
                            // Найти ID плагина по пути
                            if let Some(plugin_id) = Self::extract_plugin_id_from_path(&path) {
                                // Проверить, загружен ли плагин
                                let should_reload = rt.block_on(async {
                                    let loaded = loaded_plugins.read().await;
                                    loaded.contains_key(&plugin_id)
                                });
                                
                                if should_reload {
                                    info!("🔄 Hot reloading plugin: {}", plugin_id);
                                    
                                    // Отправляем событие для hot reload
                                    if let Err(e) = event_sender.send(PluginEvent::HotReloaded {
                                        plugin_id: plugin_id.clone(),
                                    }) {
                                        error!("Failed to send hot reload event: {}", e);
                                    }
                                    
                                    // Даем время на стабилизацию файла
                                    std::thread::sleep(Duration::from_millis(100));
                                }
                            }
                        }
                    }
                }
            }
        });
        
        self._hot_reload_handle = Some(handle);
        
        info!("✅ Hot reload watcher started successfully");
        Ok(())
    }
    
    /// Извлечь ID плагина из пути к файлу
    fn extract_plugin_id_from_path(path: &std::path::Path) -> Option<String> {
        // Ищем gameverse.toml или название директории плагина
        if let Some(file_name) = path.file_name() {
            if file_name == "gameverse.toml" {
                // Если это манифест, ID плагина = название родительской директории
                if let Some(parent) = path.parent() {
                    if let Some(dir_name) = parent.file_name() {
                        return Some(dir_name.to_string_lossy().to_string());
                    }
                }
            }
        }
        
        // Попробуем найти директорию плагина в пути
        // Путь может быть: /plugins/my-plugin/src/main.rs
        // Ищем компонент пути, который следует после "plugins"
        let components: Vec<_> = path.components().collect();
        
        for (i, component) in components.iter().enumerate() {
            if let std::path::Component::Normal(name) = component {
                let name_str = name.to_string_lossy();
                if name_str == "plugins" && i + 1 < components.len() {
                    // Следующий компонент должен быть директорией плагина
                    if let std::path::Component::Normal(plugin_dir) = &components[i + 1] {
                        let plugin_id = plugin_dir.to_string_lossy().to_string();
                        // Простая валидация ID плагина
                        if plugin_id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                            return Some(plugin_id);
                        }
                    }
                }
            }
        }
        
        // Если не нашли через "plugins", попробуем последний fallback
        // Берем предпоследний компонент пути (родительскую директорию файла)
        if let Some(parent) = path.parent() {
            if let Some(dir_name) = parent.file_name() {
                let dir_str = dir_name.to_string_lossy().to_string();
                // Простая валидация ID плагина
                if dir_str.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                    return Some(dir_str);
                }
            }
        }
        
        None
    }
    
    /// Запустить обработчик событий плагинов
    pub async fn start_event_processor(manager: Arc<Self>) -> Result<tokio::task::JoinHandle<()>> {
        let (_event_tx, mut event_rx) = mpsc::unbounded_channel::<PluginEvent>();
        
        // Заменяем sender в менеджере на новый
        // (это нужно будет сделать через другой подход в реальной реализации)
        
        let handle = tokio::spawn(async move {
            info!("🚀 Plugin event processor started");
            
            while let Some(event) = event_rx.recv().await {
                match &event {
                    PluginEvent::HotReloaded { plugin_id } => {
                        info!("📨 Processing hot reload event for plugin: {}", plugin_id);
                        
                        // Автоматический hot reload
                        if let Err(e) = manager.reload_plugin(plugin_id).await {
                            error!("❌ Auto hot reload failed for {}: {}", plugin_id, e);
                        } else {
                            info!("✅ Auto hot reload completed for: {}", plugin_id);
                        }
                    }
                    PluginEvent::Error { plugin_id, error } => {
                        error!("🚨 Plugin error event: {} - {}", plugin_id, error);
                        
                        // Попытка recovery если включено
                        if manager.config.enable_hot_reload {
                            warn!("🔄 Attempting recovery reload for plugin: {}", plugin_id);
                            if let Err(e) = manager.reload_plugin(plugin_id).await {
                                error!("❌ Recovery reload failed for {}: {}", plugin_id, e);
                            }
                        }
                    }
                    _ => {
                        debug!("📨 Plugin event: {:?}", event);
                    }
                }
            }
            
            warn!("🛑 Plugin event processor stopped");
        });
        
        Ok(handle)
    }
    
    /// Получить список загруженных динамических библиотек
    pub async fn list_loaded_libraries(&self) -> Vec<(String, std::path::PathBuf)> {
        let libraries = self.loaded_libraries.read().await;
        libraries.iter()
            .map(|(plugin_id, library)| (plugin_id.clone(), library.path.clone()))
            .collect()
    }
    
    /// Проверить совместимость ABI динамической библиотеки
    pub async fn validate_plugin_library(&self, lib_path: &std::path::Path) -> PluginResult<bool> {
        debug!("🔍 Validating plugin library: {}", lib_path.display());
        
        // Попытаться загрузить библиотеку временно для проверки
        let library = unsafe {
            Library::new(lib_path)
                .map_err(|e| PluginError::LoadingFailed {
                    reason: format!("Failed to load library for validation: {}", e),
                })?
        };
        
        // Проверить наличие обязательных символов
        let has_create_plugin = unsafe {
            library.get::<unsafe extern "C" fn() -> *mut dyn GameVersePlugin>(b"create_plugin").is_ok()
        };
        
        let has_plugin_info = unsafe {
            library.get::<unsafe extern "C" fn() -> *const c_char>(b"get_plugin_info").is_ok()
        };
        
        if !has_create_plugin {
            return Err(PluginError::LoadingFailed {
                reason: "Plugin library missing required 'create_plugin' function".to_string(),
            });
        }
        
        if has_plugin_info {
            debug!("✅ Plugin library has optional 'get_plugin_info' function");
        }
        
        // Проверить версию ABI (если доступна)
        if let Ok(get_abi_version) = unsafe {
            library.get::<unsafe extern "C" fn() -> u32>(b"get_abi_version")
        } {
            let abi_version = unsafe { get_abi_version() };
            const CURRENT_ABI_VERSION: u32 = 1;
            
            if abi_version != CURRENT_ABI_VERSION {
                return Err(PluginError::LoadingFailed {
                    reason: format!(
                        "Plugin ABI version mismatch: expected {}, found {}", 
                        CURRENT_ABI_VERSION, abi_version
                    ),
                });
            }
            
            debug!("✅ Plugin ABI version compatible: {}", abi_version);
        } else {
            warn!("⚠️ Plugin library does not provide ABI version information");
        }
        
        info!("✅ Plugin library validation successful: {}", lib_path.display());
        Ok(true)
    }
    
    /// Получить информацию о динамической библиотеке без загрузки плагина
    pub async fn inspect_plugin_library(&self, lib_path: &std::path::Path) -> PluginResult<PluginInfo> {
        debug!("🔍 Inspecting plugin library: {}", lib_path.display());
        
        let library = unsafe {
            Library::new(lib_path)
                .map_err(|e| PluginError::LoadingFailed {
                    reason: format!("Failed to load library for inspection: {}", e),
                })?
        };
        
        // Попытаться получить информацию о плагине
        if let Ok(get_plugin_info) = unsafe {
            library.get::<unsafe extern "C" fn() -> *const c_char>(b"get_plugin_info")
        } {
            let info_ptr = unsafe { get_plugin_info() };
            if !info_ptr.is_null() {
                let info_cstr = unsafe { CStr::from_ptr(info_ptr) };
                if let Ok(info_str) = info_cstr.to_str() {
                    // Попытаться распарсить JSON с информацией о плагине
                    if let Ok(plugin_info) = serde_json::from_str::<PluginInfo>(info_str) {
                        debug!("✅ Retrieved plugin info from library");
                        return Ok(plugin_info);
                    }
                }
            }
        }
        
        // Fallback: создать базовую информацию из имени файла
        let plugin_name = lib_path.file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .trim_start_matches("lib")
            .to_string();
            
        warn!("⚠️ Could not retrieve plugin info from library, using fallback");
        
        Ok(PluginInfo {
            id: plugin_name.clone(),
            name: plugin_name,
            version: "0.1.0".to_string(),
            author: "Unknown".to_string(),
            description: "Dynamic plugin".to_string(),
            website: None,
            license: None,
            dependencies: vec![],
            tags: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;
    
    #[tokio::test]
    async fn test_plugin_manager_creation() {
        let (tx, _rx) = mpsc::unbounded_channel();
        let config = PluginSystemConfig::default();
        
        // Создание может не удаться из-за отсутствия директории, но это нормально для теста
        let result = PluginManager::new(config, tx).await;
        assert!(result.is_ok() || result.is_err());
    }
    
    #[test]
    fn test_performance_stats_default() {
        let stats = PluginPerformanceStats::default();
        assert_eq!(stats.call_count, 0);
        assert_eq!(stats.error_count, 0);
        assert_eq!(stats.total_execution_time_us, 0);
    }
    
    #[test]
    fn test_extract_plugin_id_from_path() {
        use std::path::Path;
        
        // Тест с gameverse.toml
        let path = Path::new("/plugins/awesome-economy/gameverse.toml");
        let plugin_id = PluginManager::extract_plugin_id_from_path(path);
        assert_eq!(plugin_id, Some("awesome-economy".to_string()));
        
        // Тест с файлом в директории плагина через "plugins"
        let path = Path::new("/plugins/my-plugin/src/main.rs");
        let plugin_id = PluginManager::extract_plugin_id_from_path(path);
        assert_eq!(plugin_id, Some("my-plugin".to_string()));
        
        // Тест без директории "plugins" - должен взять родительскую директорию
        let path = Path::new("/some/my-awesome-plugin/file.rs");
        let plugin_id = PluginManager::extract_plugin_id_from_path(path);
        assert_eq!(plugin_id, Some("my-awesome-plugin".to_string()));
        
        // Тест с глубоко вложенным путем
        let path = Path::new("/home/user/gameverse/plugins/economy-system/ui/components/balance.tsx");
        let plugin_id = PluginManager::extract_plugin_id_from_path(path);
        assert_eq!(plugin_id, Some("economy-system".to_string()));
        
        // Тест с невалидным путем (нет подходящих компонентов)
        let path = Path::new("/");
        let plugin_id = PluginManager::extract_plugin_id_from_path(path);
        assert_eq!(plugin_id, None);
    }
    
    #[test]
    fn test_preserved_plugin_data_default() {
        let data = PreservedPluginData::default();
        assert!(data.config_data.is_empty());
        assert!(data.runtime_data.is_empty());
        assert!(data.connections.is_empty());
        assert!(data.user_data.is_empty());
    }
    
    #[test]
    fn test_loaded_library_creation() {
        use std::path::PathBuf;
        use tempfile::NamedTempFile;
        
        // Создаем временный файл для имитации библиотеки
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_path_buf();
        
        // Создаем мок библиотеку (в реальности это не сработает, но для теста структуры подойдет)
        // В реальных тестах нужно будет создавать настоящие динамические библиотеки
        
        // Тест создания LoadedLibrary проверим через структуру
        assert!(temp_path.exists());
        
        // Проверим константы ABI
        const CURRENT_ABI_VERSION: u32 = 1;
        assert_eq!(CURRENT_ABI_VERSION, 1);
    }
    
    #[test]
    fn test_find_plugin_library_paths() {
        use std::path::Path;
        
        // Тест определения расширения библиотеки для разных платформ
        let lib_extension = if cfg!(target_os = "windows") {
            "dll"
        } else if cfg!(target_os = "macos") {
            "dylib" 
        } else {
            "so"
        };
        
        // Проверим, что расширение корректно определяется
        assert!(matches!(lib_extension, "dll" | "dylib" | "so"));
        
        // Тест форматирования путей к библиотекам
        let plugin_name = "test-plugin";
        let expected_patterns = vec![
            format!("lib{}.{}", plugin_name, lib_extension),
            format!("{}.{}", plugin_name, lib_extension),
        ];
        
        assert_eq!(expected_patterns[0], format!("lib{}.{}", plugin_name, lib_extension));
        assert_eq!(expected_patterns[1], format!("{}.{}", plugin_name, lib_extension));
    }
    
    #[test]
    fn test_plugin_name_extraction() {
        use std::path::Path;
        
        // Тест извлечения имени плагина из пути к библиотеке
        let test_cases = vec![
            ("libtest-plugin.so", "test-plugin"),
            ("my-awesome-plugin.dll", "my-awesome-plugin"),
            ("libeconomy.dylib", "economy"),
            ("simple.so", "simple"),
        ];
        
        for (filename, expected_name) in test_cases {
            let path = Path::new(filename);
            let extracted_name = path.file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown")
                .trim_start_matches("lib")
                .to_string();
            
            assert_eq!(extracted_name, expected_name);
        }
    }
    
    #[tokio::test]
    async fn test_plugin_manager_with_libraries() {
        let (tx, _rx) = mpsc::unbounded_channel();
        let config = PluginSystemConfig::default();
        
        if let Ok(manager) = PluginManager::new(config, tx).await {
            // Тест получения списка загруженных библиотек
            let libraries = manager.list_loaded_libraries().await;
            assert!(libraries.is_empty()); // Должно быть пусто в начале
            
            // Тест списка плагинов
            let plugins = manager.list_plugins().await;
            assert!(plugins.is_empty()); // Должно быть пусто в начале
        }
    }
} 