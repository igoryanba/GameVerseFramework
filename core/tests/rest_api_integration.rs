//! Интеграционный тест REST API ресурсов
use std::process::{Command, Child};
use std::thread::sleep;
use std::time::Duration;
use reqwest::Client;
use serde_json::Value;

fn start_test_server() -> Child {
    Command::new("target/debug/gameverse_server")
        .arg("--test-mode")
        .spawn()
        .expect("failed to start server")
}

#[tokio::test]
async fn test_rest_api_resources() {
    let mut server = start_test_server();
    sleep(Duration::from_secs(2));
    let client = Client::new();
    // GET /api/resources
    let resp = client.get("http://localhost:30121/api/resources")
        .send().await.expect("GET /api/resources failed");
    assert!(resp.status().is_success());
    let json: Value = resp.json().await.expect("invalid json");
    assert!(json["resources"].is_array());
    // POST /api/resources/test_resource/reload
    let resp = client.post("http://localhost:30121/api/resources/test_resource/reload")
        .send().await.expect("POST /api/resources/:name/reload failed");
    assert!(resp.status().is_success());
    let json: Value = resp.json().await.expect("invalid json");
    assert_eq!(json["result"], "ok");
    // Останавливаем сервер
    let _ = server.kill();
} 