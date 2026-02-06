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

## Current Status (Q1 2026)

### Quality Assessment (Score: 6.5/10)
A comprehensive audit has identified the following:
- **Innovative Core**: The `SemanticNode` and `AIBackend` systems are genuinely world-class and functional (Data reduction works).
- **Architecture**: The trait-based abstraction is solid and idiomatic Rust.
- **Critical Gap**: The `SpatialBackend` is currently a **placeholder skeleton** (marketing vs. reality gap).
- **Stability**: Lack of automated tests (2/10 coverage) and frequent cloning impact production readiness.

### Refinement Roadmap
Future development is guided by [REFINEMENT_ROADMAP.md](file:///Users/vegarberentsen/Documents/PeakSuite/PeakUI/REFINEMENT_ROADMAP.md).
1. **Phase 1**: Complete Spatial rendering & implement testing suite.
2. **Phase 2**: Performance optimization (Arc/Cow) & Error enum implementation.
3. **Phase 3**: Async-first architecture & Memory pooling.

---

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
4. **Beware of Signal 158**: The showcase app has recently exhibited a `SIGUSR1` crash on startup; investigate `wgpu` or platform initialization if this persists.

Good luck! PeakUI is a beast, but it's the future.
