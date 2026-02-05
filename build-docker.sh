#!/bin/bash
# Build extension using Docker (no local Rust required)
# Usage: ./build-docker.sh

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_DIR"

echo "🐳 Building Mise Zed Extension in Docker..."
echo "=============================================="
echo ""

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    echo "❌ Error: Docker is not installed"
    echo "Install Docker from: https://www.docker.com/products/docker-desktop"
    exit 1
fi

# Build Docker image
echo "📦 Building Docker image..."
docker build -t zed-mise-builder:latest .

# Run build in container
echo ""
echo "🔨 Compiling in container..."
docker run --rm \
    -v "$PROJECT_DIR:/extension" \
    zed-mise-builder:latest

# Check result
OS=$(uname -s)
case "$OS" in
    Darwin)
        BINARY="target/release/libmise.dylib"
        ;;
    Linux)
        BINARY="target/release/libmise.so"
        ;;
    *)
        echo "⚠️  OS: $OS (binary extension may not work on this platform)"
        BINARY="target/release/libmise.so"
        ;;
esac

if [ -f "$BINARY" ]; then
    SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
    echo ""
    echo "✅ Build successful!"
    echo "📍 Binary: $BINARY ($SIZE)"
    echo ""
    echo "📤 Next: Load in Zed"
    echo "  1. Open Zed"
    echo "  2. Cmd+Shift+P → 'Extensions: Install Dev Extension'"
    echo "  3. Select: $PROJECT_DIR"
else
    echo "❌ Build failed: binary not found at $BINARY"
    exit 1
fi
