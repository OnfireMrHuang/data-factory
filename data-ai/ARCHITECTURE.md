# Data AI Agent Architecture

## Overview

This is a production-ready AI agent application built with **FastAPI** (async web framework) and **LangGraph** (agent orchestration). The architecture follows best practices for scalability, maintainability, and extensibility.

## Technology Stack

- **FastAPI**: Modern async web framework for building APIs
- **LangGraph**: State-based agent orchestration with full control
- **LangChain**: LLM application framework
- **OpenAI**: GPT-4/GPT-3.5 for language understanding
- **Pydantic**: Data validation and settings management
- **Uvicorn**: ASGI server with async support

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        FastAPI Application                    │
│                                                               │
│  ┌─────────────┐      ┌──────────────┐    ┌──────────────┐ │
│  │   Routes    │─────▶│ Dependencies │◀───│   Schemas    │ │
│  │  (agent.py) │      │              │    │ (requests.py)│ │
│  └─────────────┘      └──────────────┘    └──────────────┘ │
│         │                     │                              │
│         ▼                     ▼                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              LangGraph Agent Layer                      │ │
│  │                                                          │ │
│  │  ┌──────────┐    ┌──────────┐    ┌─────────────────┐  │ │
│  │  │  Graph   │───▶│   LLM    │───▶│     Tools       │  │ │
│  │  │ (State)  │    │ (OpenAI) │    │ (calculate,     │  │ │
│  │  │ Machine  │    │          │    │  generate_sql)  │  │ │
│  │  └──────────┘    └──────────┘    └─────────────────┘  │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                               │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
                  ┌──────────────┐
                  │ Configuration │
                  │   & Logging   │
                  └──────────────┘
```

## Directory Structure

```
data-ai/
├── src/
│   ├── api/                    # API layer
│   │   ├── routes/
│   │   │   ├── agent.py       # Agent endpoints
│   │   │   └── __init__.py
│   │   ├── dependencies.py     # Shared dependencies
│   │   └── __init__.py
│   │
│   ├── agents/                 # Agent layer
│   │   ├── graph.py           # LangGraph state machine
│   │   ├── tools.py           # Agent tools
│   │   └── __init__.py
│   │
│   ├── core/                   # Core utilities
│   │   ├── config.py          # Configuration management
│   │   ├── logging.py         # Logging setup
│   │   └── __init__.py
│   │
│   ├── schemas/                # Data models
│   │   ├── requests.py        # Request/response models
│   │   └── __init__.py
│   │
│   └── app.py                  # FastAPI app factory
│
├── tests/                      # Test suite
│   ├── test_api.py
│   └── __init__.py
│
├── main.py                     # Application entry point
├── pyproject.toml              # Dependencies
├── .env.example                # Environment template
├── QUICKSTART.md               # Getting started guide
└── README.md                   # Project documentation
```

## Key Components

### 1. FastAPI Application (`src/app.py`)

- **Async-first**: All endpoints use `async def` for non-blocking I/O
- **CORS middleware**: Configured for cross-origin requests
- **Lifespan management**: Proper startup/shutdown handling
- **Error handling**: Global exception handler
- **Auto-generated docs**: Swagger UI and ReDoc

### 2. LangGraph Agent (`src/agents/graph.py`)

**State Machine Design:**

```
┌─────────┐
│  START  │
└────┬────┘
     │
     ▼
┌─────────┐
│  AGENT  │──────┐
│  (LLM)  │      │
└────┬────┘      │
     │           │
     │ tools?    │ no tools
     ▼           │
┌─────────┐      │
│  TOOLS  │      │
│ (Execute)│     │
└────┬────┘      │
     │           │
     └────┬──────┘
          ▼
     ┌─────────┐
     │   END   │
     └─────────┘
```

**Features:**
- State-based workflow management
- Conditional branching based on tool calls
- Full control over agent execution flow
- Message history tracking
- Intermediate steps logging

### 3. Tools (`src/agents/tools.py`)

**Available Tools:**
1. `calculate`: Safe mathematical expression evaluation
2. `get_data_summary`: Data analysis (extensible)
3. `generate_sql`: SQL query generation (extensible)

**Tool Pattern:**
```python
@tool
def tool_name(param: Annotated[type, "description"]) -> str:
    """Tool description for the LLM."""
    # Implementation
    return result
```

### 4. Configuration (`src/core/config.py`)

**Features:**
- Environment-based configuration
- Pydantic settings validation
- Type-safe settings access
- Singleton pattern with `@lru_cache`

**Configuration Sources:**
1. `.env` file
2. Environment variables
3. Default values

### 5. API Endpoints

**POST /api/v1/agent/chat**
- Synchronous chat with the agent
- Returns complete response
- Includes intermediate steps

**POST /api/v1/agent/stream**
- Streaming response via SSE
- Real-time agent thinking
- Event-based architecture

**GET /health**
- Health check endpoint
- Service status monitoring

## Best Practices Implemented

### 1. Async All The Way

```python
# FastAPI routes
async def chat(request: AgentRequest):
    result = await agent.run(message)  # Async

