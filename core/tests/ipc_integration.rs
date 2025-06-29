//! Интеграционный тест IPC: сервер <-> CLI
use std::process::{Command, Child};
use std::thread::sleep;
use std::time::Duration;
use std::fs;

#[cfg(unix)]
fn ipc_socket_path() -> &'static str {
    "/tmp/gameverse_server.sock"
}

#[cfg(windows)]
fn ipc_socket_path() -> &'static str {
    r"\\.\pipe\gameverse_server"
}

fn start_test_server() -> Child {
    Command::new("target/debug/gameverse_server")
        .arg("--test-mode")
        .spawn()
        .expect("failed to start server")
}

#[test]
fn test_ipc_resource_commands() {
    let mut server = start_test_server();
    sleep(Duration::from_secs(2)); // Ждём старта сервера
    // resource list
    let output = Command::new("target/debug/gameverse")
        .args(&["resource", "list"])
        .output()
        .expect("failed to run cli");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("resources") || stdout.contains("test_resource"));
    // resource start
    let output = Command::new("target/debug/gameverse")
        .args(&["resource", "start", "test_resource"])
        .output()
        .expect("failed to run cli");
    assert!(output.status.success());
    // resource stop
    let output = Command::new("target/debug/gameverse")
        .args(&["resource", "stop", "test_resource"])
        .output()
        .expect("failed to run cli");
    assert!(output.status.success());
    // resource reload
    let output = Command::new("target/debug/gameverse")
        .args(&["resource", "reload", "test_resource"])
        .output()
        .expect("failed to run cli");
    assert!(output.status.success());
    // Останавливаем сервер
    let _ = server.kill();
} 