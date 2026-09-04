# GameVerse Windows launcher

This is the first deterministic launcher shell for the closed alpha. It checks GTA V Enhanced, the M2 bridge, the pinned server certificate, ScriptHook components, the adapter, and available memory. It starts the bridge before `PlayGTAV.exe`, opens logs, and creates a diagnostics archive without configuration secrets.

```powershell
GameVerse.Launcher.exe init
# edit launcher.json
GameVerse.Launcher.exe verify
GameVerse.Launcher.exe start
GameVerse.Launcher.exe diagnostics
```

The next launcher milestone adds the WebView2 sign-in/character UI, signed update manifest, DPAPI token storage, rollback, and adapter-ready progress events.
