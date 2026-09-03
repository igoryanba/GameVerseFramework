#![cfg(feature = "fivem-compat")]
//! Legacy bridge integration. Mock-native contract tests live in the adapter crate.
use gameverse_core::{
    fcl::FiveMCompat,
    game_integration::{event_system::EventSystem, native_executor::NativeValue, GameType},
    natives::NativeManager,
};
use std::sync::{Arc, Mutex};
use tokio::sync::{oneshot, RwLock};

#[tokio::test]
async fn initialized_event_dispatch_preserves_values() -> anyhow::Result<()> {
    let events = Arc::new(EventSystem::new());
    events.initialize().await?;
    let natives = Arc::new(RwLock::new(NativeManager::new(GameType::GtaV)));
    let fcl = FiveMCompat::new(natives, events.clone());
    let (tx, rx) = oneshot::channel();
    let sender = Mutex::new(Some(tx));
    fcl.register_net_event("test", move |args| {
        if let Some(tx) = sender.lock().unwrap().take() {
            let _ = tx.send(args);
        }
    })
    .await?;
    fcl.trigger_server_event("test", vec![NativeValue::Int(42), NativeValue::Entity(9)])
        .await?;
    let received = tokio::time::timeout(std::time::Duration::from_secs(2), rx).await??;
    assert!(matches!(
        received.as_slice(),
        [NativeValue::Int(42), NativeValue::Entity(9)]
    ));
    events.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn unavailable_natives_return_errors() {
    let natives = Arc::new(RwLock::new(NativeManager::new(GameType::GtaV)));
    let fcl = FiveMCompat::new(natives.clone(), Arc::new(EventSystem::new()));
    assert!(fcl.get_player_ped(1).await.is_err());
    assert!(fcl.get_player_ped(-1).await.is_err());
    assert!(!gameverse_core::fcl::verify_fcl_ready(&*natives.read().await).await);
}
