# Running PeakUI

This guide explains how to run the PeakUI framework and its showcase applications across different platforms.

## Showcase Application

The primary way to test the framework is via the `showcase` app located in `apps/showcase`.

### 🚀 Running Native (Desktop)
To run the showcase app as a native desktop application:

```bash
# Navigate to the showcase app directory
cd apps/showcase

# Run with the default (native) feature
cargo run
```

### 🌐 Running Web (WASM)
PeakUI supports the web using Iced's WGPU/WebGL backend. We use [Trunk](https://trunkrs.dev/) for the build and serving process.

#### Prerequisites
1.  **Install Trunk:**
    ```bash
    cargo install --locked trunk
    ```
2.  **Add WASM Target:**
    ```bash
    rustup target add wasm32-unknown-unknown
    ```

#### Start Development Server
```bash
# Navigate to the showcase app directory
cd apps/showcase

# Start the Trunk server
trunk serve
```
The app will be available at [http://localhost:8080](http://localhost:8080) by default.

---

## Workspace Tips

- **Relative Paths:** Note that since `PeakUI` is a standalone workspace, all commands should be run from the root of the `PeakUI` directory or specific app directories.
- **Dependencies:** If you are modifying the framework, `cargo run` will automatically recompile dependencies.
- **WASM Features:** When running for the web, Trunk uses the configurations defined in `Trunk.toml` and `index.html`.
