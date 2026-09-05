# Source and dependency provenance

GameVerse's new adapter implementation is original code. No GTA Coop, FiveM,
OpenVHook or GTA Network source has been copied into it.

- **SHVDNE v1.1.0.6**, external host/API, zlib project license plus its bundled
  third-party notices: https://github.com/Chiheb-Bacha/ScriptHookVDotNetEnhanced
  ZIP SHA256: `F10DA8819FA6814FB0A04553567D2516934EAC96B16AF06AD8F315F0DA00473A`.
  Keep the archive's `Licenses/` when distributing a permitted host bundle.
- **ScriptHookV 3889.0 / 1158.13**, external binary dependency and SDK:
  https://www.dev-c.com/gtav/scripthookv/ . Not vendored or reimplemented here.
  Local downloaded ZIP SHA256:
  `B64C97C3353906F14621E7E9511E4AEC2A7D436ECC21ED124D3816585E2E6188`.
  Download/install according to the publisher's terms; its binary license is
  independent of the framework's source license.
- **Newtonsoft.Json 13.0.3**, MIT NuGet dependency.
- **Microsoft.NETFramework.ReferenceAssemblies.net48 1.0.3**, build-only reference
  assemblies obtained from NuGet; not game/runtime code.
- **MinHook v1.3.4**, pinned revision
  `c3fcafdc10146beb5919319d0683e44e3c30d537`, BSD-2-Clause with bundled HDE
  notices. It is statically linked into the optional native bootstrap. Full
  notices are in `native/GameVerse.NativeBootstrap/THIRD_PARTY_MINHOOK.txt`.

Behavioral references reviewed:
- RAGECOOP MIT `SyncedPed`, `SyncedVehicle`, `SyncedEntity`:
  https://github.com/RAGECOOP/RAGECOOP-V . Model loading, ownership separation,
  entity cleanup and smoothing informed the design. M1 implements a narrower
  kinematic presentation and does not import RAGECOOP networking, IDs or runtime.
- GTA Network Platform MIT interpolation, on-foot sync, streamer and resource API:
  https://github.com/GTANetworkDev/platform . It is an old secondary behavioral
  reference and is not vendored.
- GTA Coop releases were used only to research Enhanced host compatibility:
  https://github.com/oldnapalm/GTACoop/releases . No GPL source copied.

Any future source extraction must record the exact upstream revision, files,
copyright notices and license in this file before merging. A repository's root
license does not override the licenses of its bundled third-party components.
Exact reviewed revisions are recorded in `docs/UPSTREAM_PROVENANCE.md`.
