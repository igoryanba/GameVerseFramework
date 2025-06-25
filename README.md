# 🎮 GameVerse Framework

> Next-generation modding platform designed to outperform FiveM and bring GTA V & Red Dead Redemption 2 multiplayer to the WebAssembly era.

[![Stars](https://img.shields.io/github/stars/igoryanba/GameVerseFramework)](https://github.com/igoryanba/GameVerseFramework/stargazers)  
[![Forks](https://img.shields.io/github/forks/igoryanba/GameVerseFramework)](https://github.com/igoryanba/GameVerseFramework/network)  
[![Issues](https://img.shields.io/github/issues/igoryanba/GameVerseFramework)](https://github.com/igoryanba/GameVerseFramework/issues)  
[![License](https://img.shields.io/github/license/igoryanba/GameVerseFramework)](LICENSE)

**English | [Русский version](README_ru.md)**

---

## Why GameVerse?

* **Micro-service architecture** – resilient & horizontally scalable
* **WebAssembly UI runtime** – 5× less memory, 10× lower latency than CEF
* **Dynamic plugin loading** – C++, Rust, Go, Zig with hot-reload
* **HTTP/3 + QUIC network stack** for ultra-low latency
* **FiveM compatibility layer** – run existing QBCore / ESX resources
* **Powerful CLI** with JWT-secured admin API

## At a Glance

| Metric               | FiveM          | GameVerse       | Improvement |
|----------------------|----------------|-----------------|-------------|
| UI memory footprint  | ~2 GB          | **400 MB**      | **5×**      |
| UI latency           | 50-100 ms      | **5-15 ms**     | **10×**     |
| Script performance   | 1× (V8)        | **10-50× (WASM)**| Up to 50×   |
| Hot-reload time      | 30-60 s        | **< 0.2 s**     | **150×**    |

## Quick Start

```bash
# 1. Build & launch the server in the background
cargo build --package gameverse_server --release
gameverse server start --background

# 2. Check server status
gameverse server status

# 3. Stream live logs
gameverse server logs -f
```

> Detailed installation and development guides live in the `docs/` directory.

## Documentation

* **English** – work-in-progress in [`docs/en`](docs/en)
* **Русский** – primary source in [`docs/`](docs)

## Contributing

We ♥ contributions!  Please read the [contributing guide](.github/CONTRIBUTING.md) and look for issues labelled `good first issue`.

## License

GameVerse Framework is released under the MIT License. See the [LICENSE](LICENSE) file for details.
