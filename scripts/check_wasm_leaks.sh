#!/bin/bash

# Leak discovery script for PeakUI WASM builds
# Checks for suspicious dependencies or features that might cause panics in WASM

TARGET="wasm32-unknown-unknown"
FEATURES="wasm"

echo "🔍 Checking for feature leaks in peak-ui-showcase..."

# Check cargo tree for wgpu when targeting wasm
echo "--- Checking for 'wgpu' in WASM dependency tree ---"
cargo tree -p peak-ui-showcase --features $FEATURES --target $TARGET -i wgpu

# Check for tokio (which doesn't work well in WASM usually without specialized features)
echo "--- Checking for 'tokio' in WASM dependency tree ---"
cargo tree -p peak-ui-showcase --features $FEATURES --target $TARGET -i tokio

# Check for filedescriptor (known issue discovered earlier)
echo "--- Checking for 'filedescriptor' in WASM dependency tree ---"
cargo tree -p peak-ui-showcase --features $FEATURES --target $TARGET -i filedescriptor

echo "--- Feature check complete ---"
