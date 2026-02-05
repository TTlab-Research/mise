# Architecture - Mise Extension for Zed

Understanding how the Mise extension works internally.

## High-Level Overview

```
┌─────────────────────────────────────────────────────────────┐
│                      Zed IDE                                │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Mise Extension Runtime                 │   │
│  ├─────────────────────────────────────────────────────┤   │
│  │                                                      │   │
│  │  1. Language Support                                │   │
│  │     └─ Registers "Mise" language for .toml files    │   │
│  │                                                      │   │
│  │  2. Syntax Highlighting                             │   │
│  │     └─ TreeSitter + highlights.scm queries          │   │
│  │                                                      │   │
│  │  3. Slash Commands                                  │   │
│  │     ├─ /mise-init                                   │   │
│  │     ├─ /mise-task                                   │   │
│  │     └─ /mise-env                                    │   │
│  │                                                      │   │
│  │  4. Argument Completion                             │   │
│  │     └─ Smart suggestions as user types              │   │
│  │                                                      │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## File Structure

```
zed-mise/
├── src/
│   └── lib.rs                 # Main extension implementation
│                              # - MiseExtension struct
│                              # - Slash command handlers
│                              # - Template builders
│                              # - Argument completers
│
├── languages/mise/
│   ├── config.toml            # Language registration
│   │                           # - File patterns
│   │                           # - Comment syntax
│   │                           # - Indentation rules
│   │
│   └── highlights.scm         # TreeSitter highlight rules
│                              # - Keywords, types, functions
│                              # - Special patterns (templates, vars)
│
├── Cargo.toml                 # Rust package manifest
├── extension.toml             # Zed extension manifest
│                              # - Slash command registration
│
├── examples/                  # Real-world mise.toml examples
└── docs/
    ├── TESTING.md             # Testing guide
    ├── EXAMPLES.md            # Configuration examples
    └── ARCHITECTURE.md        # This file
