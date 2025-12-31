"""Example of interacting with the Data AI Agent API.

This demonstrates how to use the FastAPI endpoints as a client.
"""

import logging
from typing import Optional

import httpx

from src.core.logging import setup_logging

setup_logging()
logger = logging.getLogger(__name__)

BASE_URL = "http://localhost:8000"
API_PREFIX = "/api/v1"


class AgentAPIClient:
    """Simple client for interacting with the Data AI Agent API."""

    def __init__(self, base_url: str = BASE_URL):
        """Initialize the API client."""
        self.base_url = base_url
        self.client = httpx.Client(base_url=base_url, timeout=30.0)

    def __enter__(self):
        """Context manager entry."""
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit."""
        self.client.close()

    def health_check(self) -> dict:
        """Check if the API is running."""
        try:
            response = self.client.get(f"{API_PREFIX}/health")
            response.raise_for_status()
            return response.json()
        except Exception as e:
            logger.error(f"Health check failed: {e}")
            return {"status": "error", "message": str(e)}

    def send_message(
        self, message: str, session_id: Optional[str] = None
    ) -> dict:
        """Send a message to the agent."""
        payload = {"message": message}
        if session_id:
            payload["session_id"] = session_id

        try:
            response = self.client.post(f"{API_PREFIX}/agent/chat", json=payload)
            response.raise_for_status()
            return response.json()
        except Exception as e:
            logger.error(f"Failed to send message: {e}")
            return {"error": str(e)}

    def get_session(self, session_id: str) -> dict:
        """Get session information."""
        try:
            response = self.client.get(f"{API_PREFIX}/session/{session_id}")
            response.raise_for_status()
            return response.json()
        except Exception as e:
            logger.error(f"Failed to get session: {e}")
            return {"error": str(e)}


def example_basic_interaction():
    """Example of basic interaction with the API."""
    print("\n=== Basic API Interaction ===\n")

    with AgentAPIClient() as client:
        # Health check
        print("1. Performing health check...")
        health = client.health_check()
        print(f"   Status: {health}\n")

        # Send a simple message
        print("2. Sending message: 'Calculate 25 * 4'")
        response = client.send_message("Calculate 25 * 4", session_id="demo-session")
        print(f"   Response: {response}\n")

        # Send another message
        print("3. Sending message: 'What is the square root of 144?'")
        response = client.send_message(
            "What is the square root of 144?", session_id="demo-session"
        )
        print(f"   Response: {response}\n")


def example_session_management():
    """Example of session management."""
    print("\n=== Session Management ===\n")

    with AgentAPIClient() as client:
        session_id = "test-session-123"

        print(f"1. Creating session: {session_id}")
        client.send_message("Hello, agent!", session_id=session_id)

        print(f"2. Retrieving session information...")
        session_info = client.get_session(session_id)
        print(f"   Session: {session_info}\n")


def example_data_analysis_workflow():
    """Example of a data analysis workflow."""
    print("\n=== Data Analysis Workflow ===\n")

    with AgentAPIClient() as client:
        session_id = "analysis-session"

        messages = [
            "I need to analyze sales data from Q4 2024",
            "Can you generate SQL to get top 10 products by revenue?",
            "Calculate the percentage increase: (5000 - 4200) / 4200 * 100",
        ]

        for i, message in enumerate(messages, 1):
            print(f"{i}. Sending: {message}")
            response = client.send_message(message, session_id=session_id)
            print(f"   Response: {response.get('response', response)}\n")


def main():
    """Run all API examples."""
    print("=" * 60)
    print("Data AI Agent - API Client Examples")
    print("=" * 60)
    print("\nNOTE: Make sure the API server is running:")
    print("  uv run main.py")
    print("=" * 60)

    try:
        example_basic_interaction()
        example_session_management()
        example_data_analysis_workflow()

        print("=" * 60)
        print("All examples completed!")
        print("=" * 60)

    except Exception as e:
        logger.error(f"Example failed: {e}")
        print("\nError: Make sure the API server is running!")
        print("Start it with: uv run main.py")


if __name__ == "__main__":
    main()
