#!/bin/bash

# PeakUI Size Audit Script
# This script calculates Source Lines of Code (SLOC) and binary sizes for the framework and showcase.

COLOR_CYAN='\033[0;36m'
COLOR_GREEN='\033[0;32m'
COLOR_NC='\033[0m'

echo -e "${COLOR_CYAN}--- PeakUI Size Audit ---${COLOR_NC}"

# 1. SLOC Analysis
echo -e "\n${COLOR_GREEN}[1/2] Source Code Analysis (SLOC)${COLOR_NC}"

TOTAL_SRC=$(find framework/src -name "*.rs" | xargs wc -l | tail -n 1 | awk '{print $1}')
REF_APP=$(find framework/src/reference -name "*.rs" | xargs wc -l | tail -n 1 | awk '{print $1}')
CORE_FRAMEWORK=$((TOTAL_SRC - REF_APP))
SHOWCASE_APP=$(find apps/showcase/src -name "*.rs" | xargs wc -l | tail -n 1 | awk '{print $1}')

echo "Core Framework:    $CORE_FRAMEWORK lines"
echo "Reference App:     $REF_APP lines"
echo "Showcase Main:     $SHOWCASE_APP lines"
echo "--------------------------"
echo "Total Source:      $((TOTAL_SRC + SHOWCASE_APP)) lines"

# 2. Binary Size Analysis (Release WASM)
echo -e "\n${COLOR_GREEN}[2/2] Binary Size Analysis (WASM)${COLOR_NC}"
echo "Building showcase app in release mode... (this may take a moment)"

# Navigate to showcase and build
cd apps/showcase
trunk build --release > /dev/null 2>&1

if [ -d "dist" ]; then
    WASM_SIZE=$(du -h dist/*.wasm | awk '{print $1}')
    JS_SIZE=$(du -h dist/*.js | awk '{print $1}')
    echo "Showcase WASM Size: $WASM_SIZE"
    echo "Showcase JS Size:   $JS_SIZE"
else
    echo "Error: Release build failed or dist directory not found."
fi

echo -e "\n${COLOR_CYAN}--- Audit Complete ---${COLOR_NC}"
