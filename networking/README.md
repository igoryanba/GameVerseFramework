# GameVerse Framework **Networking Stack**

> English | [Русский](README_ru.md)

## Overview
This directory contains the networking layer responsible for communication between clients and servers, entity synchronisation, and traffic optimisation.

## Structure
- **protocol/** – data-exchange protocol definitions (Protocol Buffers / FlatBuffers)
- **quic/** – QUIC transport implementation
- **sync/** – entity synchronisation engine
- **serialization/** – (de)serialisation utilities
- **routing/** – packet routing between micro-services

## Key Components
- **EntitySynchronization** – keeps game entities in sync across peers
- **NetworkBridge** – bridge between the game engine and the network layer
- **PacketCompression** – reduces bandwidth usage
- **LagCompensation** – client-side latency compensation algorithms

## Technologies
- **QUIC / HTTP-3** – fast UDP-based transport
- **Protocol Buffers / FlatBuffers** – efficient binary serialisation
- **Tokio** – asynchronous runtime
- **Quinn** – Rust QUIC implementation

## Development Roadmap
- [ ] Define on-wire protocol
- [ ] Implement baseline QUIC communication
- [ ] Build serialisation & synchronisation system
- [ ] Optimise for high-concurrency scenarios
- [ ] Integrate OneSync-like large-session support 