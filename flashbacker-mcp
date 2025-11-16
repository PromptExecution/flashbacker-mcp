#!/usr/bin/env node
/**
 * Flashbacker MCP Server
 * Wraps flashback CLI commands as MCP tools for b00t integration
 * Protocol: stdio JSON-RPC
 */

const { spawn } = require('child_process');
const readline = require('readline');

const TOOLS = [
  {
    name: 'flashback_init',
    description: 'Initialize Flashbacker in a project',
    inputSchema: {
      type: 'object',
      properties: {
        mcp: { type: 'boolean', description: 'Include MCP servers' },
        refresh: { type: 'boolean', description: 'Refresh existing installation' }
      }
    }
  },
  {
    name: 'flashback_persona',
    description: 'Get AI persona analysis',
    inputSchema: {
      type: 'object',
      properties: {
        persona: { type: 'string', description: 'Persona name (architect, security, etc.)' },
        request: { type: 'string', description: 'Analysis request' }
      },
      required: ['persona', 'request']
    }
  },
  {
    name: 'flashback_agent',
    description: 'Gather context for agent analysis',
    inputSchema: {
      type: 'object',
      properties: {
        agent: { type: 'string', description: 'Agent name' },
        context: { type: 'string', description: 'Context request' }
      },
      required: ['agent']
    }
  },
  {
    name: 'flashback_memory',
    description: 'Manage project memory',
    inputSchema: {
      type: 'object',
      properties: {
        action: { type: 'string', enum: ['add', 'list', 'search'], description: 'Memory action' },
        content: { type: 'string', description: 'Content to add or search term' }
      },
      required: ['action']
    }
  },
  {
    name: 'flashback_working_plan',
    description: 'Manage working plan',
    inputSchema: {
      type: 'object',
      properties: {
        action: { type: 'string', enum: ['add', 'list', 'update'], description: 'Plan action' },
        content: { type: 'string', description: 'Task content' }
      },
      required: ['action']
    }
  },
  {
    name: 'flashback_save_session',
    description: 'Save session insights',
    inputSchema: {
      type: 'object',
      properties: {
        context: { type: 'boolean', description: 'Include full context' }
      }
    }
  },
  {
    name: 'flashback_session_start',
    description: 'Load context at session start',
    inputSchema: {
      type: 'object',
      properties: {}
    }
  },
  {
    name: 'flashback_discuss',
    description: 'Run multi-persona discussion',
    inputSchema: {
      type: 'object',
      properties: {
        topic: { type: 'string', description: 'Discussion topic' },
        personas: { type: 'string', description: 'Comma-separated persona list' }
      },
      required: ['topic', 'personas']
    }
  },
  {
    name: 'flashback_debt_hunter',
    description: 'Detect technical debt and duplicates',
    inputSchema: {
      type: 'object',
      properties: {}
    }
  },
  {
    name: 'flashback_fix_master',
    description: 'Use surgical fix methodology',
    inputSchema: {
      type: 'object',
      properties: {
        context: { type: 'string', description: 'Issue description' }
      }
    }
  },
  {
    name: 'flashback_doctor',
    description: 'Run system diagnostics',
    inputSchema: {
      type: 'object',
      properties: {}
    }
  },
  {
    name: 'flashback_status',
    description: 'Check installation status',
    inputSchema: {
      type: 'object',
      properties: {}
    }
  }
];

async function executeFlashback(toolName, args) {
  // Convert tool name to flashback command
  const command = toolName.replace('flashback_', '').replace(/_/g, '-');
  const flashbackArgs = [command];

  // Build argument list based on tool and input
  switch (toolName) {
    case 'flashback_init':
      if (args.mcp) flashbackArgs.push('--mcp');
      if (args.refresh) flashbackArgs.push('--refresh');
      break;

    case 'flashback_persona':
      flashbackArgs.push(args.persona, args.request);
      break;

    case 'flashback_agent':
      flashbackArgs.push(args.agent);
      if (args.context) flashbackArgs.push('--context', args.context);
      break;

    case 'flashback_memory':
      flashbackArgs.push(args.action);
      if (args.content) flashbackArgs.push(args.content);
      break;

    case 'flashback_working_plan':
      flashbackArgs.push(args.action);
      if (args.content) flashbackArgs.push(args.content);
      break;

    case 'flashback_save_session':
      if (args.context) flashbackArgs.push('--context');
      break;

    case 'flashback_discuss':
      flashbackArgs.push(args.topic, '--personas', args.personas);
      break;

    case 'flashback_fix_master':
      if (args.context) flashbackArgs.push('--context', args.context);
      break;
  }

  return new Promise((resolve, reject) => {
    const proc = spawn('flashback', flashbackArgs, {
      cwd: process.env.CWD || process.cwd()
    });

    let stdout = '';
    let stderr = '';

    proc.stdout.on('data', (data) => { stdout += data.toString(); });
    proc.stderr.on('data', (data) => { stderr += data.toString(); });

    proc.on('close', (code) => {
      if (code === 0) {
        resolve({ content: [{ type: 'text', text: stdout }] });
      } else {
        reject({ error: stderr || stdout });
      }
    });

    proc.on('error', (err) => {
      reject({ error: err.message });
    });
  });
}

class FlashbackerMCPServer {
  constructor() {
    this.rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
      terminal: false
    });
  }

  async handleRequest(request) {
    try {
      const req = JSON.parse(request);

      switch (req.method) {
        case 'initialize':
          return {
            jsonrpc: '2.0',
            id: req.id,
            result: {
              protocolVersion: '2024-11-05',
              capabilities: {
                tools: {}
              },
              serverInfo: {
                name: 'flashbacker',
                version: '2.4.1'
              }
            }
          };

        case 'tools/list':
          return {
            jsonrpc: '2.0',
            id: req.id,
            result: {
              tools: TOOLS
            }
          };

        case 'tools/call':
          const { name, arguments: args } = req.params;
          const result = await executeFlashback(name, args || {});
          return {
            jsonrpc: '2.0',
            id: req.id,
            result
          };

        default:
          return {
            jsonrpc: '2.0',
            id: req.id,
            error: {
              code: -32601,
              message: `Method not found: ${req.method}`
            }
          };
      }
    } catch (error) {
      return {
        jsonrpc: '2.0',
        id: request.id,
        error: {
          code: -32603,
          message: error.message
        }
      };
    }
  }

  start() {
    console.error('[Flashbacker MCP] Starting...');

    this.rl.on('line', async (line) => {
      if (line.trim()) {
        const response = await this.handleRequest(line);
        console.log(JSON.stringify(response));
      }
    });

    this.rl.on('close', () => {
      console.error('[Flashbacker MCP] Shutting down');
      process.exit(0);
    });

    console.error('[Flashbacker MCP] Ready');
  }
}

if (require.main === module) {
  const server = new FlashbackerMCPServer();
  server.start();
}

module.exports = FlashbackerMCPServer;
