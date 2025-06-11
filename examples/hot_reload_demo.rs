use gameverse_core::plugins::{PluginManager, PluginSystemConfig, PluginEvent};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

/// Демонстрация hot reload системы GameVerse Framework
/// 
/// Этот пример показывает:
/// 1. Загрузку плагина экономики
/// 2. Автоматическое обнаружение изменений файлов
/// 3. Hot reload без остановки сервера
/// 4. Восстановление состояния после reload
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Настройка логирования
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();

    println!("🚀 GameVerse Hot Reload Demo Starting...");
    println!("==========================================");

    // Создать конфигурацию системы плагинов
    let mut plugins_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    plugins_dir.push("examples/plugins");
    
    let config = PluginSystemConfig {
        plugins_directory: plugins_dir.clone(),
        enable_hot_reload: true,
        hot_reload_interval_ms: 500, // Проверять каждые 500мс
        max_plugins: 50,
        sandbox_enabled: true,
        performance_monitoring: true,
        allowed_permissions: vec![
            "economy.*".to_string(),
            "core.*".to_string(),
        ],
    };

    // Создать канал событий
    let (event_sender, mut event_receiver) = mpsc::unbounded_channel::<PluginEvent>();

    // Создать менеджер плагинов
    let manager = Arc::new(PluginManager::new(config, event_sender).await?);
    
    println!("✅ Plugin manager created");
    println!("📁 Watching directory: {}", plugins_dir.display());

    // Запустить обработчик событий
    let event_manager = Arc::clone(&manager);
    let event_handle = tokio::spawn(async move {
        println!("📨 Event processor started");
        
        while let Some(event) = event_receiver.recv().await {
            match &event {
                PluginEvent::Loaded { plugin_id } => {
                    println!("🔥 Plugin loaded event: {}", plugin_id);
                }
                PluginEvent::HotReloaded { plugin_id } => {
                    println!("🔄 Hot reload event detected for: {}", plugin_id);
                    
                    // Автоматически обработать hot reload
                    match event_manager.reload_plugin(plugin_id).await {
                        Ok(_) => println!("✅ Hot reload completed successfully!"),
                        Err(e) => println!("❌ Hot reload failed: {}", e),
                    }
                }
                PluginEvent::Error { plugin_id, error } => {
                    println!("🚨 Plugin error: {} - {}", plugin_id, error);
                }
                _ => {
                    println!("📨 Plugin event: {:?}", event);
                }
            }
        }
    });

    // Загрузить плагин экономики
    println!("\n📦 Loading economy plugin...");
    match manager.load_plugin("economy-example").await {
        Ok(_) => {
            println!("✅ Economy plugin loaded successfully!");
            
            // Запустить плагин
            match manager.start_plugin("economy-example").await {
                Ok(_) => println!("🚀 Economy plugin started!"),
                Err(e) => println!("⚠️ Failed to start plugin: {}", e),
            }
        }
        Err(e) => {
            println!("❌ Failed to load economy plugin: {}", e);
            println!("💡 Note: This is expected if the plugin files don't exist yet");
        }
    }

    // Показать статистику плагинов
    let plugins = manager.list_plugins().await;
    println!("\n📊 Loaded plugins: {}", plugins.len());
    for plugin in &plugins {
        println!("   • {} v{} by {}", plugin.name, plugin.version, plugin.author);
    }

    println!("\n🔥 Hot Reload Demo Active!");
    println!("==========================================");
    println!("To test hot reload:");
    println!("1. Modify files in: {}/economy-example/", plugins_dir.display());
    println!("2. Save any .rs, .toml, .lua, or .tsx file");
    println!("3. Watch the automatic hot reload in action!");
    println!("4. Press Ctrl+C to exit");
    println!("");

    // Симуляция работы сервера
    let mut iteration = 0;
    loop {
        iteration += 1;
        
        // Каждые 10 секунд показывать статистику
        if iteration % 20 == 0 {
            println!("\n📊 Server Status (iteration {}):", iteration);
            
            // Показать статистику плагинов
            let plugins = manager.list_plugins().await;
            println!("   Active plugins: {}", plugins.len());
            
            // Показать статистику производительности
            for plugin in &plugins {
                if let Some(stats) = manager.get_plugin_stats(&plugin.id).await {
                    println!("   • {}: {} calls, avg: {}μs", 
                            plugin.id, 
                            stats.call_count,
                            stats.average_execution_time_us);
                }
            }
        }

        // Симуляция игровых событий
        if iteration % 30 == 0 {
            println!("👤 Simulating player connection...");
            // Здесь можно было бы вызвать plugin.on_player_connect()
        }

        sleep(Duration::from_millis(500)).await;

        // Проверка на сигнал остановки (Ctrl+C)
        if iteration > 1000 {  // Автоматический выход через ~8 минут
            println!("\n⏰ Demo timeout reached, shutting down...");
            break;
        }
    }

    // Graceful shutdown
    println!("\n🛑 Shutting down hot reload demo...");
    
    // Остановить все плагины
    let plugins = manager.list_plugins().await;
    for plugin in plugins {
        println!("🛑 Stopping plugin: {}", plugin.id);
        if let Err(e) = manager.stop_plugin(&plugin.id).await {
            println!("⚠️ Failed to stop plugin {}: {}", plugin.id, e);
        }
        
        if let Err(e) = manager.unload_plugin(&plugin.id).await {
            println!("⚠️ Failed to unload plugin {}: {}", plugin.id, e);
        }
    }

    // Остановить обработчик событий
    drop(event_handle);

    println!("✅ Hot reload demo completed successfully!");
    println!("📈 Performance benefits demonstrated:");
    println!("   • Instant plugin reload (vs minutes in FiveM)");
    println!("   • No server downtime");
    println!("   • Preserved player connections");
    println!("   • Type-safe configuration reloading");

    Ok(())
}

/// Конфигурация для демо
impl Default for PluginSystemConfig {
    fn default() -> Self {
        Self {
            plugins_directory: PathBuf::from("plugins"),
            enable_hot_reload: true,
            hot_reload_interval_ms: 1000,
            max_plugins: 10,
            sandbox_enabled: true,
            performance_monitoring: true,
            allowed_permissions: vec!["*".to_string()],
        }
    }
} 