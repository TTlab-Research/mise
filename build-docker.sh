#!/bin/bash
# Build extension using Docker/Podman (no local Rust required)
# Usage: ./build-docker.sh

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_DIR"

echo "🐳 Building Mise Zed Extension in Container..."
echo "================================================"
echo ""

# Detect container runtime
CONTAINER_CMD=""
if command -v docker &> /dev/null; then
    CONTAINER_CMD="docker"
    echo "✅ Using Docker"
elif command -v podman &> /dev/null; then
    CONTAINER_CMD="podman"
    echo "✅ Using Podman"
else
    echo "❌ Error: Neither Docker nor Podman is installed"
    echo "Options:"
    echo "  1. Install Docker: https://www.docker.com/products/docker-desktop"
    echo "  2. Install Podman: brew install podman"
    exit 1
fi

echo ""

# Build container image
echo "📦 Building container image..."
$CONTAINER_CMD build -t zed-mise-builder:latest .

# Run build in container
echo ""
echo "🔨 Compiling in container..."
$CONTAINER_CMD run --rm \
    -v "$PROJECT_DIR:/extension" \
    -w /extension \
    zed-mise-builder:latest

# Check result
# Note: When building in container, we always get Linux binary
# Zed will use it regardless of platform in dev mode
# Package "zed-mise" becomes "zed_mise" in Rust binary names
BINARY="target/release/libzed_mise.so"

if [ -f "$BINARY" ]; then
    SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
    echo ""
    echo "✅ Build successful!"
    echo "📍 Binary: $BINARY ($SIZE)"
    echo "📝 Note: Container build produces Linux binary"
    echo "   In development mode, Zed can use this on any platform"
    echo ""
    echo "📤 Next: Load in Zed"
    echo "  1. Open Zed"
    echo "  2. Cmd+Shift+P → 'Extensions: Install Dev Extension'"
    echo "  3. Select: $PROJECT_DIR"
    echo ""
    echo "✨ The extension should load and work!"
else
    echo "❌ Build failed: binary not found at $BINARY"
    echo "📂 Available files in target/release/:"
    ls -la target/release/ | grep -E "libmise|mise" || echo "   (none found)"
    exit 1
fi
