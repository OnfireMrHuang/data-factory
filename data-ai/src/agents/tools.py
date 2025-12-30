"""Agent tools for data analysis tasks."""

import logging
from typing import Annotated

from langchain_core.tools import tool

logger = logging.getLogger(__name__)


@tool
def calculate(expression: Annotated[str, "Mathematical expression to evaluate"]) -> str:
    """
    Evaluate a mathematical expression safely.

    Examples:
        - "2 + 2"
        - "(100 - 32) * 5/9"
        - "sqrt(16) + pow(2, 3)"
    """
    try:
        # Safe evaluation - only allow basic math operations
        allowed_names = {
            "abs": abs,
            "round": round,
            "min": min,
            "max": max,
            "sum": sum,
            "pow": pow,
        }

        # Add math functions
        import math
        for name in dir(math):
            if not name.startswith("_"):
                allowed_names[name] = getattr(math, name)

        result = eval(expression, {"__builtins__": {}}, allowed_names)
        logger.info(f"Calculated: {expression} = {result}")
        return str(result)

    except Exception as e:
        error_msg = f"Error evaluating expression: {str(e)}"
        logger.error(error_msg)
        return error_msg


@tool
def get_data_summary(data_description: Annotated[str, "Description of the data to analyze"]) -> str:
    """
    Get a summary of data based on description.
    This is a mock tool - replace with actual data analysis logic.

    Args:
        data_description: Description of the data to analyze

    Returns:
        A summary of the data analysis
    """
    logger.info(f"Analyzing data: {data_description}")

    # Mock response - replace with actual data analysis
    return f"""Data Analysis Summary for: {data_description}

Sample Statistics:
- Total Records: 1,000
- Columns: 5
- Missing Values: 2.3%
- Date Range: 2024-01-01 to 2024-12-31

This is a mock summary. Implement actual data analysis logic here."""


@tool
def generate_sql(
    table_description: Annotated[str, "Description of the table and query requirements"]
) -> str:
    """
    Generate SQL query based on natural language description.
    This is a mock tool - enhance with actual SQL generation logic.

    Args:
        table_description: Description of the table and what query is needed

    Returns:
        Generated SQL query
    """
    logger.info(f"Generating SQL for: {table_description}")

    # Mock response - replace with actual SQL generation
    return f"""-- Generated SQL Query
-- Based on: {table_description}

SELECT
    column1,
    column2,
    COUNT(*) as count
FROM
    table_name
WHERE
    condition = true
GROUP BY
    column1, column2
ORDER BY
    count DESC
LIMIT 100;

-- This is a mock query. Implement actual SQL generation logic."""


def get_tools() -> list:
    """Get all available tools for the agent."""
    return [
        calculate,
        get_data_summary,
        generate_sql,
    ]
