# Mobile Engine Feasibility & Implementation Plan

This document outlines the research and proposed architecture for the **PeakUI Mobile Engine**, designed to solve critical friction points in WASM and native mobile (Android/iOS) environments.

## 1. Problem Definition

### A. The "Keyboard Gap" (WASM/Mobile)
**Current State**: Tapping a `TextInput` frequently fails to open the soft keyboard.
**Root Cause**: Browsers require a direct, synchronous "user-initiated event" to focus an input and show the keyboard. PeakUI's declarative abstraction can sometimes delay or obscure this event from the browser's security model.

### B. The "Gesture Conflict" (Scroll vs. Action)
**Current State**: Tapping a button or a horizontal scrollview (inside a vertical one) interrupts vertical scrolling.
**Root Cause**: Iced widgets like `button` capture pointer events immediately on "Down". On mobile, this "stutters" the UI because the parent `Scrollable` cannot take control of the gesture once a child has "captured" it.

---

## 2. Proposed Architecture: The "Mobile Engine"

The Mobile Engine will sit between the `Backend` and the `View` layer, specifically active when `cfg!(target_arch = "wasm32")` or mobile features are enabled.

### Component 1: The Focus Bridge (Soft Keyboard)
- **Mechanism**: Use `web-sys` (for WASM) to detect a `PointerDown` event on an input element.
- **Implementation**: Instead of relying solely on Iced's internal focus message, the Mobile Engine will trigger a synchronous `element.focus()` via a `Canvas` event listener or a specialized `Runtime` hook.
- **Goal**: 100% reliable keyboard invocation on first tap.

### Component 2: The Gesture Arena (Distinguisher)
- **Mechanism**: A threshold-based event buffer.
- **Logic**:
  1. Detect `PointerDown`.
  2. Buffer the event for ~50ms or until a 10px movement threshold is reached.
  3. If movement exceeds threshold vertically, "reject" child interactions and pass events to the `Scrollable`.
  4. If time expires without significant movement, "commit" the event to the child (Button/Slider).
- **Goal**: "Silky smooth" scrolling that feels like a native iOS/Android app.

### Component 3: The Momentum & Overscroll Extension
- **Mechanism**: Enhance `IcedBackend::scroll_view` with momentum calculation.
- **Logic**: Track the velocity of the `PointerUp` event and continue the scroll offset calculation with a decay function.
- **Goal**: Enable the "fling" gesture common in touch-first environments.

---

## 3. Implementation Roadmap

### Phase 1: The WASM Focus Bridge
1. Modify `TextInput` to accept an optional `id`.
2. Implement a `WasmBridge` that uses `web-sys` to find and focus the element by ID when a touch is detected.

### Phase 2: Gesture Interceptor
1. Create a `MobileWrapper` widget that wraps `Button` and `Scrollable`.
2. Implement custom `on_event` logic to manage the "Gesture Arena".

### Phase 3: Platform Adaptation
1. Add `PeakUI::is_mobile()` utility to the `Context`.
2. Automatically adjust padding and control sizes (hit targets) when running on mobile.

## 4. Feasibility Summary
- **Overall Feasibility**: High.
- **Complexity**: Moderate (requires deep knowledge of Iced's event loop).
- **Impact**: Critical for making PeakUI a viable alternative to Flutter/React Native.
