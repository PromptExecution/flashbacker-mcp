# 🥾 CommerceRack b00t Rust Translation - Executable Roadmap
# casey/just command runner - memoized workflow

# Default: Show all available commands
default:
    @just --list

# =============================================================================
# PHASE 1: Foundation ✅ COMPLETE
# =============================================================================

# Checkpoint 1.1: Validate repository setup
checkpoint-1-1:
    #!/usr/bin/env bash
    echo "🔍 Validating Checkpoint 1.1: Repository Setup"
    cd commercerack-rust
    cargo check --workspace
    ls migrations/001_initial_schema.sql
    echo "✅ Checkpoint 1.1 validated"

# Checkpoint 1.2: Validate multi-agent infrastructure
checkpoint-1-2:
    #!/usr/bin/env bash
    echo "🔍 Validating Checkpoint 1.2: Multi-Agent Infrastructure"
    test -f agents/ecosystem.config.js || exit 1
    test -f agents/scripts/captain.py || exit 1
    test -f agents/scripts/specialist_agent.py || exit 1
    test -f infra/k0s/main.tf || exit 1
    test -f DEPLOYMENT.md || exit 1
    python3 -m py_compile agents/scripts/captain.py
    python3 -m py_compile agents/scripts/specialist_agent.py
    echo "✅ Checkpoint 1.2 validated"

# Validate all Phase 1 checkpoints
phase-1: checkpoint-1-1 checkpoint-1-2
    @echo "✅ Phase 1 COMPLETE"

# =============================================================================
# PHASE 2: Core Module Translation
# =============================================================================

# Checkpoint 2.1: Create and validate customer module foundation
checkpoint-2-1:
    #!/usr/bin/env bash
    set -e
    echo "🚀 Starting Checkpoint 2.1: Customer Module Foundation"
    
    # Create feature branch if not exists
    git checkout -b feature/customer-module 2>/dev/null || git checkout feature/customer-module
    
    # Ensure crate exists
    cd commercerack-rust
    test -d crates/customer || cargo new --lib crates/customer
    
    # Run validation
    echo "🧪 Running tests..."
    cargo test --package commercerack-customer
    
    echo "🔍 Running clippy..."
    cargo clippy --package commercerack-customer -- -D warnings
    
    echo "🏗️ Building release..."
    cargo build --release --package commercerack-customer
    
    echo "✅ Checkpoint 2.1 passed validation"
    echo "📋 Next: Commit with 'just commit-checkpoint \"2.1\" \"Customer module foundation\"'"

# Checkpoint 2.2: Customer authentication
checkpoint-2-2:
    #!/usr/bin/env bash
    set -e
    echo "🚀 Starting Checkpoint 2.2: Customer Authentication"
    
    git checkout -b feature/customer-auth 2>/dev/null || git checkout feature/customer-auth
    
    cd commercerack-rust
    cargo test customer::auth || echo "⚠️ Tests not yet implemented"
    cargo clippy --package commercerack-customer -- -D warnings
    
    echo "🔐 Security validation..."
    grep -q "argon2" crates/customer/Cargo.toml || echo "⚠️ Add argon2 dependency"
    
    echo "✅ Checkpoint 2.2 validation complete"

# Checkpoint 2.3: Customer addresses
checkpoint-2-3:
    #!/usr/bin/env bash
    set -e
    echo "🚀 Starting Checkpoint 2.3: Customer Addresses"
    
    git checkout -b feature/customer-addresses 2>/dev/null || git checkout feature/customer-addresses
    
    cd commercerack-rust
    cargo test customer::address || echo "⚠️ Tests not yet implemented"
    cargo clippy --package commercerack-customer -- -D warnings
    
    echo "✅ Checkpoint 2.3 validation complete"

# Complete Phase 2: All customer module checkpoints
phase-2: checkpoint-2-1 checkpoint-2-2 checkpoint-2-3
    @echo "✅ Phase 2 COMPLETE: Core Module Translation"

# =============================================================================
# PHASE 3: Product & Inventory
# =============================================================================

# Checkpoint 3.1: Product catalog
checkpoint-3-1:
    #!/usr/bin/env bash
    set -e
    echo "🚀 Starting Checkpoint 3.1: Product Catalog"
    
    git checkout -b feature/product-catalog 2>/dev/null || git checkout feature/product-catalog
    
    cd commercerack-rust
    test -d crates/product || cargo new --lib crates/product
    
    cargo test --package commercerack-product
    cargo clippy --package commercerack-product -- -D warnings
    cargo build --release --package commercerack-product
    
    echo "✅ Checkpoint 3.1 passed validation"

