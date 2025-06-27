use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json;
use uuid::Uuid;

use crate::natives::NativeManager;
use crate::game_integration::native_executor::NativeValue;
use crate::game_integration::event_system::{EventSystem, GameEvent, EventSource, EventData};

/// Baseline list of top-used native hashes/names for MVP.
const TOP_NATIVE_NAMES: &[&str] = &[
    "GET_PLAYER_PED",
    "GET_PLAYER_NAME",
    "GET_ENTITY_COORDS",
    "SET_ENTITY_COORDS",
    "SET_PED_COMPONENT_VARIATION",
    "GET_VEHICLE_ENGINE_HEALTH",
    "SET_VEHICLE_ENGINE_HEALTH",
];

/// FiveM-style compatibility façade.  
/// Provides thin async wrappers, сохраняя знакомые сигнатуры.
#[derive(Clone)]
pub struct FiveMCompat {
    native_manager: Arc<RwLock<NativeManager>>, // shared менеджер нативов
    event_system: Arc<EventSystem>,            // общая шина событий
}

impl FiveMCompat {
    /// Создать новый мост совместимости.
    pub fn new(native_manager: Arc<RwLock<NativeManager>>, event_system: Arc<EventSystem>) -> Self {
        Self { native_manager, event_system }
    }

    /// GET_PLAYER_PED(player_id) → entity handle
    pub async fn get_player_ped(&self, player_id: i32) -> anyhow::Result<i32> {
        let value = self
            .native_manager
            .read()
            .await
            .call_native("GET_PLAYER_PED", vec![NativeValue::Int(player_id)])
            .await?;
        Ok(value.as_int().unwrap_or_default())
    }

    /// TRIGGER_SERVER_EVENT analogue — пока заглушка (логирует вызов).
    pub async fn trigger_server_event(&self, event: &str, args: Vec<NativeValue>) -> anyhow::Result<()> {
        let evt = GameEvent {
            event_type: event.to_string(),
            source: EventSource::Server,
            data: EventData::Custom { data: serde_json::json!({ "args": args_as_json(&args) }) },
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            id: Uuid::new_v4().to_string(),
        };
        self.event_system.emit(evt).await?;
        Ok(())
    }

    /// REGISTER_NET_EVENT analogue — пока регистрирует хэндлер во внутреннем списке.
    pub async fn register_net_event<F>(&self, event: &str, handler: F) -> anyhow::Result<()>
    where
        F: Fn(Vec<NativeValue>) + Send + Sync + 'static,
    {
        let es = self.event_system.clone();
        es.on(event.to_string(), move |evt| {
            if let EventData::Custom { data } = &evt.data {
                if let Some(args_v) = data.get("args") {
                    if let Some(arr) = args_v.as_array() {
                        let converted = arr.iter().map(json_to_native_value).collect::<Vec<_>>();
                        handler(converted);
                    }
                }
            }
            Ok(())
        }).await?;
        tracing::info!(target = "fcl", "RegisterNetEvent: {} (via EventSystem)", event);
        Ok(())
    }
}

/// Helper: convert NativeValue vector → JSON for transport
fn args_as_json(args: &[NativeValue]) -> serde_json::Value {
    serde_json::to_value(args.iter().map(native_value_to_json).collect::<Vec<_>>()).unwrap_or_default()
}

fn native_value_to_json(v: &NativeValue) -> serde_json::Value {
    match v {
        NativeValue::Bool(b) => serde_json::json!(*b),
        NativeValue::Int(i) => serde_json::json!(*i),
        NativeValue::Float(f) => serde_json::json!(*f),
        NativeValue::String(s) => serde_json::json!(s),
        NativeValue::Vector3 { x, y, z } => serde_json::json!([x, y, z]),
        NativeValue::Entity(e) => serde_json::json!(*e),
        NativeValue::Pointer(p) => serde_json::json!(*p),
        NativeValue::Null => serde_json::Value::Null,
    }
}

fn json_to_native_value(val: &serde_json::Value) -> NativeValue {
    match val {
        serde_json::Value::Bool(b) => NativeValue::Bool(*b),
        serde_json::Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                NativeValue::Int(i as i32)
            } else if let Some(f) = num.as_f64() {
                NativeValue::Float(f as f32)
            } else {
                NativeValue::Null
            }
        }
        serde_json::Value::String(s) => NativeValue::String(s.clone()),
        serde_json::Value::Array(arr) => {
            if arr.len() == 3 {
                if let (Some(x), Some(y), Some(z)) = (arr[0].as_f64(), arr[1].as_f64(), arr[2].as_f64()) {
                    NativeValue::Vector3 { x: x as f32, y: y as f32, z: z as f32 }
                } else {
                    NativeValue::Null
                }
            } else {
                NativeValue::Null
            }
        }
        _ => NativeValue::Null,
    }
}

/// Проверка готовности FCL: убеждаемся, что ключевые нативы зарегистрированы.
pub async fn verify_fcl_ready(native_manager: &NativeManager) -> bool {
    TOP_NATIVE_NAMES
        .iter()
        .all(|name| native_manager.get_function_info(name).is_some())
} 