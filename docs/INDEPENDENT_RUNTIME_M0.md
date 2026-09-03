# Independent Multiplayer Runtime M0 — audit and implementation plan

Historical pre-implementation audit. For the implemented M0 and measured results,
see [M0 runtime](M0_RUNTIME.md). Statements below describe the initial checkout,
not the subsequently installed tools or completed runtime.

Audit date: 2026-09-03. Repository: igoryanba/GameVerseFramework.
Baseline: `e3545c954f3f31a693626f0ba3230ae472fc52d7` (origin/main).
Working branch: `codex/independent-runtime-m0`. Initial checkout was clean.
No commit, push, deployment, or game process interaction was performed.

## Scope and current status

M0 is an independent multiplayer platform, not a FiveM resource framework.
This change prepares a compilation boundary and records blockers; it does not
implement an operational multiplayer runtime or claim successful Rust compilation.
Game installation, DRM, entitlement, and anti-cheat bypass are outside this work.

`origin/develop` is 16 commits behind main, with no unique commits. This audit uses
the current default branch to include the latest code. Contribution guides ask
for branches/PRs from/to develop; reconcile that lag before submitting a PR.

## Repository map

| Area | Observed role / limitation |
| --- | --- |
| `core` | Rust library and `gameverse_server`; server, game integration, resources, natives, FCL mixed in one crate |
| `sdk/cli-tools` | Rust `gameverse` CLI, depends on the entire core with default features |
| `benchmarks/perf_demo` | Third member of the root Cargo workspace; demo is not a multiplayer acceptance test |
| `networking` | README files, no transport implementation in this directory |
| `services/*` | Auth, inventory, chat, logging, player-data; not members of the root workspace |
| `sdk/native-generator`, `tools/fivem-analyzer` | Separate tool manifests, not covered by root workspace tests |
| `admin-panel` | Vite/React sources; HTML entry does not load React |
| `ui/webassembly-demo` | Separate UI experiment; not a native game client |
| `.github/workflows/perf.yml` | Scheduled/manual performance demo; no PR build/test matrix |

## Actual validation

| Command / check | Result |
| --- | --- |
| `git status --short --branch` before changes | Clean main tracking origin/main |
| `cargo build --locked -p gameverse-core --bin gameverse_server` | Could not start: cargo not found |
| `cargo test --locked --workspace` | Could not start: cargo not found; zero tests executed |
| Admin `npm ci --ignore-scripts --no-audit --no-fund` | Successful after network permission; 103 packages |
| Admin `npm run build -- --outDir ../../../admin-panel-build` | Exit 0, Vite 4.5.14, 2 modules, index.html 1.49 kB |
| `git diff --check` | Passed for preparation changes |

The host has Node 20.17.0 / npm 10.8.2. Cargo/rustc were not found on PATH or in
the standard user Cargo bin directory. A Rust toolchain was not installed as part
of this quick audit. Thus there are no observed Rust compiler diagnostics.
The npm sandbox initially produced EACCES and then an erroneous exit-0 message
(`Exit handler never called`); the permitted retry succeeded. Build initially hit
EPERM reading the user directory; its permitted retry also succeeded.

Admin build success is **not** UI readiness: `admin-panel/index.html` has no
module script importing `/src/main.tsx`. Its inline SSE handlers reference
`uptime`, `players`, `tick`, and `logOutput`, absent from the static document.
No browser/backend integration test was performed. Other UI and service builds
were not run. PostgreSQL/Redis services were not started.

## Evidence-backed blockers

1. **Transport is a stub.** `core/src/net.rs` initialize only flips a flag;
   create_connection inserts metadata into a HashMap. It neither opens a socket
   nor establishes QUIC. Dependency declarations are not a working transport.
2. **Server/client responsibilities are coupled.**
   `core/src/server/runtime.rs::initialize_components` calls engine.initialize
   outside dev mode. `core/src/engine.rs::initialize` creates GameIntegrator and
   connects to a game process. `--dev` skips this, but does not provide multiplayer.
3. **No replication pipeline found in core.** The server loop increments a
   counter after a 100 ms sleep; it does not step an authoritative world, process
   client input or emit entity snapshots. Sleep is recreated after each command,
   so it is not a stable simulation clock under command traffic.
4. **No standalone client target in the root workspace.** Game integration helpers
   and CLI client templates do not implement a versioned client session runtime.
   Game hook setup contains TODOs. In-game execution was not tested.
5. **Events are local, not a protocol.** FCL forwards calls into EventSystem.
   EventSystem uses an unbounded in-process channel. Service protobuf files do
   not establish a game wire protocol with session, sequence and entity lifecycle.
6. **FCL test/API mismatch, found by inspection.** Tests call
   `NativeManager::new()` although it requires GameType; use
   `EventSystem::new().await?` although new is synchronous and returns Self; call
   `register_test_natives`, not found on NativeManager. These are source findings,
   not captured compiler errors. Disabling FCL must not be reported as fixing them.
