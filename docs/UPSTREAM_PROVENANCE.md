# Upstream provenance for the independent GTA adapter

External repositories are research inputs, not runtime dependencies. The fetcher
places them under ignored `.research/upstreams`. Derived source committed to
GameVerse must retain the applicable copyright and license notice.

| Project | Pinned revision | License | Reviewed areas | GameVerse use |
|---|---|---|---|---|
| RAGECOOP-V | `fcd7e18d9b14c7cda95783e5a7ade4b4a20f97d2` | MIT | ped/vehicle sync and entity ownership | Behavioral reference for locomotion and future vehicle ownership |
| GTA Network Platform | `f0ee9f391a0ec9a557c32df549baa6cde4ba0f53` | MIT | interpolation, on-foot sync, streamer and resource API | Secondary reference; old APIs are not copied wholesale |
| CitizenFX/FiveM | public documentation only | mixed/Creator Platform terms | resource manifest, events, exports and networking concepts | Compatibility specification only; no source copied |
| QBCore, Qbox, ESX, GTA Coop | no vendored revision | GPL or project-specific | resource shapes used by opt-in fixtures | No source in GameVerse executables |
| MinHook | `c3fcafdc10146beb5919319d0683e44e3c30d537` (`v1.3.4`) | BSD-2-Clause plus bundled HDE notices | Native hook lifecycle | Pinned build dependency of NativeBootstrap; no hook is installed in telemetry mode |
| StraightToStoryMode | `e2a5528d0a28b5fa47f1ccf19ed31a44494cc650` | GPL-3.0 | Evidence that Enhanced landing-page bypass needs build-specific memory changes | Research only; no source, patch bytes or signatures copied into GameVerse |

The M1.2 locomotion code is a new adapter-specific implementation. It uses the
already transmitted GameVerse state and GTA native tasks; it does not copy a
complete upstream source file.
