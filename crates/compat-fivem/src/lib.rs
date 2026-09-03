//! Optional local compatibility adapter. It is not a multiplayer wire protocol.
use anyhow::Result;
pub use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum NativeValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Vector3 { x: f32, y: f32, z: f32 },
    Entity(u32),
    Pointer(usize),
    Null,
}

pub type EventHandler = Arc<dyn Fn(Vec<NativeValue>) -> Result<()> + Send + Sync>;

/// Preserve the legacy untagged args field for existing EventSystem consumers.
/// Additional typed metadata preserves entity/pointer variants between new adapters.
pub fn legacy_event_payload(args: &[NativeValue]) -> serde_json::Value {
    let plain: Vec<_> = args
        .iter()
        .map(|value| match value {
            NativeValue::Bool(v) => serde_json::json!(v),
            NativeValue::Int(v) => serde_json::json!(v),
            NativeValue::Float(v) => serde_json::json!(v),
            NativeValue::String(v) => serde_json::json!(v),
            NativeValue::Vector3 { x, y, z } => serde_json::json!([x, y, z]),
            NativeValue::Entity(v) => serde_json::json!(v),
            NativeValue::Pointer(v) => serde_json::json!(v),
            NativeValue::Null => serde_json::Value::Null,
        })
        .collect();
    serde_json::json!({"args":plain,"_gameverse_typed_args":args})
}

/// Accept both old producers and the typed bridge payload; this is local data only.
pub fn legacy_event_args(data: &serde_json::Value) -> Result<Option<Vec<NativeValue>>> {
    if let Some(typed) = data.get("_gameverse_typed_args") {
        return Ok(Some(serde_json::from_value(typed.clone())?));
    }
    Ok(data.get("args").and_then(|v| v.as_array()).map(|args| {
        args.iter()
            .map(|v| match v {
                serde_json::Value::Bool(v) => NativeValue::Bool(*v),
                serde_json::Value::Number(v) => {
                    if let Some(i) = v.as_i64() {
                        NativeValue::Int(i as i32)
                    } else {
                        NativeValue::Float(v.as_f64().unwrap_or_default() as f32)
                    }
                }
                serde_json::Value::String(v) => NativeValue::String(v.clone()),
                serde_json::Value::Array(a) if a.len() == 3 => {
                    match (a[0].as_f64(), a[1].as_f64(), a[2].as_f64()) {
                        (Some(x), Some(y), Some(z)) => NativeValue::Vector3 {
                            x: x as f32,
                            y: y as f32,
                            z: z as f32,
                        },
                        _ => NativeValue::Null,
                    }
                }
                _ => NativeValue::Null,
            })
            .collect()
    }))
}

#[async_trait]
pub trait NativeHost: Send + Sync {
    async fn call(&self, name: &str, args: Vec<NativeValue>) -> Result<NativeValue>;
    async fn supports(&self, name: &str) -> bool;
}
#[async_trait]
pub trait EventBus: Send + Sync {
    async fn emit(&self, name: &str, args: Vec<NativeValue>) -> Result<()>;
    async fn register(&self, name: &str, handler: EventHandler) -> Result<()>;
}

#[derive(Clone)]
pub struct FiveMCompat {
    natives: Arc<dyn NativeHost>,
    events: Arc<dyn EventBus>,
}
impl FiveMCompat {
    pub fn new(natives: Arc<dyn NativeHost>, events: Arc<dyn EventBus>) -> Self {
        Self { natives, events }
    }
    pub async fn get_player_ped(&self, player: i32) -> Result<i32> {
        anyhow::ensure!(player >= 0, "invalid player ID");
        match self
            .natives
            .call("GET_PLAYER_PED", vec![NativeValue::Int(player)])
            .await?
        {
            NativeValue::Int(id) if id > 0 => Ok(id),
            NativeValue::Entity(id) if id > 0 && id <= i32::MAX as u32 => Ok(id as i32),
            _ => anyhow::bail!("GET_PLAYER_PED returned an invalid entity handle"),
        }
    }
    pub async fn trigger_server_event(&self, name: &str, args: Vec<NativeValue>) -> Result<()> {
        self.events.emit(name, args).await
    }
    pub async fn register_net_event<F>(&self, name: &str, handler: F) -> Result<()>
    where
        F: Fn(Vec<NativeValue>) + Send + Sync + 'static,
    {
        self.events
            .register(
                name,
                Arc::new(move |args| {
                    handler(args);
                    Ok(())
                }),
            )
            .await
    }
    pub async fn ready(&self) -> bool {
        for name in [
            "GET_PLAYER_PED",
            "GET_PLAYER_NAME",
            "GET_ENTITY_COORDS",
            "SET_ENTITY_COORDS",
            "SET_PED_COMPONENT_VARIATION",
            "GET_VEHICLE_ENGINE_HEALTH",
            "SET_VEHICLE_ENGINE_HEALTH",
        ] {
            if !self.natives.supports(name).await {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, sync::Mutex};
    #[test]
    fn legacy_payload_remains_compatible_with_old_producers_and_consumers() {
        let args = vec![NativeValue::Int(42), NativeValue::Entity(9)];
        let payload = legacy_event_payload(&args);
        assert_eq!(payload["args"], serde_json::json!([42, 9]));
        assert_eq!(legacy_event_args(&payload).unwrap(), Some(args));
        assert_eq!(
            legacy_event_args(&serde_json::json!({"args":[42,9]})).unwrap(),
            Some(vec![NativeValue::Int(42), NativeValue::Int(9)])
        );
    }
    struct TestNatives;
    #[async_trait]
    impl NativeHost for TestNatives {
        async fn call(&self, name: &str, _args: Vec<NativeValue>) -> Result<NativeValue> {
            anyhow::ensure!(name == "GET_PLAYER_PED", "unsupported native: {name}");
            Ok(NativeValue::Entity(7))
        }
        async fn supports(&self, name: &str) -> bool {
            name == "GET_PLAYER_PED"
        }
    }
    #[derive(Default)]
    struct TestEvents(Mutex<HashMap<String, Vec<EventHandler>>>);
    #[async_trait]
    impl EventBus for TestEvents {
        async fn emit(&self, name: &str, args: Vec<NativeValue>) -> Result<()> {
            let handlers = self
                .0
                .lock()
                .unwrap()
                .get(name)
                .cloned()
                .unwrap_or_default();
            for handler in handlers {
                handler(args.clone())?;
            }
            Ok(())
        }
        async fn register(&self, name: &str, handler: EventHandler) -> Result<()> {
            self.0
                .lock()
                .unwrap()
                .entry(name.into())
                .or_default()
                .push(handler);
            Ok(())
        }
    }
    #[tokio::test]
    async fn explicit_backend_and_synchronous_event_observation() {
        let fcl = FiveMCompat::new(Arc::new(TestNatives), Arc::new(TestEvents::default()));
        assert_eq!(fcl.get_player_ped(1).await.unwrap(), 7);
        assert!(fcl.get_player_ped(-1).await.is_err());
        assert!(!fcl.ready().await);
        assert!(TestNatives.call("UNSUPPORTED", vec![]).await.is_err());
        let received = Arc::new(Mutex::new(vec![]));
        let target = received.clone();
        fcl.register_net_event("example", move |args| *target.lock().unwrap() = args)
            .await
            .unwrap();
        let args = vec![NativeValue::Int(42), NativeValue::String("payload".into())];
        fcl.trigger_server_event("example", args.clone())
            .await
            .unwrap();
        assert_eq!(*received.lock().unwrap(), args);
    }
}
