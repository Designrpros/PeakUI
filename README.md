# PeakUI 🏔️
> **The Universal Interface Framework for Rust.**
>
> *Build for Pixels. Build for Terminals. Build for Intelligence.*

**PeakUI** is a high-performance, declarative UI framework tailored for building professional-grade applications. It combines the ergonomics of **SwiftUI** with the system-level power of **Rust**, enabling you to write a single view hierarchy that renders natively across every dimension of computing.

[![License](https://img.shields.io/badge/license-BSL--1.1-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![WASM Ready](https://img.shields.io/badge/platform-native%20%7C%20web%20%7C%20tui-brightgreen)](Trunk.toml)

---

## 🌟 Why PeakUI?

Most frameworks force you to choose: *Performance vs. Productivity* or *Web vs. Native*. PeakUI chooses **Physics**.

### 1. The "One Codebase" Promise
Write your component logic once. Run it everywhere.
* 🖥️ **Native Desktop:** Metal/Vulkan accelerated rendering with Glassmorphism.
* 🌐 **Web (WASM):** Compiles to lightweight WASM for edge-deployed interfaces.
* 📟 **Terminal (TUI):** **Exclusive Feature.** The *exact same code* renders as a high-fidelity ASCII/ANSI interface for SSH sessions.

### 2. AI-Native Architecture 🧠
PeakUI is the first framework built for the **Agentic Era**.
* **MCP Integration:** Implements the [Model Context Protocol](https://modelcontextprotocol.io/) out of the box.
* **Semantic State:** Your UI tree is automatically exposed as structured context to LLMs.
* **Agent Control:** AI agents can read the screen and trigger `Message` events to navigate or control the app autonomously.

### 3. Spatial & Motion Design
* **Spring Physics:** integrated `motion.rs` engine for fluid, interruptible animations.
* **Z-Index Layering:** First-class `ZStack` and `Overlay` support for complex, depth-aware interfaces.
* **Gesture System:** Unified touch and mouse handling for advanced interactions.

---

## 🚀 Quick Start

The best way to experience PeakUI is the **Showcase App**, a "Component Lab" simulating a full Operating System.

### Option A: The Desktop Experience (GUI)
Runs with full GPU acceleration (Metal/DX12).

```bash
# Run the visual showcase
cargo run --release --example showcase