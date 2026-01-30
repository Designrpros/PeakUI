# PeakDB Roadmap

PeakDB is the decentralized, reactive data layer for PeakOS. It provides a standardized data interface that is provider-agnostic and AI-ready.

## Phase 1: Foundation & Discovery (Active 🚧)
*Focus: Maturity, genericism, and multi-provider support.*

- [x] **Async Pooling**: High-performance connection pooling for PostgreSQL and SQLite.
- [x] **DataRouter Trait**: Unified implementation for routing saves/queries across providers.
- [x] **Schema Discovery**: Automatic table detection and type-safe migration engine.
- [x] **Local Cache**: FileProvider for ultra-fast local indexing and temporary storage.

---

## Phase 2: Semantic Resonance (Future 🔮)
*Focus: Vector storage and AI memory consolidation.*

- [ ] **pgvector Native**: Direct support for vector embeddings within the SQL interface.
- [ ] **Semantic Search API**: High-level query logic for finding "Relational + Vector" data.
- [ ] **Auto-Embedding**: Automatic generation of vectors for document saves.
- [ ] **Memory Consolidation**: Clear separation between "Short-term" (context) and "Long-term" (knowledge) stores.

---

## Phase 3: Reactive Swarm (Future 🔮)
*Focus: Real-time synchronization and conflict resolution.*

- [ ] **Live Queries**: Direct UI binding via Postgres `LISTEN/NOTIFY`.
- [ ] **Conflict-Safe (CRDTs)**: Implementation of Merkle-DAGs and LWW-Registers for multi-agent sync.
- [ ] **P2P Streaming**: Real-time event propagation via the PeakCloud mesh.
- [ ] **Optimistic UI**: Immediate local feedback before global swarm confirmation.

---

## Phase 4: Global Intelligence (Long-term 🌌)
*Focus: Sovereign identity and cross-device agency.*

- [ ] **Unified Identity**: Decentralized storage for user profiles and preferences.
- [ ] **Swarm Search**: Cross-device vector querying (find anything on any of your Peak machines).
- [ ] **Autonomous Migration**: AI-driven logic for balancing data across the swarm.
- [ ] **Sovereign Encryption**: End-to-end encrypted storage where the user owns the master keys.
