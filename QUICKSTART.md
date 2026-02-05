# Quick Start - Mise Extension for Zed

Get started in 3 minutes! 🚀

## 1️⃣ Install the Extension

### Option A: From Zed Registry (Soon!)
```
Open Zed → Extensions → Search "mise" → Install
```

### Option B: Install as Dev Extension (Now!)
```bash
# Clone the repo
git clone https://github.com/TTlab-Research/zed-mise.git

# Open in Zed
cd zed-mise
zed .

# Install as dev extension
Cmd+Shift+P → "Extensions: Install Dev Extension" → Select this folder
```

### Option C: Manual Symlink
```bash
mkdir -p ~/.config/zed/extensions
ln -sf /path/to/zed-mise ~/.config/zed/extensions/mise
# Restart Zed
```

## 2️⃣ Test the Extension

### Test Syntax Highlighting
```bash
# Create a test file
cat > test.mise.toml << 'EOF'
[tools]
python = "3.12"
node = "22"

[env]
NODE_ENV = "development"

[tasks.dev]
run = "npm run dev"
EOF

# Open in Zed - should have colors!
```

### Test Slash Commands
1. Press `Cmd+K` to open Zed Assistant
2. Type `/mise-init python` → should generate Python template
3. Type `/mise-task create nextjs` → should create Next.js tasks
4. Type `/mise-env` → should show environment info

## 3️⃣ Use in Your Project

### Copy a Template
```bash
# Copy an example to your project
cp examples/python-uv.toml your-project/mise.toml

# Or generate one from Assistant
# (see slash commands above)
```

### Verify Syntax Highlighting
Open `mise.toml` in Zed - should show:
- 🎨 Colored sections (`[tools]`, `[env]`, `[tasks]`)
- 📝 Highlighted keywords
- 🔤 Environment variables

## 🎯 Available Slash Commands

In Zed Assistant (Cmd+K), type:

```
/mise-init [language...]
├─ Generate mise.toml template
├─ Supports: python, node, nextjs, go, rust, deno, bun, docker, terraform
└─ Example: /mise-init python nextjs

/mise-task create [stack...]
├─ Add task templates to mise.toml
├─ Supports same stacks as /mise-init
└─ Example: /mise-task create python

/mise-env
├─ View mise environment information
├─ Shows how to access tools in shell
└─ Integration tips for your setup
```

## 📚 Learn More

- **[Testing Guide](docs/TESTING.md)** - Comprehensive feature testing
- **[Examples](docs/EXAMPLES.md)** - Real-world configurations
- **[Mise Docs](https://mise.jdx.dev)** - Official mise documentation
- **[Issues](https://github.com/TTlab-Research/zed-mise/issues)** - Report bugs

## 🔧 Troubleshooting

### "I don't see the Assistant"
1. Check Zed version: `zed --version` (need 0.140+)
2. Try: `Cmd+Shift+P` → type "Assistant" → select "Assistant: Toggle"
3. Configure AI: Settings → search "assistant" → add API key

### "Slash commands don't appear"
1. Make sure you're in Zed Assistant (Cmd+K)
2. Type `/` to see all commands
3. If still missing, reload: `Cmd+Shift+P` → "Reload Extensions"

### "Syntax highlighting not working"
1. File must be named `mise.toml` or `.mise.toml`
2. Check language mode: bottom right should say "Mise"
3. If it says "TOML": manually change to "Mise"

## ✅ What You Get

- ✨ Syntax highlighting for `mise.toml` files
- 🎯 3 powerful slash commands for generating configurations
- 📚 Smart argument completion
- 🚀 Zero external dependencies
- ⚡ Lightning-fast performance

## 🤝 Contributing

Found a bug? Have an idea? Open an issue:
https://github.com/TTlab-Research/zed-mise/issues

---

**Ready?** Press `Cmd+K` and type `/mise-init python` to get started! 🎉
