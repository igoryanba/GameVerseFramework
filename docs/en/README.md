# GameVerse Documentation (English)

> The English documentation is catching up with the primary Russian docs.  You can already build and browse the site locally while translations are being completed.

---

## 📖 Section Overview

| Section | Purpose | Path |
|---------|---------|------|
| **Getting Started** | Installation, first server run, CLI basics | `getting-started.md` |
| **Architecture Overview** | High-level design: core, micro-services, UI runtime, network stack | `architecture.md` |
| **Developer Guides** | Hands-on guides for service development, plugin creation, UI integration | `guides/` |
| **API Reference** | REST / gRPC / CLI reference, DB schemas | `api/` |
| **Advanced Topics** | Hot-reload, tracing, security, FiveM Compatibility Layer | `advanced/` |
| **RFC & Roadmaps** | Design documents and feature roadmaps | `rfcs/` |

Not all files exist yet — **PRs are welcome!**  Use the table above as a roadmap for translating or writing missing chapters.

---

## 🛠️ Building the Static Site

The documentation is rendered with **mdBook**.

```bash
# Install mdBook (single Rust binary < 5 MB)
cargo install mdbook

# Build the English book (output in ./book)
mdbook build docs/en

# Serve locally with hot-reload (http://localhost:3001)
mdbook serve docs/en -p 3001
```

CI automatically deploys the static site to GitHub Pages on every push to `main`.

---

## 🤝 Contributing

1. Write in Markdown, use `#` headings and embedded Mermaid diagrams where useful.
2. Keep parity with the Russian docs: when adding a page here, ensure a stub or full translation exists in `../ru` (and vice-versa).
3. Run the documentation linter before committing:

```bash
npm run lint:docs   # vale + markdownlint
```

4. Prefer Mermaid (`.md`) or PlantUML (`.puml`) for technical diagrams.
5. Submit a PR to a branch starting with `docs/*` or label it `documentation`.

---

_For Russian readers, the most complete docs are located one directory up in `../ru`.  Help translating them is highly appreciated!_ 