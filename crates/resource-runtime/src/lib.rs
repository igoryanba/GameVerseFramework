//! Sandboxed Lua 5.4 host for optional FiveM-compatible resources.
use anyhow::{Context, Result};
use gameverse_resource_manifest::{resolve_and_validate, ResourceManifest};
use mlua::{HookTriggers, Lua, LuaOptions, LuaSerdeExt, MultiValue, StdLib, Value, VmState};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
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
    pub correlation_id: Option<String>,
}
impl ResourceEvent {
    pub fn validate(&self, limits: &Limits) -> Result<()> {
        anyhow::ensure!(
            !self.resource.is_empty() && !self.name.is_empty(),
            "resource and event name are required"
        );
        anyhow::ensure!(
            self.resource.len() <= 128 && self.name.len() <= 128,
            "resource or event name exceeds 128 bytes"
        );
        anyhow::ensure!(self.arguments.len() <= 64, "event has too many arguments");
        anyhow::ensure!(
            self.correlation_id
                .as_ref()
                .is_none_or(|value| !value.is_empty() && value.len() <= 128),
            "invalid correlation ID"
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
    convars: Arc<Mutex<BTreeMap<String, String>>>,
    generation: u64,
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
        );
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
            convars: Arc::new(Mutex::new(BTreeMap::new())),
            generation: 0,
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
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn set_convar(&self, name: &str, value: &str) -> Result<()> {
        validate_identifier(name, "convar")?;
        anyhow::ensure!(value.len() <= 4096, "convar value is too long");
        self.convars
            .lock()
            .map_err(|_| anyhow::anyhow!("convar registry lock poisoned"))?
            .insert(name.to_string(), value.to_string());
        Ok(())
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
        let current_resource = self.manifest.name.clone();
        globals.set(
            "GetCurrentResourceName",
            self.lua
                .create_function(move |_, ()| Ok(current_resource.clone()))?,
        )?;
        let resource_states = self
            .manifest
            .dependencies
            .iter()
            .cloned()
            .chain(std::iter::once(self.manifest.name.clone()))
            .collect::<BTreeSet<_>>();
        globals.set(
            "GetResourceState",
            self.lua.create_function(move |_, name: String| {
                Ok(if resource_states.contains(&name) {
                    "started"
                } else {
                    "missing"
                })
            })?,
        )?;
        let metadata_resource = self.manifest.name.clone();
        let metadata = Arc::new(manifest_metadata(&self.manifest));
        let metadata_count = metadata.clone();
        let count_resource = metadata_resource.clone();
        globals.set(
            "GetNumResourceMetadata",
            self.lua
                .create_function(move |_, (resource, key): (String, String)| {
                    if resource != count_resource {
                        return Ok(0_i64);
                    }
                    Ok(metadata_count
                        .get(&key)
                        .map_or(0, |values| values.len() as i64))
                })?,
        )?;
        globals.set(
            "GetResourceMetadata",
            self.lua.create_function(
                move |_, (resource, key, index): (String, String, Option<usize>)| {
                    if resource != metadata_resource {
                        return Ok(None::<String>);
                    }
                    Ok(metadata
                        .get(&key)
                        .and_then(|values| values.get(index.unwrap_or(0)))
                        .cloned())
                },
            )?,
        )?;
        let convars = self.convars.clone();
        let writable_prefix = format!("{}:", self.manifest.name);
        globals.set(
            "GetConvar",
            self.lua.create_function(
                move |_, (name, fallback): (String, String)| -> mlua::Result<String> {
                    validate_identifier(&name, "convar").map_err(mlua::Error::external)?;
                    Ok(convars
                        .lock()
                        .map_err(|_| mlua::Error::runtime("convar registry lock poisoned"))?
                        .get(&name)
                        .cloned()
                        .unwrap_or(fallback))
                },
            )?,
        )?;
        let convars = self.convars.clone();
        globals.set(
            "GetConvarInt",
            self.lua.create_function(
                move |_, (name, fallback): (String, i64)| -> mlua::Result<i64> {
                    validate_identifier(&name, "convar").map_err(mlua::Error::external)?;
                    Ok(convars
                        .lock()
                        .map_err(|_| mlua::Error::runtime("convar registry lock poisoned"))?
                        .get(&name)
                        .and_then(|value| value.parse().ok())
                        .unwrap_or(fallback))
                },
            )?,
        )?;
        let convars = self.convars.clone();
        globals.set(
            "SetConvar",
            self.lua
                .create_function(move |_, (name, value): (String, String)| {
                    validate_identifier(&name, "convar").map_err(mlua::Error::external)?;
                    if !name.starts_with(&writable_prefix) {
                        return Err(mlua::Error::runtime(
                            "resource may only write namespaced convars",
                        ));
                    }
                    if value.len() > 4096 {
                        return Err(mlua::Error::runtime("convar value is too long"));
                    }
                    convars
                        .lock()
                        .map_err(|_| mlua::Error::runtime("convar registry lock poisoned"))?
                        .insert(name, value);
                    Ok(())
                })?,
        )?;
        globals.set(
            "joaat",
            self.lua
                .create_function(|_, value: String| Ok(joaat(&value)))?,
        )?;
        globals.set("GetHashKey", globals.get::<mlua::Function>("joaat")?)?;
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
            if let Ok(mut outbound) = self.outbound.lock() {
                outbound.clear();
            }
            self.state = LifecycleState::Stopped;
            return Err(error);
        }
        self.generation = self.generation.saturating_add(1);
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
            .call::<()>((event.name.clone(), args, event.source.unwrap_or(0)))?;
        Ok(())
    }

