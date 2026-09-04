# GameVerse Framework

GameVerse is an experimental independent multiplayer runtime for a locally installed GTA V Enhanced client. The active implementation is a closed-alpha prototype for up to 32 players. It does not modify GTA DRM, entitlement, GTA Online, or anti-cheat components.

## What works in this branch

- Independent Rust M0/M1 protocol, runtime, QUIC transport, headless server, and clients.
- Presence v2 component model, spatial interest management, baselines, deltas, stream-out, destroy, reconnect generations, and bounded frames.
- Separate M2 server and client entrypoints with version/capability negotiation, reliable session bootstrap, and realtime QUIC datagrams.
- Windows M2 bridge that translates the current GTA adapter IPC into Presence v2 while the tested v1 path remains available.
- GTA Enhanced adapter prototype with session bootstrap and remote ped locomotion states. The adapter supports up to 31 remote players; two-client GTA and vehicle acceptance are still pending.
- Static FiveM resource analyzer, safe manifest conversion, sandboxed Lua host, dependency ordering, exports, callbacks, timers, rollback, and two MIT acceptance resources.
- Server-authoritative closed-alpha RP domain for invites, characters, positions, wallets, immutable ledger entries, inventory, shops, courier work, and moderation.
- PostgreSQL schema for the RP vertical slice and a minimal Docker Compose deployment for the M2 server plus PostgreSQL.
- First Windows launcher shell for installation checks, ordered bridge/game startup, logs, and a redacted diagnostics archive.

The legacy `core`, old services, admin panel, Kubernetes/Terraform files, and historical performance claims are retained as research material. They are not part of the supported alpha runtime or its CI gate.

## Repository map

| Path | Purpose |
|---|---|
| `crates/protocol` | Versioned control, Presence v1/v2, adapter, and vehicle messages |
| `crates/runtime` | Server identity, replication, interest grid, baselines, ownership |
| `crates/transport` | QUIC endpoints, certificate trust, bounded streams/datagrams |
| `crates/server` | M1 and separate M2 headless server binaries |
| `crates/client` | Test clients, M2 bot, and Windows GTA bridge |
| `crates/resource-manifest` | Safe FiveM manifest parsing and `gameverse.toml` generation |
| `crates/resource-runtime` | Sandboxed per-resource Lua runtime and resource graph |
| `crates/gameverse-rp` | Closed-alpha gameplay rules and PostgreSQL migration |
| `tools/fivem-analyzer` | Static compatibility report CLI |
| `adapters/gta5` | GTA V Enhanced ScriptHookVDotNet adapter and protocol harness |
| `clients/windows/GameVerse.Launcher` | Windows launcher shell |
| `deployment/alpha` | Supported alpha Docker Compose stack |

## Build and test

The workspace toolchain is pinned in `rust-toolchain.toml`.

```sh
cargo test --locked
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

Build the M2 binaries:

```sh
cargo build --locked --release -p gameverse-server --bin gameverse-presence-server-m2
cargo build --locked --release -p gameverse-client --bin gameverse-presence-bot-m2
```

Analyze and validate a resource without executing its manifest:

```sh
cargo run --manifest-path tools/fivem-analyzer/Cargo.toml -- --path fixtures/resources/compat-basic --format json
cargo run -p gameverse-resource-runtime --bin gameverse-resource-host -- validate --manifest fixtures/resources/compat-basic/gameverse.toml
```

Build the Windows adapter and protocol harness:

```powershell
./adapters/gta5/setup.ps1
dotnet build clients/windows/GameVerse.Launcher -c Release
```

Run the development server and PostgreSQL on Linux:

```sh
cd deployment/alpha
cp .env.example .env
# set a private database password
docker compose up --build -d
```

## Alpha release gate

The branch is ready for a closed alpha only after all of these pass:

1. Windows/Linux resource import and sandbox security tests.
2. Presence v2 two-client integration plus 32-bot, 20 Hz, 30-minute soak test.
3. One GTA client plus bot movement/combat presentation acceptance.
4. Two real GTA clients and one shared vehicle with ownership migration.
5. PostgreSQL-backed invite → account → character → delivery → purchase → reconnect acceptance.
6. Clean launcher install, signed update/rollback, and diagnostic package acceptance.
7. Protected admin API with audit log and reproducible Linux deployment.

Position-based voice is the next required milestone after the base alpha.

## External code policy

FiveM is treated as an API compatibility specification. FiveM runtime code and GPL/AGPL RP resources are not linked into GameVerse Core. External canary resources remain in ignored research storage. Any imported code must have a compatible license, a pinned revision, and preserved attribution.

See `docs/M0_RUNTIME.md`, `docs/M1_GTA_PRESENCE.md`, and `docs/UPSTREAM_PROVENANCE.md` for implementation notes and recorded external research.
