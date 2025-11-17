#!/usr/bin/env python3
"""
🥾 Captain Agent - Multi-Agent Hive Orchestrator
Uses flashbacker + b00t for context management
Delegates to specialized sub-agents via MCP
"""

import os
import sys
import json
import asyncio
import subprocess
from typing import Dict, List, Any
from dataclasses import dataclass
from pathlib import Path

@dataclass
class AgentTask:
    """Task to be delegated to specialist agent"""
    agent_role: str
    task_description: str
    context: Dict[str, Any]
    priority: int = 5

class FlashbackerIntegration:
    """🥾 Flashbacker integration for context management"""
    
    def __init__(self):
        self.flashback_bin = "flashback"
        
    def save_session_context(self, context_data: Dict[str, Any]) -> bool:
        """Save session context using flashbacker"""
        try:
            # 🤓 Use flashbacker to reduce context usage
            result = subprocess.run(
                [self.flashback_bin, "save-session", "--context"],
                input=json.dumps(context_data),
                capture_output=True,
                text=True
            )
            return result.returncode == 0
        except Exception as e:
            print(f"⚠️ Flashbacker save failed: {e}")
            return False
    
    def load_session_context(self) -> Dict[str, Any]:
        """Load session context from flashbacker"""
        try:
            result = subprocess.run(
                [self.flashback_bin, "session-start"],
                capture_output=True,
                text=True
            )
            if result.returncode == 0:
                return json.loads(result.stdout)
            return {}
        except Exception as e:
            print(f"⚠️ Flashbacker load failed: {e}")
            return {}
    
    def get_persona_context(self, persona: str) -> str:
        """Get flashbacker persona context"""
        try:
            result = subprocess.run(
                [self.flashback_bin, "persona", persona, "--context"],
                capture_output=True,
                text=True
            )
            return result.stdout if result.returncode == 0 else ""
        except Exception as e:
            print(f"⚠️ Persona context failed: {e}")
            return ""

class CaptainOrchestrator:
    """🎯 Captain agent - coordinates specialized sub-agents"""
    
    def __init__(self):
        self.agent_id = os.getenv('AGENT_ID', 'captain-001')
        self.flashbacker = FlashbackerIntegration()
        self.task_queue: List[AgentTask] = []
        self.agent_status: Dict[str, str] = {}
        
    async def delegate_task(self, task: AgentTask) -> Dict[str, Any]:
        """Delegate task to specialist agent"""
        print(f"📋 Delegating to {task.agent_role}: {task.task_description}")
        
        # Get persona context from flashbacker
        persona_context = self.flashbacker.get_persona_context(task.agent_role)
        
        # 🤓 Use MCP-style communication (avoiding ACP for now)
        task_payload = {
            "agent": task.agent_role,
            "task": task.task_description,
            "context": task.context,
            "persona_context": persona_context,
            "priority": task.priority
        }
        
        # Save to task queue file for pm2 agents to pick up
        task_file = Path(f"/tmp/agent_tasks/{task.agent_role}.json")
        task_file.parent.mkdir(exist_ok=True)
        task_file.write_text(json.dumps(task_payload, indent=2))
        
        print(f"✅ Task delegated to {task.agent_role}")
        return {"status": "delegated", "task_file": str(task_file)}
    
    async def process_commercerack_migration(self):
        """Main workflow: CommerceRack Perl → Rust migration"""
        print("🦀 Starting CommerceRack migration workflow")
        
        # Define specialized tasks
        tasks = [
            AgentTask(
                agent_role="architect",
                task_description="Review Cargo workspace structure and recommend improvements",
                context={
                    "workspace_path": "commercerack-rust",
                    "crates_count": 12,
                    "focus": "modular architecture"
                },
                priority=10
            ),
            AgentTask(
                agent_role="database-architect",
                task_description="Complete Postgres schema migration for remaining 130 tables",
                context={
                    "schema_path": "migrations/001_initial_schema.sql",
                    "remaining_tables": 130,
                    "source": "/home/user/commercerack-backend/schema.sql"
                },
                priority=9
            ),
            AgentTask(
                agent_role="rust-expert",
                task_description="Translate CUSTOMER.pm to Rust customer crate",
                context={
                    "source_file": "/home/user/commercerack-backend/lib/CUSTOMER.pm",
                    "target_crate": "crates/customer",
                    "lines_of_code": 2579
                },
                priority=8
            ),
            AgentTask(
                agent_role="devops",
                task_description="Set up k0s cluster and deploy CommerceRack containers",
                context={
                    "terraform_path": "infra/k0s",
                    "use_opentofu": True,
                    "k0s_version": "latest"
                },
                priority=7
            ),
            AgentTask(
                agent_role="qa",
                task_description="Create integration test suite for database layer",
                context={
                    "test_path": "crates/db/tests",
                    "framework": "cargo test",
                    "coverage_target": 80
                },
                priority=6
            ),
            AgentTask(
                agent_role="security",
                task_description="Audit password hashing and authentication patterns",
                context={
                    "focus_areas": ["password_hashing", "session_management", "sql_injection"],
                    "target": "crates/customer"
                },
                priority=8
            ),
            AgentTask(
                agent_role="api-designer",
                task_description="Design RESTful API schema for Axum server",
                context={
                    "endpoints": ["customers", "products", "orders", "cart"],
                    "auth_type": "JWT",
                    "api_version": "v1"
                },
                priority=7
            )
        ]
        
        # Delegate tasks in priority order
        tasks.sort(key=lambda t: t.priority, reverse=True)
        
        for task in tasks:
            result = await self.delegate_task(task)
            print(f"  → {result}")
            await asyncio.sleep(1)  # Stagger task delegation
        
        print("\n✅ All tasks delegated to specialist agents")
        print("📊 Monitor with: pm2 logs")
    
    async def run(self):
        """Main captain loop"""
        print(f"🎯 Captain {self.agent_id} starting")
        print(f"🥾 b00t enabled: {os.getenv('B00T_ENABLED')}")
        print(f"💾 Flashbacker enabled: {os.getenv('FLASHBACKER_ENABLED')}")
        
        # Load previous context from flashbacker
        prev_context = self.flashbacker.load_session_context()
        if prev_context:
            print(f"📚 Loaded context from previous session: {len(prev_context)} items")
        
        # Execute main workflow
        await self.process_commercerack_migration()
        
        # Save session context
        session_context = {
            "agent_id": self.agent_id,
            "tasks_delegated": len(self.task_queue),
            "workflow": "commercerack_migration",
            "timestamp": "2025-11-17"
        }
        self.flashbacker.save_session_context(session_context)
        
        print("\n🎯 Captain workflow complete. Agents working in background.")
        print("📋 Next: Monitor specialist agents with 'pm2 status' and 'pm2 logs'")

async def main():
    captain = CaptainOrchestrator()
    await captain.run()

if __name__ == "__main__":
    asyncio.run(main())
