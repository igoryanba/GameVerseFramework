# M1 — First GTA Presence

M1 adds an Enhanced C# game adapter and named-pipe bridge to the independent Rust
runtime. It preserves M0 and its QUIC transport configuration. The presence server
is a second executable using version 1 messages; M0 remains version 0.

```
GTA Enhanced 1.0.1158.13
  ScriptHookV -> SHVDNE 1.1.0.6 -> GameVerse.GtaAdapter
  <-> local named pipe <-> gameverse-gta-bridge
  <-> existing QUIC transport <-> gameverse-presence-server
  <-> gameverse-presence-bot
```

## Build

Windows x64, .NET 8 SDK for building, .NET Framework 4.8 runtime for the GTA host,
and the repository's Rust toolchain. The C# project downloads .NET Framework
reference assemblies through NuGet; Visual Studio installation is not required.

```powershell
cargo build --locked -p gameverse-server -p gameverse-client
./adapters/gta5/setup.ps1
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
```

`setup.ps1` fetches the pinned SHVDNE archive with SHA256 verification, compiles
the adapter and harness, and runs the C# framing tests. Nothing is installed into
GTA by setup. Alternatively pass `-p:ShvdnePath=<extracted-host-path>` to dotnet build.
The GTA DLL is `adapters/gta5/GameVerse.GtaAdapter/bin/Release/net48/GameVerse.GtaAdapter.dll`.

## Local synthetic acceptance (no GTA required)

```powershell
./scripts/m1-smoke.ps1 -Seconds 30
./scripts/m1-smoke.ps1 -Seconds 1800 -Output .m1/soak
```

Pass `-Dotnet <dotnet.exe>` and `-TargetDir <cargo-target>` for portable toolchains.
This runs separate C# harness, Rust bridge, server and bot processes using a unique
named pipe and ephemeral TLS identity. It checks bidirectional 3D state, adapter
reconnect, entity create/update/destroy and zero server players after shutdown.
Reports explicitly say `gta_loaded:false`: synthetic success cannot prove a GTA
ped was rendered. Private TLS keys stay in ignored `.m1/`, outside report outputs.

## GTA installation and G1–G3

First verify that the installed game starts normally and reaches Story Mode.
Download ScriptHookV for the target version from the publisher; extract separately.
Close GTA, then install only the selected host and adapter files:

```powershell
./adapters/gta5/install.ps1 -GamePath '<GTA directory>' -ScriptHookPath '<extracted ScriptHookV>'
```

The installer verifies the executable version and refuses to replace existing
mods. It records hashes of added files in `GameVerse.install.json`. It does not
copy `args.txt`, install NativeTrainer, replace game executables, change launcher
arguments, change anti-cheat settings or alter entitlement checks. If the normal
game/host cannot start under those constraints, record the blocker rather than
modifying those systems.

In separate terminals, using the binaries from your Cargo target directory:

```powershell
New-Item -ItemType Directory -Force .m1/game
gameverse-presence-server --cert .m1/game/cert.der --key .m1/game/key.der --init-identity
gameverse-presence-server --cert .m1/game/cert.der --key .m1/game/key.der
gameverse-gta-bridge --cert .m1/game/cert.der --duration 2100
gameverse-presence-bot --cert .m1/game/cert.der --duration 1800 --report .m1/game/bot.json
```

Start GTA and enter Story Mode. The adapter automatically connects when a valid
local ped exists. The bot waits for that real pose and anchors its circular path
near it, avoiding an arbitrary remote world coordinate. Remain near the initial
position to observe the remote ped; move/turn locally for the reverse-direction test.

- **G1:** game log contains `GTA_ADAPTER_LOADED=true`, supported build, and
  `IPC_CONNECTED=true`; Rust logs `local_player_state_received` with actual XYZ.
- **G2:** adapter reports `remote_ped_created`; visually verify that ped in GTA,
  changing position and rotation. A bridge create command alone is not evidence.
- **G3:** move the real local player; compare its poses with `real_player_observed`
  in the bot log. Confirm the actual local input caused the changes.
- Disconnect the bot: ped disappears. Reconnect: new generation creates a new ped.
  Restart the bridge: old ped is removed and a fresh network session is created.
- Run 30 minutes in GTA and retain host/adapter/bridge/server/bot logs plus visual
  evidence. Label each check separately; do not infer visual success from unit tests.

The adapter log is `GameVerse.GtaAdapter.log` under the host's application base
directory; SHVDNE has its own game-root log. Game callbacks own all GTA natives.
The worker never invokes GTA APIs; invalid model and load timeout are reported.

## Current boundaries and next milestones

This milestone supports two local presence clients, ped model/transform, render
interpolation, state capture and cleanup/reconnect. Remote presentation is frozen,
non-colliding and kinematic; it does not yet play walking or combat animations.
Health/armor/movement/weapon fields are captured and relayed, not gameplay authority.
Poses are client-owned; server validation is not authoritative GTA physics.

M1.1 adds two real Windows machines and explicit LAN/TLS configuration. M1.2 adds
animation/death/aiming behavior. M1.3 adds vehicles and ownership migration. A
replaceable native GameHost comes later. No FiveM dependency, vehicles, RP, voice,
CEF, memory patching or low-level loader replacement is added here. The 33 known
legacy-core build errors are separate debt and are not changed by this feature.

On this development machine, launching with the game's working directory resolved
an initial `Could not find socialclub.dll!` dialog. ScriptHookV recognized the game
build and registered SHVDNE, but the game did not reach Story Mode or the adapter.
A control launch with all nine newly added host/adapter files removed also exited
before Story Mode, with code **4919**; the root cause is unconfirmed. The added
files were left outside the game directory after this check. G1–G3 and an in-game
30-minute soak remain **blocked** until a normal Story Mode launch is available.
See output acceptance reports for independently verified synthetic results.
