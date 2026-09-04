# GameVerse resource import

GameVerse treats FiveM manifests and public API names as an import format. It does not embed or launch the FiveM runtime.

`gameverse-resource-manifest` parses `fxmanifest.lua` and `__resource.lua` as static data and emits `gameverse.toml`. It resolves globs inside the resource directory and rejects absolute paths, parent traversal, dependency cycles, and symlink escapes. A manifest that contains executable Lua constructs is reported as blocked and is never evaluated by the analyzer.

`gameverse-resource-runtime` is an optional compatibility boundary outside the protocol and replication core. It creates one Lua 5.4 VM for each resource and side. The default limits are 16 MiB memory, 100,000 VM instructions per dispatch, 64 KiB event payload, 128 handlers, 128 callbacks, a 256-message queue, and a 5-second callback deadline. File, process, module, and network libraries are removed. Native calls go through an explicit allowlist and unknown calls fail with `UnsupportedNative`.

The included `fixtures/resources/compat-basic` resource is MIT-licensed and is the only executable acceptance resource. It covers load order, lifecycle, client/server events, callbacks, exports, threads, timers, an allowed native boundary, and cleanup on stop.

```text
fivem-analyzer --path fixtures/resources/compat-basic --format json --gameverse-manifest fixtures/resources/compat-basic/gameverse.toml
gameverse-resource-host validate --manifest fixtures/resources/compat-basic/gameverse.toml
gameverse-resource-host run --manifest fixtures/resources/compat-basic/gameverse.toml --side client
gameverse-resource-acceptance --manifest fixtures/resources/compat-basic/gameverse.toml --report resource-acceptance.json
```

Third-party canaries are fetched only into ignored `.research/resources` by `scripts/fetch-resource-canaries.ps1`. Each checkout is detached at its recorded revision and receives a metadata record with URL, commit, declared license, and a deterministic SHA256 over tracked file hashes. GPL/AGPL files remain outside committed GameVerse source and binaries.

NUI, SQL execution or migration, framework runtime APIs such as QBCore/ESX, and GTA rendering are explicit analyzer findings and are outside this milestone.
