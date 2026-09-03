# GTA adapter IPC v1

Windows byte-mode named pipe `\\.\pipe\gameverse-gta-v1`; Rust is the pipe server.
Remote pipe clients are rejected. The pipe uses the Windows process default DACL
and a first-instance check; it is a local development interface, not an authentication
boundary against other applications running as the same Windows user.

Frames are a **4-byte unsigned big-endian byte length**, then strict UTF-8 JSON.
Length is 1..65536 bytes. No BOM or newline is required. Partial reads are assembled;
truncated and oversized frames disconnect the session. `type` is a snake_case tag.
Game hashes are unsigned 32-bit integers (C# casts native signed hashes unchecked).
IDs and timestamps are integer JSON numbers, not strings. Quaternion order is XYZW.

## Connection and lifecycle

1. Adapter sends `adapter_hello` with `version:1`, `backend:"shvdne-1.1.0.6"`.
2. Adapter sends `game_info` with `edition:"enhanced"`, `build:"1.0.1158.13"`.
3. Rust validates both within five seconds and connects to the presence server.
4. Rust sends `session_begin` with `session` and `entity:{slot,generation}`.
   The adapter deletes its old remote peds before accepting this session.
5. Adapter sends `local_player_state` at up to 20 Hz, containing `sequence` and `state`.
   It never chooses a network entity ID. The server assigns state to its session owner.
6. Rust sends `remote_entity_create` / `remote_entity_update`, each containing
   `entity:{id:{slot,generation},state:{...}}`; disappearance sends
   `remote_entity_destroy` with `id:{slot,generation}`. The local player is excluded.
7. Adapter reports actual in-game creation/destruction with `adapter_status`, `event`
   (`remote_ped_created`, `remote_ped_destroyed`, `remote_model_invalid`,
   `remote_model_timeout`, `session_ready`) and nullable `id`.

Both sides send `adapter_heartbeat` with `game_ready`. Adapter heartbeat is 1 Hz;
bridge heartbeat is currently 20 Hz. Five seconds without an incoming pipe frame,
an overflowing queue, or a write timeout disconnects the pipe. The C# worker
discards queued commands and schedules a local `reset`, then retries after one
second while the game is ready. Paused/unavailable game sends `game_ready:false`;
the bridge closes the network session. Cleanup executes on the next script tick.
An aborted script also deletes only the peds it owns.
If the game callback stops supplying samples for two seconds, the worker marks
the game unavailable rather than indefinitely retransmitting its last pose.

`adapter_error` (`code`, `message`) and bridge `reset` (`reason`) are reserved/accepted
diagnostic messages. Normal EOF already triggers cleanup; shutdown does not depend
on receiving a final reset. Error strings are bounded, and no arbitrary native/RPC,
file operation or server endpoint is exposed to the GTA script.

## State

```json
{"timestamp_ms":1000,"position":[100.0,100.0,30.0],"rotation":[0.0,0.0,0.0,1.0],"velocity":[0.0,0.0,0.0],"model_hash":1885233650,"health":200,"armor":0,"movement":1,"weapon_hash":2725352035}
```

- Timestamp: monotonic milliseconds since the adapter started, not wall-clock time.
- Sequence: starts at 1 per pipe connection; duplicate/older sequence is discarded.
- Position: GTA world units, XYZ, finite and within +/-20000 per axis.
- Velocity: units/second, finite and within +/-500 per axis.
- Rotation: finite unit quaternion, squared length tolerance 0.02.
- Model: nonzero hash; the game host additionally checks that it names a ped model.
- Health and armor: 0..1000. Captured/replicated, not applied as combat rules in M1.
- Movement bits: on-foot=1, running=2, sprinting=4, jumping=8, ragdoll=16, aiming=32.
  Flags and weapon hash are captured for later animation work, not played in M1.

## Network boundary

Presence v1 uses the existing QUIC endpoint configuration, reliable bidirectional
stream and bounded framing. It runs on **127.0.0.1:30121**. Original M0 stays on
30120, version 0, with its unchanged 2D input/state API. The two versions reject
each other's handshake. M1 snapshots contain only players that have published a
valid pose; there is no dummy entity at world origin during loading.
An unspawned client sends network `heartbeat` once a second while waiting for GTA;
it does not send a fake position just to keep the connection alive.

The server owns IDs/generations/lifecycle, validates poses, rejects timestamp
regression and broadcasts full snapshots at 10 Hz. **Clients own physical poses**;
this does not claim server-authoritative GTA physics or protection against forged
but numerically valid motion. Capacity remains two players for this milestone.

The Rust bridge retains two snapshots and renders one snapshot interval behind
using position lerp and shortest-arc quaternion slerp. Corrections over 10 units
snap; no extrapolation runs after the newest snapshot. A new generation or model
does not interpolate from the old entity. C# uses kinematic, non-colliding peds so
local GTA physics cannot fight this presentation. Locomotion animations are M1.2.

`hello-v1.frame` is the shared framing fixture checked by Rust and the C# self-test.
