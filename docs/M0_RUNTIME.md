# Independent Multiplayer Runtime M0

M0 provides a headless authoritative server and standalone client harnesses.
It does not launch GTA or implement an in-game adapter. No FiveM installation,
game process, database, launcher, scripting engine or admin service is required.

## Build and run

Use the pinned Rust 1.98.0 toolchain and Node 20 or later for the process harness.
Windows requires a matching native compiler toolchain (MSVC build tools for the
normal Windows Rust installation); Linux requires a C compiler. The validated
local environment used Windows GNU Rust and LLVM-MinGW, with Rust's self-contained
link libraries enabled. This local tool setup is not required by the source tree.

From the repository root:

```sh
cargo build --locked
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
node scripts/check-m0-boundary.mjs
node scripts/m0-smoke.mjs --seconds 600 --output m0-report
```

The default workspace members are the six new packages. `cargo test --workspace`
also includes legacy core and CLI and currently fails in the old core.
Microservices and standalone generators remain separate packages; they are
explicitly excluded from the root workspace and are not covered by M0 tests.

The process harness builds nothing itself. It uses `CARGO_TARGET_DIR/debug`, or
`target/debug` when that variable is absent; `--bin <directory>` overrides this.
Use a new output directory for each run: existing identity files are never replaced.
It creates a local TLS identity, starts one server and two clients, reconnects
one client halfway through, compares their state before disconnect, checks queue
limits and verifies clean shutdown. JSON reports and process logs remain in the
output directory. The private TLS key is local test material and must not be
included in shared artifacts; CI uploads only JSON/logs.

To run manually, create a directory for the local identity, then:

```sh
cargo run --locked -p gameverse-server -- --init-identity --cert .m0/localhost.der --key .m0/localhost.key
cargo run --locked -p gameverse-server -- --cert .m0/localhost.der --key .m0/localhost.key
cargo run --locked -p gameverse-client -- --cert .m0/localhost.der --dx 1 --dy 0 --duration 30
```

Run a second client in another terminal with `--dx 0 --dy 1`. Default server address
is `127.0.0.1:30120`. M0 deliberately accepts only loopback addresses. Certificates
are trusted explicitly through `--cert`; hostname and certificate verification
remain enabled. Server Ctrl+C closes sessions; clients also support Ctrl+C and a
fixed duration. `--reconnect-after <seconds>` performs one fresh session handshake.

## Architecture and public contracts

| Package | Responsibility |
| --- | --- |
| gameverse-protocol | Message, Snapshot, EntityId, SessionId, Tick; validated bounded codec |
| gameverse-runtime | World connect/input/step/snapshot/disconnect, no I/O |
| gameverse-transport | QUIC endpoint construction, explicit certificate trust, bounded frame I/O |
| gameverse-server | Session admission, bounded input queues, authoritative tick and snapshots |
| gameverse-client | Client connection, Replica, GameAdapter, MemoryAdapter and CLI harness |
| gameverse-compat-fivem | Optional local NativeHost/EventBus adapter; no game process dependency |

Protocol and runtime have no dependency on core or compatibility. The dependency
check walks the resolved normal/build graph and fails if either is introduced.
The old `gameverse_core::fcl::FiveMCompat` constructor delegates through injected
legacy hosts to the independent adapter; the `fivem-compat` feature preserves the
legacy default. Disabling that feature does not remove old game integration.

The independent adapter requires explicit NativeHost and EventBus implementations.
Unsupported natives and invalid player/entity handles return errors. Adapter
tests use an explicit test backend; they do not claim QBCore/ESX compatibility.
Legacy bridge events retain the untagged `args` field for existing raw JSON
consumers and accept payloads from old producers. Additional typed metadata
preserves variants between new adapters. This local data is not the multiplayer
protocol.

## Protocol and runtime behavior

- Version 0 uses UTF-8 JSON with a four-byte big-endian length prefix. Frames must
  be 1..65536 bytes; length is checked before allocating the payload buffer.
  Unknown variants/fields, invalid directions and non-finite coordinates fail.
- One reliable bidirectional QUIC stream per connection carries Hello, Welcome,
  Reject, Input, Snapshot and Disconnect. Exact protocol version match is required.
  Handshake deadline is five seconds; client application inactivity is fifteen.
- Two admitted clients and at most sixteen active admission/session tasks.
  Each client's input queue holds at most 128 messages; overflow closes the session.
  QUIC flow-control windows are bounded; outbound snapshots use a latest-value
  channel, so slow clients cannot create an unbounded snapshot queue.
- World steps every 50 ms. Missed wall-clock ticks are skipped, not replayed as an
  unbounded catch-up burst. Each accepted input sets a direction until changed;
  diagonal input is normalized and speed is limited to five units per second.
  Ownership comes from the connection; inputs contain no target entity ID.
- Sequences start at one; old/duplicate input is discarded. Reconnect allocates a
  new session and increments the reused entity slot's generation; it does not
  restore the old player's position or identity.
- Full snapshots every 100 ms include tick, that client's acknowledged input and
  all visible entities (both clients in M0). Missing entities are removed. Replica
  ignores old/duplicate snapshots and keeps interpolation separate from truth.
- Snapshots ride the reliable stream in M0. Loss/reordering behavior is exercised
  by the replica test's deterministic delivery schedule, not by claiming datagram
  loss on reliable QUIC. Delta compression and interest management are deferred.

## Validation recorded on 2026-09-03

Windows GNU build, 13 tests, strict clippy and dependency boundary checks passed.
The actual 600-second separate-process run passed with:

| Metric | Result |
| --- | --- |
| Server lifetime | 603 seconds (includes shutdown grace) |
| Server ticks | 12,060 |
| Snapshots sent, excluding initial snapshots | 11,996 |
| Accepted sessions / disconnects | 3 / 3 |
| Maximum observed input queue depth | 2 / 128 |
| Players after shutdown | 0 |
| Client state convergence / reconnect / clean shutdown | Passed |

The test suite covers codec validation, deterministic movement and ownership,
entity generations, loss/reorder/duplicate snapshot handling, interpolation,
two-client QUIC convergence, third-client rejection, TLS trust, protocol version,
handshake timeout, application idle timeout and queue overflow.

Windows and Linux PR jobs are configured, with a separate failing legacy lane
that is not marked continue-on-error. Remote CI was not triggered; Linux results
are not claimed from the Windows run. Admin Vite build passed after connecting
the React entry point (829 modules, 528.61 kB JS; a chunk-size warning remains).
The admin panel still targets the old admin API, not M0 JSON stdout metrics.

## Legacy blockers and next stage

Both an unchanged checkout of e3545c9 and the current branch reproduced the same
33 core library compilation errors
after provisioning make/native tools. They include missing Windows imports,
temporary string lifetimes, an async Result/future mismatch, duplicate
get_process_handle definitions and Windows API Result/HANDLE mismatches.
The old game integration implementation was not changed in this M0 work.
Consequently the legacy FCL bridge tests cannot be certified until core compiles;
the independent FCL adapter tests pass separately.

Next: repair and review the legacy platform boundary separately, implement a
supported Windows GameAdapter, and add explicit resource lifecycle semantics.
GTA gameplay, public deployment, persistent accounts/worlds, DRM/entitlement
changes and protection bypass are outside this milestone.
