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
- **Modular Intelligence**: AI features are isolated behind the `intelligence` feature flag, allowing for ultra-lean "Light" builds by default while preserving the full Cortex experience when needed.

### 3. Integrated AI Assistant
The PeakUI AI Assistant (powered by the Cortex engine) is now available across all platforms:
- **Native**: Supports local LLM acceleration and full system integration.
- **Web (WASM)**: A high-performance, edge-ready AI interface that connects to remote intelligence models.

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
- `POST /chat`: Direct interface to the internal AI. **The AI can execute app commands based on your prompt.**

```bash
# Example 1: Chat with the internal AI
curl -X POST http://127.0.0.1:8081/chat -d '{"message": "Hello"}'

# Example 2: Instruct the AI to act (Voice Control / Agentic Loop)
curl -X POST http://127.0.0.1:8081/chat -d '{"message": "Switch the theme to dark mode"}'
# Response: "Switching to dark mode... [action: {"SetThemeTone": "Dark"})]"
# Result: The app instantly updates to Dark Mode.
```

For advanced testing, use the included `verify_exposure.py` script.

### 3. Motion & Spatial Design
- **Physics-Driven**: Spring physics engine for fluid and interruptible motion.
- **Volumetric Layout**: Support for 3D layering, rotation, and scale transformations.
- **Unified Interactions**: A single interaction model for mouse, touch, and spatial gaze/pinch.

---

## Getting Started (v0.1.0 Early Access)

PeakUI is currently in active development. The best way to get started is by forking this repository and using our integrated developer CLI.

### 1. Environment Setup

Clone your fork and run the automated setup script to install the **PeakUI CLI**:

```bash
git clone https://github.com/YOUR_USERNAME/PeakUI.git
cd PeakUI
./scripts/setup.sh
```

### 2. The PeakUI CLI (`cargo peakui`)

Once installed, you can manage your projects using the `cargo peakui` subcommand.

#### Initialize a New Project
Create a fresh PeakUI application using the interactive TUI wizard:
```bash
cargo peakui init my-project
```

#### Run Your Application
PeakUI's unified runner handles cross-platform development seamlessly:
- **Native**: `cargo peakui run` (Defaults to native GPU-accelerated desktop with all features)
- **Web**: `cargo peakui run --web` (Launches WASM build; defaults to "Light" build)
- **Intelligence**: `cargo peakui run --web --features intelligence` (Enables full AI Assistant on WASM)
- **Multi-Platform**: `cargo peakui run --all` (Simultaneously starts Native and Web environments)

---

## Exploring the Framework

The core of PeakUI is best explored through the **Showcase App**, which demonstrates our high-performance UI components and the Intelligence-Native bridge.

```bash
# From the PeakUI root directory
cargo peakui run --all
```

## Technical Core

PeakUI relies on a small set of powerful abstractions:
- **Backend Trait**: A standardized interface for rendering, interaction, and perception.
- **View Trait**: A declarative way to describe UI that is independent of the final rendering method.
- **Context**: Thread-safe application state that flows through the view hierarchy.

---

## Progress & Status against Claims

We believe in radical transparency. Here is an honest assessment of the framework's current progress against the ambitious architectural claims made in this document:

| Feature / Claim | Status | Notes |
| :--- | :--- | :--- |
| **Semantic Serialization (99.99% Data Reduction)** | 🟢 **Finished** | Benchmark proven. Seamless conversion of View logic to `SemanticNode` trees. |
| **Native Render Backend (Metal/Vulkan)** | 🟢 **Finished** | High-performance desktop targets via `iced` wgpu backend. |
| **Web Render Backend (WASM)** | 🟢 **Finished** | Edge-deployed, 60fps web rendering, completely sharing Native codebase. |
| **Terminal Render Backend (TUI)** | 🟡 **WIP** | Architectural scaffolding exists, but deep terminal rendering is still experimental. |
| **Neural Exposure API (Agentic Control)** | 🟢 **Finished** | Live on Port 8081. Agents can query schemas, read states, and execute commands. |
| **Neural Sudo (Action Safety)** | 🟢 **Finished** | Secure interception of destructive AI commands requiring human-in-the-loop approval. |
| **PeakDB (Sovereign Local Memory)** | 🟢 **Finished** | Local RAG vector database integrated directly into the UI state loop. |
| **Integrated AI Assistant** | 🟢 **Finished** | Fully functional cross-platform chat interface supporting Ollama, Llama.cpp, and OpenRouter. |
| **Spatial Engine (Volumetric UI)** | 🟡 **WIP** | Coordinate systems and basic Z-indexing exist, but true 3D spatial computing requires further robust physics implementations. |

*Legend: 🟢 Core Implementation Complete & Tested | 🟡 Architectural scaffolding done, but implementation is incomplete or experimental.*