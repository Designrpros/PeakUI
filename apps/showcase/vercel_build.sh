#!/bin/bash

# 1. Install Rust (since rustup is not in the default image path)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# 2. Add WASM target
rustup target add wasm32-unknown-unknown

# 3. Install Trunk (using pre-built binary for speed)
if ! command -v trunk &> /dev/null
then
    echo "Trunk not found, installing..."
    # Use curl instead of wget (wget is not available)
    curl -L https://github.com/trunk-rs/trunk/releases/download/v0.17.5/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
    chmod +x trunk
    # Move to a directory in PATH or just use ./trunk
    mkdir -p bin
    mv trunk bin/
    export PATH="$PWD/bin:$PATH"
fi

# 4. Build the Application
echo "Building PeakUI Showcase..."

# Create dist directory to prevent canonical path error in Trunk config
mkdir -p dist
# Ensure dependencies are updated just in case
cargo update -p peak-ui --precise 0.1.0 2>/dev/null || true # Optional specific update logic if needed
trunk build --release --public-url /
