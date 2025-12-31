# Examples and Demos

This directory contains example scripts demonstrating how to use the Data AI Agent.

## Quick Start

Make sure you have the project dependencies installed:

```bash
uv sync
```

Set up your environment variables in `.env`:

```bash
OPENAI_API_KEY=your-api-key-here
```

## Available Examples

1. **basic_agent_usage.py** - Basic usage of the agent with simple calculations
2. **tool_examples.py** - Demonstrates all available tools
3. **api_client_example.py** - Shows how to interact with the API
4. **custom_tool_example.py** - How to create and register custom tools
5. **langchain_integration.py** - Integration with LangChain components

## Running Examples

Run any example using uv:

```bash
uv run examples/basic_agent_usage.py
```

Or with the virtual environment:

```bash
source .venv/bin/activate
python examples/basic_agent_usage.py
```
