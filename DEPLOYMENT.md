# 🥾 CommerceRack b00t Multi-Agent Deployment

**Repository**: https://github.com/fungible-farm/commercerack-backend (our fork with _b00t_ submodule)  
**Infrastructure**: OpenTofu + k0s + Docker + pm2  
**Multi-Agent System**: flashbacker + b00t orchestration

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Captain Agent (pm2)                      │
│         Orchestrates specialist sub-agents via tasks         │
│              Uses flashbacker for context mgmt              │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼───────┐    ┌───────▼───────┐    ┌───────▼───────┐
│  Architect    │    │  Database     │    │  Rust Expert  │
│  Sub-Agent    │    │  Sub-Agent    │    │  Sub-Agent    │
│  (pm2)        │    │  (pm2)        │    │  (pm2)        │
└───────────────┘    └───────────────┘    └───────────────┘
        │                     │                     │
┌───────▼───────┐    ┌───────▼───────┐    ┌───────▼───────┐
│  DevOps       │    │  QA           │    │  Security     │
│  Sub-Agent    │    │  Sub-Agent    │    │  Sub-Agent    │
└───────────────┘    └───────────────┘    └───────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────────┐
│              k0s Cluster (OpenTofu managed)                 │
│  ┌─────────────┐  ┌──────────┐  ┌──────────────────────┐  │
│  │ CommerceRack│  │ Postgres │  │ Redis                │  │
│  │ Rust API    │  │ Database │  │ Cache                │  │
│  └─────────────┘  └──────────┘  └──────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Prerequisites

### 1. Clone Repository with Submodules

```bash
# Clone the fungible-farm fork
git clone --recurse-submodules https://github.com/fungible-farm/commercerack-backend.git
cd commercerack-backend

# If already cloned, initialize submodules
git submodule update --init --recursive
```

### 2. Install Dependencies

```bash
# Rust toolchain
rustup update stable
rustup default stable

# b00t CLI (from submodule)
cd _b00t_
cargo build --release
cp target/release/b00t-cli /usr/local/bin/b00t
cd ..

# Verify b00t installation
b00t whoami

# OpenTofu (Terraform alternative)
# macOS
brew install opentofu

# Linux
curl --proto '=https' --tlsv1.2 -fsSL \
  https://get.opentofu.org/install-opentofu.sh \
  -o install-opentofu.sh
chmod +x install-opentofu.sh
./install-opentofu.sh --install-method standalone

# pm2 (process manager)
npm install -g pm2

# Python dependencies for agents
pip3 install asyncio
```

## Deployment Steps

### Phase 1: Infrastructure (OpenTofu + k0s)

```bash
cd infra/k0s

# Initialize OpenTofu
tofu init

# Review infrastructure plan
tofu plan

# Deploy k0s cluster and supporting services
tofu apply -auto-approve

# Verify deployment
docker ps
# Should show: k0s-controller, postgres, redis, commercerack-api
```

**Exposed Services**:
- k0s API: https://localhost:6443
- CommerceRack API: http://localhost:8000
- PostgreSQL: localhost:5432
- Redis: localhost:6379

### Phase 2: Database Setup

```bash
# Wait for PostgreSQL to be ready
sleep 10

# Apply schema migration
docker exec -i postgres psql -U postgres -d commercerack < \
  ../../commercerack-rust/migrations/001_initial_schema.sql

# Verify tables
docker exec postgres psql -U postgres -d commercerack -c "\dt"
# Should show 22 tables: customers, orders, products, etc.
```

### Phase 3: Multi-Agent System (pm2)

```bash
cd ../../agents

# Start multi-agent ecosystem
pm2 start ecosystem.config.js

# Verify all agents running
pm2 status
# Should show: captain, architect-agent, database-agent, rust-expert, 
#              devops-agent, qa-agent, security-agent, api-designer

# Watch logs in real-time
pm2 logs

# Or monitor specific agent
pm2 logs captain
```

### Phase 4: Trigger Captain Workflow

The captain agent automatically starts the CommerceRack migration workflow on startup. It will:

1. **Delegate to Architect** - Review workspace structure
2. **Delegate to Database** - Plan remaining schema migrations
3. **Delegate to Rust Expert** - Translate CUSTOMER.pm
4. **Delegate to DevOps** - Verify k0s deployment
5. **Delegate to QA** - Create test suites
6. **Delegate to Security** - Audit security patterns
7. **Delegate to API Designer** - Design RESTful API

Tasks are delegated via file-based communication:
- Tasks written to: `/tmp/agent_tasks/{agent_role}.json`
- Results written to: `/tmp/agent_results/{agent_role}_result.json`

### View Agent Results

```bash
# Check all results
ls -la /tmp/agent_results/

# View specific agent output
cat /tmp/agent_results/architect_result.json | jq .
cat /tmp/agent_results/database-architect_result.json | jq .
cat /tmp/agent_results/rust-expert_result.json | jq .
```

## Agent Specifications

### Captain Agent (`captain-001`)
- **Role**: Orchestrator
- **Memory**: 1GB
- **Features**: Flashbacker context management, task delegation
- **Script**: `agents/scripts/captain.py`

