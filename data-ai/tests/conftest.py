"""Pytest configuration and shared fixtures."""

from typing import Generator

import pytest
from fastapi.testclient import TestClient

from src.app import app
from src.core.config import Settings, get_settings


@pytest.fixture
def test_settings() -> Settings:
    """Provide test settings with safe defaults."""
    return Settings(
        app_name="Test Data AI Agent",
        debug=True,
        api_port=8001,
        openai_api_key="test-key",
        openai_model="gpt-4o-mini",
        langchain_tracing_v2=False,
    )


@pytest.fixture
def override_settings(test_settings: Settings) -> Generator:
    """Override settings for testing."""
    app.dependency_overrides[get_settings] = lambda: test_settings
    yield
    app.dependency_overrides.clear()


@pytest.fixture
def client(override_settings: Generator) -> TestClient:
    """Provide FastAPI test client."""
    return TestClient(app)


@pytest.fixture
def mock_session_id() -> str:
    """Provide a mock session ID for testing."""
    return "test-session-123"


@pytest.fixture
def sample_message() -> str:
    """Provide a sample message for testing."""
    return "What is 2 + 2?"


@pytest.fixture
def temp_env_file(tmp_path):
    """Create a temporary .env file for testing."""
    env_file = tmp_path / ".env"
    env_file.write_text(
        """
APP_NAME=Test App
DEBUG=true
OPENAI_API_KEY=test-key
OPENAI_MODEL=gpt-4o-mini
"""
    )
    return env_file
