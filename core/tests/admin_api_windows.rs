#[cfg(windows)]
#[tokio::test]
async fn test_windows_named_pipe_and_rest_api() {
    use gameverse_core::server::runtime::ServerRuntime;
    use std::path::Path;
    use tempfile::TempDir;
    use tokio::net::windows::named_pipe::{ClientOptions, PipeMode};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    // Подготовим временную конфигурацию
    const BASIC_CONFIG: &str = r#"[server]
name = "TestServerWindows"
port = 30120
bind_address = "127.0.0.1"
resources_path = "resources"

[network]
admin_port = 40124
"#;

    let dir = TempDir::new().unwrap();
    let cfg_path = dir.path().join("server.toml");
    std::fs::write(&cfg_path, BASIC_CONFIG).unwrap();

    // Запускаем сервер в фоне
    let mut runtime = ServerRuntime::new(Some(&cfg_path), true).expect("runtime");
    let handle = tokio::spawn(async move {
        let _ = runtime.start().await;
    });

    // Ждем старта сервера
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Тест 1: Named Pipe IPC
    let pipe_name = r"\\.\pipe\gameverse_server";
    let mut client = ClientOptions::new()
        .pipe_mode(PipeMode::Message)
        .open(pipe_name)
        .expect("Failed to connect to named pipe");
    
    // Отправляем status команду
    client.write_all(b"status").await.expect("write");
    
    let mut buf = vec![0u8; 1024];
    let n = client.read(&mut buf).await.expect("read");
    let response = String::from_utf8_lossy(&buf[..n]);
    
    // Проверяем JSON ответ
    let json: serde_json::Value = serde_json::from_str(&response).expect("parse json");
    assert_eq!(json["status"], "running");
    assert!(json["uptime_secs"].is_number());

    // Тест 2: REST API
    let resp = reqwest::get("http://127.0.0.1:40124/api/server/status")
        .await
        .expect("rest request");
    assert!(resp.status().is_success());
    let rest_json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(rest_json["status"], "running");

    // Завершаем сервер через Named Pipe
    client.write_all(b"shutdown").await.expect("shutdown write");
    let mut buf = vec![0u8; 64];
    let _ = client.read(&mut buf).await;

    handle.await.unwrap();
}

#[cfg(not(windows))]
#[tokio::test]
async fn test_unix_socket_and_rest_api() {
    // Заглушка для не-Windows платформ
    println!("Windows named pipe test skipped on non-Windows platform");
} 