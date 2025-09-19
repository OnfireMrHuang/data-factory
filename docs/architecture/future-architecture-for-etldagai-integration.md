# Future Architecture for ETL/DAG/AI Integration

## Planned Integration Points

**For AI agents implementing ETL pipeline logic:**

1. **Workflow Definition API**: REST endpoints for DAG creation and management
2. **Node Registration**: Dynamic data processing node registration system
3. **Execution Engine Integration**: Communication layer with data-engine for job execution
4. **AI Agent Coordination**: Interfaces for AI-driven workflow optimization and generation

**Expected Data Flow:**
```
data-terminal (UI/Management) → data-engine (Processing) → data-agent (AI Optimization)
```

## Code Generation Target Areas

**For AI agents generating data node code:**

- **Workflow Nodes**: `backend/src/workflow/nodes/` (to be created)
- **DAG Definition**: `backend/src/workflow/dag/` (to be created)
- **Processing Logic**: Integration with `../data-engine/` Java components
- **AI Integration**: Communication with `../data-agent/` Python components
