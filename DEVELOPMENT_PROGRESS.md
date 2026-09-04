# GameVerse development status

This file describes only functionality demonstrated by the active independent runtime. Historical prototypes under `core`, `services`, `admin-panel`, `ui`, Kubernetes and Terraform are research material and are not release claims.

## Demonstrated

- Versioned M0/M1/M2 protocols, QUIC transport, bounded queues, replication deltas, interest management and reconnect generations.
- A Windows GTA V Enhanced adapter prototype with session bootstrap and remote pedestrian presentation.
- A PostgreSQL-backed closed-alpha RP path covering invites, accounts, sessions, characters, wallets, inventory, a shop and courier work.
- Static FiveM resource analysis plus an isolated Lua compatibility harness for GameVerse-owned MIT fixtures.
- A Windows launcher and a separate WinForms/WebView2 UI connected to the M2 bridge through a bounded local protocol.

## Required before closed alpha

1. One end-to-end UI, bridge, QUIC server, PostgreSQL and fake-adapter acceptance test.
2. A reproducible signed Windows client package with update and rollback verification.
3. One real GTA client plus a bot for locomotion and combat presentation acceptance.
4. Two real GTA clients sharing a vehicle with ownership migration.
5. Protected health/admin endpoints, reproducible Linux deployment and backup restore.
6. A 32-client, 20 Hz, 30-minute release soak without stale entities or unbounded growth.

## Compatibility status

FiveM is used as an API reference. GameVerse does not embed the FiveM runtime. QBCore and ESX resources are analysis targets only; no broad runtime-compatibility percentage is claimed. Performance comparisons remain unverified until reproducible benchmarks are published.
