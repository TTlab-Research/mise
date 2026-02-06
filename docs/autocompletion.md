# Autocompletion Roadmap

This document outlines the plan for adding comprehensive autocompletion to the mise extension for Zed.

## Current Status

✅ **Phase 1: Foundation**
- Syntax highlighting with TreeSitter (complete)
- Official mise JSON schema downloaded (complete)
- Schema covers 100% of mise configuration options

## Future Phases

### Phase 2: Language Server Protocol (LSP)
- Implement basic LSP server for validation
- Provide intelligent property suggestions
- Display type information on hover
- Timeline: TBD (based on community feedback)

### Phase 3: Dynamic Completion
- Real-time tool version suggestions
- Task dependency autocompletion
- File path suggestions for `sources`/`outputs`
- Timeline: TBD

## Schema Coverage

The official mise JSON schema includes completions for all configuration options. See `schemas/mise.json` for the complete specification.

## References

- [mise Configuration Guide](https://mise.jdx.dev/configuration.html)
- [mise JSON Schema](https://mise.jdx.dev/schema/mise.json)
- [LSP Specification](https://microsoft.github.io/language-server-protocol/)
