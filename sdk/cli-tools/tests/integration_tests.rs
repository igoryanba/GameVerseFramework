use std::process::Command;
use tempfile::tempdir;
use assert_cmd::prelude::*;

#[test]
fn test_server_init_creates_files() {
    let temp_dir = tempdir().unwrap();
    let server_path = temp_dir.path().join("test_server");
    
    let mut cmd = Command::cargo_bin("gameverse").unwrap();
    cmd.arg("server")
       .arg("init")
       .arg(&server_path);
    
    cmd.assert().success();
    
    // Проверяем созданные файлы
    assert!(server_path.join("config/server-config.toml").exists());
    assert!(server_path.join("docker-compose.yml").exists());
    assert!(server_path.join("systemd/gameverse.service").exists());
    assert!(server_path.join("install_nssm.ps1").exists());
    assert!(server_path.join("resources").is_dir());
}

#[test]
fn test_server_init_config_content() {
    let temp_dir = tempdir().unwrap();
    let server_path = temp_dir.path().join("test_server");
    
    let mut cmd = Command::cargo_bin("gameverse").unwrap();
    cmd.arg("server")
       .arg("init")
       .arg(&server_path);
    
    cmd.assert().success();
    
    let config_content = std::fs::read_to_string(
        server_path.join("config/server-config.toml")
    ).unwrap();
    
    // Проверяем ключевые поля конфига
    assert!(config_content.contains("[server]"));
    assert!(config_content.contains("name = \"MyGameVerseServer\""));
    assert!(config_content.contains("admin_port = 30121"));
    assert!(config_content.contains("jwt_secret = \"CHANGE_ME\""));
    assert!(config_content.contains("resources_dir = \"./resources\""));
}

#[test]
fn test_server_init_systemd_service() {
    let temp_dir = tempdir().unwrap();
    let server_path = temp_dir.path().join("test_server");
    
    let mut cmd = Command::cargo_bin("gameverse").unwrap();
    cmd.arg("server")
       .arg("init")
       .arg(&server_path);
    
    cmd.assert().success();
    
    let service_content = std::fs::read_to_string(
        server_path.join("systemd/gameverse.service")
    ).unwrap();
    
    // Проверяем systemd unit структуру
    assert!(service_content.contains("[Unit]"));
    assert!(service_content.contains("Description=GameVerse Server"));
    assert!(service_content.contains("[Service]"));
    assert!(service_content.contains("ExecStart="));
    assert!(service_content.contains("[Install]"));
    assert!(service_content.contains("WantedBy=multi-user.target"));
} 