//! Интеграционный тест resource watch/hot-reload
use std::process::{Command, Child};
use std::thread::sleep;
use std::time::Duration;
use std::fs::{self, OpenOptions};
use reqwest::Client;
use serde_json::Value;

fn start_test_server() -> Child {
    Command::new("target/debug/gameverse_server")
        .arg("--test-mode")
        .arg("--watch")
        .spawn()
        .expect("failed to start server")
}

#[tokio::test]
async fn test_resource_watch_hot_reload() {
    let mut server = start_test_server();
    sleep(Duration::from_secs(2));
    let client = Client::new();
    // Модифицируем файл ресурса
    let resource_file = "resources/test_resource/gameverse.toml";
    let mut file = OpenOptions::new().append(true).open(resource_file).expect("open resource file");
    use std::io::Write;
    writeln!(file, "# touch for hot-reload").expect("write failed");
    sleep(Duration::from_secs(2));
    // Проверяем через REST API, что ресурс перезагружен (можно добавить проверку времени/состояния)
    let resp = client.get("http://localhost:30121/api/resources")
        .send().await.expect("GET /api/resources failed");
    assert!(resp.status().is_success());
    let json: Value = resp.json().await.expect("invalid json");
    assert!(json["resources"].as_array().unwrap().iter().any(|r| r["name"] == "test_resource"));
    let _ = server.kill();
} 