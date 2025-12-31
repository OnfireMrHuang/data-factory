"""Basic example of using the Data AI Agent.

This example demonstrates:
- Using the calculate tool
- Basic agent interaction
- Handling responses
"""

import asyncio
import logging

from src.agents.tools import calculate, get_data_summary, get_tools
from src.core.config import get_settings
from src.core.logging import setup_logging

# Set up logging
setup_logging()
logger = logging.getLogger(__name__)


def basic_tool_usage():
    """Demonstrate basic tool usage."""
    print("\n=== Basic Tool Usage ===\n")

    # Example 1: Simple calculation
    print("1. Simple calculation: 2 + 2")
    result = calculate.invoke({"expression": "2 + 2"})
    print(f"   Result: {result}\n")

    # Example 2: Complex calculation
    print("2. Complex calculation: (100 - 32) * 5/9")
    result = calculate.invoke({"expression": "(100 - 32) * 5/9"})
    print(f"   Result: {result} (Fahrenheit to Celsius)\n")

    # Example 3: Math functions
    print("3. Using math functions: sqrt(144)")
    result = calculate.invoke({"expression": "sqrt(144)"})
    print(f"   Result: {result}\n")

    # Example 4: Data summary
    print("4. Getting data summary")
    result = get_data_summary.invoke({"data_description": "sales data from Q4 2024"})
    print(f"   Result:\n{result}\n")


def list_available_tools():
    """List all available tools."""
    print("\n=== Available Tools ===\n")

    tools = get_tools()
    for i, tool in enumerate(tools, 1):
        print(f"{i}. {tool.name}")
        print(f"   Description: {tool.description}")
        print()


def main():
    """Main function to run examples."""
    print("=" * 60)
    print("Data AI Agent - Basic Usage Example")
    print("=" * 60)

    # Load settings
    settings = get_settings()
    logger.info(f"Running {settings.app_name} v{settings.app_version}")

    # Demonstrate basic tool usage
    basic_tool_usage()

    # List available tools
    list_available_tools()

    print("=" * 60)
    print("Example completed!")
    print("=" * 60)


if __name__ == "__main__":
    main()
