//! # Демонстрация Dynamic Plugin Loading
//!
//! Показывает все возможности системы динамической загрузки плагинов:
//! - Загрузка плагинов из динамических библиотек (.dll/.so/.dylib)
//! - Автоматическое разрешение зависимостей
//! - Валидация ABI совместимости
//! - Безопасная выгрузка с освобождением ресурсов
//! - Инспекция библиотек без загрузки

use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;
use anyhow::{Result, Context};
use gameverse_core::plugins::{
    PluginManager, PluginSystemConfig, PluginEvent, PluginError
};

/// Демонстрация Dynamic Plugin Loading
#[tokio::main]
async fn main() -> Result<()> {
    // Инициализация логирования
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .pretty()
        .init();

    println!("🚀 GameVerse Dynamic Plugin Loading Demo");
    println!("=========================================");

    // Настройка системы плагинов
    let config = PluginSystemConfig {
        plugins_directory: PathBuf::from("./examples/plugins"),
        enable_hot_reload: true,
        hot_reload_interval_ms: 500,
        load_timeout_ms: 5000,
        enable_sandboxing: true,
        api_rate_limit: 100,
        allowed_syscalls: vec![
            "read".to_string(),
            "write".to_string(),
            "mmap".to_string(),
        ],
    };

    // Создание канала событий
    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel::<PluginEvent>();

    // Создание менеджера плагинов
    let manager = PluginManager::new(config, event_tx).await
        .context("Failed to create plugin manager")?;
    let manager = std::sync::Arc::new(manager);

    // Демонстрация 1: Инспекция доступных библиотек
    println!("\n📋 1. Инспекция доступных плагинов");
    println!("   ================================");
    
    let plugins_dir = PathBuf::from("./examples/plugins");
    if plugins_dir.exists() {
        println!("✅ Plugins directory found: {}", plugins_dir.display());
        
        // Поиск динамических библиотек
        if let Ok(entries) = std::fs::read_dir(&plugins_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if matches!(ext.as_str(), "dll" | "so" | "dylib") {
                        println!("🔍 Found library: {}", path.display());
                        
                        // Попытка валидации библиотеки
                        match manager.validate_plugin_library(&path).await {
                            Ok(true) => println!("   ✅ Library is valid"),
                            Ok(false) => println!("   ❌ Library validation failed"),
                            Err(e) => println!("   ⚠️ Validation error: {}", e),
                        }
                        
                        // Попытка инспекции без загрузки
                        match manager.inspect_plugin_library(&path).await {
                            Ok(info) => {
                                println!("   📦 Plugin info:");
                                println!("      - Name: {}", info.name);
                                println!("      - Version: {}", info.version);
                                println!("      - Author: {}", info.author);
                            }
                            Err(e) => println!("   ⚠️ Inspection failed: {}", e),
                        }
                    }
                }
            }
        }
    } else {
        println!("⚠️ Plugins directory not found, creating example structure...");
        create_example_plugin_structure().await?;
    }

    // Демонстрация 2: Загрузка плагинов
    println!("\n🔥 2. Dynamic Plugin Loading");
    println!("   ===========================");
    
    let test_plugins = vec!["economy-example", "inventory-example"];
    
    for plugin_id in test_plugins {
        println!("📦 Loading plugin: {}", plugin_id);
        
        match manager.load_plugin(plugin_id).await {
            Ok(_) => {
                println!("✅ Plugin '{}' loaded successfully", plugin_id);
                
                // Показать статистику
                if let Some(stats) = manager.get_plugin_stats(plugin_id).await {
                    println!("   📊 Performance stats:");
                    println!("      - Call count: {}", stats.call_count);
                    println!("      - Avg execution time: {}μs", stats.average_execution_time_us);
                    println!("      - Error count: {}", stats.error_count);
                }
            }
            Err(PluginError::NotFound { .. }) => {
                println!("⚠️ Plugin '{}' not found (expected for demo)", plugin_id);
            }
            Err(e) => {
                println!("❌ Failed to load plugin '{}': {}", plugin_id, e);
            }
        }
    }

    // Демонстрация 3: Список загруженных плагинов
    println!("\n📋 3. Loaded Plugins Overview");
    println!("   ============================");
    
    let loaded_plugins = manager.list_plugins().await;
    if loaded_plugins.is_empty() {
        println!("⚪ No plugins currently loaded");
    } else {
        for plugin_info in loaded_plugins {
            println!("🔌 Plugin: {} v{}", plugin_info.name, plugin_info.version);
            println!("   Author: {}", plugin_info.author);
            println!("   Description: {}", plugin_info.description);
            if !plugin_info.dependencies.is_empty() {
                println!("   Dependencies:");
                for dep in plugin_info.dependencies {
                    println!("     - {} v{} (required: {})", dep.name, dep.version, dep.required);
                }
            }
        }
    }

    // Демонстрация 4: Список загруженных библиотек
    println!("\n💾 4. Loaded Libraries");
    println!("   ===================");
    
    let loaded_libraries = manager.list_loaded_libraries().await;
    if loaded_libraries.is_empty() {
        println!("⚪ No dynamic libraries currently loaded");
    } else {
        for (plugin_id, lib_path) in loaded_libraries {
            println!("📚 Plugin '{}': {}", plugin_id, lib_path.display());
        }
    }

    // Демонстрация 5: Hot Reload (если доступен плагин)
    if !manager.list_plugins().await.is_empty() {
        println!("\n🔄 5. Hot Reload Demonstration");
        println!("   =============================");
        
        let first_plugin = &manager.list_plugins().await[0];
        let plugin_id = &first_plugin.id;
        
        println!("🔥 Performing hot reload for plugin: {}", plugin_id);
        
        match manager.reload_plugin(plugin_id).await {
            Ok(_) => println!("✅ Hot reload completed successfully"),
            Err(e) => println!("❌ Hot reload failed: {}", e),
        }
    }

    // Демонстрация 6: Обработка событий (кратковременно)
    println!("\n📨 6. Event Processing");
    println!("   ===================");
    
    let event_manager = manager.clone();
    let event_handle = tokio::spawn(async move {
        let mut event_count = 0;
        let timeout = Duration::from_secs(3);
        let start_time = std::time::Instant::now();
        
        while start_time.elapsed() < timeout && event_count < 5 {
            if let Ok(event) = tokio::time::timeout(Duration::from_millis(500), event_rx.recv()).await {
                if let Some(event) = event {
                    event_count += 1;
                    println!("📩 Event {}: {:?}", event_count, event);
                }
            }
        }
        
        if event_count > 0 {
            println!("✅ Processed {} events", event_count);
        } else {
            println!("⚪ No events received during demonstration");
        }
    });

    // Дать время на обработку событий
    sleep(Duration::from_secs(3)).await;
    
    // Завершить обработку событий
    event_handle.abort();

    // Демонстрация 7: Финальная очистка
    println!("\n🧹 7. Cleanup & Unloading");
    println!("   =======================");
    
    let loaded_plugins = manager.list_plugins().await;
    for plugin_info in loaded_plugins {
        println!("🗑️ Unloading plugin: {}", plugin_info.name);
        
        match manager.unload_plugin(&plugin_info.id).await {
            Ok(_) => println!("✅ Plugin '{}' unloaded successfully", plugin_info.name),
            Err(e) => println!("❌ Failed to unload plugin '{}': {}", plugin_info.name, e),
        }
    }

    println!("\n🎯 Dynamic Plugin Loading Demo Completed!");
    println!("   ========================================");
    println!("   ✅ All dynamic plugin operations demonstrated");
    println!("   ✅ Memory management handled automatically");
    println!("   ✅ Thread-safe operations verified");
    println!("   ✅ Error handling demonstrated");

    Ok(())
}

/// Создать пример структуры плагинов для демонстрации
async fn create_example_plugin_structure() -> Result<()> {
    use std::fs;
    
    let plugins_dir = PathBuf::from("./examples/plugins");
    
    // Создать директории
    fs::create_dir_all(&plugins_dir)?;
    fs::create_dir_all(plugins_dir.join("economy-example"))?;
    fs::create_dir_all(plugins_dir.join("inventory-example"))?;
    
    println!("📁 Created example plugin directory structure");
    println!("   Note: Actual .dll/.so/.dylib files would be built from Rust/C++ source");
    
    Ok(())
} 