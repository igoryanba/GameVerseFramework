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

## 🛠️ Installation from Source
```bash
cd GameVerseFramework/sdk/cli-tools
cargo build --release
cp target/release/gameverse /usr/local/bin  # Linux/macOS
```

### Shell Completions
```bash
# Bash
gameverse completions bash > ~/.bash_completions/gameverse

# Zsh
gameverse completions zsh > ~/.zsh_completions/_gameverse

# Fish
gameverse completions fish > ~/.config/fish/completions/gameverse.fish
```

## 📖 Detailed Usage
### Creating a New Plugin
```bash
# Interactive mode
gameverse plugin new my-economy

# Fast track – Rust plugin with economy template
gameverse plugin new my-economy --template economy --language rust
```

### Building & Cross-Compiling
```bash
# Local build
gameverse plugin build

# Cross-compile for all platforms
gameverse plugin build --cross-compile all
```

### Testing
```bash
# Unit tests
gameverse plugin test

# Integration tests
gameverse plugin test --integration
```

### Deployment & Packaging
```bash
# Deploy to a dev server
gameverse plugin deploy --server localhost:8080 --environment dev

# Package for distribution
gameverse plugin package --output my-economy-v1.0.0.gvp
```

## 🎨 Templates & Languages
- **Templates:** `basic`, `economy`, `roleplay`, `admin`, `custom`
- **Languages:** Rust, TypeScript, Lua

## ⚙️ Configuration Example (`gameverse.toml`)
```toml
[plugin]
default_template = "basic"
default_language = "rust"

[server]
default_host = "localhost"
default_port = 8080

[marketplace]
url = "https://marketplace.gameverse.dev"
```

## 🔧 Command Reference (excerpt)
| Category  | Command examples |
|-----------|-----------------|
| Plugin    | `gameverse plugin new/build/test/deploy/package` |
| Server    | `gameverse server start/stop/status/logs` |
| Marketplace | `gameverse marketplace search/install/publish` |
| Utilities | `gameverse init`, `gameverse completions <shell>` |

## 🚀 Advantages over FiveM CLI
| Feature | GameVerse CLI | FiveM |
|---------|---------------|-------|
| Template Engine | ✅ Handlebars | ❌ Manual setup |
| Type Safety | ✅ Rust/TS | ⚠️ Lua only |
| Hot Reload | ✅ Automatic | ❌ Restart required |
| Cross-compilation | ✅ Built-in | ❌ Manual |
| Marketplace | ✅ Integrated | ⚠️ Third-party |

## 🗂️ Project Structure
```plaintext
sdk/cli-tools/
├── src/
│   ├── commands/      # CLI sub-commands
│   ├── templates/     # Template engine helpers
│   └── main.rs        # Entry point
├── Cargo.toml         # Dependencies
└── README.md          # Documentation
```

## 📦 Dependencies
Key crates: `clap`, `tokio`, `handlebars`, `indicatif`, `dialoguer`, `reqwest`, `serde`, `git2`.

## 🧪 Testing & Benchmarks
```bash
cargo test            # unit + integration
cargo bench           # performance benchmarks
```

## 🛣️ Roadmap (highlights)
- v0.2 – Template ecosystem improvements
- v0.3 – Hot reload & debugging tools
- v0.4 – Integrated marketplace
- v1.0 – Production-ready CLI

## 🤝 Support & Contributing
Issues: GitHub tracker • Chat: Discord `#cli-tools`  
See CONTRIBUTING.md for guidelines. 