# LangGraph agent
async def run(self, message: str):
    result = await self.graph.ainvoke(state)  # Async
```

### 2. Dependency Injection

```python
@router.post("/chat")
async def chat(
    request: AgentRequest,
    agent: Annotated[DataAnalysisAgent, Depends(get_agent_dependency)]
):
    # Agent injected automatically
```

### 3. Type Safety

```python
# Pydantic models for validation
class AgentRequest(BaseModel):
    message: str = Field(..., min_length=1)
    session_id: str | None = None

# Type hints everywhere
async def run(self, message: str) -> dict:
```

### 4. Configuration Management

```python
# Environment-based settings
class Settings(BaseSettings):
    openai_api_key: str
    openai_model: str = "gpt-4o-mini"

# Cached singleton
@lru_cache
def get_settings() -> Settings:
    return Settings()
```

### 5. Structured Logging

```python
# Consistent logging
logger = logging.getLogger(__name__)
logger.info(f"Processing: {message}")
logger.error(f"Error: {e}", exc_info=True)
```

### 6. Error Handling

```python
# Route-level handling
try:
    result = await agent.run(message)
except Exception as e:
    raise HTTPException(status_code=500, detail=str(e))

# Global handler
@app.exception_handler(Exception)
async def global_exception_handler(request, exc):
    logger.error(f"Unhandled: {exc}", exc_info=True)
```

### 7. Testing

```python
# Async tests with pytest
@pytest.mark.asyncio
async def test_chat_endpoint(app):
    async with AsyncClient(app=app) as client:
        response = await client.post("/api/v1/agent/chat", ...)
```

## Scalability Considerations

### Current Architecture
- Single agent instance (singleton)
- In-memory state
- Suitable for: Development, small deployments

### Production Enhancements
1. **Session Management**: Add Redis for distributed sessions
2. **Agent Pool**: Multiple agent instances with load balancing
3. **Message Queue**: Celery/RQ for background tasks
4. **Database**: PostgreSQL for conversation history
5. **Caching**: Redis for frequently accessed data
6. **Rate Limiting**: Token bucket or sliding window
7. **Authentication**: JWT tokens or OAuth2
8. **Observability**: LangSmith, Prometheus, Grafana

## Extending the System

### Adding New Tools

1. Define the tool in `src/agents/tools.py`:
```python
@tool
def my_tool(param: Annotated[str, "description"]) -> str:
    """Tool documentation."""
    return result
```

2. Add to `get_tools()`:
```python
def get_tools() -> list:
    return [calculate, get_data_summary, generate_sql, my_tool]
```

### Adding New Routes

1. Create route file in `src/api/routes/`:
```python
router = APIRouter(prefix="/myroute", tags=["myroute"])

@router.post("/endpoint")
async def my_endpoint():
    pass
```

2. Include in `src/app.py`:
```python
from .api.routes import agent_router, my_router
app.include_router(my_router, prefix=settings.api_prefix)
```

### Customizing the Agent

Modify `src/agents/graph.py`:
- Change system prompt
- Add/remove nodes in the graph
- Customize conditional logic
- Add memory/state persistence

## Performance Tips

1. **Use streaming for long responses**: Reduces perceived latency
2. **Configure timeouts appropriately**: Prevent hanging requests
3. **Limit max_iterations**: Prevent infinite loops
4. **Use appropriate OpenAI model**: Balance cost vs. capability
5. **Add caching**: Cache frequently requested analyses
6. **Monitor token usage**: Track OpenAI costs

## Security Considerations

1. **API Key Management**: Never commit `.env` file
2. **Input Validation**: Pydantic handles this automatically
3. **Rate Limiting**: Add for production
4. **CORS**: Configure allowed origins properly
5. **Safe Eval**: Tools use restricted eval contexts
6. **Error Messages**: Don't expose stack traces in production

## Monitoring and Debugging

### LangSmith Integration

Enable in `.env`:
```bash
LANGCHAIN_TRACING_V2=true
LANGCHAIN_API_KEY=your-key
```

Benefits:
- Trace agent execution
- Debug tool calls
- Monitor performance
- Track costs

### Logs

- Application logs: `logs/app.log`
- Console output: Formatted for readability
- Structured logging: Easy to parse and analyze

## Deployment Options

1. **Docker**:
```dockerfile
FROM python:3.12
COPY . /app
WORKDIR /app
RUN pip install -e .
CMD ["python", "main.py"]
```

2. **Cloud Platforms**:
- AWS Lambda + API Gateway
- Google Cloud Run
- Azure Container Apps
- Heroku

3. **Traditional**:
- Systemd service
- Nginx reverse proxy
- Gunicorn + Uvicorn workers

## Summary

This architecture provides:
- ✅ Production-ready async FastAPI application
- ✅ Sophisticated LangGraph agent with state management
- ✅ Extensible tool system
- ✅ Proper configuration and logging
- ✅ Type safety and validation
- ✅ Comprehensive documentation
- ✅ Testing infrastructure
- ✅ Clear separation of concerns

The design is modular, maintainable, and ready for both development and production use.
