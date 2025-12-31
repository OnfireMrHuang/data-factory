# Testing Guide

This document provides comprehensive information about testing the Data AI Agent project.

## Table of Contents

- [Overview](#overview)
- [Test Structure](#test-structure)
- [Running Tests](#running-tests)
- [Writing Tests](#writing-tests)
- [Test Coverage](#test-coverage)
- [CI/CD Integration](#cicd-integration)

## Overview

The project uses **pytest** as the testing framework with the following key features:

- **Unit tests** for individual functions and modules
- **Integration tests** for API endpoints and workflows
- **Fixtures** for reusable test components
- **Async support** via pytest-asyncio
- **Coverage reports** via pytest-cov

## Test Structure

```
tests/
├── __init__.py              # Test package initialization
├── conftest.py              # Shared fixtures and configuration
├── test_app.py              # Application-level tests
├── test_utils/              # Utility module tests
│   ├── __init__.py
│   ├── test_text.py        # Text utility tests
│   └── test_validation.py  # Validation utility tests
├── test_core/               # Core module tests
│   ├── __init__.py
│   └── test_config.py      # Configuration tests
├── test_agents/             # Agent module tests
│   ├── __init__.py
│   └── test_tools.py       # Agent tools tests
└── test_api/                # API endpoint tests
    ├── __init__.py
    └── test_agent.py        # Agent API tests
```

## Running Tests

### Run All Tests

```bash
# Using uv (recommended)
uv run pytest

# Or with activated virtual environment
pytest
```

### Run Specific Test Files

```bash
# Test a specific module
uv run pytest tests/test_utils/test_text.py

# Test a specific directory
uv run pytest tests/test_agents/
```

### Run Specific Test Cases

```bash
# Run a specific test class
uv run pytest tests/test_utils/test_text.py::TestTruncateText

# Run a specific test function
uv run pytest tests/test_utils/test_text.py::TestTruncateText::test_truncate_long_text
```

### Run with Verbose Output

```bash
# Show detailed output
uv run pytest -v

# Show even more details
uv run pytest -vv
```

### Run with Coverage Report

```bash
# Generate coverage report
uv run pytest --cov=src --cov-report=term-missing

# Generate HTML coverage report
uv run pytest --cov=src --cov-report=html

# Then open htmlcov/index.html in your browser
```

### Run Only Unit Tests (Exclude Integration Tests)

```bash
# Skip integration tests
uv run pytest -m "not integration"
```

### Run Only Integration Tests

```bash
# Run only integration tests
uv run pytest -m integration
```

### Run with Parallel Execution

```bash
# Install pytest-xdist first
uv add --dev pytest-xdist

# Run tests in parallel
uv run pytest -n auto
```

## Writing Tests

### Basic Test Structure

```python
"""Tests for my_module."""

import pytest
from src.my_module import my_function


class TestMyFunction:
    """Tests for my_function."""

    def test_basic_case(self):
        """Test basic functionality."""
        result = my_function("input")
        assert result == "expected_output"

    def test_edge_case(self):
        """Test edge case."""
        result = my_function("")
        assert result == ""

    def test_error_handling(self):
        """Test error handling."""
        with pytest.raises(ValueError):
            my_function(None)
```

### Using Fixtures

```python
"""Tests using fixtures."""

import pytest


@pytest.fixture
def sample_data():
    """Provide sample data for tests."""
    return {"key": "value"}


def test_with_fixture(sample_data):
    """Test using a fixture."""
    assert sample_data["key"] == "value"
```

### Async Tests

```python
"""Tests for async functions."""

import pytest


@pytest.mark.asyncio
async def test_async_function():
    """Test async functionality."""
    result = await my_async_function()
    assert result is not None
```

### Parametrized Tests

```python
"""Tests with multiple parameter sets."""

import pytest


@pytest.mark.parametrize(
    "input_value,expected",
    [
        ("hello", "HELLO"),
        ("world", "WORLD"),
        ("", ""),
    ],
)
def test_uppercase(input_value, expected):
    """Test with multiple inputs."""
    assert input_value.upper() == expected
```

### Integration Test Marking

```python
"""Mark tests as integration tests."""

import pytest


@pytest.mark.integration
def test_api_endpoint(client):
    """Test API endpoint (integration test)."""
    response = client.get("/api/v1/health")
    assert response.status_code == 200
```

## Test Coverage

### Coverage Goals

- **Overall coverage**: Aim for >80%
- **Critical paths**: Aim for >95%
- **Utility functions**: Aim for 100%

### Checking Coverage

```bash
# Generate coverage report
uv run pytest --cov=src --cov-report=term-missing

# Generate HTML report
uv run pytest --cov=src --cov-report=html
open htmlcov/index.html
```

### Coverage Configuration

Coverage settings are configured in `pyproject.toml`:

```toml
[tool.pytest.ini_options]
asyncio_mode = "auto"
testpaths = ["tests"]
```

## Available Fixtures

### From conftest.py

- **test_settings**: Test configuration settings
- **override_settings**: Override app settings for tests
- **client**: FastAPI test client
- **mock_session_id**: Mock session ID
- **sample_message**: Sample message for testing
- **temp_env_file**: Temporary environment file

### Using Fixtures

```python
def test_with_client(client):
    """Test using the test client."""
    response = client.get("/health")
    assert response.status_code == 200


def test_with_settings(test_settings):
    """Test using test settings."""
    assert test_settings.debug is True
```

## Best Practices

### Test Organization

1. **Group related tests** in classes
2. **Use descriptive names** for test functions
3. **One assertion per test** when possible
4. **Test edge cases** and error conditions
5. **Keep tests independent** - no test should depend on another

### Test Naming

```python
# Good test names
def test_calculate_returns_correct_sum()
def test_validate_rejects_empty_input()
def test_api_returns_404_for_missing_resource()

# Poor test names
def test_function()
def test_case_1()
def test_works()
```

### Assertions

```python
# Use specific assertions
assert result == expected
assert result is None
assert "error" in response

# Use pytest assertions for better error messages
assert result == expected, f"Expected {expected}, got {result}"

# Use pytest.raises for exceptions
with pytest.raises(ValueError, match="invalid input"):
    function_that_raises()
```

### Mocking

```python
from unittest.mock import Mock, patch


def test_with_mock():
    """Test using mocks."""
    mock_service = Mock()
    mock_service.get_data.return_value = {"key": "value"}

    result = my_function(mock_service)
    assert result is not None
    mock_service.get_data.assert_called_once()


@patch("src.module.external_api")
def test_with_patch(mock_api):
    """Test using patch."""
    mock_api.return_value = {"status": "ok"}
    result = call_external_api()
    assert result["status"] == "ok"
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: '3.12'
      - name: Install dependencies
        run: |
          pip install uv
          uv sync
      - name: Run tests
        run: uv run pytest --cov=src --cov-report=xml
      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

## Troubleshooting

### Common Issues

**Import errors:**
```bash
# Make sure the package is installed in development mode
uv sync
```

**Fixture not found:**
```bash
# Check that conftest.py is in the tests directory
# Ensure the fixture is properly decorated with @pytest.fixture
```

**Async tests not running:**
```bash
# Verify pytest-asyncio is installed
uv add --dev pytest-asyncio

# Check that asyncio_mode is set in pyproject.toml
```

**Tests passing locally but failing in CI:**
```bash
# Check for environment-specific issues
# Ensure all dependencies are in pyproject.toml
# Verify that .env is not required for tests
```

## Additional Resources

- [Pytest Documentation](https://docs.pytest.org/)
- [Pytest Best Practices](https://docs.pytest.org/en/stable/goodpractices.html)
- [FastAPI Testing](https://fastapi.tiangolo.com/tutorial/testing/)
- [Coverage.py](https://coverage.readthedocs.io/)

## Getting Help

If you encounter issues with tests:

1. Check this guide first
2. Review test output carefully
3. Use `-vv` flag for verbose output
4. Check the project's issue tracker
5. Ask in the team chat or create an issue
