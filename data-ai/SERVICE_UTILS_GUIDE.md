# Service and Utils Layers Guide

## Overview

This document describes the service and utils layers added to the Data AI Agent project, explaining their purpose, design, and usage.

## Architecture Layers

The project now follows a clean layered architecture:

```
┌─────────────────────────────────────────┐
│           API Layer (FastAPI)           │  ← Routes, endpoints, HTTP handling
├─────────────────────────────────────────┤
│          Service Layer                  │  ← Business logic, validation
├─────────────────────────────────────────┤
│          Agent Layer (LangGraph)        │  ← AI agent, tools, LLM interaction
├─────────────────────────────────────────┤
│          Utils Layer                    │  ← Shared utilities, helpers
└─────────────────────────────────────────┘
```

## Utils Layer

The utils layer provides reusable utility functions organized by concern.

### Location
`src/utils/`

### Modules

#### 1. `text.py` - Text Processing

**Purpose**: Text manipulation and formatting utilities

**Functions**:
- `truncate_text(text, max_length, suffix)`: Truncate long text safely
- `clean_whitespace(text)`: Remove excessive whitespace
- `extract_code_blocks(text)`: Parse markdown code blocks
- `sanitize_input(text, max_length)`: Clean and validate user input
- `format_prompt(template, **kwargs)`: Format prompt templates

**Example**:
```python
from src.utils import truncate_text, sanitize_input

# Truncate for logging
short_msg = truncate_text(user_message, 100)
logger.info(f"Processing: {short_msg}")

# Sanitize user input
clean_input = sanitize_input(user_message)
```

#### 2. `validation.py` - Input Validation

**Purpose**: Validate data formats and constraints

**Functions**:
- `is_valid_session_id(session_id)`: Check session ID format
- `validate_message_length(message, min, max)`: Validate message size
- `is_safe_sql(query)`: Basic SQL safety check
- `validate_dict_schema(data, required_keys)`: Ensure required keys exist

**Example**:
```python
from src.utils import validate_message_length, is_valid_session_id

# Validate message
is_valid, error = validate_message_length(message, min_length=1, max_length=10000)
if not is_valid:
    raise ValidationError(error)

# Validate session ID
if session_id and not is_valid_session_id(session_id):
    raise ValidationError("Invalid session ID format")
```

#### 3. `datetime.py` - Time Utilities

**Purpose**: Time and date operations

**Functions**:
- `utc_now()`: Get current UTC timestamp
- `format_timestamp(dt, format_str)`: Format datetime to string
- `parse_timestamp(timestamp_str, format_str)`: Parse string to datetime
- `get_elapsed_seconds(start_time)`: Calculate elapsed time
- `timestamp_to_iso(dt)`: Convert to ISO format

**Example**:
```python
from src.utils import utc_now, get_elapsed_seconds

start_time = utc_now()
# ... do work ...
execution_time = get_elapsed_seconds(start_time)
logger.info(f"Completed in {execution_time:.2f}s")
```

#### 4. `errors.py` - Error Handling

**Purpose**: Custom exceptions and error formatting

**Classes**:
- `ServiceError`: Base service exception
- `ValidationError`: Input validation errors
- `AgentError`: Agent execution errors
- `TimeoutError`: Operation timeout errors

**Functions**:
- `log_error(error, context)`: Log error with context
- `format_error_response(error)`: Format for API response

**Example**:
```python
from src.utils import ValidationError, AgentError, log_error

try:
    # Validate input
    if not is_valid:
        raise ValidationError("Invalid input", details={"field": "message"})

    # Process with agent
    result = await agent.run(message)

except Exception as e:
    log_error(e, context={"user_id": user_id})
    raise AgentError(f"Processing failed: {str(e)}") from e
```

## Service Layer

The service layer encapsulates business logic and provides a clean interface between the API and agent layers.

### Location
`src/services/`

### Services

#### 1. `agent_service.py` - Agent Business Logic

**Purpose**: Handle agent operations with validation, error handling, and monitoring

**Class**: `AgentService`

