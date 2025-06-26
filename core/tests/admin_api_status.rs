#[cfg(unix)]
#[tokio::test]
async fn test_admin_api_status() {
    use gameverse_core::server::runtime::ServerRuntime;
    use std::path::Path;
    use tempfile::TempDir;

    // Подготовим временную конфигурацию
    const BASIC_CONFIG: &str = r#"[server]
name = "TestServer"
port = 30120
bind_address = "127.0.0.1"
resources_path = "resources"

[network]
admin_port = 40123
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
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Запрос к REST API
    let resp = reqwest::get("http://127.0.0.1:40123/api/server/status")
        .await
        .expect("request");
    assert!(resp.status().is_success());
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["status"], "running");

    // Завершаем сервер
    let _ = reqwest::Client::new()
        .post("http://127.0.0.1:40123/api/server/shutdown")
        .send()
        .await;

    handle.await.unwrap();
} 