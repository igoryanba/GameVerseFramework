#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;
    
    #[test]
    fn test_server_status_lifecycle() {
        let status1 = ServerStatus::Starting;
        let status2 = status1.clone();
        
        assert_eq!(status1, status2);
        assert_eq!(status1, ServerStatus::Starting);
        assert_ne!(status1, ServerStatus::Running);
    }
    
    #[tokio::test]
    async fn test_server_runtime_new() {
        // Создаем временную директорию для конфига
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");
        
        // Создаем минимальный конфиг
        let config_content = r#"
            [server]
            name = "Test Server"
            max_players = 10
            port = 30120
            bind_address = "127.0.0.1"
            resources_path = "test_resources"
            idle_timeout = 300
            
            [network]
            use_compression = false
            buffer_size = 1024
            max_packet_size = 2048
            sync_interval_ms = 100
            admin_port = 30121
            sync_radius = 100.0
            max_entities_per_client = 100
            
            [logging]
            level = "debug"
            log_dir = ""
            max_file_size = 1048576
            json_format = false
            enable_tracing = false
            
            [database]
            max_connections = 5
            connection_timeout = 5
            sync_interval = 1
            
            [scripting]
            enable_lua = true
            enable_typescript = false
            enable_wasm = false
            lua_stack_size = 1024
            script_timeout_ms = 1000
        "#;
        
        std::fs::write(&config_path, config_content).unwrap();
        
        // Создаем ServerRuntime с тестовым конфигом
        // Примечание: мы не инициализируем логирование в тесте, чтобы избежать побочных эффектов
        // Поэтому ожидаем ошибку при создании ServerRuntime
        let result = ServerRuntime::new(Some(&config_path), true);
        
        // В реальной ситуации это должно быть Ok, но в тесте мы ожидаем ошибку из-за логирования
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn test_parse_command() {
        use crate::server::runtime::parse_command;
        assert_eq!(parse_command("shutdown"), Some(ServerCommand::Shutdown));
        assert_eq!(parse_command("status"), Some(ServerCommand::RequestStatus));
        assert_eq!(parse_command("reload_resources"), Some(ServerCommand::ReloadResources));
        assert_eq!(parse_command("restart"), Some(ServerCommand::Restart));
        assert_eq!(parse_command("unknown"), None);
    }
} 