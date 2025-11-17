/**
 * 🥾 b00t Multi-Agent Hive - pm2 Ecosystem
 * Specialized sub-agents with flashbacker context management
 * Captain: Main orchestrator
 */

module.exports = {
  apps: [
    {
      name: 'captain',
      script: './agents/scripts/captain.py',
      interpreter: 'python3',
      instances: 1,
      autorestart: true,
      watch: false,
      max_memory_restart: '1G',
      env: {
        AGENT_ROLE: 'captain',
        AGENT_ID: 'captain-001',
        B00T_ENABLED: 'true',
        FLASHBACKER_ENABLED: 'true',
        LOG_LEVEL: 'info'
      }
    },
    {
      name: 'architect-agent',
      script: './agents/scripts/specialist_agent.py',
      interpreter: 'python3',
      instances: 1,
      autorestart: true,
      max_memory_restart: '800M',
      env: {
        AGENT_ROLE: 'architect',
        AGENT_ID: 'architect-001',
        SPECIALIST_TYPE: 'architecture',
        FLASHBACKER_PERSONA: 'architect',
        B00T_ENABLED: 'true'
      }
    },
    {
      name: 'database-agent',
      script: './agents/scripts/specialist_agent.py',
      interpreter: 'python3',
      instances: 1,
      autorestart: true,
      max_memory_restart: '800M',
      env: {
        AGENT_ROLE: 'database-architect',
        AGENT_ID: 'database-001',
        SPECIALIST_TYPE: 'database',
        FLASHBACKER_PERSONA: 'database-architect',
        B00T_ENABLED: 'true'
      }
    },
    {
      name: 'rust-expert',
      script: './agents/scripts/specialist_agent.py',
      interpreter: 'python3',
      instances: 1,
      autorestart: true,
      max_memory_restart: '800M',
      env: {
        AGENT_ROLE: 'rust-expert',
        AGENT_ID: 'rust-001',
        SPECIALIST_TYPE: 'rust-translation',
        FLASHBACKER_PERSONA: 'refactorer',
        B00T_ENABLED: 'true'
      }
    },
    {
      name: 'devops-agent',
      script: './agents/scripts/specialist_agent.py',
      interpreter: 'python3',
      instances: 1,
      autorestart: true,
      max_memory_restart: '800M',
      env: {
        AGENT_ROLE: 'devops',
        AGENT_ID: 'devops-001',
        SPECIALIST_TYPE: 'infrastructure',
        FLASHBACKER_PERSONA: 'devops',
        B00T_ENABLED: 'true'
      }
    },
    {
      name: 'qa-agent',
      script: './agents/scripts/specialist_agent.py',
      interpreter: 'python3',
      instances: 1,
      autorestart: true,
      max_memory_restart: '600M',
      env: {
        AGENT_ROLE: 'qa',
        AGENT_ID: 'qa-001',
        SPECIALIST_TYPE: 'testing',
        FLASHBACKER_PERSONA: 'qa',
        B00T_ENABLED: 'true'
      }
    },
    {
      name: 'security-agent',
      script: './agents/scripts/specialist_agent.py',
      interpreter: 'python3',
      instances: 1,
      autorestart: true,
      max_memory_restart: '600M',
      env: {
        AGENT_ROLE: 'security',
        AGENT_ID: 'security-001',
        SPECIALIST_TYPE: 'security-audit',
        FLASHBACKER_PERSONA: 'security',
        B00T_ENABLED: 'true'
      }
    },
    {
      name: 'api-designer',
      script: './agents/scripts/specialist_agent.py',
      interpreter: 'python3',
      instances: 1,
      autorestart: true,
      max_memory_restart: '600M',
      env: {
        AGENT_ROLE: 'api-designer',
        AGENT_ID: 'api-001',
        SPECIALIST_TYPE: 'api-design',
        FLASHBACKER_PERSONA: 'api-designer',
        B00T_ENABLED: 'true'
      }
    }
  ]
};
