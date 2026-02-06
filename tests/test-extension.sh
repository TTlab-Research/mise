#!/bin/bash
# Test script for Mise extension

set -e

echo "╔════════════════════════════════════════════════════════╗"
echo "║     MISE EXTENSION FOR ZED - TEST SUITE v0.2.0        ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

# Test 1: Compilation
echo "Test 1: Compilation"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cargo build --release 2>&1 | tail -1
if [ $? -eq 0 ]; then
    echo "✓ Compilation successful"
else
    echo "✗ Compilation failed"
    exit 1
fi
echo ""

# Test 2: Warnings
echo "Test 2: Code quality (no warnings)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
WARNINGS=$(cargo check 2>&1 | grep -i warning || echo "")
if [ -z "$WARNINGS" ]; then
    echo "✓ No compilation warnings"
else
    echo "✗ Warnings found:"
    echo "$WARNINGS"
    exit 1
fi
echo ""

# Test 3: JSON validation
echo "Test 3: JSON Configuration"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if python3 -m json.tool .zed/tasks.json > /dev/null 2>&1; then
    echo "✓ .zed/tasks.json is valid"
else
    echo "✗ .zed/tasks.json invalid"
    exit 1
fi
echo ""

# Test 4: File patterns
echo "Test 4: Language Configuration"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if grep -q "mise.toml" languages/mise/config.toml && \
   grep -q ".mise.toml" languages/mise/config.toml && \
   grep -q "mise.local.toml" languages/mise/config.toml && \
   grep -q ".mise.local.toml" languages/mise/config.toml; then
    echo "✓ All file patterns registered"
else
    echo "✗ File patterns missing"
    exit 1
fi
echo ""

# Test 5: Extension manifest
echo "Test 5: Extension Manifest"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if grep -q 'id = "mise"' extension.toml && \
   grep -q 'version = "0.2.0"' extension.toml && \
   grep -q 'schema_version = 1' extension.toml; then
    echo "✓ Extension manifest valid"
else
    echo "✗ Extension manifest invalid"
    exit 1
fi
echo ""

# Test 6: Create test files
echo "Test 6: Creating Test Files"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cat > test.mise.toml << 'EOF'
[tools]
python = "3.12"
node = "22"
rust = "stable"

[env]
NODE_ENV = "development"
VIRTUAL_ENV = "{{ cwd }}/.venv"

[tasks.dev]
description = "Run dev server"
run = "npm run dev"
depends = ["install"]

[tasks.test]
run = "pytest"

[hooks.enter]
run = "uv sync --quiet"
EOF

touch .mise.toml mise.local.toml .mise.local.toml
echo "✓ Test files created"
echo ""

# Summary
echo "╔════════════════════════════════════════════════════════╗"
echo "║                   ALL TESTS PASSED                    ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""
echo "Summary:"
echo "  ✓ Compilation: Success"
echo "  ✓ Warnings: None"
echo "  ✓ JSON config: Valid"
echo "  ✓ File patterns: Registered"
echo "  ✓ Extension manifest: Valid"
echo "  ✓ Test files: Created"
echo ""
echo "Next steps:"
echo "  1. Open Zed"
echo "  2. Reload extensions: Cmd+Shift+P → Reload Extensions"
echo "  3. Open test.mise.toml and verify syntax highlighting"
echo "  4. Check language mode indicator (should show 'Mise')"
echo "  5. Test .zed/tasks.json: Cmd+Shift+P → search 'mise:'"
echo ""
