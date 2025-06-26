#[cfg(unix)]
#[tokio::test]
async fn test_ipc_status_unix() {
    use gameverse_core::server::runtime::ServerRuntime;
    use std::path::Path;
    use tempfile::TempDir;
    use tokio::io::{AsyncWriteExt, AsyncReadExt};
    use tokio::net::UnixStream;

    // 1. Подготовим временный конфиг TOML
    let dir = TempDir::new().unwrap();
    let cfg_path = dir.path().join("test_server.toml");
    std::fs::write(&cfg_path, BASIC_CONFIG).unwrap();

    // 2. Запускаем сервер в отдельной задаче
    let mut runtime = ServerRuntime::new(Some(&cfg_path), true).expect("runtime create");
    let handle = tokio::spawn(async move {
        // ignore error on shutdown for test
        let _ = runtime.start().await;
    });

    // Дадим время серверу подняться и создать сокет
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // 3. Подключаемся к IPC и отправляем команду status
    let mut stream = UnixStream::connect("/tmp/gameverse_server.sock").await.expect("connect socket");
    stream.write_all(b"status").await.unwrap();
    stream.shutdown().await.unwrap();

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.unwrap();
    let resp = String::from_utf8_lossy(&buf);
    let json: serde_json::Value = serde_json::from_str(&resp).expect("json parse");

    assert_eq!(json["status"], "running");
    assert!(json["uptime_secs"].as_u64().unwrap() >= 0);
    assert!(json["mem_rss_mb"].as_u64().unwrap() >= 0);

    // 4. Завершаем сервер
    let mut stream2 = UnixStream::connect("/tmp/gameverse_server.sock").await.unwrap();
    stream2.write_all(b"shutdown").await.unwrap();
    stream2.shutdown().await.unwrap();

    let _ = handle.await;
}

const BASIC_CONFIG: &str = r#"
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
level = "error"
log_dir = ""
max_file_size = 1048576
json_format = false
enable_tracing = false

[database]
max_connections = 5
connection_timeout = 5
sync_interval = 1

[scripting]
enable_lua = false
enable_typescript = false
enable_wasm = false
lua_stack_size = 1024
script_timeout_ms = 1000
"#;