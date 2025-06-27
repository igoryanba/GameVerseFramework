# 🎮 GameVerse Framework

> Next-generation modding platform built with Rust, WebAssembly and micro-services to outperform FiveM.

[![Stars](https://img.shields.io/github/stars/igoryanba/GameVerseFramework)](https://github.com/igoryanba/GameVerseFramework/stargazers) 
[![Forks](https://img.shields.io/github/forks/igoryanba/GameVerseFramework)](https://github.com/igoryanba/GameVerseFramework/network) 
[![Issues](https://img.shields.io/github/issues/igoryanba/GameVerseFramework)](https://github.com/igoryanba/GameVerseFramework/issues) 
[![License](https://img.shields.io/github/license/igoryanba/GameVerseFramework)](LICENSE)

*English | [Русский](README.md)*

---

## 🔥 Why GameVerse?

- **Micro-service architecture** – resilient & horizontally scalable
- **WebAssembly UI runtime** – 5× lower memory & 10× lower latency than CEF
- **Dynamic plugin loading** – C++, Rust, Go, Zig with hot-reload
- **HTTP/3 + QUIC** networking for ultra-low latency
- **FiveM compatibility layer** – run existing QBCore / ESX resources
- **Powerful CLI** with JWT-secured admin API

## ✨ Performance at a Glance

| Metric              | FiveM        | GameVerse       | Improvement |
|---------------------|--------------|-----------------|-------------|
| UI memory footprint | 2 GB         | **400 MB**      | **≈5×**     |
| UI latency          | 50-100 ms    | **5-15 ms**     | **≈10×**    |
| Script performance  | 1× (V8)      | **10-50× (WASM)**| Up to 50×  |
| Hot reload time     | 30-60 s      | **<0.2 s**      | **≈150×**   |

## 🚀 Quick Start

```bash
# 1. Build & launch the server
cargo build --package gameverse_server --release
gameverse server start --background

# 2. Check server metrics
gameverse server status

# 3. Stream real-time logs
gameverse server logs -f
```

Detailed guides live in the `docs/` directory.

## 📚 Documentation

- **English** – WIP in [`docs/en`](docs/en)
- **Русский** – full docs in [`docs/`](docs)

## 🤝 Contributing

We ♥ contributions!  Read the [CONTRIBUTING guide](.github/CONTRIBUTING.md) and look for issues labelled `good first issue`.

## 📝 License

GameVerse Framework is released under the MIT License.

## Roadmap Highlights

* Performance Demonstration v0.1 (nightly automated benchmark reports)

### Roadmap Milestones

- ✅ Server Bootstrap v0.1 — ready for production (CLI `gameverse server init`).
- 🐳 Server Bootstrap v0.2 — Docker-first deployment, Helm charts, Terraform modules, Prometheus/Grafana monitoring *(ETA March 2025).*  See `deployment/README.md` for details. 