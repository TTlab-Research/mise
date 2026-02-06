# Zed Extension Best Practices - Mise Case Study

**Documento:** Best practices per estensioni Zed  
**Basato su:** Ricerca ufficiale Zed documentation + community patterns  
**Data:** 2026-02-05

---

## 1. ARCHITETTURA CONSIGLIATA

### Pattern Vincente: Language Extension + Syntax Highlighting

```
┌─────────────────────────────────────────┐
│    Zed Editor (Utente finale)           │
├─────────────────────────────────────────┤
│                                         │
│  ┌─────────────────────────────────┐   │
│  │  Language Support (TOML base)   │   │
│  └─────────────────────────────────┘   │
│               ↓                         │
│  ┌─────────────────────────────────┐   │
│  │  Syntax Highlighting Rules      │   │
│  │  (highlights.scm)               │   │
│  └─────────────────────────────────┘   │
│               ↓                         │
│  ┌─────────────────────────────────┐   │
│  │  Completions (LSP)              │   │
│  │  (optional, per future)         │   │
│  └─────────────────────────────────┘   │
│                                         │
└─────────────────────────────────────────┘
```

### Analisi della scelta:

✅ **PERCHÉ questo pattern per Mise:**

1. **No Slash Commands** (Assistant a pagamento)
   - ❌ Escludere `/mise-init` ecc.
   - ✅ Usare solo features native di Zed

2. **Syntax Highlighting** (Always Free)
   - ✅ Treesitter-based
   - ✅ Zero dipendenze
   - ✅ Performance ottima

3. **Language Features** (Future)
   - Completions per keywords
   - Diagnostica (validazione TOML)
   - Code snippets
   - Hover information

4. **File Generation** (Alternative)
   - Se Zed lo supporta: Code Actions
   - Altrimenti: Documentazione su come creare file manualmente

---

## 2. ANATOMIA DI UNA BUONA EXTENSION ZED

### Struttura Minima

```
my-language-extension/
├── extension.toml           # Manifest (metadata + registrazioni)
├── Cargo.toml               # Rust package
├── src/
│   └── lib.rs               # Extension implementation
├── languages/my-language/
│   ├── config.toml          # Language configuration
│   ├── highlights.scm       # TreeSitter highlighting rules
│   ├── indents.scm          # (optional) Indentation rules
│   ├── outline.scm          # (optional) Outline/breadcrumbs
│   └── injections.scm       # (optional) Syntax injection
├── README.md                # Documentation
└── docs/
    ├── ARCHITECTURE.md
    ├── EXAMPLES.md
    └── TESTING.md
```

### extension.toml - Esempio Completo

```toml
id = "my-language"
name = "My Language"
version = "0.1.0"
schema_version = 1
description = "Support for MyLang files"
repository = "https://github.com/user/zed-my-language"
authors = ["Author Name"]
license = "MIT"

# Language registration (NO slash commands!)
[languages.my-language]
name = "MyLang"
grammar = "mylang"  # or use embedded grammar
path_suffixes = ["*.my", ".mylang"]
```

### Key Principles

| Principio | ✅ DO | ❌ DON'T |
|-----------|------|---------|
| **Dependencies** | Zero external | Process execution if avoidable |
| **Licensing** | MIT, Apache | GPL (incompatible with Zed) |
| **Performance** | <100ms load | Blocking operations |
| **Updates** | Automatic via registry | Manual downloads |
| **UX** | Native Zed patterns | Custom UI frameworks |

---

## 3. MISE EXTENSION - NUOVA ARCHITETTURA

### Cosa Tenere ✅

```rust
// src/lib.rs
struct MiseExtension;

impl zed::Extension for MiseExtension {
    fn new() -> Self { MiseExtension }
    
    // FUTURE: Code actions per generare template
    // fn code_actions(...) { }
}

zed::register_extension!(MiseExtension);
```

```toml
# languages/mise/config.toml
name = "Mise"
grammar = "toml"
path_suffixes = ["mise.toml", ".mise.toml"]

[indent]
unit = "  "
```

```scheme
# languages/mise/highlights.scm
((bare_key) @keyword
  (#any-of? @keyword "tools" "env" "tasks"))
```

### Cosa Rimuovere ❌

```toml
# extension.toml - REMOVE THESE:
[slash_commands.mise-init]      # ← REMOVE (Assistant only)
[slash_commands.mise-task]      # ← REMOVE
[slash_commands.mise-env]       # ← REMOVE
```

### Cosa Aggiungere (Futuro) ✅

```rust
// Code actions (quando Zed lo supporterà)
fn code_actions(
    &self,
    language: LanguageServerId,
    path: PathBuf,
    range: Range<Anchor>,
) -> Vec<CodeAction> {
    // "Generate mise.toml for Python"
    // "Add task template"
    // etc.
}
```

---

## 4. BEST PRACTICE PER SYNTAX HIGHLIGHTING

### TreeSitter Query Patterns

