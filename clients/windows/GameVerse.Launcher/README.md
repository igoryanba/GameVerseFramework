# GameVerse Windows launcher

This is the deterministic launcher shell for the closed alpha. It checks GTA V Enhanced, WebView2 Runtime, GameVerse UI, the M2 bridge, the pinned server certificate, ScriptHook components, the adapter, and available memory. It starts the UI first, waits for explicit UI and bridge readiness events, and only then starts `PlayGTAV.exe`. It remains alive while GTA is running and cleans up only the UI/bridge processes it created. It can also open logs and create a diagnostics archive without configuration secrets.

```powershell
GameVerse.Launcher.exe init
# edit launcher.json
GameVerse.Launcher.exe verify
GameVerse.Launcher.exe start
GameVerse.Launcher.exe diagnostics
```

The next launcher milestone connects UI commands to the M2 control stream, adds DPAPI token storage, a signed update manifest, rollback, and adapter-ready progress events.
