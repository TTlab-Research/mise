# Syntax Highlighting Test Report

## Test Files Created

### test.mise.toml (Basic)
```toml
[tools]
python = "3.12"
node = "22"
rust = "stable"

[env]
NODE_ENV = "development"
VIRTUAL_ENV = "{{ cwd }}/.venv"
MY_CUSTOM_VAR = "test"

[tasks.dev]
description = "Run development server"
run = "npm run dev"
depends = ["install"]

[tasks.test]
description = "Run tests"
run = "pytest"

[hooks.enter]
run = "uv sync --quiet"
```

### .mise.toml (Hidden variant)
Same content as test.mise.toml to verify file pattern recognition

### mise.local.toml (Local override)
Same content to test all registered patterns

## Expected Highlighting Results

### Section Headers
- `[tools]` → KEYWORD color
- `[env]` → KEYWORD color
- `[tasks]` → KEYWORD color
- `[hooks]` → KEYWORD color

### Tool Definitions
- `python = "3.12"` → KEY as bare_key, STRING in quotes
- `node = "22"` → KEY as bare_key, STRING in quotes
- `rust = "stable"` → KEY as bare_key, STRING in quotes

### Environment Variables
- `NODE_ENV` → VARIABLE (UPPERCASE pattern)
- `VIRTUAL_ENV` → VARIABLE (UPPERCASE pattern)
- `MY_CUSTOM_VAR` → VARIABLE (UPPERCASE pattern)

### Template Expressions
- `{{ cwd }}` → STRING.SPECIAL (template syntax)
- `{{ env.HOME }}` → STRING.SPECIAL (template syntax)

### Task Properties
- `description =` → PROPERTY
- `run =` → PROPERTY
- `depends =` → PROPERTY

### Values
- `"development"` → STRING
- `"pytest"` → STRING
- `"uv sync --quiet"` → STRING

## Testing Checklist

### Syntax Highlighting
- [ ] Open test.mise.toml in Zed
- [ ] Verify section headers are highlighted
- [ ] Verify tool names and versions are colored
- [ ] Verify environment variables stand out
- [ ] Verify template syntax is distinct
- [ ] Verify task properties are highlighted

### Language Recognition
- [ ] test.mise.toml → Shows "Mise" in language mode
- [ ] .mise.toml → Shows "Mise" in language mode
- [ ] mise.local.toml → Shows "Mise" in language mode
- [ ] .mise.local.toml → Shows "Mise" in language mode
- [ ] Non-matching names → Shows "TOML" or auto-detected

### Performance
- [ ] File opens instantly (< 1 second)
- [ ] Syntax highlighting renders in < 100ms
- [ ] No visual stuttering or lag
- [ ] No CPU spike when scrolling

### Theme Compatibility
- [ ] Works with default Zed theme
- [ ] Works with dark themes
- [ ] Works with light themes
- [ ] Colors are theme-aware (not hardcoded)

## Test Results

### Date: 2026-02-05
### Zed Version: 0.222.2
### Extension Version: 0.2.0

### Compilation
- Status: PASS
- Warnings: 0
- Errors: 0
- Build time: 3.42s

### Language Detection
- [ ] Pending manual test in Zed

### Syntax Highlighting
- [ ] Pending manual test in Zed

### Performance
- [ ] Pending manual test in Zed

## Notes

- TreeSitter grammar is standard TOML (ikatyang/tree-sitter-toml)
- Highlights are defined in languages/mise/highlights.scm
- All queries use performance-optimized predicates
- No regex patterns used (all #any-of? and simple matches)

## Test Commands

To manually test:
```bash
# Open in Zed
zed test.mise.toml

# Or with all test variants
touch test.mise.toml .mise.toml mise.local.toml .mise.local.toml
zed .
```

Then verify:
1. Bottom right shows "Mise" language mode
2. Colors are applied to all elements
3. No errors in developer console
4. Performance is smooth