**Methods**:
- `process_message(message, session_id, timeout)`: Process a single message
- `stream_message(message, session_id)`: Stream agent responses
- `_validate_message(message)`: Validate user message
- `_validate_session_id(session_id)`: Validate session ID

**Features**:
- Input validation and sanitization
- Timeout management
- Execution time tracking
- Comprehensive error handling
- Logging and monitoring

**Example**:
```python
from src.services import get_agent_service

service = get_agent_service()

# Process message
result = await service.process_message(
    message="Calculate 2 + 2",
    session_id="user-123",
    timeout=300
)

# Result includes metadata
print(result["response"])  # "4"
print(result["metadata"]["execution_time"])  # 1.23
```

**Integration in API Routes**:
```python
from fastapi import Depends
from src.services import AgentService
from src.api.dependencies import get_agent_service_dependency

@router.post("/chat")
async def chat(
    request: AgentRequest,
    service: Annotated[AgentService, Depends(get_agent_service_dependency)],
):
    result = await service.process_message(
        message=request.message,
        session_id=request.session_id,
    )
    return AgentResponse(**result)
```

#### 2. `session_service.py` - Session Management

**Purpose**: Manage user sessions and conversation context

**Class**: `SessionManager`

**Methods**:
- `create_session(session_id, metadata)`: Create new session
- `get_session(session_id)`: Retrieve session data
- `update_session(session_id, **updates)`: Update session
- `delete_session(session_id)`: Remove session
- `get_or_create_session(session_id, metadata)`: Get existing or create new
- `increment_message_count(session_id)`: Track message count
- `get_stats()`: Get session statistics

**Features**:
- In-memory session storage (easily replaceable with Redis)
- Automatic expiration (TTL)
- Session metadata tracking
- Message count tracking
- Automatic cleanup of expired sessions

**Example**:
```python
from src.services import get_session_manager

manager = get_session_manager()

# Create session
session = manager.create_session("user-123", metadata={"user_name": "Alice"})

# Track messages
manager.increment_message_count("user-123")

# Get session info
session = manager.get_session("user-123")
print(session["message_count"])  # 1

# Get stats
stats = manager.get_stats()
print(stats["total_sessions"])
```

**Integration in API Routes**:
```python
from fastapi import Depends
from src.services import SessionManager
from src.api.dependencies import get_session_manager_dependency

@router.post("/chat")
async def chat(
    request: AgentRequest,
    session_manager: Annotated[SessionManager, Depends(get_session_manager_dependency)],
):
    if request.session_id:
        session_manager.get_or_create_session(request.session_id)
        session_manager.increment_message_count(request.session_id)
    # ... process message
```

## Benefits of Layered Architecture

### 1. Separation of Concerns
- **API Layer**: HTTP concerns (request/response, status codes)
- **Service Layer**: Business logic (validation, processing)
- **Agent Layer**: AI concerns (LLM, tools, prompts)
- **Utils Layer**: Reusable utilities

### 2. Testability
Each layer can be tested independently:

```python
# Test service layer without API
def test_agent_service():
    service = AgentService()
    result = await service.process_message("test")
    assert result["response"]

# Test utils independently
def test_validation():
    is_valid, error = validate_message_length("", min_length=1)
    assert not is_valid
```

### 3. Reusability
Utilities and services can be used across multiple routes:

```python
# Use validation in multiple places
from src.utils import validate_message_length

# Route 1
is_valid, error = validate_message_length(message)

# Route 2
is_valid, error = validate_message_length(feedback)
```

### 4. Maintainability
Changes are localized:
- Update validation logic → edit `utils/validation.py`
- Change business rules → edit `services/agent_service.py`
- Modify API response → edit `api/routes/agent.py`

### 5. Consistency
Shared utilities ensure consistent behavior:
- All inputs sanitized the same way
- All timestamps in UTC
- All errors formatted consistently

## New API Endpoints

### Session Management

#### GET `/api/v1/sessions/{session_id}`
Get session information

