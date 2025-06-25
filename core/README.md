# GameVerse Framework **Core**

## Overview
The core module contains fundamental components written in **Rust** and **C++**. It is responsible for baseline functionality, interaction with native game APIs, and coordination of the micro-services that make up the GameVerse ecosystem.

**English | [Русский](README_ru.md)**

---

## Directory structure
- **server/** – server-side runtime
- **client/** – client-side runtime
- **common/** – shared code used by both server and client

## Key technologies
- **Rust** – primary language for speed and safety
- **C++20** – for performance-critical low-level parts
- **Tokio** – asynchronous runtime
- **QUIC / HTTP-3** – modern network stack

## Development progress
- [ ] Rust project scaffolding & workspace
- [ ] Structured logging with `tracing`
- [ ] Core architecture skeleton (messaging, runtime layers)
- [ ] Micro-service integration hooks 