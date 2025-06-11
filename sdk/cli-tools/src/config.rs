//! Configuration management for GameVerse CLI

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::{Result, Context};
use std::collections::HashMap;

/// Main configuration structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Plugin development settings
    pub plugin: PluginConfig,
    /// Server connection settings
    pub server: ServerConfig,
    /// Marketplace settings
    pub marketplace: MarketplaceConfig,
    /// Build settings
    pub build: BuildConfig,
    /// Template settings
    pub templates: TemplateConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginConfig {
    /// Default plugin template
    pub default_template: String,
    /// Default language for new plugins
    pub default_language: String,
    /// Plugin development directory
    pub dev_directory: Option<PathBuf>,
    /// Auto-reload plugins on changes
    pub auto_reload: bool,
    /// Default plugin author
    pub default_author: Option<String>,
    /// Default plugin license
    pub default_license: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    /// Default server host
    pub default_host: String,
    /// Default server port
    pub default_port: u16,
    /// Server API key for deployment
    pub api_key: Option<String>,
    /// SSL/TLS settings
    pub use_tls: bool,
    /// Connection timeout (seconds)
    pub timeout: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketplaceConfig {
    /// Marketplace URL
    pub url: String,
    /// User authentication token
    pub auth_token: Option<String>,
    /// Auto-update check
    pub auto_update_check: bool,
    /// Cache directory
    pub cache_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildConfig {
    /// Target platforms for building
    pub default_targets: Vec<String>,
    /// Optimization level (debug, release)
    pub optimization: String,
    /// Cross-compilation settings
    pub cross_compile: HashMap<String, String>,
    /// Build output directory
    pub output_dir: Option<PathBuf>,
    /// Enable parallel builds
    pub parallel: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TemplateConfig {
    /// Template repository URL
    pub repository: String,
    /// Local template directory
    pub local_dir: Option<PathBuf>,
    /// Template cache duration (hours)
    pub cache_duration: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            plugin: PluginConfig {
                default_template: "basic".to_string(),
                default_language: "rust".to_string(),
                dev_directory: None,
                auto_reload: true,
                default_author: None,
                default_license: "MIT".to_string(),
            },
            server: ServerConfig {
                default_host: "localhost".to_string(),
                default_port: 8080,
                api_key: None,
                use_tls: false,
                timeout: 30,
            },
            marketplace: MarketplaceConfig {
                url: "https://marketplace.gameverse.dev".to_string(),
                auth_token: None,
                auto_update_check: true,
                cache_dir: None,
            },
            build: BuildConfig {
                default_targets: vec![
                    "windows".to_string(),
                    "linux".to_string(),
                    "macos".to_string(),
                ],
                optimization: "release".to_string(),
                cross_compile: HashMap::new(),
                output_dir: None,
                parallel: true,
            },
            templates: TemplateConfig {
                repository: "https://github.com/gameverse/plugin-templates".to_string(),
                local_dir: None,
                cache_duration: 24,
            },
        }
    }
}

/// Load configuration from file or create default
pub async fn load_config(config_path: Option<&str>) -> Result<Config> {
    let config_file = config_path
        .map(PathBuf::from)
        .or_else(|| default_config_path())
        .unwrap_or_else(|| PathBuf::from("gameverse.toml"));

    if config_file.exists() {
        let content = tokio::fs::read_to_string(&config_file)
            .await
            .with_context(|| format!("Failed to read config file: {}", config_file.display()))?;
        
        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_file.display()))?;
        
        Ok(config)
    } else {
        // Create default config and save it
        let config = Config::default();
        save_config(&config, &config_file).await?;
        Ok(config)
    }
}

/// Save configuration to file
pub async fn save_config(config: &Config, path: &PathBuf) -> Result<()> {
    let content = toml::to_string_pretty(config)
        .context("Failed to serialize config")?;
    
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .context("Failed to create config directory")?;
    }
    
    tokio::fs::write(path, content)
        .await
        .with_context(|| format!("Failed to write config file: {}", path.display()))?;
    
    Ok(())
}

/// Get default configuration file path
fn default_config_path() -> Option<PathBuf> {
    // Try to find config in:
    // 1. Current directory
    // 2. User config directory
    // 3. Home directory
    
    let current_dir = std::env::current_dir().ok()?;
    let config_file = current_dir.join("gameverse.toml");
    if config_file.exists() {
        return Some(config_file);
    }
    
    // Try user config directory
    if let Some(config_dir) = dirs::config_dir() {
        let config_file = config_dir.join("gameverse").join("config.toml");
        if config_file.exists() {
            return Some(config_file);
        }
    }
    
    // Try home directory
    if let Some(home_dir) = dirs::home_dir() {
        let config_file = home_dir.join(".gameverse.toml");
        if config_file.exists() {
            return Some(config_file);
        }
    }
    
    None
}

/// Update configuration with environment variables
pub fn update_from_env(config: &mut Config) {
    // Server settings
    if let Ok(host) = std::env::var("GAMEVERSE_SERVER_HOST") {
        config.server.default_host = host;
    }
    if let Ok(port) = std::env::var("GAMEVERSE_SERVER_PORT") {
        if let Ok(port) = port.parse() {
            config.server.default_port = port;
        }
    }
    if let Ok(api_key) = std::env::var("GAMEVERSE_API_KEY") {
        config.server.api_key = Some(api_key);
    }
    
    // Plugin settings
    if let Ok(author) = std::env::var("GAMEVERSE_PLUGIN_AUTHOR") {
        config.plugin.default_author = Some(author);
    }
    if let Ok(license) = std::env::var("GAMEVERSE_PLUGIN_LICENSE") {
        config.plugin.default_license = license;
    }
    
    // Marketplace settings
    if let Ok(token) = std::env::var("GAMEVERSE_MARKETPLACE_TOKEN") {
        config.marketplace.auth_token = Some(token);
    }
} 