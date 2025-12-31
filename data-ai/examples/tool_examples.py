"""Comprehensive examples of all available agent tools.

This example demonstrates how to use each tool with various inputs.
"""

import logging

from src.agents.tools import calculate, generate_sql, get_data_summary
from src.core.logging import setup_logging

setup_logging()
logger = logging.getLogger(__name__)


def calculator_examples():
    """Demonstrate calculator tool with various expressions."""
    print("\n" + "=" * 60)
    print("CALCULATOR TOOL EXAMPLES")
    print("=" * 60 + "\n")

    examples = [
        ("Basic arithmetic", "10 + 5 * 2"),
        ("Percentage", "100 * 0.15"),
        ("Square root", "sqrt(225)"),
        ("Power", "pow(2, 10)"),
        ("Trigonometry", "sin(3.14159/2)"),
        ("Absolute value", "abs(-42)"),
        ("Min/Max", "max(10, 25, 5, 30)"),
        ("Rounding", "round(3.14159, 2)"),
    ]

    for description, expression in examples:
        print(f"{description}:")
        print(f"  Expression: {expression}")
        result = calculate.invoke({"expression": expression})
        print(f"  Result: {result}\n")


def data_summary_examples():
    """Demonstrate data summary tool."""
    print("\n" + "=" * 60)
    print("DATA SUMMARY TOOL EXAMPLES")
    print("=" * 60 + "\n")

    examples = [
        "E-commerce sales data from 2024",
        "Customer demographics across regions",
        "Website traffic metrics for Q4",
        "Product inventory levels by warehouse",
    ]

    for i, description in enumerate(examples, 1):
        print(f"Example {i}: {description}")
        print("-" * 60)
        result = get_data_summary.invoke({"data_description": description})
        print(result)
        print("\n")


def sql_generation_examples():
    """Demonstrate SQL generation tool."""
    print("\n" + "=" * 60)
    print("SQL GENERATION TOOL EXAMPLES")
    print("=" * 60 + "\n")

    examples = [
        "Get top 10 customers by total purchases",
        "Calculate monthly revenue aggregated by product category",
        "Find active users who made a purchase in the last 30 days",
        "Count orders by status and payment method",
    ]

    for i, description in enumerate(examples, 1):
        print(f"Example {i}: {description}")
        print("-" * 60)
        result = generate_sql.invoke({"table_description": description})
        print(result)
        print("\n")


def error_handling_examples():
    """Demonstrate error handling in tools."""
    print("\n" + "=" * 60)
    print("ERROR HANDLING EXAMPLES")
    print("=" * 60 + "\n")

    print("1. Invalid math expression:")
    result = calculate.invoke({"expression": "invalid syntax!@#"})
    print(f"   Result: {result}\n")

    print("2. Division by zero:")
    result = calculate.invoke({"expression": "10 / 0"})
    print(f"   Result: {result}\n")

    print("3. Undefined function:")
    result = calculate.invoke({"expression": "undefined_func(5)"})
    print(f"   Result: {result}\n")


def main():
    """Run all tool examples."""
    print("\n" + "=" * 60)
    print("DATA AI AGENT - TOOL EXAMPLES")
    print("=" * 60)

    calculator_examples()
    data_summary_examples()
    sql_generation_examples()
    error_handling_examples()

    print("=" * 60)
    print("All examples completed successfully!")
    print("=" * 60 + "\n")


if __name__ == "__main__":
    main()
