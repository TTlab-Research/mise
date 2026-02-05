# Mise for Zed

Syntax highlighting and utilities for [mise](https://mise.jdx.dev) configuration files in Zed IDE.

## Features

- 🎨 **Syntax Highlighting** for `.mise.toml` and `mise.toml` files
- 📝 Support for all mise sections: `[tools]`, `[env]`, `[tasks]`, `[vars]`, `[hooks]`, `[plugins]`, `[settings]`
- 🏷️ Task property highlighting: `run`, `depends`, `description`, `sources`, `dir`, `env`, etc.
- 🔤 Environment variable detection (UPPERCASE identifiers)
- 🎯 Tera template syntax highlighting (`{{ env.VAR }}`, `{{ cwd }}`)
- ⚡ Performance-optimized with `#any-of?` predicates

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
src/lib.rs              # Main extension logic with slash commands
languages/mise/
├── config.toml         # Language configuration
└── highlights.scm      # Tree-sitter highlighting rules
Cargo.toml              # Rust dependencies
extension.toml          # Zed extension manifest
```

### Slash Commands

This extension provides helpful utilities for working with mise:

- **/mise-init [language...]** - Generate mise.toml templates for specific stacks (python, node, nextjs, go, rust, deno, bun, docker, terraform)
- **/mise-task create [stack...]** - Add task templates to mise.toml
- **/mise-env** - Display mise environment information

### Testing

To test the extension:

1. Build: `cargo build`
2. Open Zed and press `Cmd+Shift+P` → "Reload Extensions" or restart Zed
3. Create a test file: `test.mise.toml`
4. Verify syntax highlighting works
5. Try slash commands: type `/` and look for `mise-init`, `mise-task`, `mise-env`

### Architecture

The extension uses Zed's extension API to provide:

1. **Syntax Highlighting** - Tree-sitter based via `highlights.scm`
2. **Slash Commands** - Interactive utilities for generating mise configurations
3. **Command Completion** - Smart argument suggestions for each command

## Contributing

Contributions are welcome! Please ensure:

- Code compiles without warnings: `cargo build --release`
- Syntax highlighting improvements in `languages/mise/highlights.scm`
- New slash commands follow the existing pattern in `src/lib.rs`

## Issues & Feature Requests

Found a bug or have a feature idea? Open an issue on [GitHub](https://github.com/TTlab-Research/zed-mise/issues).

## License

MIT License - Copyright (c) 2024-2026 Franco Tampieri - TTlab
