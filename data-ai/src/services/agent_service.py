"""Agent service layer for business logic."""

import asyncio
import logging
from typing import Any

from ..agents.graph import DataAnalysisAgent, get_agent
from ..core.config import get_settings
from ..utils import (
    AgentError,
    ValidationError,
    get_elapsed_seconds,
    is_valid_session_id,
    log_error,
    sanitize_input,
    truncate_text,
    utc_now,
    validate_message_length,
)

logger = logging.getLogger(__name__)


class AgentService:
    """
    Service layer for agent operations.

    This service encapsulates business logic for agent interactions,
    including validation, error handling, and monitoring.
    """

    def __init__(self, agent: DataAnalysisAgent | None = None):
        """
        Initialize agent service.

        Args:
            agent: Optional agent instance (defaults to singleton)
        """
        self.agent = agent or get_agent()
        self.settings = get_settings()

    async def process_message(
        self,
        message: str,
        session_id: str | None = None,
        timeout: int | None = None,
    ) -> dict[str, Any]:
        """
        Process a user message through the agent.

        Args:
            message: User's input message
            session_id: Optional session ID
            timeout: Optional timeout in seconds (overrides config)

        Returns:
            Agent response with metadata

        Raises:
            ValidationError: If input validation fails
            AgentError: If agent execution fails
            TimeoutError: If execution times out
        """
        start_time = utc_now()

        try:
            # Validate inputs
            self._validate_message(message)
            if session_id:
                self._validate_session_id(session_id)

            # Sanitize message
            sanitized_message = sanitize_input(message)

            # Log request
            logger.info(
                f"Processing message: {truncate_text(sanitized_message, 100)}, "
                f"session_id: {session_id or 'none'}"
            )

            # Run agent with timeout
            timeout_seconds = timeout or self.settings.agent_timeout
            result = await asyncio.wait_for(
                self.agent.run(message=sanitized_message, session_id=session_id),
                timeout=timeout_seconds,
            )

            # Calculate execution time
            execution_time = get_elapsed_seconds(start_time)

            # Enhance result with service metadata
            result["metadata"] = result.get("metadata", {})
            result["metadata"]["execution_time"] = execution_time
            result["metadata"]["timestamp"] = start_time.isoformat()

            logger.info(f"Message processed successfully in {execution_time:.2f}s")

            return result

        except asyncio.TimeoutError as e:
            error_msg = f"Agent execution timed out after {timeout_seconds}s"
            logger.error(error_msg)
            raise AgentError(
                error_msg,
                details={"timeout": timeout_seconds, "execution_time": get_elapsed_seconds(start_time)},
            ) from e

        except ValidationError:
            raise

        except Exception as e:
            log_error(e, context={"message": truncate_text(message, 50), "session_id": session_id})
            raise AgentError(f"Failed to process message: {str(e)}") from e

    async def stream_message(
        self,
        message: str,
        session_id: str | None = None,
    ):
        """
        Stream agent responses for a user message.

        Args:
            message: User's input message
            session_id: Optional session ID

        Yields:
            Agent response chunks

        Raises:
            ValidationError: If input validation fails
            AgentError: If agent execution fails
        """
        start_time = utc_now()

        try:
            # Validate inputs
            self._validate_message(message)
            if session_id:
                self._validate_session_id(session_id)

            # Sanitize message
            sanitized_message = sanitize_input(message)

            logger.info(
                f"Streaming message: {truncate_text(sanitized_message, 100)}, "
                f"session_id: {session_id or 'none'}"
            )

            # Stream from agent
            async for chunk in self.agent.stream(message=sanitized_message, session_id=session_id):
                yield chunk

            execution_time = get_elapsed_seconds(start_time)
            logger.info(f"Streaming completed in {execution_time:.2f}s")

        except ValidationError:
            raise

        except Exception as e:
            log_error(e, context={"message": truncate_text(message, 50), "session_id": session_id})
            raise AgentError(f"Failed to stream message: {str(e)}") from e

    def _validate_message(self, message: str) -> None:
        """
        Validate user message.

        Args:
            message: Message to validate

        Raises:
            ValidationError: If validation fails
        """
        is_valid, error_msg = validate_message_length(
            message, min_length=1, max_length=self.settings.openai_max_tokens
        )

        if not is_valid:
            raise ValidationError(error_msg or "Invalid message", details={"message_length": len(message)})

    def _validate_session_id(self, session_id: str) -> None:
        """
        Validate session ID.

        Args:
            session_id: Session ID to validate

        Raises:
            ValidationError: If validation fails
        """
        if not is_valid_session_id(session_id):
            raise ValidationError(
                "Invalid session ID format. Must be alphanumeric with hyphens/underscores, 1-100 chars.",
                details={"session_id": session_id},
            )


# Singleton instance
_service_instance: AgentService | None = None


def get_agent_service() -> AgentService:
    """Get or create the agent service instance."""
    global _service_instance
    if _service_instance is None:
        _service_instance = AgentService()
    return _service_instance