**Response**:
```json
{
  "session_id": "user-123",
  "created_at": "2024-01-01T00:00:00Z",
  "last_accessed": "2024-01-01T00:05:00Z",
  "message_count": 5,
  "metadata": {}
}
```

#### DELETE `/api/v1/sessions/{session_id}`
Delete a session

**Response**:
```json
{
  "message": "Session deleted successfully",
  "session_id": "user-123"
}
```

#### GET `/api/v1/sessions/`
Get session statistics

**Response**:
```json
{
  "total_sessions": 42,
  "max_sessions": 1000,
  "session_ttl": 3600
}
```

## Migration Guide

### Before (Direct Agent Access)
```python
@router.post("/chat")
async def chat(request: AgentRequest, agent: DataAnalysisAgent):
    result = await agent.run(message=request.message)
    return result
```

### After (Service Layer)
```python
@router.post("/chat")
async def chat(
    request: AgentRequest,
    service: Annotated[AgentService, Depends(get_agent_service_dependency)],
):
    try:
        result = await service.process_message(
            message=request.message,
            session_id=request.session_id,
        )
        return AgentResponse(**result)
    except ValidationError as e:
        raise HTTPException(status_code=400, detail=format_error_response(e))
```

**Improvements**:
- Input validation built-in
- Better error handling
- Execution tracking
- Session management
- Consistent logging

## Best Practices

### 1. Always Use Service Layer in Routes
```python
# ✅ Good
service = get_agent_service()
result = await service.process_message(message)

# ❌ Bad
agent = get_agent()
result = await agent.run(message)  # Bypasses validation, logging
```

### 2. Use Utils for Common Operations
```python
# ✅ Good
from src.utils import truncate_text, sanitize_input
clean_msg = sanitize_input(message)
logger.info(f"Processing: {truncate_text(clean_msg, 100)}")

# ❌ Bad
clean_msg = message[:10000]  # Reimplementing logic
logger.info(f"Processing: {message[:100]}...")  # Inconsistent
```

### 3. Handle Errors Appropriately
```python
# ✅ Good
from src.utils import ValidationError, AgentError

try:
    result = await service.process_message(message)
except ValidationError as e:
    raise HTTPException(status_code=400, detail=format_error_response(e))
except AgentError as e:
    raise HTTPException(status_code=500, detail=format_error_response(e))

# ❌ Bad
try:
    result = await service.process_message(message)
except Exception as e:
    raise HTTPException(status_code=500, detail=str(e))  # Loses context
```

### 4. Use Session Management
```python
# ✅ Good
if session_id:
    manager.get_or_create_session(session_id)
    manager.increment_message_count(session_id)

# ❌ Bad
# Not tracking sessions at all
```

## Future Enhancements

### 1. Persistent Session Storage
Replace in-memory sessions with Redis:

```python
class RedisSessionManager(SessionManager):
    def __init__(self, redis_client):
        self.redis = redis_client

    def get_session(self, session_id):
        data = self.redis.get(f"session:{session_id}")
        return json.loads(data) if data else None
```

### 2. Advanced Validation
Add schema validation, rate limiting, content filtering:

```python
def validate_content_safety(message: str) -> tuple[bool, str | None]:
    """Check message for inappropriate content."""
    # Use content moderation API
    pass
```

### 3. Caching Layer
Add caching for frequently requested analyses:

```python
from functools import lru_cache

@lru_cache(maxsize=100)
def get_cached_result(message_hash: str):
    """Cache agent responses for identical queries."""
    pass
```

### 4. Metrics and Monitoring
Track service metrics:

```python
from prometheus_client import Counter, Histogram

request_counter = Counter("agent_requests_total", "Total agent requests")
latency_histogram = Histogram("agent_latency_seconds", "Agent latency")
```

## Summary

The service and utils layers provide:

- **Clean separation** of concerns
- **Better testability** through isolation
- **Consistent validation** and error handling
- **Reusable utilities** across the codebase
- **Session management** for conversation context
- **Monitoring and logging** built-in
- **Easy extensibility** for future features

This architecture makes the codebase more maintainable, testable, and production-ready.