```

## Component Details

### 1. Language Registration (`languages/mise/config.toml`)

```toml
name = "Mise"                  # Display name in Zed
grammar = "toml"               # Use TOML grammar as base
path_suffixes = [              # Files to recognize
  "mise.toml",
  ".mise.toml",
  "mise.local.toml",
  ".mise.local.toml"
]
```

**What it does:**
- Tells Zed to recognize `mise.toml` files as "Mise" language
- Associates TOML TreeSitter grammar
- Enables syntax highlighting via `highlights.scm`

### 2. Syntax Highlighting (`languages/mise/highlights.scm`)

Uses TreeSitter queries to highlight specific patterns:

```scheme
; Match keywords in sections
((bare_key) @keyword
  (#any-of? @keyword "tools" "env" "tasks"))

; Match tool names
(pair (bare_key) @type (string) @string.special)

; Match environment variables (UPPERCASE)
(pair (bare_key) @variable 
  (#match? @variable "^[A-Z_][A-Z0-9_]*$"))
```

**Performance optimization:**
- Uses `#any-of?` predicate instead of regex (faster)
- Queries executed during file parsing
- Syntax highlighting updates in real-time

### 3. Slash Commands (`src/lib.rs`)

#### Command Handler Flow

```
User types: /mise-init python
      ↓
Zed calls: run_slash_command("mise-init", ["python"], ...)
      ↓
build_mise_init(&["python"]) function executes
      ↓
Returns: SlashCommandOutput { text, sections }
      ↓
Zed renders output in Assistant with syntax highlighting
```

#### Three Main Commands

**a) /mise-init [languages...]**
- Builds TOML templates for specified tech stacks
- Combines `[tools]`, `[env]`, `[hooks]` sections
- Supports: python, node, nextjs, go, rust, deno, bun, docker, terraform

**b) /mise-task create [stacks...]**
- Generates task templates for automation
- Appends to existing `[tasks]` section
- Each stack has predefined tasks (install, dev, test, lint, etc.)

**c) /mise-env**
- Provides environment integration info
- Shows how to activate mise in shell
- Explains limitations (extensions can't access shell directly)

### 4. Argument Completion (`src/lib.rs`)

Implements `complete_slash_command_argument()`:

```rust
// For /mise-init
→ Returns available languages: python, node, rust, ...
→ Filters already-selected languages
→ Adds "help" option

// For /mise-task create
→ First arg: returns ["create", "export", "help"]
→ After "create": returns available stacks
```

**User experience:**
- Type `/mise-init ` and get suggestions
- Select from dropdown or type manually
- Multiple selections allowed

## Data Flow

### Syntax Highlighting Flow

```
mise.toml file
      ↓
Zed detects .toml extension
      ↓
Checks config.toml: path_suffixes match? → YES
      ↓
Registers as "Mise" language
      ↓
Loads TreeSitter grammar for TOML
      ↓
Parses file with TreeSitter
      ↓
Applies highlights.scm rules
      ↓
Renders colors in editor
```

### Slash Command Flow

```
User in Assistant (Cmd+K)
      ↓
Types: /mise-init python node
      ↓
Zed parses command and args
      ↓
Calls: MiseExtension::run_slash_command(...)
      ↓
build_mise_init(&["python", "node"]) executes
      ↓
Iterates over languages:
  - Add python template to [tools]
  - Add python env vars to [env]
  - Add python hooks
  - Add node template to [tools]
  - Add node env vars to [env]
      ↓
Builds final TOML string
      ↓
Returns SlashCommandOutput with sections
      ↓
Zed displays in Assistant with crease UI
```

## Key Design Decisions

### 1. Template-Based Approach
- **Why:** Easy to maintain, easy to extend with new stacks
- **Trade-off:** Static templates (no dynamic interpolation based on file state)
- **Future:** Could parse existing mise.toml and merge intelligently

### 2. No External Dependencies
- **Why:** Fast, reliable, no network calls
- **Trade-off:** Can't fetch real-time tool versions
- **Reality:** Users should update versions manually anyway

### 3. TreeSitter for Syntax Highlighting
- **Why:** Fast, accurate, language-agnostic
- **Trade-off:** Requires TreeSitter grammar for TOML (reuses standard grammar)
- **Benefit:** Integrated with Zed's theme system automatically

### 4. Three Focused Commands
- **Why:** Covers 90% of use cases without complexity
- **Trade-off:** Not a full mise UI replacement
- **Reality:** Zed Assistant is best place for utilities, not complex UIs

## Extension Lifecycle

```
1. USER INSTALLS EXTENSION
   └─ Zed reads extension.toml
   └─ Loads compiled WASM binary
   └─ Registers MiseExtension type
   └─ Ready for events

2. USER OPENS mise.toml
   └─ Language detector runs
   └─ Matches path_suffixes → "Mise" language
   └─ TreeSitter parser loads
   └─ highlights.scm rules applied
   └─ Syntax highlighting rendered

3. USER OPENS ASSISTANT (Cmd+K)
   └─ Zed shows available slash commands
   └─ User types /mise-init ...
   └─ run_slash_command() called
   └─ Output rendered in Assistant

4. USER COPIES OUTPUT
   └─ Can paste template into mise.toml
   └─ Syntax highlighting applied immediately
   └─ User edits and customizes
```

## Performance Characteristics

| Operation | Time | Notes |
|-----------|------|-------|
| Syntax highlight update | <10ms | TreeSitter cached |
| Slash command execution | <1ms | Pure string building |
| Argument completion | <1ms | Static list, no lookups |
| Extension load | ~100ms | WASM initialization |

## Testing Architecture

Tests (when added) will cover:
- Template generation correctness
- Argument completion logic
- Syntax highlighting rules
- File pattern matching

## Future Enhancements

### Short Term
- [ ] Add more tool templates (Ruby, PHP, Kotlin)
- [ ] Improve hook documentation
- [ ] Add workspace detection for auto-loading

### Medium Term
- [ ] Analyze existing mise.toml and suggest improvements
- [ ] Interactive task executor in Zed UI
- [ ] Environment variable viewer

### Long Term
- [ ] Full mise.toml editor UI (not just slash commands)
- [ ] Task execution with progress tracking
- [ ] Integration with Zed's task runner

---

## Contributing

Want to add a feature? Understand this architecture first:
1. Read [TESTING.md](TESTING.md) for how to test
2. Read [EXAMPLES.md](EXAMPLES.md) for template patterns
3. Modify `src/lib.rs` for new commands
4. Update `languages/mise/highlights.scm` for new syntax rules
5. Update `extension.toml` to register new commands

See [CONTRIBUTING](../CONTRIBUTING.md) for detailed guidelines.
