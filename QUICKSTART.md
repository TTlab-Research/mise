# Quick Start - Mise Extension for Zed

Complete in 3 minutes.

## Installation

### From Zed Registry
```
Open Zed → Extensions → Search "mise" → Install
```

### As Development Extension
```bash
git clone https://github.com/TTlab-Research/zed-mise.git
cd zed-mise
zed .
# Then: Cmd+Shift+P → "Extensions: Install Dev Extension" → Select folder
```

### Manual Installation
```bash
mkdir -p ~/.config/zed/extensions
ln -sf /path/to/zed-mise ~/.config/zed/extensions/mise
# Restart Zed
```

## Testing Syntax Highlighting

Create a test configuration file:

```bash
cat > test.mise.toml << 'EOF'
[tools]
python = "3.12"
node = "22"

[env]
NODE_ENV = "development"

[tasks.dev]
run = "npm run dev"
EOF
```

Open the file in Zed. You should see:
- Color-coded section headers ([tools], [env], [tasks])
- Highlighted keywords and properties
- Environment variables in distinct color

## Using Configuration Templates

### Direct Copy Method
```bash
cp examples/python-uv.toml mise.toml
# Or choose from:
# - examples/nextjs-fullstack.toml
# - examples/go-minimal.toml
# - examples/rust-release.toml
```

### Zed Native Tasks
When working in this project:
```
Cmd+Shift+P → search "mise:"
Select desired template → configuration file created
```

### Customization
Edit the generated mise.toml for your project:
- Update tool versions as needed
- Add project-specific environment variables
- Customize task definitions

## Available Templates

| Template | Purpose |
|----------|---------|
| python-uv.toml | Python development with uv package manager |
| nextjs-fullstack.toml | Next.js frontend with Python backend |
| go-minimal.toml | Go project setup |
| rust-release.toml | Rust with optimized release builds |

## Documentation

- **[Testing Guide](docs/TESTING.md)** - Comprehensive feature testing procedures
- **[Examples](docs/EXAMPLES.md)** - Production-ready configurations for different stacks
- **[Architecture](docs/ARCHITECTURE.md)** - Technical implementation details
- **[Mise Official Docs](https://mise.jdx.dev)** - Complete mise documentation

## Troubleshooting

### Syntax highlighting not showing colors
1. Verify file is named mise.toml or .mise.toml
2. Check the language mode indicator at bottom right of editor
3. Should display "Mise" as the language mode
4. If showing "TOML", manually change: Cmd+Shift+P → "Change Language Mode" → "Mise"

### Zed tasks not appearing
1. Ensure you are in the mise extension project directory
2. Run: Cmd+Shift+P → search "mise:"
3. Verify .zed/tasks.json file exists in project root

## What This Extension Provides

- Professional syntax highlighting for mise configuration files
- Complete support for all mise sections and keywords
- TreeSitter-based syntax engine with sub-10ms performance per file
- Zero external dependencies or licensing requirements
- Automatic language detection and recognition

## Contributing and Support

Found an issue or have a feature request? Please open an issue:
https://github.com/TTlab-Research/zed-mise/issues

---

Refer to the documentation directory for detailed testing information and configuration examples.