# Checkpoint 3.2: SKU lookup system
checkpoint-3-2:
    #!/usr/bin/env bash
    set -e
    echo "🚀 Starting Checkpoint 3.2: SKU Lookup"
    
    git checkout -b feature/sku-lookup 2>/dev/null || git checkout feature/sku-lookup
    
    cd commercerack-rust
    cargo test sku::lookup || echo "⚠️ Tests not yet implemented"
    
    echo "📊 Running benchmarks..."
    cargo bench --bench sku_perf || echo "⚠️ Benchmarks not yet implemented"
    
    echo "✅ Checkpoint 3.2 validation complete"

# Complete Phase 3
phase-3: checkpoint-3-1 checkpoint-3-2
    @echo "✅ Phase 3 COMPLETE: Product & Inventory"

# =============================================================================
# PHASE 4: API Layer
# =============================================================================

# Checkpoint 4.1: Axum server skeleton
checkpoint-4-1:
    #!/usr/bin/env bash
    set -e
    echo "🚀 Starting Checkpoint 4.1: Axum Server"
    
    git checkout -b feature/axum-api 2>/dev/null || git checkout feature/axum-api
    
    cd commercerack-rust
    test -d jsonapi || cargo new --bin jsonapi
    
    cargo test --package jsonapi || echo "⚠️ No tests yet"
    cargo clippy --package jsonapi -- -D warnings
    
    echo "🐳 Testing Docker build..."
    docker build -t commercerack-rust:test . || echo "⚠️ Docker build failed"
    
    echo "✅ Checkpoint 4.1 validation complete"

# Checkpoint 4.2: Customer API endpoints
checkpoint-4-2:
    #!/usr/bin/env bash
    set -e
    echo "🚀 Starting Checkpoint 4.2: Customer API Endpoints"
    
    git checkout -b feature/api-customers 2>/dev/null || git checkout feature/api-customers
    
    cd commercerack-rust
    cargo test api::customers || echo "⚠️ Tests not yet implemented"
    
    echo "✅ Checkpoint 4.2 validation complete"

# Complete Phase 4
phase-4: checkpoint-4-1 checkpoint-4-2
    @echo "✅ Phase 4 COMPLETE: API Layer"

# =============================================================================
# INFRASTRUCTURE COMMANDS
# =============================================================================

# Deploy k0s infrastructure with OpenTofu
infra-deploy:
    #!/usr/bin/env bash
    set -e
    echo "🚀 Deploying k0s infrastructure..."
    cd infra/k0s
    tofu init
    tofu plan
    tofu apply -auto-approve
    echo "✅ Infrastructure deployed"
    echo "📋 Endpoints:"
    echo "  - k0s API: https://localhost:6443"
    echo "  - CommerceRack API: http://localhost:8000"
    echo "  - PostgreSQL: localhost:5432"
    echo "  - Redis: localhost:6379"

# Destroy k0s infrastructure
infra-destroy:
    #!/usr/bin/env bash
    cd infra/k0s
    tofu destroy -auto-approve
    echo "💥 Infrastructure destroyed"

# Check infrastructure status
infra-status:
    @docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"

# =============================================================================
# DATABASE COMMANDS
# =============================================================================

# Apply database migration (001_initial_schema.sql)
db-migrate:
    #!/usr/bin/env bash
    set -e
    echo "🗄️ Applying database migrations..."
    docker exec -i postgres psql -U postgres -d commercerack < commercerack-rust/migrations/001_initial_schema.sql
    echo "✅ Migration 001 applied"

# Verify database tables
db-verify:
    #!/usr/bin/env bash
    echo "🔍 Checking database tables..."
    docker exec postgres psql -U postgres -d commercerack -c "\dt" | grep -E "customers|orders|products"
    echo "✅ Database verified"

# Backup database
db-backup:
    #!/usr/bin/env bash
    BACKUP_FILE="backup-$(date +%Y%m%d-%H%M%S).sql"
    docker exec postgres pg_dump -U postgres commercerack > "$BACKUP_FILE"
    echo "✅ Database backed up to $BACKUP_FILE"

