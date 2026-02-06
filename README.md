# Mise for Zed

Professional syntax highlighting and configuration support for [mise](https://mise.jdx.dev) configuration files in Zed IDE.

## Features

- **Syntax Highlighting** - Complete, theme-aware syntax highlighting for mise.toml and .mise.toml files
- **Section Support** - Full support for all mise sections: [tools], [env], [tasks], [vars], [hooks], [plugins], [settings]
- **Property Recognition** - Accurate highlighting of task properties: run, depends, description, sources, dir, env, etc.
- **Environment Variables** - Smart detection and highlighting of environment variable identifiers (UPPERCASE)
- **Template Syntax** - Proper highlighting of Tera template expressions ({{ env.VAR }}, {{ cwd }})
- **Performance** - TreeSitter-based syntax engine, <10ms per file
- **Zero Cost** - No external dependencies, no paywalls, completely free

## Installation

1. Open Zed
2. Open Extensions (`Cmd+Shift+X`)
3. Search for "mise"
4. Click Install

## Supported Files

- `mise.toml`
- `.mise.toml`
- `mise.local.toml`
- `.mise.local.toml`

## Example

```toml
[tools]
python = "3.12"
node = "22"
uv = "latest"

[env]
NODE_ENV = "development"
DATABASE_URL = "{{ env.DATABASE_URL }}"

[tasks.dev]
description = "Start development server"
run = "uv run python -m uvicorn main:app --reload"
depends = ["install"]

[hooks.enter]
run = "uv sync --quiet"
```

## Development

### Building Locally

```bash
# Build the extension
cargo build

# Build for release (optimized)
cargo build --release

# Run tests (when added)
cargo test
```

### Project Structure

```
src/lib.rs              # Extension implementation (minimal)
languages/mise/
├── config.toml         # Language configuration
├── highlights.scm      # TreeSitter highlighting rules
└── indents.scm         # Indentation rules
examples/               # Production-ready configurations
├── python-uv.toml
├── nextjs-fullstack.toml
├── go-minimal.toml
└── rust-release.toml
.zed/tasks.json         # Native Zed tasks for config generation
docs/                   # Comprehensive documentation
Cargo.toml              # Rust dependencies
extension.toml          # Zed extension manifest
```

### Using Configuration Templates

Instead of manual creation, use Zed's native task support:

1. **Copy templates directly**
   ```bash
   cp examples/python-uv.toml mise.toml
   ```

2. **Or use Zed tasks** (when in this project)
   - `Cmd+Shift+P` → search "mise"
   - Select a template generation task
   - Configuration created instantly

3. **Edit for your project**
   - Customize versions and paths
   - Add project-specific settings

### Testing

Run the automated test suite:

```bash
./tests/test-extension.sh
```

For manual testing in Zed:

1. Build: `cargo build --release`
2. Reload extensions: `Cmd+Shift+P` → "Reload Extensions"
3. Open test files: `tests/test.mise.toml`
4. Verify syntax highlighting with colors
5. Test Zed tasks: `Cmd+Shift+P` → search "mise:"

See [tests/README.md](tests/README.md) for comprehensive testing guide.

### Architecture

The extension provides:

1. **Syntax Highlighting** - TreeSitter-based via `highlights.scm`
2. **Language Recognition** - Automatic detection of `mise.toml` files
3. **Zero Dependencies** - Pure Rust, minimal footprint

## Contributing

Contributions are welcome! Please ensure:

- Code compiles without warnings: `cargo build --release`
- Syntax highlighting improvements in `languages/mise/highlights.scm`
- New slash commands follow the existing pattern in `src/lib.rs`

## Issues & Feature Requests

Found a bug or have a feature idea? Open an issue on [GitHub](https://github.com/TTlab-Research/zed-mise/issues).

## License

MIT License - Copyright (c) 2024-2026 Franco Tampieri - TTlab
