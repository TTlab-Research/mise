# Building Mise for Zed

This guide explains how to build and test the Zed extension locally.

## Prerequisites

You need **ONE** of the following:
- Rust toolchain installed locally
- Docker installed

## Option 1: Build Locally (Rust Required)

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Build the Extension
```bash
cd zed-mise
./build-extension.sh
```

The script will:
1. Check for cargo
2. Compile release binary
3. Show binary location and size

## Option 2: Build in Docker (No Local Rust)

### Requirements
- Docker Desktop installed
- ~5 minutes for first build (caches afterwards)

### Build
```bash
cd zed-mise
./build-docker.sh
```

The script will:
1. Check Docker availability
2. Build Docker image with Rust
3. Compile in container
4. Extract binary to `target/release/`

## Installing in Zed

Once built (via either method), load the extension:

### Method 1: Zed UI (Easiest)
1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
3. Search: `Extensions: Install Dev Extension`
4. Select the `zed-mise` folder
5. Extension loads! 🎉

### Method 2: Manual Configuration
```bash
# Create extensions directory if needed
mkdir -p ~/.config/zed/extensions

# Link to project (development mode)
ln -s /path/to/zed-mise ~/.config/zed/extensions/mise

# Restart Zed
```

## Testing the Extension

After loading in Zed:

### 1. Test Syntax Highlighting
```bash
# Create a test file
echo '[tools]
python = "3.12"
node = "22"

[env]
NODE_ENV = "development"

[tasks.dev]
run = "npm run dev"
' > test.mise.toml
```

Open it in Zed - you should see syntax highlighting!

### 2. Test Slash Commands
- Type `/` in any file
- Look for: `/mise-init`, `/mise-task`, `/mise-env`
- Click one to see it work

### Example: Generate Python Template
```
Type: /mise-init python
Press Enter
→ See generated mise.toml template
```

## Troubleshooting

### "Error: Failed to install dev extension"
**Cause**: Binary not compiled or in wrong location

**Fix**:
```bash
# Verify binary exists
ls -la target/release/libmise.*

# Rebuild
./build-extension.sh
# or
./build-docker.sh

# Restart Zed
```

### "cargo: command not found"
**You need Rust**. Choose one:
- Install locally: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Use Docker: `./build-docker.sh`

### "Docker: command not found"
Install Docker from: https://www.docker.com/products/docker-desktop

### Extension loads but slash commands don't appear
1. Check Zed console (Cmd+Shift+P → "Toggle Developer Console")
2. Verify binary compiled successfully: `ls -lh target/release/libmise.*`
3. Restart Zed: `Cmd+Q` → Open again

## Development Workflow

```bash
# 1. Make code changes in src/lib.rs or languages/mise/highlights.scm

# 2. Rebuild
./build-extension.sh

# 3. Reload in Zed
Cmd+Shift+P → "Reload Extensions"

# 4. Test changes
# (open test.mise.toml, try /mise-init, etc.)

# 5. Iterate
```

## File Structure for Building

```
zed-mise/
├── src/
│   └── lib.rs              # Main extension code
├── languages/mise/
│   ├── config.toml         # Language config
│   └── highlights.scm      # Syntax highlighting
├── Cargo.toml              # Rust package config
├── extension.toml          # Zed extension manifest
├── Dockerfile              # Docker build environment
├── build-extension.sh      # Build script (local Rust)
├── build-docker.sh         # Build script (Docker)
└── target/
    └── release/
        └── libmise.dylib   # ← Compiled binary (macOS)
```

## Platform-Specific Binaries

The build process generates different binaries:
- **macOS**: `target/release/libmise.dylib`
- **Linux**: `target/release/libmise.so`
- **Windows**: `target/release/mise.dll`

Zed automatically picks the correct one for your platform.

## CI/CD (GitHub Actions)

See `.github/workflows/` for automated builds on every commit.

These generate pre-compiled binaries, so users don't need to build locally!

## Advanced: Manual Compilation

If scripts don't work:

```bash
# Simple build
cargo build --release

# Debug build
cargo build

# Check without building
cargo check

# Clean build artifacts
cargo clean
```

## Next Steps

- ✅ Build locally and test
- 📤 Set up GitHub Actions for automated builds
- 🚀 Create releases with pre-compiled binaries
- 📦 Submit to Zed extension marketplace

---

**Questions?** Open an issue: https://github.com/TTlab-Research/zed-mise/issues
