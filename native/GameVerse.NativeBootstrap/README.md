# GameVerse Native Bootstrap

This x64 C++20 ASI component runs before the managed GTA adapter. Its initial
milestone is intentionally telemetry-only: it verifies the exact supported GTA V
Enhanced executable, a signed compatibility manifest, and safe runtime state.
It refuses to install hooks when the executable is unknown, Online/BattlEye is
active, or world-loader signatures are absent or ambiguous.

Supported executable:

- edition/build: `Enhanced 1.0.1158.13`
- size: `56,064,632` bytes
- SHA-256: `0C52864D4521D9C9D441348AA1156958792DDE8825D0297C851753F167336401`

Build with Visual Studio 2022 x64 tools:

```powershell
cmake -S native/GameVerse.NativeBootstrap -B .build/native-bootstrap -A x64
cmake --build .build/native-bootstrap --config Release
ctest --test-dir .build/native-bootstrap -C Release --output-on-failure
```

The produced `GameVerse.NativeBootstrap.asi` and the two files under
`compatibility/` belong beside the existing ASI loader. The bridge must expose
`\\.\pipe\gameverse-bootstrap-v1` before GTA starts.

`world_loader` mode is rejected until independently verified patterns for this
exact executable are recorded, signed, uniquely matched, checked against the PE
executable section and covered by tests. Do not derive patterns from DRM,
entitlement, Social Club, GTA Online or anti-cheat functions.
