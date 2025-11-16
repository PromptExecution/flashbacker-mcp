# Poly-Proxy MCP Tool Registry

**Status**: Aspirational / In Development

This directory contains the **poly-proxy MCP tool registry** - a codified opinion system for configuring and using MCP tools within the b00t framework.

## Concept

The poly-proxy registry provides:

1. **Codified Opinions** - Best practices for tool configuration and usage
2. **Standardized Patterns** - Consistent interfaces across diverse tools
3. **Proxy Layer** - Intelligent routing and tool selection
4. **Version Management** - Compatibility tracking and requirements
5. **Tool Discovery** - Automatic capability detection and registration

## Structure

```
registry/
├── tools/          # Tool configurations with codified opinions
├── opinions/       # Shared opinion templates and patterns
├── schemas/        # Validation schemas for tool definitions
└── README.md       # This file
```

## Philosophy

Rather than requiring agents to learn each tool's unique quirks, the poly-proxy registry:

- **Abstracts complexity** - Standard interfaces hide tool-specific details
- **Enforces best practices** - Codified opinions prevent common mistakes
- **Enables discovery** - Tools self-describe their capabilities
- **Supports composition** - Tools can be chained through the proxy

## Example: Flashbacker

Flashbacker's registry entry codifies opinions such as:

- **Node.js version**: Prefer 22.x LTS for optimal compatibility
- **Initialization**: Always run `flashback init` before other commands
- **Session management**: Use automatic hooks, not manual saves
- **Persona vs Agent**: Personas for quick analysis, agents for deep work
- **Memory patterns**: Document decisions (why), not just facts (what)

These opinions are enforced through the proxy layer, ensuring consistent usage across the agent hive.

## Integration with B00t

The poly-proxy registry extends b00t's datum system:

- **Datum TOML** - Defines what the tool is and how to install it
- **Registry Entry** - Defines how the tool should be used (opinions)
- **Proxy Layer** - Enforces opinions and provides standard interface

## Status

This is an **aspirational** design being built out iteratively:

- ✅ Datum integration (flashbacker.cli.toml, flashbacker.mcp.toml)
- ✅ MCP server wrapper (flashbacker-mcp)
- ✅ Docker fallback support
- 🚧 Codified opinions configuration
- 🚧 Proxy layer implementation
- 🚧 Tool discovery automation
- 🚧 Cross-tool composition patterns

## Vision

The end goal is a registry where:

```bash
# Agent queries the registry
b00t registry discover state-management

# Registry returns flashbacker with codified opinions
# Agent follows opinions automatically:
b00t registry use flashbacker init --follow-opinions

# Proxy enforces: checks Node.js version, validates env, runs init
# Agent gets consistent, optimal behavior without memorizing details
```

## Contributing

As this is aspirational, contributions should:

1. Define clear opinions backed by real-world usage
2. Provide validation for those opinions
3. Document the rationale (why this opinion?)
4. Consider multi-agent coordination scenarios

## See Also

- [B00T_INTEGRATION.md](../B00T_INTEGRATION.md) - Current b00t integration
- [_b00t_/](../_b00t_/) - Datum configurations
- [flashbacker-mcp](../flashbacker-mcp) - MCP server implementation
