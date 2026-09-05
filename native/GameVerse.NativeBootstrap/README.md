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
executable sections. String inventory also reports x64 RIP-relative instruction
RVAs that reference an exact match, providing the first independent candidate
chain without copying third-party signatures.

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

For a research trace, start the M2 bridge with `--telemetry-story`. The probe
captures the verified image and stable frontend, then samples only window and
module readiness every 250 ms while the user enters Story Mode manually. A full
section/module snapshot is written only when ScriptHook/SHVDN or the adapter
appears. Reports are bounded JSONL files under
`%LOCALAPPDATA%\GameVerse\telemetry`; they contain hashes and relative facts,
never memory contents or absolute addresses. A telemetry-only manifest does not
advertise the `world_loader` capability, so the bridge cannot request
`begin_world` accidentally.

When the full M2 server is not needed, the bounded research harness can capture
the same probe stream without opening a network connection:

```powershell
pwsh Research/run_native_telemetry.ps1 `
  -OutputPath .research/telemetry/manual-1.jsonl
```

It sends only `start_telemetry`, enforces the 64 KiB frame limit, validates the
hello and stops on `adapter_ready`, failure or timeout. Compare completed traces
before selecting a candidate:

```powershell
python Research/analyze_native_telemetry.py `
  .research/telemetry/manual-1.jsonl `
  .research/telemetry/manual-2.jsonl `
  .research/telemetry/control.jsonl
```

The candidate gate remains false until two adapter-ready traces and one
frontend-only control trace share the same executable fingerprint.
For an observe-only candidate, both adapter-ready traces must record a positive,
monotonic call-count delta while every frontend-only control records a zero
delta. The analyzer reports this separately as `observe_gate_satisfied`; a page
correlation alone never authorizes a behavioral hook.

After a candidate passes that correlation, the probe inventories direct relative
call sites in the loaded executable and reports only owning-function RVAs,
call-site counts and entry hashes. This read-only caller inventory narrows the
one-shot transition initiator without exposing process addresses or code bytes
and without installing additional hooks.

`world_loader` mode is rejected until independently verified patterns for this
exact executable are recorded, signed, uniquely matched, checked against the PE
executable section and covered by tests. Do not derive patterns from DRM,
entitlement, Social Club, GTA Online or anti-cheat functions.

Compatibility manifests are signed with ECDSA P-256 in fixed-field `R || S`
format. `tools/sign-manifest.ps1` requires the private key to live outside the
repository and regenerates both the detached `.sig` and embedded public-key
header. Release signing should use a protected CI secret or an offline key.
