//! # Реестр плагинов GameVerse
//!
//! Управление реестром доступных плагинов

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::Result;
use crate::plugins::{PluginConfig, PluginError, PluginResult};

/// Реестр плагинов
#[derive(Debug)]
pub struct PluginRegistry {
    /// Директория с плагинами
    plugins_directory: PathBuf,
    /// Кэш конфигураций плагинов
    plugin_configs: HashMap<String, PluginConfig>,
}

impl PluginRegistry {
    /// Создать новый реестр плагинов
    pub fn new(plugins_directory: &Path) -> Result<Self> {
        Ok(Self {
            plugins_directory: plugins_directory.to_path_buf(),
            plugin_configs: HashMap::new(),
        })
    }
    
    /// Получить конфигурацию плагина
    pub async fn get_plugin_config(&self, plugin_id: &str) -> PluginResult<PluginConfig> {
        // TODO: Реализовать загрузку конфигурации плагина
        Err(PluginError::NotFound {
            plugin_id: plugin_id.to_string(),
        })
    }
    
    /// Сканировать директорию плагинов
    pub async fn scan_plugins(&mut self) -> Result<Vec<String>> {
        // TODO: Реализовать сканирование директории плагинов
        Ok(Vec::new())
    }
    
    /// Проверить существование плагина
    pub fn plugin_exists(&self, plugin_id: &str) -> bool {
        self.plugin_configs.contains_key(plugin_id)
    }
} 