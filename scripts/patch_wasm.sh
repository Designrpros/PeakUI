#!/bin/bash
# scripts/patch_wasm.sh
# This script patches the workspace for WASM building on CI (Vercel)

echo "--- Patching workspace for WASM ---"

# 1. Detach apps/hub from workspace if it exists (prevents sqlite3/PeakDB conflicts)
if grep -q "apps/hub" Cargo.toml; then
    echo "Detaching apps/hub from workspace..."
    sed -i 's/"apps\/hub",//g' Cargo.toml
fi

# 2. Patch framework/Cargo.toml
# - Ensure peak-os-intelligence doesn't pull 'llm' (which pulls llama-server/zstd-sys)
# - Ensure reqwest doesn't pull 'rustls-tls' (which pulls ring/clang)
if [ -f "framework/Cargo.toml" ]; then
    echo "Patching framework/Cargo.toml..."
    # Remove 'llm' feature from peak-os-intelligence dependency
    sed -i 's/features = \["llm"\]//g' framework/Cargo.toml
    # Remove 'rustls-tls' from reqwest in main dependencies
    sed -i 's/"rustls-tls",//g' framework/Cargo.toml
    sed -i 's/, "rustls-tls"//g' framework/Cargo.toml
fi

# 3. Patch peak-core/Cargo.toml
if [ -f "crates/peak-core/Cargo.toml" ]; then
    echo "Patching crates/peak-core/Cargo.toml..."
    sed -i 's/"rustls-tls",//g' crates/peak-core/Cargo.toml
    sed -i 's/, "rustls-tls"//g' crates/peak-core/Cargo.toml
fi

echo "--- Workspace patched ---"
