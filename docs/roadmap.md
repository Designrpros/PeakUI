# PeakUI Roadmap

This roadmap outlines the evolution of the PeakUI framework, tracking the journey from prototype to production-grade UI system.

## Phase 1: Foundation & Decoupling (Completed ✅)
*Focus: Maturity, genericism, and stability.*

### [PeakUI Core]
- [x] **Backend Abstraction**: Generic `Backend` trait implemented (wGPU / Glow).
- [x] **Web Support (WASM)**: Confirmed rendering via `trunk` and `glow`.
- [x] **Cupertino Theme**: Glassmorphism, blurred backgrounds, and high-fidelity shadows.
- [x] **Layout Engine**: `NavigationSplitView` with automatic mobile/desktop adaptation.
- [x] **Showcase App**: A "Living Reference" application (`examples/showcase.rs`).

---

## Phase 2: Intelligence & Ecosystem (In Progress 🚧)
*Focus: Interactive tooling, AI integration, and developer adoption.*

### [Component Labs]
- [x] **Button Lab**: Live playground for testing button variants, sizes, and intents.
- [ ] **Typography Lab**: Interactive type scale tester.
- [ ] **Motion Lab**: Playground for testing spring animations and transitions.

### [AI Integration]
- [ ] **Semantic Description**: Implement `.describe()` on Views to output UI structure for LLMs.
- [ ] **Voice Command Loop**: Connect `peak-intelligence` to UI actions (e.g., "Open Settings").

### [Documentation]
- [ ] **Standard Library**: Finalize API docs for `DatePicker`, `RichText`, and `Charts`.
- [ ] **Developer Guide**: Expand `PEAKUI_GUIDE.md` with "Zero to App" tutorials.

---

## Phase 3: Ubiquity (Future 🔮)
*Focus: Mobile dominance and enterprise readiness.*

### [Platform Expansion]
- [ ] **Mobile Native**: Automated iOS/Android build pipelines (APK/IPA generation).
- [ ] **Enterprise Shell**: Specialized modes for Kiosks and Smart Home displays.

### [Tooling]
- [ ] **Visual Inspector**: Real-time layout debugging tool (similar to Safari Web Inspector).
- [ ] **Cloud Sync**: Seamless state synchronization across Desktop and Mobile instances.