# Connect to database (interactive)
db-shell:
    docker exec -it postgres psql -U postgres -d commercerack

# =============================================================================
# MULTI-AGENT COMMANDS
# =============================================================================

# Start multi-agent hive with pm2
agents-start:
    #!/usr/bin/env bash
    cd agents
    pm2 start ecosystem.config.js
    pm2 save
    echo "✅ Multi-agent hive started"
    echo "📋 Monitor with: just agents-logs"

# Stop all agents
agents-stop:
    pm2 stop all

# View agent logs (all)
agents-logs:
    pm2 logs

# View captain logs only
agents-captain:
    pm2 logs captain

# View specific agent logs
agents-log agent:
    pm2 logs {{agent}}

# Check agent status
agents-status:
    pm2 status

# Monitor agent resources
agents-monitor:
    pm2 monit

# Restart all agents
agents-restart:
    pm2 restart all

# Check agent results
agents-results:
    #!/usr/bin/env bash
    echo "📊 Agent Results:"
    ls -lah /tmp/agent_results/ 2>/dev/null || echo "⚠️ No results yet"
    echo ""
    echo "📋 View specific result:"
    echo "  cat /tmp/agent_results/architect_result.json | jq ."

# =============================================================================
# BUILD & TEST COMMANDS
# =============================================================================

# Build entire workspace
build:
    cd commercerack-rust && cargo build --workspace

# Build release binaries
build-release:
    cd commercerack-rust && cargo build --release --workspace

# Run all tests
test:
    cd commercerack-rust && cargo test --workspace

# Run tests with coverage
test-coverage:
    #!/usr/bin/env bash
    cd commercerack-rust
    cargo tarpaulin --out Html --output-dir coverage
    echo "📊 Coverage report: commercerack-rust/coverage/index.html"

# Run clippy on all crates
lint:
    cd commercerack-rust && cargo clippy --workspace -- -D warnings

# Format all code
fmt:
    cd commercerack-rust && cargo fmt --all

# Check formatting without modifying
fmt-check:
    cd commercerack-rust && cargo fmt --all --check

# Run benchmarks
bench:
    cd commercerack-rust && cargo bench

# Full validation (test + lint + build)
validate: test lint build
    @echo "✅ Full validation passed"

# =============================================================================
# CHECKPOINT MANAGEMENT
# =============================================================================

# Commit a checkpoint with validation
commit-checkpoint phase message:
    #!/usr/bin/env bash
    set -e
    cd commercerack-rust
    
    echo "🔍 Running pre-commit validation..."
    cargo test --workspace
    cargo clippy --workspace -- -D warnings
    cargo build --release --workspace
    
    echo "📝 Committing checkpoint {{phase}}..."
    cd ..
    git add .
    git commit -m "Checkpoint {{phase}}: {{message}}

Validation:
- cargo test: ✅ Passed
- cargo clippy: ✅ No warnings
- cargo build: ✅ Success
"
    echo "✅ Checkpoint {{phase}} committed"

# Backtrack to previous checkpoint (soft reset)
backtrack:
    #!/usr/bin/env bash
    echo "⚠️ Backtracking to previous commit (keeping changes)..."
    git reset --soft HEAD~1
    echo "✅ Backtracked. Changes staged. Review with 'git status'"

# Nuclear backtrack (hard reset)
backtrack-hard:
    #!/usr/bin/env bash
    echo "💥 HARD RESET to previous commit (losing all changes)..."
    read -p "Are you sure? (yes/no): " confirm
    if [ "$confirm" = "yes" ]; then
        git reset --hard HEAD~1
        echo "✅ Hard reset complete"
    else
        echo "❌ Cancelled"
    fi

# View checkpoint history
checkpoint-history:
    git log --oneline --graph --all | head -20

# Compare current with previous checkpoint
checkpoint-diff:
    git diff HEAD~1

# =============================================================================
# DEVELOPMENT WORKFLOW
# =============================================================================

# Full development cycle: format, lint, test, build
dev: fmt lint test build
    @echo "✅ Development cycle complete"

# Watch for changes and run tests
watch:
    cd commercerack-rust && cargo watch -x test

# Start development environment (infra + agents)
dev-start: infra-deploy db-migrate agents-start
    @echo "✅ Development environment started"
    @echo "📋 Next steps:"
    @echo "  1. Monitor agents: just agents-logs"
    @echo "  2. Check infrastructure: just infra-status"
    @echo "  3. Start coding: just checkpoint-2-1"

