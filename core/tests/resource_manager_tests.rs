use gameverse_core::resource::{ResourceManager, ResourceState};
use tempfile::TempDir;
use std::path::PathBuf;
use tokio::runtime::Runtime;

fn create_test_manifest(dir: &PathBuf, name: &str) -> anyhow::Result<()> {
    let manifest_content = format!(
        r#"[resource]
name = "{name}"
version = "0.1.0"
author = "Tester"
description = "Test resource"
"#
    );
    let manifest_path = dir.join("gameverse.toml");
    let mut file = std::fs::File::create(&manifest_path)?;
    use std::io::Write as _;
    file.write_all(manifest_content.as_bytes())?;
    Ok(())
}

#[test]
fn test_resource_lifecycle() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let resources_dir = temp_dir.path().to_path_buf();
        let res_name = "test_resource";
        let res_dir = resources_dir.join(res_name);
        tokio::fs::create_dir_all(&res_dir).await.expect("Failed to create resource dir");
        create_test_manifest(&res_dir, res_name).expect("Failed to create manifest");
        let mut manager = ResourceManager::new(resources_dir.clone());
        manager.initialize().await.expect("Failed to initialize");
        assert_eq!(manager.list_resources().len(), 1);
        // Старт ресурса
        manager.start_resource(res_name).await.expect("start_resource failed");
        let res = manager.get_resource(res_name).unwrap();
        assert_eq!(res.state, ResourceState::Started);
        // Остановка ресурса
        manager.stop_resource(res_name).await.expect("stop_resource failed");
        let res = manager.get_resource(res_name).unwrap();
        assert_eq!(res.state, ResourceState::Stopped);
        // Перезапуск ресурса
        manager.start_resource(res_name).await.expect("start_resource failed");
        let res = manager.get_resource(res_name).unwrap();
        assert_eq!(res.state, ResourceState::Started);
        // Перезагрузка ресурса
        manager.reload_resource(res_name).await.expect("reload_resource failed");
        let res = manager.get_resource(res_name).unwrap();
        assert_eq!(res.state, ResourceState::Started);
    });
}

#[test]
fn test_resource_errors() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let resources_dir = temp_dir.path().to_path_buf();
        let mut manager = ResourceManager::new(resources_dir.clone());
        manager.initialize().await.expect("Failed to initialize");
        // Попытка стартовать несуществующий ресурс
        let err = manager.start_resource("not_found").await;
        assert!(err.is_err());
        // Попытка остановить несуществующий ресурс
        let err = manager.stop_resource("not_found").await;
        assert!(err.is_err());
        // Попытка перезагрузить несуществующий ресурс
        let err = manager.reload_resource("not_found").await;
        assert!(err.is_err());
    });
}

#[tokio::test]
async fn test_resource_hot_reload() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let resources_dir = temp_dir.path().to_path_buf();
    let res_name = "hot_reload_resource";
    let res_dir = resources_dir.join(res_name);
    tokio::fs::create_dir_all(&res_dir).await.expect("Failed to create resource dir");
    create_test_manifest(&res_dir, res_name).expect("Failed to create manifest");
    let mut manager = ResourceManager::new(resources_dir.clone());
    manager.initialize().await.expect("Failed to initialize");
    manager.start_resource(res_name).await.expect("start_resource failed");
    // Модифицируем манифест (эмулируем hot-reload)
    std::thread::sleep(std::time::Duration::from_millis(100));
    create_test_manifest(&res_dir, res_name).expect("Failed to update manifest");
    manager.reload_resource(res_name).await.expect("reload_resource failed");
    let res = manager.get_resource(res_name).unwrap();
    assert_eq!(res.state, ResourceState::Started);
}

#[test]
fn test_resource_manager_cross_platform_paths() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let resources_dir = temp_dir.path().to_path_buf();
    let res_name = if cfg!(windows) { "ресурс_win" } else { "ресурс_unix" };
    let res_dir = resources_dir.join(res_name);
    std::fs::create_dir_all(&res_dir).expect("Failed to create resource dir");
    create_test_manifest(&res_dir, res_name).expect("Failed to create manifest");
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut manager = ResourceManager::new(resources_dir.clone());
        manager.initialize().await.expect("Failed to initialize");
        assert!(manager.get_resource(res_name).is_some(), "Resource must be found on all platforms");
    });
} 