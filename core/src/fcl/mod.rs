//! Legacy constructor backed by the independent compatibility adapter.
use crate::{
    game_integration::{
        event_system::{EventData, EventSource, EventSystem, GameEvent},
        native_executor::NativeValue,
    },
    natives::NativeManager,
};
pub use adapter::{EventBus, EventHandler, NativeHost};
use gameverse_compat_fivem as adapter;
use std::sync::Arc;
use tokio::sync::RwLock;

struct LegacyNatives(Arc<RwLock<NativeManager>>);
#[adapter::async_trait]
impl NativeHost for LegacyNatives {
    async fn call(
        &self,
        name: &str,
        args: Vec<adapter::NativeValue>,
    ) -> anyhow::Result<adapter::NativeValue> {
        let value = self
            .0
            .read()
            .await
            .call_native(name, args.into_iter().map(to_legacy).collect())
            .await?;
        Ok(from_legacy(value))
    }
    async fn supports(&self, name: &str) -> bool {
        self.0.read().await.get_function_info(name).is_some()
    }
}

struct LegacyEvents(Arc<EventSystem>);
#[adapter::async_trait]
impl EventBus for LegacyEvents {
    async fn emit(&self, name: &str, args: Vec<adapter::NativeValue>) -> anyhow::Result<()> {
        self.0
            .emit(GameEvent {
                event_type: name.into(),
                source: EventSource::Server,
                data: EventData::Custom {
                    data: adapter::legacy_event_payload(&args),
                },
                timestamp: chrono::Utc::now().timestamp_millis() as u64,
                id: uuid::Uuid::new_v4().to_string(),
            })
            .await
    }
    async fn register(&self, name: &str, handler: EventHandler) -> anyhow::Result<()> {
        self.0
            .on(name.into(), move |event| {
                if let EventData::Custom { data } = event.data {
                    if let Some(args) = adapter::legacy_event_args(&data)? {
                        handler(args)?;
                    }
                }
                Ok(())
            })
            .await
    }
}

/// Old core API; game-independent consumers use the adapter crate directly.
#[derive(Clone)]
pub struct FiveMCompat(adapter::FiveMCompat);
impl FiveMCompat {
    pub fn new(natives: Arc<RwLock<NativeManager>>, events: Arc<EventSystem>) -> Self {
        Self(adapter::FiveMCompat::new(
            Arc::new(LegacyNatives(natives)),
            Arc::new(LegacyEvents(events)),
        ))
    }
    pub async fn get_player_ped(&self, player: i32) -> anyhow::Result<i32> {
        self.0.get_player_ped(player).await
    }
    pub async fn trigger_server_event(
        &self,
        name: &str,
        args: Vec<NativeValue>,
    ) -> anyhow::Result<()> {
        self.0
            .trigger_server_event(name, args.into_iter().map(from_legacy).collect())
            .await
    }
    pub async fn register_net_event<F>(&self, name: &str, handler: F) -> anyhow::Result<()>
    where
        F: Fn(Vec<NativeValue>) + Send + Sync + 'static,
    {
        self.0
            .register_net_event(name, move |args| {
                handler(args.into_iter().map(to_legacy).collect())
            })
            .await
    }
}

pub async fn verify_fcl_ready(natives: &NativeManager) -> bool {
    [
        "GET_PLAYER_PED",
        "GET_PLAYER_NAME",
        "GET_ENTITY_COORDS",
        "SET_ENTITY_COORDS",
        "SET_PED_COMPONENT_VARIATION",
        "GET_VEHICLE_ENGINE_HEALTH",
        "SET_VEHICLE_ENGINE_HEALTH",
    ]
    .iter()
    .all(|name| natives.get_function_info(name).is_some())
}

fn from_legacy(value: NativeValue) -> adapter::NativeValue {
    match value {
        NativeValue::Bool(v) => adapter::NativeValue::Bool(v),
        NativeValue::Int(v) => adapter::NativeValue::Int(v),
        NativeValue::Float(v) => adapter::NativeValue::Float(v),
        NativeValue::String(v) => adapter::NativeValue::String(v),
        NativeValue::Vector3 { x, y, z } => adapter::NativeValue::Vector3 { x, y, z },
        NativeValue::Entity(v) => adapter::NativeValue::Entity(v),
        NativeValue::Pointer(v) => adapter::NativeValue::Pointer(v),
        NativeValue::Null => adapter::NativeValue::Null,
    }
}
fn to_legacy(value: adapter::NativeValue) -> NativeValue {
    match value {
        adapter::NativeValue::Bool(v) => NativeValue::Bool(v),
        adapter::NativeValue::Int(v) => NativeValue::Int(v),
        adapter::NativeValue::Float(v) => NativeValue::Float(v),
        adapter::NativeValue::String(v) => NativeValue::String(v),
        adapter::NativeValue::Vector3 { x, y, z } => NativeValue::Vector3 { x, y, z },
        adapter::NativeValue::Entity(v) => NativeValue::Entity(v),
        adapter::NativeValue::Pointer(v) => NativeValue::Pointer(v),
        adapter::NativeValue::Null => NativeValue::Null,
    }
}
