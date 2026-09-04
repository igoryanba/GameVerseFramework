//! Sandboxed Lua 5.4 host for optional FiveM-compatible resources.
use anyhow::{Context, Result};
use gameverse_resource_manifest::{resolve_and_validate, ResourceManifest};
use mlua::{HookTriggers, Lua, LuaOptions, LuaSerdeExt, MultiValue, StdLib, Value, VmState};
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    fs,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Instant,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HostSide {
    Client,
    Server,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleState {
    Stopped,
    Starting,
    Started,
    Stopping,
}

#[derive(Clone, Debug)]
pub struct Limits {
    pub memory_bytes: usize,
    pub instructions_per_dispatch: usize,
    pub event_payload_bytes: usize,
    pub handlers: usize,
    pub callbacks: usize,
    pub queue: usize,
    pub callback_timeout_ms: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            memory_bytes: 16 * 1024 * 1024,
            instructions_per_dispatch: 100_000,
            event_payload_bytes: 64 * 1024,
            handlers: 128,
            callbacks: 128,
            queue: 256,
            callback_timeout_ms: 5_000,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceEvent {
    pub resource: String,
    pub name: String,
    pub source: Option<u64>,
    pub target: Option<u64>,
    pub arguments: Vec<serde_json::Value>,
    pub correlation_id: Option<u64>,
}
impl ResourceEvent {
    pub fn validate(&self, limits: &Limits) -> Result<()> {
        anyhow::ensure!(
            !self.resource.is_empty() && !self.name.is_empty(),
            "resource and event name are required"
        );
        anyhow::ensure!(
            serde_json::to_vec(self)?.len() <= limits.event_payload_bytes,
            "event payload exceeds {} bytes",
            limits.event_payload_bytes
        );
        Ok(())
    }
}

pub struct ResourceHost {
    lua: Lua,
    root: PathBuf,
    manifest: ResourceManifest,
    side: HostSide,
    limits: Limits,
    state: LifecycleState,
    outbound: Arc<Mutex<VecDeque<ResourceEvent>>>,
    instruction_count: Arc<AtomicUsize>,
    dispatch_started: Arc<Mutex<Instant>>,
}

impl ResourceHost {
    pub fn new(
        root: impl AsRef<Path>,
        manifest: ResourceManifest,
        side: HostSide,
        limits: Limits,
    ) -> Result<Self> {
        let root = root.as_ref().canonicalize()?;
        resolve_and_validate(&root, &manifest)?;
        let lua = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::default())?;
        lua.set_memory_limit(limits.memory_bytes)?;
        let instruction_count = Arc::new(AtomicUsize::new(0));
        let counter = instruction_count.clone();
        let maximum = limits.instructions_per_dispatch;
        let dispatch_started = Arc::new(Mutex::new(Instant::now()));
        let started = dispatch_started.clone();
        let deadline_ms = limits.callback_timeout_ms;
        lua.set_hook(
            HookTriggers::new().every_nth_instruction(1_000),
            move |_, _| {
                let current = counter.fetch_add(1_000, Ordering::Relaxed) + 1_000;
                if current > maximum {
                    return Err(mlua::Error::runtime("instruction limit exceeded"));
                }
                if started
                    .lock()
                    .map_err(|_| mlua::Error::runtime("deadline lock poisoned"))?
                    .elapsed()
                    .as_millis()
                    > deadline_ms as u128
                {
                    return Err(mlua::Error::runtime("dispatch timeout exceeded"));
                }
                Ok(VmState::Continue)
            },
        )?;
        let outbound = Arc::new(Mutex::new(VecDeque::new()));
        let mut host = Self {
            lua,
            root,
            manifest,
            side,
            limits,
            state: LifecycleState::Stopped,
            outbound,
            instruction_count,
            dispatch_started,
        };
        host.install_sandbox()?;
        Ok(host)
    }

    pub fn state(&self) -> LifecycleState {
        self.state
    }
    pub fn manifest(&self) -> &ResourceManifest {
        &self.manifest
    }

    fn reset_budget(&self) {
        self.instruction_count.store(0, Ordering::Relaxed);
        if let Ok(mut started) = self.dispatch_started.lock() {
            *started = Instant::now();
        }
    }

    fn install_sandbox(&mut self) -> Result<()> {
        let globals = self.lua.globals();
        for forbidden in [
            "os", "io", "debug", "package", "require", "load", "loadfile", "dofile",
        ] {
            globals.set(forbidden, Value::Nil)?;
        }
        let queue = self.outbound.clone();
        let resource = self.manifest.name.clone();
        let side = self.side;
        let limits = self.limits.clone();
        globals.set(
            "__gv_emit",
            self.lua.create_function(
                move |lua, (name, target, values): (String, Option<u64>, MultiValue)| {
                    let arguments = values
                        .into_iter()
                        .map(|value| lua.from_value(value))
                        .collect::<mlua::Result<Vec<serde_json::Value>>>()?;
                    let event = ResourceEvent {
                        resource: resource.clone(),
                        name,
                        source: if side == HostSide::Client {
                            Some(1)
                        } else {
                            None
                        },
                        target,
                        arguments,
                        correlation_id: None,
                    };
                    event.validate(&limits).map_err(mlua::Error::external)?;
                    let mut queue = queue
                        .lock()
                        .map_err(|_| mlua::Error::runtime("event queue lock poisoned"))?;
                    if queue.len() >= limits.queue {
                        return Err(mlua::Error::runtime("event queue is full"));
                    }
                    queue.push_back(event);
                    Ok(())
                },
            )?,
        )?;
        globals.set(
            "InvokeNative",
            self.lua
                .create_function(
                    |_, (name, _args): (String, MultiValue)| match name.as_str() {
                        "GET_GAME_TIMER" => Ok(0_i64),
                        "PLAYER_ID" => Ok(0_i64),
                        _ => Err(mlua::Error::runtime(format!("UnsupportedNative: {name}"))),
                    },
                )?,
        )?;
        self.lua
            .load(BOOTSTRAP)
            .set_name("@gameverse/bootstrap.lua")
            .exec()?;
        let configure: mlua::Function = globals.get("__gv_configure")?;
        configure.call::<()>((self.limits.handlers, self.limits.callbacks))?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        anyhow::ensure!(
            self.state == LifecycleState::Stopped,
            "resource is not stopped"
        );
        self.state = LifecycleState::Starting;
        let result = self.load_scripts();
        if let Err(error) = result {
            let _ = self.reset_registrations();
            self.state = LifecycleState::Stopped;
            return Err(error);
        }
        self.state = LifecycleState::Started;
        Ok(())
    }

    fn load_scripts(&self) -> Result<()> {
        let scripts = self.manifest.shared_scripts.iter().chain(match self.side {
            HostSide::Client => self.manifest.client_scripts.iter(),
            HostSide::Server => self.manifest.server_scripts.iter(),
        });
        for pattern in scripts {
            if pattern.starts_with('@') {
                anyhow::bail!("external script reference is blocked: {pattern}");
            }
            let matcher = globset::Glob::new(&pattern.replace('\\', "/"))?.compile_matcher();
            let mut paths: Vec<_> = walkdir::WalkDir::new(&self.root)
                .follow_links(false)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().is_file())
                .filter(|entry| {
                    entry
                        .path()
                        .strip_prefix(&self.root)
                        .ok()
                        .is_some_and(|path| {
                            matcher.is_match(path.to_string_lossy().replace('\\', "/"))
                        })
                })
                .map(|entry| entry.into_path())
                .collect();
            paths.sort();
            for path in paths {
                self.reset_budget();
                let source = fs::read_to_string(&path)
                    .with_context(|| format!("read {}", path.display()))?;
                self.lua
                    .load(&source)
                    .set_name(format!("@{}", path.display()))
                    .exec()
                    .with_context(|| format!("execute {}", path.display()))?;
            }
        }
        Ok(())
    }

    pub fn dispatch(&self, event: &ResourceEvent) -> Result<()> {
        anyhow::ensure!(
            self.state == LifecycleState::Started,
            "resource is not started"
        );
        event.validate(&self.limits)?;
        self.reset_budget();
        let args = self.lua.to_value(&event.arguments)?;
        self.lua
            .globals()
            .get::<mlua::Function>("__gv_dispatch")?
            .call::<()>((event.name.clone(), args))?;
        Ok(())
    }

    pub fn call_callback(
        &self,
        name: &str,
        arguments: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>> {
        self.reset_budget();
        let args = self.lua.to_value(&arguments)?;
        let values: Value = self
            .lua
            .globals()
            .get::<mlua::Function>("__gv_callback")?
            .call((name, args))?;
        Ok(self.lua.from_value(values)?)
    }

    pub fn call_export(
        &self,
        name: &str,
        arguments: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>> {
        self.reset_budget();
        let args = self.lua.to_value(&arguments)?;
        let values: Value = self
            .lua
            .globals()
            .get::<mlua::Function>("__gv_export")?
            .call((name, args))?;
        Ok(self.lua.from_value(values)?)
    }

    pub fn advance(&self, elapsed_ms: u64) -> Result<()> {
        self.reset_budget();
        self.lua
            .globals()
            .get::<mlua::Function>("__gv_advance")?
            .call::<()>(elapsed_ms)?;
        Ok(())
    }

    pub fn drain_outbound(&self) -> Vec<ResourceEvent> {
        self.outbound
            .lock()
            .map(|mut queue| queue.drain(..).collect())
            .unwrap_or_default()
    }

    pub fn stop(&mut self) -> Result<()> {
        anyhow::ensure!(
            self.state == LifecycleState::Started,
            "resource is not started"
        );
        self.state = LifecycleState::Stopping;
        self.reset_registrations()?;
        self.outbound
            .lock()
            .map_err(|_| anyhow::anyhow!("event queue lock poisoned"))?
            .clear();
        self.state = LifecycleState::Stopped;
        Ok(())
    }

    fn reset_registrations(&self) -> Result<()> {
        self.lua
            .globals()
            .get::<mlua::Function>("__gv_reset")?
            .call::<()>(())?;
        Ok(())
    }
}

pub struct ResourceCluster {
    hosts: Vec<ResourceHost>,
}
impl ResourceCluster {
    pub fn new(hosts: Vec<ResourceHost>) -> Self {
        Self { hosts }
    }
    pub fn call_export(
        &self,
        resource: &str,
        export: &str,
        arguments: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>> {
        self.hosts
            .iter()
            .find(|host| host.manifest.name == resource)
            .ok_or_else(|| anyhow::anyhow!("unknown resource: {resource}"))?
            .call_export(export, arguments)
    }
}

const BOOTSTRAP: &str = r#"
local handlers, callbacks, resource_exports, jobs = {}, {}, {}, {}
local next_handler, now, max_handlers, max_callbacks = 1, 0, 128, 128
function __gv_configure(h, c) max_handlers, max_callbacks = h, c end
local function count(t) local n=0 for _ in pairs(t) do n=n+1 end return n end
function RegisterNetEvent(name, handler) if handler then return AddEventHandler(name, handler) end end
function AddEventHandler(name, handler)
  if count(handlers) >= max_handlers then error('handler limit exceeded') end
  local id=next_handler; next_handler=next_handler+1
  handlers[id]={name=name, fn=handler}; return id
end
function RemoveEventHandler(id) handlers[id]=nil end
function TriggerEvent(name, ...) for _,h in pairs(handlers) do if h.name==name then h.fn(...) end end end
function TriggerServerEvent(name, ...) __gv_emit(name, nil, ...) end
function TriggerClientEvent(name, target, ...) __gv_emit(name, target, ...) end
function RegisterCallback(name, handler)
  if count(callbacks) >= max_callbacks then error('callback limit exceeded') end
  callbacks[name]=handler
end
function TriggerCallback(name, ...) return __gv_callback(name, {...}) end
function exports(name, handler) resource_exports[name]=handler end
function Wait(ms) return coroutine.yield(ms or 0) end
function CreateThread(fn) local co=coroutine.create(fn); jobs[#jobs+1]={at=now, co=co} return co end
function SetTimeout(ms, fn) jobs[#jobs+1]={at=now+(ms or 0), fn=fn} end
function __gv_advance(ms)
  now=now+ms
  local pending=jobs; jobs={}
  for _,job in ipairs(pending) do
    if job.at<=now then
      if job.fn then job.fn() else
        local ok, delay=coroutine.resume(job.co)
        if not ok then error(delay) end
        if coroutine.status(job.co)~='dead' then jobs[#jobs+1]={at=now+(delay or 0), co=job.co} end
      end
    else jobs[#jobs+1]=job end
  end
end
function __gv_dispatch(name, args) TriggerEvent(name, table.unpack(args)) end
function __gv_callback(name, args) if not callbacks[name] then error('unknown callback: '..name) end return {callbacks[name](table.unpack(args))} end
function __gv_export(name, args) if not resource_exports[name] then error('unknown export: '..name) end return {resource_exports[name](table.unpack(args))} end
function __gv_reset() handlers={}; callbacks={}; resource_exports={}; jobs={} end
"#;

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(source: &str) -> (tempfile::TempDir, ResourceManifest) {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("main.lua"), source).unwrap();
        let manifest = ResourceManifest {
            name: "fixture".into(),
            client_scripts: vec!["main.lua".into()],
            server_scripts: vec![],
            shared_scripts: vec![],
            dependencies: vec![],
            files: vec![],
            exports: vec![],
            data_files: vec![],
            ui_page: None,
            source: gameverse_resource_manifest::SourceMetadata {
                manifest: "gameverse.toml".into(),
                legacy: false,
                license: Some("MIT".into()),
            },
        };
        (dir, manifest)
    }
    #[test]
    fn lifecycle_events_callbacks_exports_timers_and_cleanup() {
        let (dir, manifest) = fixture("RegisterNetEvent('ping', function(v) TriggerServerEvent('pong', v+1) end)\nRegisterCallback('sum', function(a,b) return a+b end)\nexports('answer', function() return 42 end)\nSetTimeout(10, function() TriggerServerEvent('timer', true) end)");
        let mut host =
            ResourceHost::new(dir.path(), manifest, HostSide::Client, Limits::default()).unwrap();
        host.start().unwrap();
        host.dispatch(&ResourceEvent {
            resource: "fixture".into(),
            name: "ping".into(),
            source: None,
            target: None,
            arguments: vec![1.into()],
            correlation_id: None,
        })
        .unwrap();
        assert_eq!(
            host.drain_outbound()[0].arguments,
            vec![serde_json::json!(2)]
        );
        assert_eq!(
            host.call_callback("sum", vec![2.into(), 3.into()]).unwrap(),
            vec![serde_json::json!(5)]
        );
        assert_eq!(
            host.call_export("answer", vec![]).unwrap(),
            vec![serde_json::json!(42)]
        );
        host.advance(10).unwrap();
        assert_eq!(host.drain_outbound()[0].name, "timer");
        host.stop().unwrap();
        assert_eq!(host.state(), LifecycleState::Stopped);
    }
    #[test]
    fn blocks_unsafe_libraries_unknown_native_and_infinite_script() {
        let (dir, manifest) = fixture("assert(os == nil and io == nil and debug == nil and load == nil)\nInvokeNative('UNKNOWN')");
        let mut host =
            ResourceHost::new(dir.path(), manifest, HostSide::Client, Limits::default()).unwrap();
        assert!(host.start().unwrap_err().to_string().contains("execute"));
        assert_eq!(host.state(), LifecycleState::Stopped);
        fs::write(dir.path().join("main.lua"), "while true do end").unwrap();
        let mut host = ResourceHost::new(
            dir.path(),
            host.manifest.clone(),
            HostSide::Client,
            Limits::default(),
        )
        .unwrap();
        assert!(host.start().is_err());
    }
}
