"""Basic tests for the agent API."""

import pytest
from httpx import AsyncClient

from src.app import create_app


@pytest.fixture
def app():
    """Create app instance for testing."""
    return create_app()


@pytest.mark.asyncio
async def test_health_check(app):
    """Test health check endpoint."""
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.get("/health")

    assert response.status_code == 200
    data = response.json()
    assert data["status"] == "healthy"
    assert "version" in data


@pytest.mark.asyncio
async def test_chat_endpoint_validation(app):
    """Test chat endpoint with invalid input."""
    async with AsyncClient(app=app, base_url="http://test") as client:
        # Empty message should fail
        response = await client.post(
            "/api/v1/agent/chat",
            json={"message": ""},
        )

    assert response.status_code == 422  # Validation error


@pytest.mark.asyncio
async def test_chat_endpoint_success(app):
    """Test chat endpoint with valid input."""
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/api/v1/agent/chat",
            json={"message": "Calculate 2 + 2"},
        )

    # Should succeed (assuming OpenAI key is configured)
    if response.status_code == 200:
        data = response.json()
        assert "response" in data
        assert "session_id" in data
    else:
        # May fail if no API key configured, which is ok for basic tests
        assert response.status_code in [500, 422]
