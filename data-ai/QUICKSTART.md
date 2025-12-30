# Quick Start Guide

## Prerequisites

- Python 3.12+
- OpenAI API key
- uv (Python package manager)

## Installation

1. Clone the repository and navigate to the project directory:

```bash
cd data-ai
```

2. Create a `.env` file from the example:

```bash
cp .env.example .env
```

3. Edit `.env` and add your OpenAI API key:

```bash
OPENAI_API_KEY=your-actual-api-key-here
```

4. Install dependencies using uv:

```bash
uv pip install -e .
```

For development dependencies:

```bash
uv pip install -e ".[dev]"
```

## Running the Application

### Development Mode

```bash
uv run python main.py
```

Or with hot reload:

```bash
uv run uvicorn src.app:app --reload
```

The API will be available at `http://localhost:8000`

### API Documentation

Once running, visit:
- Swagger UI: `http://localhost:8000/api/v1/docs`
- ReDoc: `http://localhost:8000/api/v1/redoc`

## Usage Examples

### 1. Health Check

```bash
curl http://localhost:8000/health
```

### 2. Simple Chat Request

```bash
curl -X POST http://localhost:8000/api/v1/agent/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Calculate the square root of 144"
  }'
```

### 3. Data Analysis Request

```bash
curl -X POST http://localhost:8000/api/v1/agent/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Generate a SQL query to find the top 10 customers by revenue"
  }'
```

### 4. Streaming Response

```bash
curl -X POST http://localhost:8000/api/v1/agent/stream \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Explain how to analyze customer churn data"
  }'
```

### 5. Using Python Requests

```python
import requests

# Simple chat
response = requests.post(
    "http://localhost:8000/api/v1/agent/chat",
    json={
        "message": "What's the best way to handle missing data in a dataset?",
        "session_id": "user-123"
    }
)
print(response.json())

# Streaming response
response = requests.post(
    "http://localhost:8000/api/v1/agent/stream",
    json={"message": "Generate SQL for customer segmentation"},
    stream=True
)

for line in response.iter_lines():
    if line:
        print(line.decode('utf-8'))
```

## Agent Capabilities

The agent can help with:

1. **Mathematical Calculations**: Evaluate expressions and perform complex math
2. **Data Analysis**: Provide summaries and insights on data
3. **SQL Generation**: Create SQL queries from natural language
4. **General Questions**: Answer data-related questions

## Configuration

Key environment variables in `.env`:

- `OPENAI_API_KEY`: Your OpenAI API key (required)
- `OPENAI_MODEL`: Model to use (default: gpt-4o-mini)
- `DEBUG`: Enable debug mode (default: false)
- `API_PORT`: Port to run on (default: 8000)

For LangSmith tracing (optional):

```bash
LANGCHAIN_TRACING_V2=true
LANGCHAIN_API_KEY=your-langsmith-key
LANGCHAIN_PROJECT=data-ai-agent
```

## Project Structure

```
data-ai/
├── src/
│   ├── api/              # FastAPI routes and dependencies
│   │   └── routes/       # API endpoints
│   ├── agents/           # LangGraph agent implementation
│   ├── core/             # Configuration and logging
│   └── schemas/          # Pydantic models
├── tests/                # Test files
├── logs/                 # Application logs
├── main.py               # Application entry point
└── pyproject.toml        # Dependencies and configuration
```

## Development

### Running Tests

```bash
uv run pytest
```

With coverage:

```bash
uv run pytest --cov=src tests/
```

### Code Quality

Format and lint code:

```bash
uv run ruff check .
uv run ruff format .
```

## Extending the Agent

### Adding New Tools

1. Create a new tool in `src/agents/tools.py`:

```python
from typing import Annotated
from langchain_core.tools import tool

@tool
def my_custom_tool(input: Annotated[str, "Description of input"]) -> str:
    """Tool description."""
    # Your implementation
    return "result"
```

2. Add to the tools list in `get_tools()`:

```python
def get_tools() -> list:
    return [
        calculate,
        get_data_summary,
        generate_sql,
        my_custom_tool,  # Add your tool
    ]
```

The agent will automatically use the new tool!

### Customizing the Agent

Edit `src/agents/graph.py` to modify:
- System prompt
- Graph structure
- Tool calling behavior
- State management

## Troubleshooting

### Port Already in Use

Change the port in `.env`:

```bash
API_PORT=8001
```

### OpenAI API Errors

- Verify your API key is correct
- Check your OpenAI account has credits
- Ensure network connectivity

### Import Errors

Reinstall dependencies:

```bash
uv pip install -e ".[dev]"
```

## Next Steps

- Add custom tools for your specific use case
- Integrate with your data sources
- Implement authentication and rate limiting
- Add conversation history with a database
- Deploy to production (Docker, Cloud platforms)

## Support

For issues or questions, refer to:
- LangChain docs: https://python.langchain.com
- LangGraph docs: https://langchain-ai.github.io/langgraph
- FastAPI docs: https://fastapi.tiangolo.com
