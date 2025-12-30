"""Application configuration management."""

from functools import lru_cache

from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    """Application settings loaded from environment variables."""

    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        case_sensitive=False,
        extra="ignore",
    )

    # Application
    app_name: str = "Data AI Agent"
    app_version: str = "0.1.0"
    debug: bool = False

    # API
    api_prefix: str = "/api/v1"
    api_host: str = "0.0.0.0"
    api_port: int = 8000

    # OpenAI
    openai_api_key: str
    openai_model: str = "gpt-4o-mini"
    openai_temperature: float = 0.7
    openai_max_tokens: int = 4096

    # LangSmith (optional for tracing)
    langchain_tracing_v2: bool = False
    langchain_api_key: str | None = None
    langchain_project: str = "data-ai-agent"

    # Agent
    max_iterations: int = 15
    agent_timeout: int = 300  # seconds


@lru_cache
def get_settings() -> Settings:
    """Get cached settings instance."""
    return Settings()
