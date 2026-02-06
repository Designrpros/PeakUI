# PeakUI Refinement Roadmap

This document outlines a systematic plan to transition PeakUI from a promising proof-of-concept to a production-ready "Top Tier" framework.

## Core Objective
Address the gaps between marketing claims and actual implementation, particularly regarding Spatial rendering, testing coverage, and performance.

---

## Phase 1: Critical Foundation (High Priority)

### 1. Complete SpatialBackend Implementation
The existing `SpatialBackend` is largely a skeleton. We need to move beyond placeholder zero-sized elements to actual 3D layout and rendering.
- **Task**: Implement actual text shaping and bounds calculation in 3D space.
- **Task**: Integrate a real 3D rendering pipeline (e.g., `wgpu` or `vello` for 2D-in-3D).

### 2. Comprehensive Testing Suite
Lack of testing is the single biggest blocker for reliable production use.
- **Target**: Unified backend consistency tests.
- **Target**: Semantic Node serialization verification.
- **Action**: Create `framework/tests/` with cross-backend validation.

---

## Phase 2: Performance & Architecture (Medium Priority)

### 3. Memory Optimization (Reduce Cloning)
Current implementation relies heavily on `.clone()`. We need to move toward shared ownership and zero-copy patterns.
- **Pattern**: Use `Arc<str>` or `SmolStr` for roles and labels.
- **Pattern**: Implement `Cow` (Clone-on-Write) for `describe()` calls to avoid unnecessary allocations.

### 4. Consolidation & Error Handling
- **Consolidation**: Reduce `#[cfg]` sprawl by abstracting platform-specific effects (vibrancy, tray) into traits.
- **Errors**: Replace `Result<(), String>` with a structured `PeakUIError` enum using `thiserror`.

---

## Phase 3: Code Quality & Polish (Future)

### 5. Memory Pooling & Async
- **Pooling**: Implement an object pool for frequently created `SpatialNode` and `SemanticNode` objects.
- **Async**: Introduce `AsyncBackend` for heavy operations (heavy serialization, complex layout).

### 6. Developer Experience (DX)
- **Macros**: Create boilerplate-reduction macros for common `View` implementations.
- **Benchmarking**: Rigorously benchmark the "99.99% AI data reduction" claim to provide quantitative proof.

---

## Marketing vs. Reality Alignment
- [ ] **Spatial Rendering**: Current: Placeholder. Target: Functional.
- [ ] **Testing**: Current: Benching only. Target: 80%+ coverage.
- [ ] **Production-Ready**: Current: PoC. Target: Beta-stable.
