# GameVerse Windows launcher

This is the deterministic launcher shell for the closed alpha. It checks GTA V Enhanced, WebView2 Runtime, GameVerse UI, the M2 bridge, the pinned server certificate, ScriptHook components, the adapter, and available memory. It starts the UI first, waits for explicit UI and bridge readiness events, and only then starts `PlayGTAV.exe`. It remains alive while GTA is running and cleans up only the UI/bridge processes it created. It can also open logs and create a diagnostics archive without configuration secrets.

The normal `start` command requires 4 GiB of free physical memory to reduce GTA DirectX out-of-memory failures. This is a conservative launch guard rather than a protocol requirement. A tester with a configured pagefile can explicitly run `start --allow-low-memory`; the launcher records the override and available memory while continuing to enforce every other installation and security check.

```powershell
GameVerse.Launcher.exe init
# edit launcher.json
GameVerse.Launcher.exe verify
GameVerse.Launcher.exe start
GameVerse.Launcher.exe update
GameVerse.Launcher.exe diagnostics
```

UI commands reach the M2 control stream through the local bridge, and refresh tokens are stored by the UI host with DPAPI. The package builder creates a detached ECDSA P-256/SHA-256 update manifest and embeds only the public verification key when a release signing key and HTTPS base URL are supplied. The launcher verifies the exact manifest bytes, downloads each file into a same-volume staging directory, checks sizes and hashes, preserves local configuration, and atomically switches directories. It retains one previous version and restores it when the new launcher's self-test fails or times out.
