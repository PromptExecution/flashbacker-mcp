# Flashbacker B00t Integration

This document describes how flashbacker integrates with the [b00t framework](https://github.com/elasticdotventures/_b00t_) - an "agentic hive operating system" that provides AI agents with comprehensive environmental awareness and tool capabilities.

## What is b00t?

b00t is a context-aware development framework that:
- Provides intelligent abstraction and unified tool discovery
- Enables AI agents to learn available tools on-demand via `b00t learn {skill}`
- Manages 50+ MCP (Model Context Protocol) tools
- Uses a **Datum Registry** (TOML files) for tool configuration
- Operates as a "context manager" for multi-agent coordination

## How Flashbacker Integrates

Flashbacker integrates with b00t through the **Datum Registry** system:

### 1. CLI Datum (`_b00t_/flashbacker.cli.toml`)

Defines flashbacker as a CLI tool that b00t can:
- **Detect** - Check if flashbacker is installed
- **Install** - Install via npm globally
- **Version Check** - Track version requirements
- **Learn** - Provide usage examples and documentation
- **Use** - Execute flashback commands

**Key capabilities:**
- 20 AI personas (architect, security, refactorer, etc.)
- 12 core commands (init, persona, agent, memory, etc.)
- Session continuity management
- Claude Code integration (slash commands, agents, hooks)

### 2. MCP Datum (`_b00t_/flashbacker.mcp.toml`)

Exposes flashbacker as an MCP server with two implementation options:

**Priority 0 (Preferred):** Direct CLI wrapper via `b00t-mcp`
- Lightweight integration through b00t's MCP bridge
- Wraps flashback CLI commands as MCP tools
- Requires flashbacker CLI to be installed

**Priority 10 (Fallback):** Docker container
- Runs flashbacker in containerized environment
- Auto-builds `flashbacker-b00t:latest` image if missing
- Includes MCP server wrapper (`mcp-server.js`)
- Volume-mounts current directory to `/workspace`

### 3. Docker Image (Optional)

For containerized deployment:
- Based on Node.js 22 Alpine
- Pre-built flashback CLI
- MCP server wrapper included
- Optimized for b00t integration

## Installation

### Option 1: As a b00t Datum (Recommended)

```bash
# Copy datum files to b00t's registry (choose appropriate location)
# Option A: User-level datums
cp _b00t_/*.toml ~/.b00t/

# Option B: Legacy dotfiles path (if using older b00t)
cp _b00t_/*.toml ~/.dotfiles/_b00t_/

# Option C: Project-specific datums (for this project only)
# Files already in _b00t_/ will be discovered automatically

# Let b00t install flashbacker
b00t install flashbacker

# Or use b00t's learning system
b00t learn flashbacker
```

### Option 2: Manual Installation

```bash
# Install CLI globally
npm install -g flashbacker

# Build Docker image (optional)
docker build -t flashbacker-b00t:latest .

# Initialize in a project
flashback init
```

## Usage with b00t

### Via b00t CLI

```bash
# Learn about flashbacker
b00t learn flashbacker

# Check if installed
b00t detect flashbacker

# Get status
b00t status flashbacker

# Use through b00t
b00t flashback init
b00t flashback persona architect "review auth system"
```

### Via b00t MCP

```bash
# Call flashbacker through MCP
b00t-mcp call flashback_init
b00t-mcp call flashback_persona --persona architect --request "review auth"
b00t-mcp call flashback_memory --action add --content "Uses PostgreSQL 15"
```

### In Agent Conversations

When b00t agents have flashbacker available, they can:

```
# Agent using b00t learn
@agent please learn about flashbacker and initialize it in this project

# Agent using MCP tools
@agent use flashbacker to analyze our architecture with the architect persona

# Agent managing project memory
@agent add "Uses microservices architecture" to flashbacker memory
```

## Datum Structure

### CLI Datum Fields

```toml
[b00t]
name = "flashbacker"              # Tool identifier
type = "cli"                       # Datum type
hint = "Description"               # Human-readable description
lfmf_category = "development"      # Category for LFMF system

install = "npm install -g flashbacker"  # Installation command
update = "npm update -g flashbacker"    # Update command
version = "flashback --version"         # Version check
version_regex = '''regex pattern'''     # Version extraction

[b00t.desires]
version = ">=2.4.1"               # Desired version constraint

[b00t.requires]
node = ">=18.0.0 <25.0.0"        # Node.js requirement
npm = ">=9.0.0"                  # npm requirement

[b00t.learn]
topic = "flashbacker"            # Learning topic
auto_digest = true               # Auto-process documentation

[[b00t.usage]]
description = "..."              # Usage example description
command = "..."                  # Example command
```

### MCP Datum Fields

```toml
[b00t]
name = "flashbacker"             # MCP server identifier
type = "mcp"                     # MCP datum type
hint = "Description"             # Human-readable description

[[b00t.mcp_server]]
priority = 0                     # Implementation priority (0=preferred)
command = "b00t-mcp"            # Execution command
requires = ["flashbacker"]       # Dependencies
transport = "stdio"              # Communication protocol
```

## MCP Tools Exposed

The flashbacker MCP server exposes these tools:

| Tool | Description |
|------|-------------|
| `flashback_init` | Initialize project |
| `flashback_persona` | AI persona analysis |
| `flashback_agent` | Gather agent context |
| `flashback_memory` | Manage project memory |
| `flashback_working_plan` | Manage working plan |
| `flashback_save_session` | Save session insights |
| `flashback_session_start` | Load session context |
| `flashback_discuss` | Multi-persona discussion |
| `flashback_debt_hunter` | Detect technical debt |
| `flashback_fix_master` | Surgical fix methodology |
| `flashback_doctor` | System diagnostics |
| `flashback_status` | Installation status |

## AI Personas Available

20 specialist personas for analysis:

**Core Development:**
- architect, refactorer, performance, security

**Domain-Specific:**
- frontend, backend, database-architect, api-designer

**Infrastructure:**
- devops, platform-engineer, data-engineer, docker-master

**Quality & Analysis:**
- qa, code-critic, debt-hunter, analyzer

**Specialized:**
- john-carmack, fix-master, mentor, product

## B00t Philosophy Alignment

Flashbacker aligns with b00t's core principles:

✅ **Lazy Loading** - Personas/agents loaded on-demand, not preloaded
✅ **DRY Principle** - Leverages existing OSS (tree-sitter, ripgrep)
✅ **Context Awareness** - Session continuity and project memory
✅ **Tribal Knowledge** - REMEMBER.md captures team knowledge
✅ **Multi-Agent Coordination** - Multi-persona discussions
✅ **Tool Discovery** - Self-documenting via datum TOML
✅ **Toil Reduction** - Automates session management and context loading

## Integration Workflow

1. **Installation**: b00t detects Node.js/npm, installs flashbacker
2. **Detection**: b00t verifies `flashback` command availability
3. **Learning**: Agents can `b00t learn flashbacker` for capabilities
4. **Usage**: Direct CLI, MCP tools, or b00t-wrapped commands
5. **Coordination**: Multiple agents share project memory (REMEMBER.md)

## Docker Integration

The Docker image supports b00t's containerized workflows:

```bash
# Build image
docker build -t flashbacker-b00t:latest .

# Run with volume mount
docker run -v $(pwd):/workspace flashbacker-b00t:latest flashback init

# Run MCP server
docker run -i flashbacker-b00t:latest node /app/mcp-server.js
```

The b00t MCP datum automatically handles building and running the container when the CLI is unavailable.

## Files Created

When flashbacker is used in a project, it creates:

```
.claude/
├── agents/              # Claude Code agent definitions
│   ├── architect.md
│   ├── security.md
│   └── ... (20 specialists)
├── commands/fb/         # Slash commands
│   ├── persona.md
│   ├── memory.md
│   └── ... (12 commands)
├── flashback/
│   ├── config/
│   │   └── flashback.json
│   ├── memory/
│   │   ├── REMEMBER.md
│   │   └── WORKING_PLAN.md
│   ├── personas/        # Persona templates
│   └── scripts/
│       └── session-start.sh
└── hooks.json          # SessionStart hook
```

These files integrate with:
- Claude Code's native agent system
- Slash command infrastructure
- Hook system for automatic context loading

## Version Requirements

- **Node.js**: 18.x, 20.x, or 22.x LTS
- **npm**: 9.x or later
- **b00t**: Compatible with b00t datum registry system
- **Flashbacker**: 2.4.1+

## Troubleshooting

### b00t can't detect flashbacker

```bash
# Check installation
which flashback
flashback --version

# Reinstall
npm install -g flashbacker

# Update datum
b00t detect flashbacker --refresh
```

### MCP server fails to start

```bash
# Test MCP server directly
node mcp-server.js

# Test with Docker
docker run -i flashbacker-b00t:latest node /app/mcp-server.js

# Check b00t MCP logs
b00t-mcp status flashbacker
```

### Docker image not building

```bash
# Clean build
docker build --no-cache -t flashbacker-b00t:latest .

# Check Node.js version in container
docker run --rm flashbacker-b00t:latest node --version
```

## Resources

- **b00t Framework**: https://github.com/elasticdotventures/_b00t_
- **Flashbacker**: https://github.com/agentsea/flashbacker
- **User Guide**: https://github.com/agentsea/flashbacker/blob/main/docs/user-guide/USER_GUIDE.md
- **MCP Protocol**: https://modelcontextprotocol.io/

## Contributing

To improve b00t integration:

1. Test with actual b00t installation
2. Update datum TOML files in `_b00t_/`
3. Follow b00t's datum patterns
4. Document in this file
5. Submit PR

## Poly-Proxy MCP Tool Registry (Aspirational)

Flashbacker includes an **aspirational** poly-proxy MCP tool registry with codified opinions on how to configure and use MCP tools.

### Concept

The poly-proxy registry provides:

- **Codified Opinions** - Best practices for tool configuration (e.g., "prefer Node.js 22.x", "always init before use")
- **Standardized Patterns** - Consistent interfaces across diverse tools
- **Proxy Layer** - Intelligent routing and enforcement of best practices
- **Tool Discovery** - Automatic capability detection
- **Multi-Agent Coordination** - Shared patterns for hive missions

### Registry Structure

```
registry/
├── tools/
│   └── flashbacker.opinions.toml  # Codified opinions for flashbacker
├── opinions/                        # Shared opinion templates
├── schemas/
│   └── tool-opinions.schema.toml   # Validation schema
└── README.md
```

### Codified Opinions for Flashbacker

The registry includes opinions such as:

**Environment:**
- Preferred Node.js: 22.x LTS
- Minimum: 18.0.0 (ESM support)
- Why: Native modules require specific versions

**Initialization:**
- Required command: `flashback init`
- Required before: All other commands
- Why: Sets up complete infrastructure

**Session Management:**
- Strategy: Automatic (via hooks)
- Manual saves: Discouraged
- Why: Hooks ensure consistency

**Persona vs Agent:**
- Persona: Quick analysis, current conversation, moderate depth
- Agent: Deep analysis, full project context, comprehensive
- Why: Different use cases have different optimal tools

**Memory Patterns:**
- Style: Decision-oriented (capture "why", not just "what")
- Examples: "Uses JWT because..." not just "Uses JWT"
- Why: Future agents need context for decisions

**Working Plan:**
- Task size: Small and actionable (1-2 hours)
- Status tracking: Required
- Why: Better progress tracking and estimates

### Proxy Behaviors (Future)

When implemented, the proxy will:

- **Pre-flight checks**: Verify Node.js version, init status
- **Auto-correct**: Upgrade `init` to `init --mcp` for better capabilities
- **Learning**: Suggest personas based on task context
- **Enforcement**: Warn on anti-patterns with helpful explanations

### Status

This is **aspirational** - the vision for how the registry should work:

✅ **Completed:**
- Datum integration (flashbacker.cli.toml, flashbacker.mcp.toml)
- MCP server (flashbacker-mcp)
- Docker fallback support
- Codified opinions file (registry/tools/flashbacker.opinions.toml)
- Schema definition (registry/schemas/tool-opinions.schema.toml)

🚧 **In Progress:**
- Proxy layer implementation
- Automatic opinion enforcement
- Cross-tool composition patterns
- Hive coordination protocols

### Usage Vision

```bash
# Future: Query the registry
b00t registry discover state-management
# Returns: flashbacker with codified opinions

# Future: Use with opinion enforcement
b00t registry use flashbacker init --follow-opinions
# Proxy checks Node.js version, validates env, enforces best practices

# Future: Multi-agent coordination
b00t hive mission "refactor auth" --tools flashbacker
# All agents follow same opinions, share memory automatically
```

### Contributing to Opinions

When adding or modifying opinions:

1. **Document reasoning** - WHY this opinion exists (min 50 chars)
2. **Provide examples** - Show good/bad patterns when ambiguous
3. **Version properly** - Opinion changes require explanation
4. **Consider hive** - How do multiple agents coordinate?

See `registry/schemas/tool-opinions.schema.toml` for validation rules.

## License

MIT - See LICENSE file