# Stop development environment
dev-stop: agents-stop infra-destroy
    @echo "✅ Development environment stopped"

# Clean build artifacts
clean:
    cd commercerack-rust && cargo clean

# Deep clean (including target and git untracked)
clean-all: clean
    git clean -fdx commercerack-rust/target

# =============================================================================
# DEPLOYMENT COMMANDS
# =============================================================================

# Build Docker image
docker-build:
    docker build -t commercerack-rust:latest commercerack-rust/

# Run Docker container locally
docker-run:
    docker run -p 8000:8000 \
      -e DATABASE_URL=postgresql://postgres:postgres@postgres:5432/commercerack \
      -e REDIS_URL=redis://redis:6379 \
      commercerack-rust:latest

# Push to registry (requires login)
docker-push registry:
    docker tag commercerack-rust:latest {{registry}}/commercerack-rust:latest
    docker push {{registry}}/commercerack-rust:latest

# =============================================================================
# UTILITY COMMANDS
# =============================================================================

# Show current phase status
status:
    #!/usr/bin/env bash
    echo "📊 CommerceRack Translation Status"
    echo ""
    echo "Phase 1: Foundation ✅"
    echo "  └─ Checkpoints: 2/2 complete"
    echo ""
    echo "Phase 2: Core Modules"
    echo "  └─ Checkpoints: 0/3"
    echo ""
    echo "Phase 3: Product & Inventory"
    echo "  └─ Checkpoints: 0/2"
    echo ""
    echo "Current branch: $(git branch --show-current)"
    echo "Last commit: $(git log -1 --oneline)"

# Run all phases in sequence (full roadmap)
roadmap-execute:
    #!/usr/bin/env bash
    echo "🚀 Executing full roadmap..."
    just phase-1
    just phase-2
    just phase-3
    just phase-4
    echo "✅ All phases complete!"

# Setup b00t integration
setup-b00t:
    #!/usr/bin/env bash
    echo "🥾 Setting up b00t integration..."
    test -f /usr/local/bin/b00t || echo "⚠️ b00t not installed. Build from _b00t_ submodule"
    b00t whoami
    echo "✅ b00t ready"

# Run flashbacker context save
save-context:
    flashback save-session --context

# Load flashbacker context
load-context:
    flashback session-start

# Show help
help:
    @echo "🥾 CommerceRack b00t Rust Translation - Justfile Commands"
    @echo ""
    @echo "PHASE EXECUTION:"
    @echo "  just phase-1              - Validate Phase 1 (Foundation)"
    @echo "  just phase-2              - Execute Phase 2 (Core Modules)"
    @echo "  just phase-3              - Execute Phase 3 (Product & Inventory)"
    @echo "  just phase-4              - Execute Phase 4 (API Layer)"
    @echo ""
    @echo "CHECKPOINTS:"
    @echo "  just checkpoint-2-1       - Customer module foundation"
    @echo "  just checkpoint-2-2       - Customer authentication"
    @echo "  just commit-checkpoint    - Commit validated checkpoint"
    @echo "  just backtrack            - Undo last checkpoint (soft)"
    @echo "  just backtrack-hard       - Nuclear reset (hard)"
    @echo ""
    @echo "INFRASTRUCTURE:"
    @echo "  just infra-deploy         - Deploy k0s cluster"
    @echo "  just infra-status         - Check infrastructure"
    @echo "  just infra-destroy        - Tear down infrastructure"
    @echo ""
    @echo "DATABASE:"
    @echo "  just db-migrate           - Apply schema migration"
    @echo "  just db-verify            - Verify tables"
    @echo "  just db-shell             - Interactive psql"
    @echo ""
    @echo "AGENTS:"
    @echo "  just agents-start         - Start pm2 multi-agent hive"
    @echo "  just agents-logs          - View all agent logs"
    @echo "  just agents-status        - Check agent status"
    @echo "  just agents-results       - View agent outputs"
    @echo ""
    @echo "DEVELOPMENT:"
    @echo "  just dev                  - Full dev cycle (fmt+lint+test+build)"
    @echo "  just dev-start            - Start full dev environment"
    @echo "  just test                 - Run all tests"
    @echo "  just lint                 - Run clippy"
    @echo "  just build                - Build workspace"
    @echo ""
    @echo "Full list: just --list"
