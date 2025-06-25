# GameVerse CLI Tools

> English | [Русский](README_ru.md)

Comprehensive command-line toolkit for building, testing and shipping GameVerse plugins. Delivers a modern developer experience that surpasses legacy FiveM tooling.

## ✨ Key Features

| Category            | Highlights |
|---------------------|------------|
| Modern Architecture | Async-first (Tokio), type-safe Rust, rich interactive UX |
| Plugin Management   | Template engine, multi-language support (Rust/TS/Lua), hot reload, cross-compilation |
| Marketplace         | Discover, publish, version and update plugins from CLI |

## 🚀 Quick Install
```bash
cargo install --path GameVerseFramework/sdk/cli-tools
```

## 📚 Usage Snapshot
```bash
# New plugin from template
gameverse plugin new my-economy --template economy --language rust

# Build & test
gameverse plugin build --cross-compile all
gameverse plugin test --integration
```

For full command reference: `gameverse --help` or see the Russian docs. 