# GameVerse development status

This file describes only functionality demonstrated by the active independent runtime. Historical prototypes under `core`, `services`, `admin-panel`, `ui`, Kubernetes and Terraform are research material and are not release claims.

## Demonstrated

- Versioned M0/M1/M2 protocols, QUIC transport, bounded queues, replication deltas, interest management and reconnect generations.
- A Windows GTA V Enhanced adapter prototype with session bootstrap and remote pedestrian presentation.
- A PostgreSQL-backed closed-alpha RP path covering invites, accounts, sessions, characters, wallets, inventory, a shop and courier work.
- Static FiveM resource analysis plus an isolated Lua compatibility harness for GameVerse-owned MIT fixtures.
- A single compact native Windows launcher owns the text interface, the bounded M2 UI pipe, bridge startup, and the one-shot GTA launch gate. WebView2 is no longer part of the alpha client.
- A PostgreSQL acceptance that drives the real UI protocol and fake adapter through the bridge and QUIC server from registration to reconnect.
- A reproducible self-contained Windows package, detached ECDSA update-manifest verification, and a separate symbols archive.
- A Docker Compose acceptance that verifies PostgreSQL migrations, seed content, readiness, version and Prometheus metrics.
- A 32-client, 20 Hz, 60-second CI smoke with clean disconnects and no reported client errors.

## Required before closed alpha

1. Atomic client update installation and rollback, using a protected release signing key.
2. One real GTA client plus a bot for locomotion and combat presentation acceptance.
3. Two real GTA clients sharing a vehicle with ownership migration.
4. Protected admin endpoints and a tested PostgreSQL backup restore.
5. A 32-client, 20 Hz, 30-minute release soak without stale entities or unbounded growth.

## Compatibility status

FiveM is used as an API reference. GameVerse does not embed the FiveM runtime. QBCore and ESX resources are analysis targets only; no broad runtime-compatibility percentage is claimed. Performance comparisons remain unverified until reproducible benchmarks are published.
