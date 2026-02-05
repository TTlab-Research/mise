#!/bin/bash
# Build script for Mise Zed Extension
# Usage: ./build-extension.sh

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_DIR"

echo "🔨 Building Mise Zed Extension..."
echo "=================================="

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: cargo not found in PATH"
    echo ""
    echo "Option 1: Install Rust globally"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "Option 2: Build in Docker"
    echo "  docker build -t zed-mise-builder ."
    echo "  docker run -v \$(pwd):/extension zed-mise-builder"
    exit 1
fi

echo "✅ cargo version: $(cargo --version)"
echo "✅ rustc version: $(rustc --version)"
echo ""

# Build release binary
echo "📦 Building release binary..."
cargo build --release

# Determine binary name based on OS
OS=$(uname -s)
case "$OS" in
    Darwin)
        BINARY="target/release/libmise.dylib"
        ;;
    Linux)
        BINARY="target/release/libmise.so"
        ;;
    MINGW*|MSYS*)
        BINARY="target/release/mise.dll"
        ;;
    *)
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

if [ -f "$BINARY" ]; then
    SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
    echo ""
    echo "✅ Build successful!"
    echo "📍 Binary: $BINARY ($SIZE)"
    echo ""
    echo "📤 Next steps:"
    echo "  1. Open Zed"
    echo "  2. Cmd+Shift+P → 'Extensions: Install Dev Extension'"
    echo "  3. Select this folder: $PROJECT_DIR"
    echo "  4. The extension will load with compiled binary"
else
    echo "❌ Binary not found at $BINARY"
    exit 1
fi
