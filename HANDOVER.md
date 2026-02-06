# PeakUI Handover Document

## Project Vision
PeakUI is a **Sovereign Interface Engine** designed for intelligence-native operating systems. It enables a single UI definition to be rendered across multiple "realities":
- **Canvas**: Traditional high-performance GUI (Iced).
- **Terminal**: ANSI-based text interface for consoles.
- **Neural**: Structured JSON for LLMs to "see" and interact with the UI.
- **Spatial**: 3D-aware nodes for spatial computing and simulators.

---

## Core Architecture

### 1. The `Backend` Trait (`core.rs`)
The heart of the framework. Every UI atom (Text, Icon, Button) and layout (HStack, VStack) is defined as a method on this trait.
- **Return Types**: Backends return `Self::AnyView<Message>`. When writing generic views, you must often return `Box<dyn View<Message, B>>` to satisfy trait requirements.
- **Trait Plumbing**: If you add a parameter to a `Backend` method (e.g., I just added `height` to `button`), you **must** update:
    - The `Backend` trait definition.
    - `IcedBackend`, `TermBackend`, `AIBackend`, and `SpatialBackend` implementations.
    - All recursive call sites (e.g., `B::button` inside `controls.rs`, `chat.rs`, `gesture.rs`).

### 2. Documentation Templates
All documentation pages (under `reference/pages/`) follow a "Premium Template":
- **Hero**: Title and subtitle.
- **The Lab**: A 4-mode switcher (Canvas/Term/Neural/Spatial) using `ProxyView`.
- **Usage**: Rust code examples.
- **Theory**: Semantic and architectural explanation.

### 3. Layout and Spacing
- **Safe Areas**: Managed in `ContentView` and `ProxyView`. It uses a `Context` object to inject top/bottom padding to clear the floating Header and Dock.
- **Sizing**: Uses Iced's `Length` system. I recently refactored `Button` to support explicit `height` settings to avoid clipping large icons.
- **Grids**: Use `ResponsiveGrid` for library-style layouts. Note: `ResponsiveGrid::new()` takes no args; use `.push()` in a loop.

---

## The "Good" (Wins)
- **High Portability**: Once the trait bounds are correct, the UI just works across all backends.
- **Semantic Data**: The `describe()` method on views provides a rich tree for AI agents (the `AIBackend`), making PeakUI the best framework for building agentic apps.
- **Consistency**: The `DSL` and `Atoms` provide a very clean way to write complex UIs quickly.

---

## The "Bad" (Friction Points)
- **Trait Bound Hell**: Rust's type system can get very angry when nesting generic views inside `ProxyView` closures. If you get `expected B::AnyView, found ProxyView`, you usually need to wrap the return in a `Box::new()`.
- **Hardcoded Backend Limits**: Some backends have hardcoded values (like minimum button heights or default scroll speeds). When you hit a visual limit, go to `core.rs` and check the backend implementation.
- **Unused Imports**: The codebase has many unused imports (`SpatialBackend`, `Arc`, etc.) across doc pages. Cleaning these up with `cargo fix` is periodically necessary.

---

## Recent Breakthroughs
- **Icon Library Compression**: Shrunken the grid to 5 columns with **48px** icons and **140px** buttons.
- **Dynamic Sizing**: Added explicit width/height support to the `Button` control and `Backend` trait.
- **Safe Area Protocol**: Content now correctly clears the Dock and Header across all pages.

---

## Tips for the Next Agent
1. **Check the Lab**: If a component looks weird, switch to **Neural** mode to see the raw semantic tree. It often reveals layout nesting issues.
2. **Follow the Pattern**: Look at `icon.rs` or `colors.rs` for the current documentation standard.
3. **Rust Analyzer is your friend**: Trust the lint errors regarding argument counts; the `Backend` trait is strictly enforced.

Good luck! PeakUI is a beast, but it's the future.
