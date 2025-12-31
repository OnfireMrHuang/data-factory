"""Example demonstrating LangChain integration patterns.

This shows how to integrate with LangChain components like chains,
memory, and agents.
"""

import logging
from typing import List

from langchain_core.messages import AIMessage, HumanMessage, SystemMessage
from langchain_core.prompts import ChatPromptTemplate, MessagesPlaceholder

from src.agents.tools import get_tools
from src.core.config import get_settings
from src.core.logging import setup_logging

setup_logging()
logger = logging.getLogger(__name__)


def example_prompt_templates():
    """Demonstrate LangChain prompt templates."""
    print("\n=== Prompt Template Examples ===\n")

    # Simple template
    simple_template = ChatPromptTemplate.from_messages(
        [
            ("system", "You are a helpful data analysis assistant."),
            ("human", "{user_input}"),
        ]
    )

    print("1. Simple template:")
    messages = simple_template.format_messages(user_input="What is 2 + 2?")
    for msg in messages:
        print(f"   {msg.type}: {msg.content}")
    print()

    # Template with history
    history_template = ChatPromptTemplate.from_messages(
        [
            ("system", "You are a data analyst. Help with {task_type}."),
            MessagesPlaceholder(variable_name="history"),
            ("human", "{user_input}"),
        ]
    )

    print("2. Template with message history:")
    messages = history_template.format_messages(
        task_type="SQL generation",
        history=[
            HumanMessage(content="Show me sales data"),
            AIMessage(content="Here's a query for sales data..."),
        ],
        user_input="Now filter by region",
    )
    for msg in messages:
        print(f"   {msg.type}: {msg.content[:50]}...")
    print()


def example_conversation_history():
    """Demonstrate managing conversation history."""
    print("\n=== Conversation History Management ===\n")

    # Simulate a conversation
    conversation: List = []

    # Add system message
    conversation.append(
        SystemMessage(content="You are a helpful data analysis assistant.")
    )

    # Simulate turns
    turns = [
        ("What is the average of 10, 20, and 30?", "The average is 20."),
        ("Calculate the sum instead", "The sum is 60."),
        ("What percentage is 20 of 60?", "20 is 33.33% of 60."),
    ]

    for human_msg, ai_msg in turns:
        conversation.append(HumanMessage(content=human_msg))
        conversation.append(AIMessage(content=ai_msg))

    print("Conversation history:")
    for i, msg in enumerate(conversation, 1):
        role = msg.type
        content = msg.content[:60] + "..." if len(msg.content) > 60 else msg.content
        print(f"  {i}. [{role}] {content}")
    print()

    print(f"Total messages: {len(conversation)}")
    print(f"Memory size: ~{sum(len(m.content) for m in conversation)} characters")
    print()


def example_tool_integration():
    """Demonstrate integrating tools with LangChain."""
    print("\n=== Tool Integration ===\n")

    tools = get_tools()

    print(f"Available tools: {len(tools)}\n")

    for tool in tools:
        print(f"Tool: {tool.name}")
        print(f"  Description: {tool.description}")
        print(f"  Args: {tool.args}")
        print()

    # Example: Using a tool directly
    print("Example tool usage:")
    calculate_tool = next(t for t in tools if t.name == "calculate")
    result = calculate_tool.invoke({"expression": "sqrt(144)"})
    print(f"  calculate('sqrt(144)') = {result}")
    print()


def example_structured_output():
    """Demonstrate structured output patterns."""
    print("\n=== Structured Output Patterns ===\n")

    from pydantic import BaseModel, Field

    # Define output schema
    class AnalysisResult(BaseModel):
        """Structured analysis result."""

        query: str = Field(description="The SQL query")
        estimated_rows: int = Field(description="Estimated number of rows")
        complexity: str = Field(description="Query complexity: simple, medium, complex")
        warnings: List[str] = Field(
            default_factory=list, description="Any warnings or notes"
        )

    # Example result
    result = AnalysisResult(
        query="SELECT * FROM users WHERE active = true",
        estimated_rows=1500,
        complexity="simple",
        warnings=["Consider adding LIMIT clause"],
    )

    print("Structured output example:")
    print(f"  Query: {result.query}")
    print(f"  Estimated rows: {result.estimated_rows}")
    print(f"  Complexity: {result.complexity}")
    print(f"  Warnings: {', '.join(result.warnings) if result.warnings else 'None'}")
    print()

    print("JSON representation:")
    print(f"  {result.model_dump_json(indent=2)}")
    print()


def example_error_handling():
    """Demonstrate error handling patterns."""
    print("\n=== Error Handling Patterns ===\n")

    from src.agents.tools import calculate

    test_cases = [
        ("Valid", "2 + 2"),
        ("Invalid syntax", "2 +"),
        ("Division by zero", "10 / 0"),
        ("Undefined function", "unknown_func()"),
    ]

    for description, expression in test_cases:
        print(f"{description}: {expression}")
        try:
            result = calculate.invoke({"expression": expression})
            print(f"  Result: {result}")
        except Exception as e:
            print(f"  Error: {e}")
        print()


def main():
    """Run all LangChain integration examples."""
    print("=" * 60)
    print("LangChain Integration Examples")
    print("=" * 60)

    settings = get_settings()
    print(f"\nUsing {settings.app_name} v{settings.app_version}")
    print(f"OpenAI Model: {settings.openai_model}")
    print("=" * 60)

    example_prompt_templates()
    example_conversation_history()
    example_tool_integration()
    example_structured_output()
    example_error_handling()

    print("=" * 60)
    print("All examples completed!")
    print("=" * 60)
    print("\nNext steps:")
    print("  - Review LangChain docs: https://python.langchain.com")
    print("  - Explore LangGraph: https://langchain-ai.github.io/langgraph")
    print("  - Check out the agent implementation in src/agents/")
    print("=" * 60 + "\n")


if __name__ == "__main__":
    main()