    pub fn call_callback(
        &self,
        name: &str,
        arguments: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>> {
        anyhow::ensure!(
            self.state == LifecycleState::Started,
            "resource is not started"
        );
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
        anyhow::ensure!(
            self.state == LifecycleState::Started,
            "resource is not started"
        );
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
        anyhow::ensure!(
            self.state == LifecycleState::Started,
            "resource is not started"
        );
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

fn validate_identifier(value: &str, kind: &str) -> Result<()> {
    anyhow::ensure!(
        !value.is_empty() && value.len() <= 128,
        "invalid {kind} name"
    );
    anyhow::ensure!(
        value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"_:-.".contains(&byte)),
        "invalid {kind} name"
    );
    Ok(())
}

fn manifest_metadata(manifest: &ResourceManifest) -> BTreeMap<String, Vec<String>> {
    let mut output = BTreeMap::new();
    let mut insert = |key: &str, value: &Option<String>| {
        if let Some(value) = value {
            output.insert(key.to_string(), vec![value.clone()]);
        }
    };
    insert("fx_version", &manifest.metadata.fx_version);
    insert("lua54", &manifest.metadata.lua54);
    insert("author", &manifest.metadata.author);
    insert("version", &manifest.metadata.version);
    insert("repository", &manifest.metadata.repository);
    output.insert("game".into(), manifest.metadata.games.clone());
    output.insert("provide".into(), manifest.metadata.provides.clone());
    output.insert("dependency".into(), manifest.dependencies.clone());
    output.insert("client_script".into(), manifest.client_scripts.clone());
    output.insert("server_script".into(), manifest.server_scripts.clone());
    output.insert("shared_script".into(), manifest.shared_scripts.clone());
    output.insert("export".into(), manifest.exports.clone());
    output
}

fn joaat(value: &str) -> u32 {
    let mut hash = 0_u32;
    for byte in value.bytes().map(|byte| byte.to_ascii_lowercase()) {
        hash = hash.wrapping_add(byte as u32);
        hash = hash.wrapping_add(hash << 10);
        hash ^= hash >> 6;
    }
    hash = hash.wrapping_add(hash << 3);
    hash ^= hash >> 11;
    hash.wrapping_add(hash << 15)
}

pub struct ResourceCluster {
    hosts: BTreeMap<String, ResourceHost>,
    start_order: Vec<String>,
}
impl ResourceCluster {
    pub fn new(hosts: Vec<ResourceHost>) -> Result<Self> {
        let hosts: BTreeMap<_, _> = hosts
            .into_iter()
            .map(|host| (host.manifest.name.clone(), host))
            .collect();
        let start_order = dependency_order(&hosts)?;
        Ok(Self { hosts, start_order })
    }

    pub fn start_order(&self) -> &[String] {
        &self.start_order
    }

    pub fn start_all(&mut self) -> Result<()> {
        let mut started = Vec::new();
        for name in self.start_order.clone() {
            if let Err(error) = self
                .hosts
                .get_mut(&name)
                .expect("ordered host exists")
                .start()
            {
                for started_name in started.into_iter().rev() {
                    let _ = self
                        .hosts
                        .get_mut(&started_name)
                        .expect("started host exists")
                        .stop();
                }
                return Err(error).with_context(|| format!("start resource {name}"));
            }
            started.push(name);
        }
        Ok(())
    }

    pub fn stop_all(&mut self) -> Result<()> {
        for name in self.start_order.clone().into_iter().rev() {
            let host = self.hosts.get_mut(&name).expect("ordered host exists");
            if host.state() == LifecycleState::Started {
                host.stop()
                    .with_context(|| format!("stop resource {name}"))?;
            }
        }
        Ok(())
    }

    pub fn host(&self, resource: &str) -> Option<&ResourceHost> {
        self.hosts.get(resource)
    }

    pub fn dispatch(&self, event: &ResourceEvent) -> Result<()> {
        self.hosts
            .get(&event.resource)
            .ok_or_else(|| anyhow::anyhow!("unknown resource: {}", event.resource))?
            .dispatch(event)
    }

    pub fn advance_all(&self, elapsed_ms: u64) -> Result<()> {
        for name in &self.start_order {
            self.hosts
                .get(name)
                .expect("ordered host exists")
                .advance(elapsed_ms)
                .with_context(|| format!("advance resource {name}"))?;
        }
        Ok(())
    }

    pub fn drain_outbound(&self) -> Vec<ResourceEvent> {
        let mut events = Vec::new();
        for name in &self.start_order {
            events.extend(
                self.hosts
                    .get(name)
                    .expect("ordered host exists")
                    .drain_outbound(),
            );
        }
        events
    }

    pub fn restart(&mut self, resource: &str) -> Result<u64> {
        let host = self
            .hosts
            .get_mut(resource)
            .ok_or_else(|| anyhow::anyhow!("unknown resource: {resource}"))?;
        if host.state() == LifecycleState::Started {
            host.stop()
                .with_context(|| format!("stop resource {resource}"))?;
        }
        host.start()
            .with_context(|| format!("start resource {resource}"))?;
        Ok(host.generation())
    }
    pub fn call_export(
        &self,
        resource: &str,
        export: &str,
        arguments: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>> {
        self.hosts
            .get(resource)
            .ok_or_else(|| anyhow::anyhow!("unknown resource: {resource}"))?
            .call_export(export, arguments)
    }

    /// Calls an export on behalf of another resource.
    ///
    /// The caller must declare the provider as a dependency. This keeps the
    /// compatibility harness deterministic and prevents a resource from
    /// reaching arbitrary peers in the cluster.
    pub fn call_export_for(
        &self,
        caller: &str,
        provider: &str,
        export: &str,
        arguments: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>> {
        let caller_host = self
            .hosts
            .get(caller)
            .ok_or_else(|| anyhow::anyhow!("unknown resource: {caller}"))?;
        anyhow::ensure!(
            caller_host.state() == LifecycleState::Started,
            "caller resource is not started: {caller}"
        );
        anyhow::ensure!(
            caller_host
                .manifest()
                .dependencies
                .iter()
                .any(|dependency| dependency == provider),
            "resource {caller} does not declare dependency {provider}"
        );
        self.call_export(provider, export, arguments)
            .with_context(|| format!("resource {caller} call {provider}.{export}"))
    }
}

fn dependency_order(hosts: &BTreeMap<String, ResourceHost>) -> Result<Vec<String>> {
    fn visit(
        name: &str,
        hosts: &BTreeMap<String, ResourceHost>,
        visiting: &mut BTreeSet<String>,
        visited: &mut BTreeSet<String>,
        output: &mut Vec<String>,
    ) -> Result<()> {
        if visited.contains(name) {
            return Ok(());
        }
        anyhow::ensure!(
            visiting.insert(name.into()),
            "cyclic resource dependency at {name}"
        );
        for dependency in &hosts[name].manifest.dependencies {
            anyhow::ensure!(
                hosts.contains_key(dependency),
                "missing resource dependency {dependency} for {name}"
            );
            visit(dependency, hosts, visiting, visited, output)?;
        }
        visiting.remove(name);
        visited.insert(name.into());
        output.push(name.into());
        Ok(())
    }
    let mut output = Vec::new();
    let mut visited = BTreeSet::new();
    for name in hosts.keys() {
        visit(name, hosts, &mut BTreeSet::new(), &mut visited, &mut output)?;
    }
    Ok(output)
}

const BOOTSTRAP: &str = r#"
local handlers, callbacks, resource_exports, jobs, commands = {}, {}, {}, {}, {}
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
local function register_export(name, handler) resource_exports[name]=handler end
exports=setmetatable({}, {
  __call=function(_, name, handler) register_export(name, handler) end,
  __index=function(_, resource)
    return setmetatable({}, {__index=function(_, export)
      return function(...) error('cross-resource bracket export requires cluster routing: '..resource..'.'..export) end
    end})
  end
})
function RegisterCommand(name, handler, restricted)
  if type(name)~='string' or #name==0 or #name>128 or not string.match(name, '^[%w_:%-%.]+$') then error('invalid command name') end
  if type(handler)~='function' then error('command handler is required') end
  commands[name]={fn=handler, restricted=restricted==true}
end
function ExecuteCommand(command)
  if type(command)~='string' or #command>4096 then error('invalid command') end
  local args={}; for value in string.gmatch(command, '%S+') do args[#args+1]=value end
  local name=table.remove(args, 1); local registered=commands[name]
  if not registered then error('unknown command: '..tostring(name)) end
  if registered.restricted then error('restricted command requires permission bridge') end
  return registered.fn(0, args, command)
end
local function vector(kind, ...)
  local values={...}; local result={__type=kind}
  local names={'x','y','z','w'}; for i,value in ipairs(values) do result[i]=value; result[names[i]]=value end
  return result
end
function vector2(x,y) return vector('vector2',x,y) end
function vector3(x,y,z) return vector('vector3',x,y,z) end
function vector4(x,y,z,w) return vector('vector4',x,y,z,w) end
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
function __gv_dispatch(name, args, event_source)
  local previous=source; source=event_source or 0
  local ok, result=pcall(TriggerEvent, name, table.unpack(args))
  source=previous
  if not ok then error(result) end
end
function __gv_callback(name, args) if not callbacks[name] then error('unknown callback: '..name) end return {callbacks[name](table.unpack(args))} end
function __gv_export(name, args) if not resource_exports[name] then error('unknown export: '..name) end return {resource_exports[name](table.unpack(args))} end
function __gv_reset() handlers={}; callbacks={}; resource_exports={}; jobs={}; commands={} end
"#;

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(source: &str) -> (tempfile::TempDir, ResourceManifest) {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("main.lua"), source).unwrap();
        let manifest = ResourceManifest {
            name: "fixture".into(),
            metadata: Default::default(),
            client_scripts: vec![],
            server_scripts: vec![],
            shared_scripts: vec!["main.lua".into()],
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

    #[test]
    fn enforces_payload_queue_registration_and_memory_limits() {
        let limits = Limits {
            handlers: 1,
            callbacks: 1,
            queue: 2,
            ..Limits::default()
        };
        let event = ResourceEvent {
            resource: "fixture".into(),
            name: "large".into(),
            source: None,
            target: None,
            arguments: vec![serde_json::json!("x".repeat(limits.event_payload_bytes))],
            correlation_id: None,
        };
        assert!(event.validate(&limits).is_err());

        let (dir, manifest) =
            fixture("AddEventHandler('a', function() end)\nAddEventHandler('b', function() end)");
        let mut host =
            ResourceHost::new(dir.path(), manifest, HostSide::Client, limits.clone()).unwrap();
        assert!(host.start().is_err());
        assert!(host.drain_outbound().is_empty());

        fs::write(
            dir.path().join("main.lua"),
            "local value = string.rep('x', 20000000)",
        )
        .unwrap();
        let mut host =
            ResourceHost::new(dir.path(), host.manifest.clone(), HostSide::Client, limits).unwrap();
        assert!(host.start().is_err());
    }

    #[test]
    fn starts_dependencies_calls_cross_resource_export_and_stops_in_reverse() {
        let (provider_dir, mut provider) = fixture("exports('answer', function() return 42 end)");
        provider.name = "provider".into();
        let (consumer_dir, mut consumer) = fixture("RegisterNetEvent('ready', function() end)");
        consumer.name = "consumer".into();
        consumer.dependencies = vec!["provider".into()];
        let provider_host = ResourceHost::new(
            provider_dir.path(),
            provider,
            HostSide::Server,
            Limits::default(),
        )
        .unwrap();
        let consumer_host = ResourceHost::new(
            consumer_dir.path(),
            consumer,
            HostSide::Server,
            Limits::default(),
        )
        .unwrap();
        let mut cluster = ResourceCluster::new(vec![consumer_host, provider_host]).unwrap();
        assert_eq!(cluster.start_order(), ["provider", "consumer"]);
        cluster.start_all().unwrap();
        assert_eq!(
            cluster
                .call_export_for("consumer", "provider", "answer", vec![])
                .unwrap(),
            [serde_json::json!(42)]
        );
        assert!(cluster
            .call_export_for("provider", "consumer", "answer", vec![])
            .unwrap_err()
            .to_string()
            .contains("does not declare dependency"));
        cluster.stop_all().unwrap();
        assert!(cluster
            .call_export("provider", "answer", vec![])
            .unwrap_err()
            .to_string()
            .contains("not started"));
        assert_eq!(
            cluster.host("provider").unwrap().state(),
            LifecycleState::Stopped
        );
        assert_eq!(
            cluster.host("consumer").unwrap().state(),
            LifecycleState::Stopped
        );
    }

    #[test]
    fn restart_does_not_duplicate_old_handlers() {
        let (dir, manifest) =
            fixture("RegisterNetEvent('ping', function() TriggerServerEvent('pong') end)");
        let mut host =
            ResourceHost::new(dir.path(), manifest, HostSide::Client, Limits::default()).unwrap();
        host.start().unwrap();
        assert_eq!(host.generation(), 1);
        host.stop().unwrap();
        host.start().unwrap();
        assert_eq!(host.generation(), 2);
        host.dispatch(&ResourceEvent {
            resource: "fixture".into(),
            name: "ping".into(),
            source: None,
            target: None,
            arguments: vec![],
            correlation_id: None,
        })
        .unwrap();
        assert_eq!(host.drain_outbound().len(), 1);
    }

    #[test]
    fn cluster_routes_events_drains_outputs_and_restarts_generation() {
        let (dir, manifest) = fixture(
            "RegisterNetEvent('ping', function(value) TriggerServerEvent('pong', value) end)",
        );
        let host =
            ResourceHost::new(dir.path(), manifest, HostSide::Server, Limits::default()).unwrap();
        let mut cluster = ResourceCluster::new(vec![host]).unwrap();
        cluster.start_all().unwrap();
        cluster
            .dispatch(&ResourceEvent {
                resource: "fixture".into(),
                name: "ping".into(),
                source: Some(7),
                target: None,
                arguments: vec![serde_json::json!(42)],
                correlation_id: Some("request-1".into()),
            })
            .unwrap();
        let output = cluster.drain_outbound();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0].name, "pong");
        assert_eq!(output[0].arguments, [serde_json::json!(42)]);
        assert_eq!(cluster.restart("fixture").unwrap(), 2);
        cluster
            .dispatch(&ResourceEvent {
                resource: "fixture".into(),
                name: "ping".into(),
                source: Some(7),
                target: None,
                arguments: vec![serde_json::json!(43)],
                correlation_id: None,
            })
            .unwrap();
        assert_eq!(cluster.drain_outbound().len(), 1);
    }

    #[test]
    fn provides_resource_convar_command_vector_hash_and_source_apis() {
        let source = r#"
assert(GetCurrentResourceName() == 'fixture')
assert(GetResourceState('fixture') == 'started')
assert(GetNumResourceMetadata('fixture', 'version') == 1)
assert(GetResourceMetadata('fixture', 'version', 0) == '1.0.0')
assert(GetConvar('fixture:locale', 'en') == 'ru')
assert(GetConvarInt('fixture:number', 7) == 42)
SetConvar('fixture:written', 'yes')
assert(GetConvar('fixture:written', 'no') == 'yes')
local position=vector3(1,2,3); assert(position.x==1 and position[3]==3 and position.__type=='vector3')
assert(joaat('adder') == GetHashKey('ADDER'))
RegisterCommand('hello', function(command_source, args) TriggerServerEvent('command', command_source, args[1]) end, false)
ExecuteCommand('hello world')
RegisterNetEvent('source-check', function() TriggerServerEvent('source-result', source) end)
"#;
        let (dir, mut manifest) = fixture(source);
        manifest.metadata.version = Some("1.0.0".into());
        let mut host =
            ResourceHost::new(dir.path(), manifest, HostSide::Server, Limits::default()).unwrap();
        host.set_convar("fixture:locale", "ru").unwrap();
        host.set_convar("fixture:number", "42").unwrap();
        host.start().unwrap();
        assert_eq!(host.drain_outbound()[0].name, "command");
        host.dispatch(&ResourceEvent {
            resource: "fixture".into(),
            name: "source-check".into(),
            source: Some(37),
            target: None,
            arguments: vec![],
            correlation_id: None,
        })
        .unwrap();
        assert_eq!(host.drain_outbound()[0].arguments, [serde_json::json!(37)]);
    }
}