✅ **GOOD:**
```scheme
; Usa predicati veloci
((bare_key) @keyword
  (#any-of? @keyword "tools" "env"))

; Usa match per patterns semplici
(pair (bare_key) @variable
  (#match? @variable "^[A-Z_]+$"))
```

❌ **BAD:**
```scheme
; Evita regex complessi
((bare_key) @keyword
  (#match? @keyword "^(tool|env|task|hook|var|plugin|setting)s?$"))
```

### Performance Tips

1. **Use `#any-of?` instead of multiple patterns**
   ```scheme
   ✅ (#any-of? @keyword "a" "b" "c")
   ❌ (#match? @keyword "^(a|b|c)$")
   ```

2. **Minimize regex usage**
   - Regex evaluates at runtime
   - Predicates compile to bytecode
   - Budget: ~100 predicates per file type

3. **Cache TreeSitter queries**
   - Zed handles this automatically
   - Keep `highlights.scm` under 500 lines

### Color Naming

Use Zed theme colors, non custom:

```scheme
; ✅ GOOD - Zed will respect theme
((keyword) @keyword)    ; Uses theme's keyword color

; ❌ BAD - Overrides theme
((keyword) @blue)       ; Hardcoded color (ignored)
```

---

## 5. TESTING & VALIDATION

### Checklist Minima

- [ ] Extension compiles: `cargo build --release`
- [ ] No warnings: `cargo check 2>&1 | grep warn`
- [ ] Syntax highlighting works on example files
- [ ] Language recognized: bottom-right shows "Mise"
- [ ] Load time < 100ms
- [ ] Zero external network calls

### File Pattern Matching

Test all registered patterns:
```bash
touch test.mise.toml      # Should be "Mise" language
touch .mise.toml          # Should be "Mise" language
touch mise.local.toml     # Should be "Mise" language
```

---

## 6. DOCUMENTAZIONE ESSENZIALE

Ogni extension deve avere:

1. **README.md** (5 min read)
   - What it does
   - How to install
   - Basic features

2. **TESTING.md** (10 min read)
   - How to test each feature
   - Troubleshooting

3. **ARCHITECTURE.md** (15 min read)
   - Technical overview
   - Design decisions
   - Maintenance guide

**Esempio:** Vedi nostro progetto mise

---

## 7. SUBMISSION CHECKLIST

Quando pronto per Zed Registry:

- [ ] Grammar/syntax highlighting works perfectly
- [ ] No slash commands (or optional via Cmd+K)
- [ ] Comprehensive README
- [ ] GitHub repository with MIT/Apache license
- [ ] Clean git history (conventional commits)
- [ ] 0.1.0+ version bump
- [ ] Works with latest Zed (0.150+)

---

## 8. ALTERNATIVE APPROACHES

Se gli utenti VOGlIONO comandi senza pagare:

### Option A: .zed/tasks.json (Recommended)
```json
{
  "tasks": {
    "Mise: Generate Python": {
      "label": "mise: Generate Python config",
      "command": "bash",
      "args": ["-c", "cat examples/python-uv.toml > mise.toml"]
    }
  }
}
```
- ✅ Nativo in Zed
- ✅ Sempre disponibile
- ✅ No extension needed

### Option B: Snippet di documentazione
```
Docs → "Come generare mise.toml"
1. Copia examples/python-uv.toml → mise.toml
2. Modifica per il tuo progetto
```

### Option C: CLI tool separato
```bash
npm install -g mise-init
mise-init python > mise.toml
```
- ✅ Cross-editor
- ✅ Sempre gratis

---

## 9. ROADMAP CONSIGLIATA

### v0.1.0 (Current)
- ✅ Syntax highlighting
- ✅ Language recognition
- ✅ Comprehensive docs

### v0.2.0 (Next)
- [ ] Add LSP server for completions
- [ ] Add diagnostic validation
- [ ] Performance metrics

### v0.3.0+ (Future)
- [ ] Code actions
- [ ] Snippet library
- [ ] Integration with mise CLI

---

## 10. CONCLUSIONE

### La Best Practice per Mise:

```
┌──────────────────────────────────┐
│  ✅ Language Extension GRATUITA  │
├──────────────────────────────────┤
│ • Syntax highlighting            │
│ • Language recognition           │
│ • TreeSitter integration         │
│ • No dependencies                │
│ • Always available               │
│                                  │
│ + Documentation                  │
│ + Examples                       │
│ + Testing guide                  │
└──────────────────────────────────┘
```

### Avoid:
- ❌ Slash commands (Assistant = paid)
- ❌ External network calls
- ❌ Heavy dependencies
- ❌ Complex UI frameworks

### Embrace:
- ✅ Simple, focused features
- ✅ Native Zed patterns
- ✅ TreeSitter for syntax
- ✅ Clear documentation

---

**Fonti:**
- [Zed Extension Development](https://zed.dev/docs/extensions/developing-extensions)
- [Language Extensions](https://zed.dev/docs/extensions/languages)
- [Zed Extensions Registry](https://github.com/zed-industries/extensions)

---

**Prossimo step:** Refactorizziamo l'extension per seguire questa best practice?
