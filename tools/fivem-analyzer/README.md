# FiveM resource analyzer

This tool inventories a resource without executing its Lua or JavaScript. It
recognizes `fxmanifest.lua` and `__resource.lua`, scripts, dependencies, events,
callbacks, exports, native calls, NUI, SQL files and common RP framework markers.

```text
cargo run --manifest-path tools/fivem-analyzer/Cargo.toml -- \
  --path path/to/resource --format json \
  --gameverse-manifest path/to/gameverse.toml \
  --required-capabilities path/to/required-capabilities.json \
  --unknown-natives path/to/unknown-natives.json \
  --license-report path/to/license-report.json
```

The report assigns concrete findings to `supported`, `convertible`, `manual` or
`blocked`. The generated manifest is only a structural conversion; `manual` and
`blocked` findings still require implementation or review.

Capability entries contain source-file and line evidence when it is statically
available. Network events remain `convertible` until the resource cluster is
connected to M2. NUI, state bags, asset mounting, JavaScript and C# are not
reported as supported merely because the scanner recognizes their syntax.
