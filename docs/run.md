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

## Environment Variables

Peak Intelligence requires an **OpenRouter API Key** for the AI Chat features to function in a hosted environment.

### 🔑 Configuration
The framework checks for the `OPENROUTER_API_KEY` in the following order:
1.  **Local File:** `.peak/settings.json` (Native only)
2.  **Build Environment:** Captured at compile-time via `option_env!("OPENROUTER_API_KEY")`.

### 🚀 Vercel Deployment
For Vercel or other CI/CD pipelines, add your secret to the environment variables:
- **Key:** `OPENROUTER_API_KEY`
- **Value:** `your-sk-or-key-here`

During the build process (`trunk build`), Trunk will bake this key into the WASM binary.
