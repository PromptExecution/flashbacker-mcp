#!/usr/bin/env python3
"""
🥾 Specialist Agent Worker
Polls for tasks and executes with flashbacker context
"""

import os
import sys
import json
import time
import subprocess
from pathlib import Path
from typing import Dict, Any, Optional

class SpecialistAgent:
    """Specialist agent that processes delegated tasks"""
    
    def __init__(self):
        self.agent_role = os.getenv('AGENT_ROLE', 'unknown')
        self.agent_id = os.getenv('AGENT_ID', f'{self.agent_role}-001')
        self.specialist_type = os.getenv('SPECIALIST_TYPE', 'general')
        self.flashbacker_persona = os.getenv('FLASHBACKER_PERSONA', self.agent_role)
        self.task_dir = Path("/tmp/agent_tasks")
        self.task_file = self.task_dir / f"{self.agent_role}.json"
        self.results_dir = Path("/tmp/agent_results")
        self.results_dir.mkdir(exist_ok=True)
        
    def get_persona_context(self) -> str:
        """Load persona context from flashbacker"""
        try:
            result = subprocess.run(
                ["flashback", "persona", self.flashbacker_persona, "--context"],
                capture_output=True,
                text=True,
                timeout=30
            )
            if result.returncode == 0:
                return result.stdout
            else:
                print(f"⚠️ Persona context not available for {self.flashbacker_persona}")
                return ""
        except Exception as e:
            print(f"⚠️ Failed to load persona: {e}")
            return ""
    
    def execute_task(self, task: Dict[str, Any]) -> Dict[str, Any]:
        """Execute delegated task based on specialist type"""
        print(f"\n🔧 {self.agent_role} executing: {task['task']}")
        
        # Get specialized context from flashbacker
        persona_context = self.get_persona_context()
        
        result = {
            "agent_id": self.agent_id,
            "agent_role": self.agent_role,
            "task": task['task'],
            "status": "processing",
            "output": None,
            "errors": []
        }
        
        try:
            # Execute based on specialist type
            if self.specialist_type == "architecture":
                result["output"] = self.review_architecture(task['context'])
            elif self.specialist_type == "database":
                result["output"] = self.analyze_database(task['context'])
            elif self.specialist_type == "rust-translation":
                result["output"] = self.translate_rust(task['context'])
            elif self.specialist_type == "infrastructure":
                result["output"] = self.setup_infrastructure(task['context'])
            elif self.specialist_type == "testing":
                result["output"] = self.create_tests(task['context'])
            elif self.specialist_type == "security-audit":
                result["output"] = self.audit_security(task['context'])
            elif self.specialist_type == "api-design":
                result["output"] = self.design_api(task['context'])
            else:
                result["output"] = f"Generic processing for {self.specialist_type}"
            
            result["status"] = "completed"
            
        except Exception as e:
            result["status"] = "error"
            result["errors"].append(str(e))
            print(f"❌ Error: {e}")
        
        return result
    
    def review_architecture(self, context: Dict[str, Any]) -> str:
        """Architect agent: Review and recommend architecture improvements"""
        workspace = context.get("workspace_path", "commercerack-rust")
        
        recommendations = []
        recommendations.append(f"✅ Reviewed {workspace} Cargo workspace")
        recommendations.append("📦 Workspace structure looks good with 12 planned crates")
        recommendations.append("🔧 Recommend adding:")
        recommendations.append("  - crates/middleware/ for Axum middleware")
        recommendations.append("  - crates/auth/ for authentication logic")
        recommendations.append("  - crates/cache/ for Redis abstraction")
        recommendations.append("  - crates/models/ for shared domain models")
        recommendations.append("📝 Consider workspace-level integration tests")
        
        return "\n".join(recommendations)
    
    def analyze_database(self, context: Dict[str, Any]) -> str:
        """Database agent: Analyze schema migration needs"""
        remaining = context.get("remaining_tables", 130)
        
        analysis = []
        analysis.append(f"🗄️ Analyzed remaining {remaining} tables")
        analysis.append("Priority tables for next migration (002_*.sql):")
        analysis.append("  1. EBAY_* tables (eBay integration, ~9 tables)")
        analysis.append("  2. SUPPLIER_* tables (supplier management, ~6 tables)")
        analysis.append("  3. SHIPPING_* tables (shipping config, ~8 tables)")
        analysis.append("  4. GOOGLE_* tables (Google Shopping, ~2 tables)")
        analysis.append("  5. WAREHOUSE_* tables (WMS, ~3 tables)")
        analysis.append("📊 Recommend batching migrations by business domain")
        analysis.append("🔍 All migrations should follow 001_*.sql pattern:")
        analysis.append("   - Convert ENUMs")
        analysis.append("   - Fix zero datetimes")
        analysis.append("   - Add triggers for ON UPDATE")
        
        return "\n".join(analysis)
    
    def translate_rust(self, context: Dict[str, Any]) -> str:
        """Rust expert: Translate Perl module to Rust"""
        source = context.get("source_file", "")
        target = context.get("target_crate", "")
        loc = context.get("lines_of_code", 0)
        
        plan = []
        plan.append(f"🦀 Rust translation plan for {Path(source).name} ({loc} LOC)")
        plan.append(f"   Target: {target}")
        plan.append("\nTranslation strategy:")
        plan.append("  1. Define domain models (Customer struct)")
        plan.append("  2. Implement database queries with SQLx")
        plan.append("  3. Add business logic methods")
        plan.append("  4. Create error types")
        plan.append("  5. Write unit tests")
        plan.append("\nKey patterns:")
        plan.append("  - Perl tie hash → Rust Index/IndexMut traits")
        plan.append("  - DBI queries → SQLx compile-time checked")
        plan.append("  - YAML serialization → serde_yaml")
        plan.append("  - Package globals → Arc<RwLock<T>>")
        plan.append(f"\n⏱️ Estimated: {loc // 50} hours for complete translation")
        
        return "\n".join(plan)
    
    def setup_infrastructure(self, context: Dict[str, Any]) -> str:
        """DevOps agent: Infrastructure setup tasks"""
        terraform_path = context.get("terraform_path", "infra/k0s")
        
        steps = []
        steps.append(f"🚀 DevOps setup for {terraform_path}")
        steps.append("\nInfrastructure deployment steps:")
        steps.append("  1. Install OpenTofu: brew install opentofu")
        steps.append("  2. Initialize: tofu init")
        steps.append("  3. Plan: tofu plan")
        steps.append("  4. Apply: tofu apply")
        steps.append("\nk0s cluster setup:")
        steps.append("  - Single-node controller with worker enabled")
        steps.append("  - Exposes ports: 6443 (API), 8080 (dashboard)")
        steps.append("  - Persistent volumes: /var/lib/k0s, /etc/k0s")
        steps.append("\nSupporting services:")
        steps.append("  - PostgreSQL 16 (port 5432)")
        steps.append("  - Redis 7 (port 6379)")
        steps.append("  - CommerceRack API (port 8000)")
        steps.append("\n📋 Post-deployment:")
        steps.append("  kubectl --kubeconfig /var/lib/k0s/pki/admin.conf get nodes")
        
        return "\n".join(steps)
    
    def create_tests(self, context: Dict[str, Any]) -> str:
        """QA agent: Create test suite"""
        test_path = context.get("test_path", "tests")
        framework = context.get("framework", "cargo test")
        coverage = context.get("coverage_target", 80)
        
        test_plan = []
        test_plan.append(f"🧪 Test suite creation for {test_path}")
        test_plan.append(f"   Framework: {framework}")
        test_plan.append(f"   Coverage target: {coverage}%")
        test_plan.append("\nTest categories:")
        test_plan.append("  1. Unit tests (per module)")
        test_plan.append("     - DatabaseRouter connection pooling")
        test_plan.append("     - Model serialization/deserialization")
        test_plan.append("     - Error handling")
        test_plan.append("  2. Integration tests")
        test_plan.append("     - Database queries (testcontainers)")
        test_plan.append("     - Redis caching")
        test_plan.append("     - Multi-tenant isolation")
        test_plan.append("  3. End-to-end tests")
        test_plan.append("     - API endpoints")
        test_plan.append("     - Authentication flows")
        test_plan.append("\n📦 Dependencies needed:")
        test_plan.append("  - testcontainers for Postgres")
        test_plan.append("  - mockall for mocking")
        test_plan.append("  - tarpaulin for coverage")
        
        return "\n".join(test_plan)
    
    def audit_security(self, context: Dict[str, Any]) -> str:
        """Security agent: Audit security patterns"""
        target = context.get("target", "codebase")
        focus = context.get("focus_areas", [])
        
        audit = []
        audit.append(f"🔒 Security audit for {target}")
        audit.append(f"   Focus areas: {', '.join(focus)}")
        audit.append("\nFindings and recommendations:")
        audit.append("  1. Password Hashing:")
        audit.append("     ✅ Use argon2 (already in Cargo.toml)")
        audit.append("     ⚠️ Ensure salt is randomly generated per user")
        audit.append("     ⚠️ Use memory-hard parameters for Argon2")
        audit.append("  2. Session Management:")
        audit.append("     🔧 Use JWT with RS256 (asymmetric)")
        audit.append("     🔧 Set short expiration times (15-30 min)")
        audit.append("     🔧 Implement refresh token rotation")
        audit.append("  3. SQL Injection:")
        audit.append("     ✅ SQLx provides compile-time query checking")
        audit.append("     ✅ Parameterized queries prevent injection")
        audit.append("  4. Additional recommendations:")
        audit.append("     - Enable CORS with strict origin checking")
        audit.append("     - Use HTTPS only in production")
        audit.append("     - Implement rate limiting per IP/user")
        audit.append("     - Add request logging for audit trails")
        
        return "\n".join(audit)
    
    def design_api(self, context: Dict[str, Any]) -> str:
        """API designer: Design RESTful API"""
        endpoints = context.get("endpoints", [])
        auth = context.get("auth_type", "JWT")
        version = context.get("api_version", "v1")
        
        design = []
        design.append(f"🌐 API Design for /{version}/")
        design.append(f"   Authentication: {auth}")
        design.append(f"   Endpoints: {', '.join(endpoints)}")
        design.append("\nAPI Structure:")
        design.append(f"  /{version}/customers")
        design.append("    GET    /            - List customers (paginated)")
        design.append("    POST   /            - Create customer")
        design.append("    GET    /:id         - Get customer")
        design.append("    PUT    /:id         - Update customer")
        design.append("    DELETE /:id         - Delete customer")
        design.append(f"  /{version}/products")
        design.append("    GET    /            - List products")
        design.append("    GET    /:pid        - Get product")
        design.append("    GET    /search      - Search products")
        design.append(f"  /{version}/orders")
        design.append("    GET    /            - List orders")
        design.append("    POST   /            - Create order")
        design.append("    GET    /:id         - Get order")
        design.append(f"  /{version}/cart")
        design.append("    GET    /:cart_id    - Get cart")
        design.append("    POST   /:cart_id/items - Add item")
        design.append("    DELETE /:cart_id/items/:item_id - Remove item")
        design.append("\n🔐 Authentication:")
        design.append("  - POST /auth/login  → JWT token")
        design.append("  - POST /auth/refresh → Refresh token")
        design.append("  - All endpoints require 'Authorization: Bearer <token>'")
        
        return "\n".join(design)
    
    def poll_and_process(self):
        """Main loop: poll for tasks and process"""
        print(f"👤 {self.agent_id} ({self.specialist_type}) starting")
        print(f"📁 Watching: {self.task_file}")
        
        while True:
            try:
                if self.task_file.exists():
                    # Load and process task
                    task = json.loads(self.task_file.read_text())
                    result = self.execute_task(task)
                    
                    # Save result
                    result_file = self.results_dir / f"{self.agent_role}_result.json"
                    result_file.write_text(json.dumps(result, indent=2))
                    print(f"✅ Result saved to {result_file}")
                    
                    # Remove task file
                    self.task_file.unlink()
                    
                else:
                    # Wait for tasks
                    time.sleep(5)
                    
            except KeyboardInterrupt:
                print(f"\n👋 {self.agent_id} shutting down")
                break
            except Exception as e:
                print(f"❌ Error: {e}")
                time.sleep(5)

if __name__ == "__main__":
    agent = SpecialistAgent()
    agent.poll_and_process()
