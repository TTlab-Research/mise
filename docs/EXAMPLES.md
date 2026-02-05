# Mise Configuration Examples

Ready-to-use `mise.toml` examples for common tech stacks.

## Python with uv

**File:** `examples/python-uv.toml`

Modern Python development with the `uv` package manager:

- Python 3.12 + uv
- Virtual environment in `.venv`
- Tasks: install, dev, test, lint, format, typecheck, clean
- Hook: auto-sync on directory entry

**Best for:**
- FastAPI/Django projects
- Data science with notebooks
- Any Python package development

## Next.js Full-Stack (Python Backend)

**File:** `examples/nextjs-fullstack.toml`

Combined Node.js frontend + Python backend:

- Node 22 (for Next.js)
- Python 3.12 + uv (for API)
- Shared environment variables
- Tasks run both stacks together
- Dev task runs both servers in parallel

**Best for:**
- Next.js + FastAPI projects
- React + Django projects
- Full-stack TypeScript + Python apps

## Go Minimal

**File:** `examples/go-minimal.toml`

Lightweight Go project setup:

- Go 1.23 + golangci-lint
- Standard Go build/test/lint tasks
- Coverage report generation
- CGO support enabled

**Best for:**
- CLI tools
- Microservices
- Standard Go applications

## Rust Release

**File:** `examples/rust-release.toml`

Production Rust project setup:

- Stable Rust toolchain
- Debug + optimized release builds
- Comprehensive testing (unit + benches)
- Documentation generation
- Clippy linting with strict warnings

**Best for:**
- Libraries and binaries
- Performance-critical code
- Production deployments

## Using Examples

### Copy and Customize

```bash
# Copy an example to your project
cp examples/python-uv.toml mise.toml

# Edit for your project specifics
vim mise.toml
```

### Generate from Zed Assistant

In Zed's Assistant, use `/mise-init` to generate templates:

```
/mise-init python
/mise-init nextjs python
/mise-init go
/mise-init rust
```

### Add Tasks to Existing Configuration

Use `/mise-task create` to add task templates:

```
/mise-task create python
/mise-task create python nextjs
```

## Common Patterns

### Running Multiple Services

```toml
[tasks.dev]
description = "Run all development servers"
run = """
echo "Starting services..."
npm run dev & \
python -m uvicorn api:app --reload & \
wait
"""
```

### Conditional Task Execution

```toml
[tasks.test-all]
description = "Test all components"
run = """
{{ if executable("npm") }}npm test && {{ end }} \
{{ if executable("pytest") }}pytest{{ end }}
"""
```

### Watch Mode

```toml
[tasks.watch]
description = "Watch for changes and rebuild"
run = "cargo watch -x build -x test"
```

### Pre-flight Checks

```toml
[tasks.check]
description = "Check everything before commit"
run = """
cargo fmt --check && \
cargo clippy -- -D warnings && \
cargo test --all-features
"""
depends = ["lint", "typecheck"]
```

## Best Practices

1. **Keep tasks focused** - One task = one responsibility
2. **Use descriptions** - Help team members understand each task
3. **Depend on others** - Use `depends` to ensure prerequisites
4. **Document assumptions** - Add comments about tool versions
5. **Use environment variables** - Keep paths and configs in `[env]`
6. **Leverage hooks** - Auto-setup on directory entry with `[hooks.enter]`

## Extending Examples

Feel free to:
- Add more tasks specific to your project
- Include pre-commit hooks
- Add environment-specific configurations
- Create local overrides with `.mise.local.toml`

For full mise documentation, visit: https://mise.jdx.dev

---

**Questions?** Check the [Testing Guide](TESTING.md) or open an issue on GitHub.