### Architect Agent (`architect-001`)
- **Specialty**: Architecture review and recommendations
- **Persona**: `flashback persona architect`
- **Memory**: 800MB

### Database Agent (`database-001`)
- **Specialty**: Schema design and migration planning
- **Persona**: `flashback persona database-architect`
- **Memory**: 800MB

### Rust Expert (`rust-001`)
- **Specialty**: Perl-to-Rust translation
- **Persona**: `flashback persona refactorer`
- **Memory**: 800MB

### DevOps Agent (`devops-001`)
- **Specialty**: Infrastructure deployment
- **Persona**: `flashback persona devops`
- **Memory**: 800MB

### QA Agent (`qa-001`)
- **Specialty**: Test creation and coverage
- **Persona**: `flashback persona qa`
- **Memory**: 600MB

### Security Agent (`security-001`)
- **Specialty**: Security auditing
- **Persona**: `flashback persona security`
- **Memory**: 600MB

### API Designer (`api-001`)
- **Specialty**: RESTful API design
- **Persona**: `flashback persona api-designer`
- **Memory**: 600MB

## Monitoring & Management

### pm2 Commands

```bash
# Status dashboard
pm2 status

# Monitor resource usage
pm2 monit

# View logs (all agents)
pm2 logs

# View specific agent logs
pm2 logs captain
pm2 logs rust-expert

# Restart agent
pm2 restart architect-agent

# Stop all agents
pm2 stop all

# Start all agents
pm2 start all

# Delete all agents (cleanup)
pm2 delete all

# Save pm2 configuration
pm2 save

# Setup pm2 startup script
pm2 startup
```

### k0s Cluster Management

```bash
# Get kubeconfig
export KUBECONFIG=/var/lib/k0s/pki/admin.conf

# Check cluster status
kubectl get nodes
kubectl get pods --all-namespaces

# Access k0s controller
docker exec -it k0s-controller k0s status
```

### Database Management

```bash
# Connect to PostgreSQL
docker exec -it postgres psql -U postgres -d commercerack

# Check table counts
\dt+

# View customer data
SELECT cid, email FROM customers LIMIT 5;

# Backup database
docker exec postgres pg_dump -U postgres commercerack > backup.sql
```

## Troubleshooting

### Agent Not Processing Tasks

```bash
# Check agent logs
pm2 logs {agent-name}

# Verify task file exists
ls -la /tmp/agent_tasks/

# Manually check task
cat /tmp/agent_tasks/architect.json | jq .

# Restart agent
pm2 restart {agent-name}
```

### Infrastructure Issues

```bash
# Check container status
docker ps -a

# View container logs
docker logs k0s-controller
docker logs postgres
docker logs commercerack-api

# Rebuild infrastructure
cd infra/k0s
tofu destroy -auto-approve
tofu apply -auto-approve
```

### Flashbacker Issues

```bash
# Verify flashback installation
which flashback
flashback --version

# Test persona loading
flashback persona architect

# Check memory system
flashback memory --show

# Verify working plan
flashback working-plan --show
```

## Development Workflow

### Adding New Agents

1. Update `agents/ecosystem.config.js`:
```javascript
{
  name: 'new-agent',
  script: './agents/scripts/specialist_agent.py',
  env: {
    AGENT_ROLE: 'new-role',
    SPECIALIST_TYPE: 'new-specialty',
    FLASHBACKER_PERSONA: 'persona-name'
  }
}
```

2. Add specialist logic in `specialist_agent.py`

3. Restart pm2 ecosystem:
```bash
pm2 restart all
```

### Scaling Agents

```bash
# Scale specific agent (multiple instances)
pm2 scale rust-expert 3

# Back to single instance
pm2 scale rust-expert 1
```

## Environment Variables

Create `.env` file:

```bash
# Database
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/commercerack

# Redis
REDIS_URL=redis://localhost:6379

# b00t
B00T_ENABLED=true
B00T_PATH=/usr/local/bin/b00t

# Flashbacker
FLASHBACKER_ENABLED=true
FLASHBACKER_BIN=flashback

# Logging
RUST_LOG=info
LOG_LEVEL=info
```

## Next Steps

1. **Monitor agent execution**: `pm2 logs`
2. **Review agent outputs**: Check `/tmp/agent_results/`
3. **Implement agent recommendations**: Follow specialist guidance
4. **Iterate on translation**: Rust experts will guide Perl → Rust conversion
5. **Deploy to production**: Scale k0s cluster, add monitoring

## Resources

- **b00t Documentation**: See `_b00t_/README.md`
- **OpenTofu Docs**: https://opentofu.org/docs/
- **k0s Documentation**: https://docs.k0sproject.io/
- **pm2 Guide**: https://pm2.keymetrics.io/docs/usage/quick-start/
- **flashbacker**: See `.claude/flashback/` in this repo

## Support

For issues with:
- **b00t framework**: https://github.com/elasticdotventures/_b00t_/issues
- **CommerceRack fork**: https://github.com/fungible-farm/commercerack-backend/issues
- **flashbacker**: Check project documentation

---

**Status**: Multi-agent hive operational 🥾  
**Captain**: Ready to orchestrate specialized sub-agents  
**Infrastructure**: k0s cluster with Postgres + Redis  
**Translation**: Perl → b00t Rust in progress 🦀
