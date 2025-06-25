# GameVerse **Micro-Services**

> English | [Русский](README_ru.md)

This directory contains standalone micro-services powering the GameVerse ecosystem. Each service owns a single responsibility and can be deployed, scaled and updated independently.

## Service Catalog
- **authentication/** – authentication & authorization
- **logging/** – centralised structured logging
- **inventory/** – player inventory backend
- **chat/** – in-game chat & voice
- **player-data/** – persistent player profiles
- **world-state/** – global world synchronisation
- **resource-manager/** – resource loading & hot-reload
- **metrics/** – telemetry & performance metrics

## Tech Stack
- **Rust** for performance-critical logic
- **gRPC / Protocol Buffers** for fast RPC
- **Docker** & **Kubernetes** for deployment
- **PostgreSQL / Redis** for storage & caching

## Architecture Principles
1. **Clean Architecture** – clear separation of concerns
2. **Event-Driven** – services publish/consume domain events
3. **Observability** – metrics, tracing, structured logs

## Development Roadmap (excerpt)
- [x] Service skeletons & shared libs
- [x] Central logging library
- [ ] Complete authentication service MVP
- [ ] ElasticSearch log collector
- [ ] Prometheus + Grafana dashboards 