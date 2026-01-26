#!/bin/bash

# 1. Install Rust target (if not already present via rust-toolchain)
rustup target add wasm32-unknown-unknown

# 2. Install Trunk (using pre-built binary for speed)
# We use a simple curl download to avoid compiling trunk from source (which takes various minutes)
if ! command -v trunk &> /dev/null
then
    echo "Trunk not found, installing..."
    wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.17.5/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
    chmod +x trunk
    # Move to a directory in PATH or just use ./trunk
    mkdir -p bin
    mv trunk bin/
    export PATH="$PWD/bin:$PATH"
fi

# 3. Build the Application
echo "Building PeakUI Showcase..."
trunk build --release --public-url /
