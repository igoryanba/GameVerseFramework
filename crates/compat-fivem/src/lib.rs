//! Optional local compatibility adapter. It is not a multiplayer wire protocol.
use anyhow::Result;
pub use async_trait::async_trait;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

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
pub type CallbackHandler = Arc<dyn Fn(Vec<NativeValue>) -> Result<Vec<NativeValue>> + Send + Sync>;
pub type ExportHandler = Arc<dyn Fn(Vec<NativeValue>) -> Result<NativeValue> + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceState {
    Stopped,
    Starting,
    Started,
    Stopping,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ResourceManifest {
    pub name: String,
    pub client_scripts: Vec<String>,
    pub server_scripts: Vec<String>,
    pub shared_scripts: Vec<String>,
    pub dependencies: Vec<String>,
    pub ui_page: Option<String>,
    pub files: Vec<String>,
}
pub fn to_gameverse_toml(manifest: &ResourceManifest) -> Result<String> {
    anyhow::ensure!(!manifest.name.is_empty(), "resource name is empty");
    Ok(toml::to_string_pretty(manifest)?)
}

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
    async fn emit_to(&self, _client: u64, name: &str, args: Vec<NativeValue>) -> Result<()> {
        self.emit(name, args).await
    }
    async fn register(&self, name: &str, handler: EventHandler) -> Result<()>;
}

#[derive(Clone)]
pub struct FiveMCompat {
    natives: Arc<dyn NativeHost>,
    events: Arc<dyn EventBus>,
    callbacks: Arc<Mutex<HashMap<String, CallbackHandler>>>,
    exports: Arc<Mutex<HashMap<(String, String), ExportHandler>>>,
    resources: Arc<Mutex<HashMap<String, ResourceState>>>,
}
impl FiveMCompat {
    pub fn new(natives: Arc<dyn NativeHost>, events: Arc<dyn EventBus>) -> Self {
        Self {
            natives,
            events,
            callbacks: Default::default(),
            exports: Default::default(),
            resources: Default::default(),
        }
    }
    pub async fn call_native(&self, name: &str, args: Vec<NativeValue>) -> Result<NativeValue> {
        anyhow::ensure!(
            self.natives.supports(name).await,
            "unsupported native: {name}"
        );
        self.natives.call(name, args).await
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
    pub async fn trigger_client_event(
        &self,
        client: u64,
        name: &str,
        args: Vec<NativeValue>,
    ) -> Result<()> {
        anyhow::ensure!(client > 0, "invalid client ID");
        self.events.emit_to(client, name, args).await
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
    pub fn register_callback<F>(&self, name: &str, handler: F) -> Result<()>
    where
        F: Fn(Vec<NativeValue>) -> Result<Vec<NativeValue>> + Send + Sync + 'static,
    {
        anyhow::ensure!(!name.is_empty(), "callback name is empty");
        let mut callbacks = self.callbacks.lock().unwrap();
        match callbacks.entry(name.into()) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(Arc::new(handler));
                Ok(())
            }
            std::collections::hash_map::Entry::Occupied(_) => {
                anyhow::bail!("callback already registered: {name}")
            }
        }
    }
    pub fn trigger_callback(&self, name: &str, args: Vec<NativeValue>) -> Result<Vec<NativeValue>> {
        let handler = self
            .callbacks
            .lock()
            .unwrap()
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("unknown callback: {name}"))?;
        handler(args)
    }
    pub fn register_export<F>(&self, resource: &str, name: &str, handler: F) -> Result<()>
    where
        F: Fn(Vec<NativeValue>) -> Result<NativeValue> + Send + Sync + 'static,
    {
        anyhow::ensure!(
            !resource.is_empty() && !name.is_empty(),
            "resource and export names are required"
        );
        let mut exports = self.exports.lock().unwrap();
        match exports.entry((resource.into(), name.into())) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(Arc::new(handler));
                Ok(())
            }
            std::collections::hash_map::Entry::Occupied(_) => {
                anyhow::bail!("export already registered: {resource}:{name}")
            }
        }
    }
    pub fn call_export(
        &self,
        resource: &str,
        name: &str,
        args: Vec<NativeValue>,
    ) -> Result<NativeValue> {
        let key = (resource.into(), name.into());
        let handler = self
            .exports
            .lock()
            .unwrap()
            .get(&key)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("unknown export: {resource}:{name}"))?;
        handler(args)
    }
    pub fn transition_resource(&self, name: &str, next: ResourceState) -> Result<()> {
        anyhow::ensure!(!name.is_empty(), "resource name is empty");
        let mut states = self.resources.lock().unwrap();
        let current = states.get(name).copied().unwrap_or(ResourceState::Stopped);
        let valid = matches!(
            (current, next),
            (ResourceState::Stopped, ResourceState::Starting)
                | (ResourceState::Starting, ResourceState::Started)
                | (ResourceState::Started, ResourceState::Stopping)
                | (ResourceState::Stopping, ResourceState::Stopped)
        );
        anyhow::ensure!(
            valid,
            "invalid resource transition: {current:?} -> {next:?}"
        );
        states.insert(name.into(), next);
        Ok(())
    }
    pub fn resource_state(&self, name: &str) -> ResourceState {
        self.resources
            .lock()
            .unwrap()
            .get(name)
            .copied()
            .unwrap_or(ResourceState::Stopped)
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
        fcl.register_callback("sum", |args| Ok(vec![NativeValue::Int(args.len() as i32)]))
            .unwrap();
        assert_eq!(
            fcl.trigger_callback("sum", vec![NativeValue::Null])
                .unwrap(),
            vec![NativeValue::Int(1)]
        );
        fcl.register_export("bank", "balance", |_| Ok(NativeValue::Int(50)))
            .unwrap();
        assert_eq!(
            fcl.call_export("bank", "balance", vec![]).unwrap(),
            NativeValue::Int(50)
        );
        fcl.transition_resource("bank", ResourceState::Starting)
            .unwrap();
        fcl.transition_resource("bank", ResourceState::Started)
            .unwrap();
        assert_eq!(fcl.resource_state("bank"), ResourceState::Started);
        assert!(fcl.call_native("UNSUPPORTED", vec![]).await.is_err());
        let manifest = ResourceManifest {
            name: "bank".into(),
            client_scripts: vec!["client.lua".into()],
            server_scripts: vec![],
            shared_scripts: vec![],
            dependencies: vec![],
            ui_page: None,
            files: vec![],
        };
        let converted = to_gameverse_toml(&manifest).unwrap();
        assert!(converted.contains("name = \"bank\""));
    }
}
