"""Tests for agent tools."""

import math

import pytest

from src.agents.tools import calculate, generate_sql, get_data_summary, get_tools


class TestCalculateTool:
    """Tests for calculate tool."""

    def test_basic_arithmetic(self):
        """Test basic arithmetic operations."""
        assert calculate.invoke({"expression": "2 + 2"}) == "4"
        assert calculate.invoke({"expression": "10 - 3"}) == "7"
        assert calculate.invoke({"expression": "4 * 5"}) == "20"
        assert calculate.invoke({"expression": "15 / 3"}) == "5.0"

    def test_complex_expressions(self):
        """Test complex mathematical expressions."""
        result = calculate.invoke({"expression": "(100 - 32) * 5/9"})
        assert float(result) == pytest.approx(37.777, rel=0.01)

    def test_math_functions(self):
        """Test mathematical functions."""
        assert calculate.invoke({"expression": "sqrt(16)"}) == "4.0"
        # pow can return int or float depending on the operation
        assert calculate.invoke({"expression": "pow(2, 3)"}) in ["8", "8.0"]
        assert calculate.invoke({"expression": "abs(-5)"}) == "5"
        assert calculate.invoke({"expression": "round(3.7)"}) == "4"

    def test_trigonometric_functions(self):
        """Test trigonometric functions."""
        result = calculate.invoke({"expression": "sin(0)"})
        assert float(result) == pytest.approx(0.0, abs=0.01)

        result = calculate.invoke({"expression": "cos(0)"})
        assert float(result) == pytest.approx(1.0, abs=0.01)

    def test_min_max_sum(self):
        """Test aggregate functions."""
        assert calculate.invoke({"expression": "min(1, 2, 3)"}) == "1"
        assert calculate.invoke({"expression": "max(1, 2, 3)"}) == "3"
        assert calculate.invoke({"expression": "sum([1, 2, 3, 4])"}) == "10"

    def test_invalid_expression(self):
        """Test handling of invalid expressions."""
        result = calculate.invoke({"expression": "invalid_expr"})
        assert "Error" in result

    def test_division_by_zero(self):
        """Test division by zero error handling."""
        result = calculate.invoke({"expression": "1 / 0"})
        assert "Error" in result

    def test_unsafe_operations_blocked(self):
        """Test that unsafe operations are blocked."""
        # These should fail because __builtins__ is restricted
        result = calculate.invoke({"expression": "open('/etc/passwd')"})
        assert "Error" in result


class TestGetDataSummaryTool:
    """Tests for get_data_summary tool."""

    def test_returns_summary(self):
        """Test that tool returns a data summary."""
        result = get_data_summary.invoke({"data_description": "sales data"})
        assert isinstance(result, str)
        assert "Data Analysis Summary" in result
        assert "sales data" in result

    def test_contains_statistics(self):
        """Test that summary contains expected statistics."""
        result = get_data_summary.invoke({"data_description": "customer data"})
        assert "Total Records" in result
        assert "Columns" in result
        assert "Missing Values" in result
        assert "Date Range" in result

    def test_mock_values(self):
        """Test that mock values are present."""
        result = get_data_summary.invoke({"data_description": "test data"})
        assert "1,000" in result
        assert "This is a mock summary" in result


class TestGenerateSqlTool:
    """Tests for generate_sql tool."""

    def test_returns_sql_query(self):
        """Test that tool returns SQL query."""
        result = generate_sql.invoke({"table_description": "get all users"})
        assert isinstance(result, str)
        assert "SELECT" in result

    def test_sql_format(self):
        """Test that generated SQL has proper format."""
        result = generate_sql.invoke({"table_description": "aggregate sales by region"})
        assert "SELECT" in result
        assert "FROM" in result
        assert "WHERE" in result
        assert "GROUP BY" in result
        assert "ORDER BY" in result

    def test_includes_description(self):
        """Test that SQL includes the table description."""
        description = "query customer orders"
        result = generate_sql.invoke({"table_description": description})
        assert description in result

    def test_mock_indicator(self):
        """Test that mock query indicator is present."""
        result = generate_sql.invoke({"table_description": "test query"})
        assert "mock" in result.lower()


class TestGetTools:
    """Tests for get_tools function."""

    def test_returns_list_of_tools(self):
        """Test that get_tools returns a list."""
        tools = get_tools()
        assert isinstance(tools, list)
        assert len(tools) > 0

    def test_contains_expected_tools(self):
        """Test that all expected tools are present."""
        tools = get_tools()
        tool_names = [tool.name for tool in tools]

        assert "calculate" in tool_names
        assert "get_data_summary" in tool_names
        assert "generate_sql" in tool_names

    def test_all_tools_callable(self):
        """Test that all tools are callable."""
        tools = get_tools()
        for tool in tools:
            assert callable(tool.invoke)
