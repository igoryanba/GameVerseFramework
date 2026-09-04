# GameVerse UI

The closed-alpha Windows interface is a separate .NET 8 WinForms process hosting local HTML/CSS/JavaScript in WebView2. Russian is the initial locale.

Security boundaries:

- the page is served from the fixed `https://gameverse.local` virtual origin;
- navigation, popups, downloads, context menus and developer tools are disabled;
- every message requires schema version 1, a bounded request ID, an allowlisted command and an object payload;
- messages over 64 KiB are rejected;
- the web page has no filesystem, process, database or arbitrary network API;
- the server remains authoritative for character, inventory and economy data.
- the native host connects to the Rust bridge through `\\.\pipe\gameverse-ui-v1`;
- refresh tokens are encrypted for the current Windows user with DPAPI and are never returned to page JavaScript.

Run `GameVerse.UI.exe --self-test` for the message-boundary and packaged-assets smoke test. Normal startup prints a JSON `ui_ready` event after WebView2 has initialized so the launcher can advance without a fixed delay.
