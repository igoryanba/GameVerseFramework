# GameVerse Native Bootstrap

This x64 C++20 ASI component runs before the managed GTA adapter. Its initial
milestone is intentionally telemetry-only: it verifies the exact supported GTA V
Enhanced executable, a signed compatibility manifest, and safe runtime state.
It refuses to install hooks when the executable is unknown, Online/BattlEye is
active, or world-loader signatures are absent or ambiguous.

## Offline inspector

`gameverse-native-inspector` reads an executable from disk and never loads it.
It can inventory string locations by their query hash or generate a wildcard
signature from an independently selected RVA. RIP-relative displacements and
relative immediates are masked and the resulting pattern is counted across all
executable sections.

```text
gameverse-native-inspector --image GTA5_Enhanced.exe --string frontend
gameverse-native-inspector --image GTA5_Enhanced.exe --candidate-rva 0x1234 --length 32
```

Generated patterns are research candidates only. They cannot be added to the
signed compatibility manifest until two matching dynamic traces and an
observe-only call count confirm the same non-online initialization behavior.

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

Compatibility manifests are signed with ECDSA P-256 in fixed-field `R || S`
format. `tools/sign-manifest.ps1` requires the private key to live outside the
repository and regenerates both the detached `.sig` and embedded public-key
header. Release signing should use a protected CI secret or an offline key.
