"""Tests for configuration module."""

import os
from unittest.mock import patch

import pytest

from src.core.config import Settings, get_settings


class TestSettings:
    """Tests for Settings class."""

    def test_default_values(self):
        """Test default configuration values."""
        settings = Settings()
        assert settings.app_name == "Data AI Agent"
        assert settings.app_version == "0.1.0"
        assert settings.debug is False
        assert settings.api_prefix == "/api/v1"
        assert settings.api_host == "0.0.0.0"
        assert settings.api_port == 8000

    def test_openai_defaults(self):
        """Test OpenAI default configuration."""
        settings = Settings()
        assert settings.openai_model == "gpt-4o-mini"
        assert settings.openai_temperature == 0.7
        assert settings.openai_max_tokens == 4096

    def test_agent_defaults(self):
        """Test agent default configuration."""
        settings = Settings()
        assert settings.max_iterations == 15
        assert settings.agent_timeout == 300

    def test_langchain_defaults(self):
        """Test LangChain default configuration."""
        settings = Settings()
        assert settings.langchain_tracing_v2 is False
        assert settings.langchain_project == "data-ai-agent"

    def test_custom_values(self):
        """Test overriding configuration values."""
        settings = Settings(
            app_name="Custom App",
            debug=True,
            api_port=9000,
            openai_model="gpt-4",
        )
        assert settings.app_name == "Custom App"
        assert settings.debug is True
        assert settings.api_port == 9000
        assert settings.openai_model == "gpt-4"

    @patch.dict(os.environ, {"APP_NAME": "Env App", "API_PORT": "7000", "DEBUG": "true"})
    def test_load_from_environment(self):
        """Test loading configuration from environment variables."""
        settings = Settings()
        assert settings.app_name == "Env App"
        assert settings.api_port == 7000
        assert settings.debug is True


class TestGetSettings:
    """Tests for get_settings function."""

    def test_get_settings_returns_settings(self):
        """Test that get_settings returns Settings instance."""
        settings = get_settings()
        assert isinstance(settings, Settings)

    def test_get_settings_is_cached(self):
        """Test that get_settings uses caching."""
        settings1 = get_settings()
        settings2 = get_settings()
        # Should return the same instance due to lru_cache
        assert settings1 is settings2

    def test_settings_cache_can_be_cleared(self):
        """Test that settings cache can be cleared."""
        settings1 = get_settings()
        get_settings.cache_clear()
        settings2 = get_settings()
        # After cache clear, might be different instances
        assert isinstance(settings1, Settings)
        assert isinstance(settings2, Settings)
