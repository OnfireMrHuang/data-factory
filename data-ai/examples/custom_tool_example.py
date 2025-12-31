"""Example of creating and registering custom tools for the agent.

This demonstrates how to extend the agent with your own tools.
"""

import logging
from typing import Annotated

from langchain_core.tools import tool

from src.agents.tools import get_tools
from src.core.logging import setup_logging

setup_logging()
logger = logging.getLogger(__name__)


# Define custom tools


@tool
def format_currency(
    amount: Annotated[float, "Amount to format"],
    currency: Annotated[str, "Currency code (USD, EUR, etc.)"] = "USD",
) -> str:
    """
    Format a number as currency.

    Args:
        amount: The numeric amount
        currency: Currency code (default: USD)

    Returns:
        Formatted currency string
    """
    currency_symbols = {
        "USD": "$",
        "EUR": "€",
        "GBP": "£",
        "JPY": "¥",
        "CNY": "¥",
    }

    symbol = currency_symbols.get(currency, currency + " ")

    if currency == "JPY":
        # Japanese Yen doesn't use decimal places
        return f"{symbol}{amount:,.0f}"

    return f"{symbol}{amount:,.2f}"


@tool
def convert_temperature(
    value: Annotated[float, "Temperature value"],
    from_unit: Annotated[str, "Source unit (C, F, K)"],
    to_unit: Annotated[str, "Target unit (C, F, K)"],
) -> str:
    """
    Convert temperature between Celsius, Fahrenheit, and Kelvin.

    Args:
        value: Temperature value to convert
        from_unit: Source unit (C, F, or K)
        to_unit: Target unit (C, F, or K)

    Returns:
        Converted temperature with unit
    """
    from_unit = from_unit.upper()
    to_unit = to_unit.upper()

    # Convert to Celsius first
    if from_unit == "C":
        celsius = value
    elif from_unit == "F":
        celsius = (value - 32) * 5 / 9
    elif from_unit == "K":
        celsius = value - 273.15
    else:
        return f"Error: Unknown unit {from_unit}"

    # Convert from Celsius to target unit
    if to_unit == "C":
        result = celsius
    elif to_unit == "F":
        result = celsius * 9 / 5 + 32
    elif to_unit == "K":
        result = celsius + 273.15
    else:
        return f"Error: Unknown unit {to_unit}"

    return f"{result:.2f}°{to_unit}"


@tool
def parse_csv_line(
    line: Annotated[str, "CSV line to parse"],
    delimiter: Annotated[str, "Delimiter character"] = ",",
) -> str:
    """
    Parse a CSV line and return structured data.

    Args:
        line: CSV line to parse
        delimiter: Delimiter character (default: comma)

    Returns:
        JSON representation of parsed data
    """
    import json

    fields = line.split(delimiter)
    fields = [field.strip() for field in fields]

    result = {"field_count": len(fields), "fields": fields}

    return json.dumps(result, indent=2)


def demonstrate_custom_tools():
    """Demonstrate the custom tools."""
    print("\n=== Custom Tool Demonstrations ===\n")

    # Example 1: Currency formatting
    print("1. Currency Formatting Tool:")
    amounts = [(1234.56, "USD"), (9876.54, "EUR"), (5000.0, "JPY")]

    for amount, currency in amounts:
        result = format_currency.invoke({"amount": amount, "currency": currency})
        print(f"   {amount} {currency} → {result}")
    print()

    # Example 2: Temperature conversion
    print("2. Temperature Conversion Tool:")
    conversions = [
        (100, "C", "F"),
        (32, "F", "C"),
        (273.15, "K", "C"),
        (0, "C", "K"),
    ]

    for value, from_unit, to_unit in conversions:
        result = convert_temperature.invoke(
            {"value": value, "from_unit": from_unit, "to_unit": to_unit}
        )
        print(f"   {value}°{from_unit} → {result}")
    print()

    # Example 3: CSV parsing
    print("3. CSV Parsing Tool:")
    csv_lines = [
        "John,Doe,30,Engineer",
        "Jane,Smith,28,Designer",
        "Bob;Johnson;35;Manager",  # Using semicolon
    ]

    for i, line in enumerate(csv_lines, 1):
        delimiter = ";" if ";" in line else ","
        result = parse_csv_line.invoke({"line": line, "delimiter": delimiter})
        print(f"   Line {i}: {line}")
        print(f"   Parsed:\n{result}\n")


def create_extended_tool_list():
    """Create an extended list of tools including custom ones."""
    print("\n=== Extended Tool List ===\n")

    # Get built-in tools
    builtin_tools = get_tools()

    # Add custom tools
    custom_tools = [
        format_currency,
        convert_temperature,
        parse_csv_line,
    ]

    all_tools = builtin_tools + custom_tools

    print(f"Total tools available: {len(all_tools)}\n")

    print("Built-in tools:")
    for tool in builtin_tools:
        print(f"  - {tool.name}")

    print("\nCustom tools:")
    for tool in custom_tools:
        print(f"  - {tool.name}")

    return all_tools


def main():
    """Run custom tool examples."""
    print("=" * 60)
    print("Custom Tool Examples")
    print("=" * 60)

    demonstrate_custom_tools()
    create_extended_tool_list()

    print("\n" + "=" * 60)
    print("Tip: You can register these custom tools in your agent")
    print("by modifying src/agents/tools.py and adding them to")
    print("the get_tools() function.")
    print("=" * 60 + "\n")


if __name__ == "__main__":
    main()
