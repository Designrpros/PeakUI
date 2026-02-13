#!/bin/bash

# PeakUI Development Setup Script v0.1.0
# "For pixels, for terminals, for intelligence."

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${CYAN}"
echo "    ____             __  __  ______"
echo "   / __ \___  ____ _/ /_/ / / /_  _/"
echo "  / /_/ / _ \/ __ \`/ //_/ / / / / /  "
echo " / ____/  __/ /_/ / ,< / /_/ /_/ /   "
echo "/_/    \___/\__,_/_/|_|\____/___/    "
echo -e "${NC}"
echo -e "${YELLOW}PeakUI Development Setup v0.1.0${NC}"
echo "--------------------------------------------------"

# 1. Check for Rust
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed.${NC}"
    echo "Please install Rust first: https://www.rust-lang.org/tools/install"
    exit 1
fi

# 2. Check for Trunk (optional but recommended for web)
if ! command -v trunk &> /dev/null; then
    echo -e "${YELLOW}Warning: 'trunk' not found.${NC}"
    echo "Trunk is required for Web (WASM) development."
    read -p "Would you like to install trunk now? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cargo install trunk
    fi
fi

# 3. Install PeakUI CLI
echo -e "${CYAN}Building and installing PeakUI CLI...${NC}"
if cargo install --path crates/peakui-cli --force; then
    echo -e "${GREEN}✅ PeakUI CLI (cargo-peakui) installed successfully!${NC}"
else
    echo -e "${RED}Failed to install PeakUI CLI.${NC}"
    exit 1
fi

echo "--------------------------------------------------"
echo -e "${GREEN}Setup complete!${NC}"
echo "You can now use '${CYAN}cargo peakui${NC}' from anywhere."
echo ""
echo "Try initializing a project:"
echo -e "  ${YELLOW}cargo peakui init my-cool-app${NC}"
echo ""
echo "Or run the showcase in this repo:"
echo -e "  ${YELLOW}cargo peakui run --all${NC}"
