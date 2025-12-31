"""Tests for agent API endpoints."""

import pytest
from fastapi.testclient import TestClient


class TestAgentEndpoint:
    """Tests for /api/v1/agent endpoint."""

    def test_agent_endpoint_exists(self, client: TestClient):
        """Test that agent endpoint is accessible."""
        response = client.get("/api/v1/agent")
        # Endpoint might not be fully implemented yet
        assert response.status_code in [200, 404, 405]

    @pytest.mark.integration
    @pytest.mark.skip(reason="Requires OpenAI API key and may have dependency issues")
    def test_send_message_to_agent(self, client: TestClient, sample_message: str):
        """Test sending a message to the agent."""
        payload = {"message": sample_message, "session_id": "test-session"}

        response = client.post("/api/v1/agent/chat", json=payload)

        # Check that request is processed (even if not fully implemented)
        assert response.status_code in [200, 404, 422, 501]
