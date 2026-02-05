# Testing Mise Extension for Zed

Complete guide to test all features of the Mise extension.

## Prerequisites

- Zed 0.140+ installed
- Extension linked or installed as dev extension
- `mise` CLI installed (optional, for full testing)

## Installation for Testing

### Method 1: Link Extension (Development)

```bash
# Create symlink to extensions folder
mkdir -p ~/.config/zed/extensions
ln -sf /path/to/zed-mise ~/.config/zed/extensions/mise

# Reload Zed or restart the application
```

### Method 2: Install via Zed UI

1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
3. Search: `Extensions: Install Dev Extension`
4. Select the `zed-mise` folder
5. Click "Install"

## Feature Testing

### 1. Syntax Highlighting

**Test Files:**
- Create `test.mise.toml` with various sections
- Create `.mise.local.toml` in a project
- Create `mise.toml` in root

**Expected Highlighting:**
- `[tools]` section header → **Keyword color**
- `[env]` section header → **Keyword color**
- `[tasks]` section header → **Keyword color**
- `[hooks]` section header → **Keyword color**
- Tool names (python, node, rust) → **Type color**
- Tool versions ("3.12", "22", "latest") → **String color**
- Environment variables (UPPERCASE) → **Variable color**
- Task definitions → **Function color**
- Comments (#) → **Comment color**

**Test with provided file:**
```bash
cd /path/to/zed-mise
cat test-mise.toml
```

Open in Zed and verify colors match theme.

### 2. Slash Commands (in Zed Assistant)

**Access Zed Assistant:**
- Press `Cmd+K` (macOS) or `Ctrl+K` (Linux/Windows)
- Or use `Cmd+Shift+P` → search "Assistant: Toggle"

**Test `/mise-init` Command**

```
Type: /mise-init
Expected: List of available languages (python, node, rust, go, etc.)

Type: /mise-init python
Expected: Python template with:
  - python = "3.12"
  - uv = "latest"
  - Environment variables for .venv
  - Hooks for uv sync

Type: /mise-init python node nextjs
Expected: Combined template with all three stacks
```

**Test `/mise-task` Command**

```
Type: /mise-task
Expected: Help message or subcommand options

Type: /mise-task create python
Expected: Python task templates (install, dev, test, lint, format, typecheck)

Type: /mise-task create python nextjs
Expected: Both Python and Next.js task templates combined

Type: /mise-task export
Expected: Instructions for exporting mise tasks to Zed
```

**Test `/mise-env` Command**

```
Type: /mise-env
Expected: Information about how to view mise environment
  - Commands to run in terminal
  - Information about shell integration
```

### 3. Language Detection

**Test file recognition:**

Create files with these names and verify they're recognized as "Mise" language:
- `mise.toml` ✓
- `.mise.toml` ✓
- `mise.local.toml` ✓
- `.mise.local.toml` ✓

Verification:
- Open file in Zed
- Bottom right corner should show "Mise" as language
- Or use: `Cmd+Shift+P` → "Change Language Mode" → should list "Mise"

## Test Checklist

- [ ] Extension installed/linked without errors
- [ ] `test-mise.toml` opens with correct syntax highlighting
- [ ] All syntax elements colored appropriately
- [ ] Zed Assistant accessible (`Cmd+K`)
- [ ] `/mise-init` command visible in Assistant
- [ ] `/mise-init python` generates valid TOML template
- [ ] `/mise-task create` generates task templates
- [ ] `/mise-env` shows environment help
- [ ] Argument completion works (suggestions appear as you type)
- [ ] Language mode correctly identified as "Mise" for .toml files

## Troubleshooting

### Extension doesn't load

1. Check symlink: `ls -la ~/.config/zed/extensions/mise`
2. Rebuild: `cargo build --release`
3. Restart Zed completely
4. Check Zed console: `Cmd+Shift+P` → "Toggle Developer Console"

### Slash commands not visible

1. Verify Zed Assistant is open (`Cmd+K`)
2. Type `/` to see all available commands
3. If not visible, check Zed version: `zed --version` (need 0.140+)
4. Check extension loaded: `Cmd+Shift+P` → "Extensions: Show Extensions"

### Syntax highlighting not working

1. Verify file extension: must be `mise.toml` or `.mise.toml`
2. Check language mode: bottom right should show "Mise"
3. If showing as "TOML", manually change: `Cmd+Shift+P` → "Change Language Mode" → "Mise"
4. Reload: `Cmd+Shift+P` → "Reload Extensions"

### Prettier formatting errors

These are harmless warnings (Prettier doesn't know TOML format). They don't affect functionality.

## Advanced Testing

### Test with Real mise.toml

```bash
# Create a test project
mkdir test-project
cd test-project

# Use /mise-init to generate a mise.toml template
# Then open in Zed to verify syntax highlighting
```

### Test Argument Completion

1. Open Assistant
2. Type `/mise-init ` (with space)
3. Should show completion suggestions: python, node, rust, etc.
4. Type `/mise-task create ` (with space)
5. Should show: python, node, rust, etc.

### Test Multiple Languages

```
/mise-init python go rust docker
```

Should generate a combined template with all four tech stacks.

## Performance Notes

- Syntax highlighting uses TreeSitter (fast, always available)
- Slash commands are instant (no network calls)
- No external dependencies required
- Extension is ~1-2 MB compiled

## Next Steps

- ✅ Test all features locally
- 📤 Submit to Zed Registry (when ready)
- 📦 Set up GitHub releases with pre-compiled binaries
- 🚀 Announce on community channels

---

**Need help?** Open an issue: https://github.com/TTlab-Research/zed-mise/issues
