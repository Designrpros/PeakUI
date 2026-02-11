# PeakUI

<p align="center">
  <img src="apps/showcase/assets/peak_logo.png" alt="Peak Logo" width="300"/>
  <br>
  <b>The Universal Interface Framework for Rust.</b>
  <br>
  <i>Build for Pixels. Build for Terminals. Build for Intelligence.</i>
</p>

PeakUI is a high-performance, declarative UI framework tailored for building professional-grade applications. It combines the ergonomics of SwiftUI with the system-level power of Rust, enabling you to write a single view hierarchy that renders natively across every dimension of computing.

[![License](https://img.shields.io/badge/license-BSL--1.1-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![WASM Ready](https://img.shields.io/badge/platform-native%20%7C%20web%20%7C%20tui-brightgreen)](Trunk.toml)
[![Tests](https://img.shields.io/badge/tests-6%2F6%20passing-success)](framework/tests/)

---

## Validated Performance

**Research Prototype** with proven results:

```
PeakUI Semantic Size:    0.68 KB (700 bytes)
Vision Equivalent:       7.91 MB (8,294,400 bytes)
Data Reduction:          99.9916%
Energy Factor:           ~11,849x reduction in data handling
```

**Run the benchmark yourself:**
```bash
cargo test benchmark_green_ai_reduction -- --nocapture
```

---

## Core Philosophy

Most frameworks force a choice between performance and productivity, or web and native. PeakUI focuses on **Physicality and Intent**.

### 1. Universal Rendering
Write your component logic once and deploy it across fundamentally different backends:
- **Native Desktop**: Metal/Vulkan accelerated rendering with native vibrancy and glassmorphism.
- **Web (WASM)**: High-performance WASM builds for edge-deployed modern interfaces.
- **Terminal (TUI)**: The same codebase renders as a high-fidelity terminal interface.

### 2. Intelligence-Native Architecture
PeakUI is the first framework designed for the Agentic Era.
- **Semantic Contract**: The UI tree is automatically exposed as structured data (Semantic Nodes) for AI agents to perceive with zero-latency.
- **Spatial Engine**: A full 3D coordinate system with hierarchical depth, enabling agents to navigate interfaces via volumetric raycasting.
- **Sovereign Interface**: Move beyond accessibility hacks; the UI communicates its exact "Intent" through a deterministic semantic protocol.
- **Neural Exposure API**: An optional TCP-based API server (Port 8081) that allows external AI agents to "remote control" the application, retrieve UI schemas, and execute commands via raw network sockets.

---

## Neural Exposure (AI Remote Control)

PeakUI allows you to expose the entire application interface to external intelligence via the **Neural Exposure API**.

### Enabling Exposure
1. Launch the **Showcase App**.
2. Navigate to **Settings -> AI**.
3. Toggle **Neural Exposure** to **ON**.

### API Endpoints (Localhost:8081)
- `GET /schema`: Retrieves the full framework component schema (MCP Protocol).
- `GET /instructions`: Retrieves the "Neural Protocol" manual for AI interaction.
- `GET /view`: Retrieves the **live, dynamic UI structure** (JSON) of the running application.
- `POST /command`: Executes an application command (e.g., `{"SetTab": "Colors"}`).

```bash
# Example: Navigate the app to the Colors page via terminal
curl -X POST http://127.0.0.1:8081/command -d '{"SetTab": "Colors"}'
```

For advanced testing, use the included `verify_exposure.py` script to validate schema retrieval and instruction delivery.

### 3. Motion & Spatial Design
- **Physics-Driven**: Spring physics engine for fluid and interruptible motion.
- **Volumetric Layout**: Support for 3D layering, rotation, and scale transformations.
- **Unified Interactions**: A single interaction model for mouse, touch, and spatial gaze/pinch.

---

## Getting Started

The recommended way to explore PeakUI is through the **Showcase App**, a component laboratory simulating a modern Operating System.

### Desktop Experience (GUI)
Run with full GPU acceleration:

```bash
cargo run --release --example showcase
```



---

## Technical Core

PeakUI relies on a small set of powerful abstractions:
- **Backend Trait**: A standardized interface for rendering, interaction, and perception.
- **View Trait**: A declarative way to describe UI that is independent of the final rendering method.
- **Context**: Thread-safe application state that flows through the view hierarchy.