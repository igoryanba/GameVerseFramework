# GameVerse Windows launcher

The launcher now owns server selection and M2 preflight. It displays the
directory, authenticates or resumes the account, selects the last character,
and starts GTA only after the server returns a reserved session. The ordinary
Play button is disabled while the installed compatibility manifest remains in
`telemetry_only`; this prevents accidentally dropping a player into Story Mode.

The supported production sequence is:

`server → auth/resume → character → reserved → PlayGTAV → native world loader → adapter → SpawnAck`

Manual Story and telemetry traces are developer-only modes.

This is the single visible Windows client for the closed alpha. It provides a compact native text interface, checks GTA V Enhanced, the M2 bridge, the pinned server certificate, ScriptHook components, the adapter, and available memory. It owns the bridge and launches `PlayGTAV.exe` exactly once after readiness checks. A named mutex restores the existing window when the executable is started again. On exit it cleans up only processes created by that launcher instance.

The normal `start` command requires 4 GiB of free physical memory to reduce GTA DirectX out-of-memory failures. This is a conservative launch guard rather than a protocol requirement. A tester with a configured pagefile can explicitly run `start --allow-low-memory`; the launcher records the override and available memory while continuing to enforce every other installation and security check.

```powershell
GameVerse.Launcher.exe init
# edit launcher.json
GameVerse.Launcher.exe verify
GameVerse.Launcher.exe start
GameVerse.Launcher.exe update
GameVerse.Launcher.exe diagnostics
```

The normal window accepts `help`, `login`, `register`, `resume`, `characters`, `create`, `play`, `status`, `chat`, `inventory`, `shop`, `buy`, `job start`, `job finish`, `logout`, `reconnect`, `clear`, and `exit`. `--ui-only` runs the window without starting the bridge or GTA for visual acceptance tests. `--attach-existing` is a developer-only opt-in; normal startup refuses to attach to an existing GTA process.

UI commands reach the M2 control stream through the local bridge, and refresh tokens are stored by the UI host with DPAPI. The package builder creates a detached ECDSA P-256/SHA-256 update manifest and embeds only the public verification key when a release signing key and HTTPS base URL are supplied. The launcher verifies the exact manifest bytes, downloads each file into a same-volume staging directory, checks sizes and hashes, preserves local configuration, and atomically switches directories. It retains one previous version and restores it when the new launcher's self-test fails or times out.