7. **Test/CI coverage gap.** `core/src/server/runtime_tests.rs` is not declared
   by the server module or runtime file. Root workspace tests omit services.
   Existing FCL simulations do not prove actual QBCore compatibility or gameplay.
8. **Scripting is incomplete.** `core/src/scripting.rs` has TODO implementations
   for initialize/load/start/stop/hot reload. Keep resource execution out of the
   first replication milestone until lifecycle semantics are real.

## Preparation change and compatibility

`gameverse-core` now exposes `fivem-compat`; the fcl module and its integration
test are gated by that feature. The existing default enables it to preserve
existing consumers. The server-basic Rust template explicitly requests it.
No FCL implementation or test assertions were removed or repaired.

Commands to validate after provisioning Rust and native build prerequisites:

```powershell
cargo check --locked -p gameverse-core --no-default-features
cargo test --locked -p gameverse-core --no-default-features
cargo test --locked -p gameverse-core --features fivem-compat
cargo build --locked -p gameverse-core --bin gameverse_server
cargo test --locked --workspace
```

These commands have **not** passed here. The first two verify exclusion of FCL
only; game integration and existing legacy modules remain compiled. Lua and
other dependencies are not yet optional. The CLI still activates core defaults;
Cargo feature unification means a workspace build is not proof of FCL exclusion.
Use the package-scoped command to check the temporary boundary.

## Target dependency direction

Proposed next-stage crates (not yet created):

```text
gameverse-protocol       (bounded wire types/codec; no game or FCL dependencies)
gameverse-runtime       -> protocol (session/world/tick/replication)
gameverse-server        -> runtime + transport
gameverse-client        -> protocol + transport + game-adapter interface
gameverse-compat-fivem  -> public runtime/resource/native interfaces
```

The dedicated server must run without a game process. No dependency from
protocol/runtime into compatibility, game memory access, launcher or UI.
Use a fake game adapter for initial client tests. Extract event and entity types
from game integration before moving FCL to its own crate. Keep current public
paths behind a temporary re-export if needed for migration.

## Ordered backlog with acceptance criteria

1. **Restore a reproducible Rust baseline.** Provision Windows Rust/native build
   tools, select and pin a tested toolchain; run locked check/test and record exact
   failures. Fix compilation in small changes before protocol work. Acceptance:
   successful headless library build; remaining legacy failures explicit.
2. **Extract protocol/runtime and isolate platform adapters.** Move neutral types
   and server lifecycle into crates with no game/FCL dependencies. Dedicated
   server entry must not initialize GameIntegrator. Acceptance: dependency graph
   plus server start/stop test on a machine without GTA.
3. **Add PR build/test gates.** Windows + Linux for portable runtime; explicit
   legacy/FCL lane so feature-gating cannot silently erase its failures. Decide
   service workspace/exclusion policy. Acceptance: failures block relevant lane
   and test discovery includes intended runtime tests.
4. **Specify protocol v0 and bounded codec.** Hello/Welcome/Reject, major/minor
   version, session ID, sequence, tick, input, spawn/despawn and snapshot records.
   Fixed limits before allocation; no raw pointers/native handles on the wire.
   Acceptance: round-trip/golden vectors and malformed/oversize/version tests.
5. **Implement one transport and session state machine.** Start with QUIC using
   reliable control messages; set connection/time/queue limits, disconnect and
   reconnect semantics. Bind loopback for M0. Acceptance: two separate client
   processes handshake and reconnect; incompatible version is rejected.
6. **Implement authoritative world/tick.** Stable fixed-step interval, server
   entity IDs with generation, input validation and monotonic sequences.
   Acceptance: deterministic replay and stale/duplicate input rejection;
   one player cannot control another player's entity.
7. **Implement full-snapshot replication first.** Spawn/update/despawn, per-client
   visibility, tick/sequence ordering and full resync on reconnect. Defer deltas
   and bandwidth optimization. Acceptance: both clients converge to server state
   under simulated loss, jitter, duplicate and reordered application updates.
8. **Build standalone client harness and adapter contract.** Input capture,
   local entity mapping, snapshot application, interpolation and clean shutdown
   using a fake adapter first. Acceptance: two clients move distinct entities;
   no GTA/FiveM dependency. Evaluate a supported Windows game adapter separately.
9. **Extract the FCL adapter and repair its tests.** Depend only on public runtime
   interfaces, replace stale test helpers, explicitly initialize event dispatch.
   Acceptance: compatibility-enabled tests pass, runtime builds without adapter;
   do not claim QBCore support from event-name simulations.
10. **Add observable end-to-end M0 run.** Log session/tick/queue/snapshot counters;
    fix admin entry separately. Acceptance: 10-minute two-client loopback run,
    disconnect/reconnect recovery, bounded queues, clean shutdown, recorded
    machine and measured results. No invented player-capacity target.

M0 exit gate: headless server plus two standalone clients exchanging validated
inputs and replicated entity state, repeatable integration tests, bounded
resource use, and no required FiveM component. This gate is not yet achieved.
