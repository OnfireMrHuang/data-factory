"""Error handling utilities."""

import logging
from typing import Any

logger = logging.getLogger(__name__)


class ServiceError(Exception):
    """Base exception for service layer errors."""

    def __init__(self, message: str, code: str | None = None, details: dict[str, Any] | None = None):
        """
        Initialize service error.

        Args:
            message: Error message
            code: Optional error code
            details: Optional additional details
        """
        self.message = message
        self.code = code or "SERVICE_ERROR"
        self.details = details or {}
        super().__init__(self.message)


class ValidationError(ServiceError):
    """Validation error."""

    def __init__(self, message: str, details: dict[str, Any] | None = None):
        super().__init__(message, code="VALIDATION_ERROR", details=details)


class AgentError(ServiceError):
    """Agent execution error."""

    def __init__(self, message: str, details: dict[str, Any] | None = None):
        super().__init__(message, code="AGENT_ERROR", details=details)


class TimeoutError(ServiceError):
    """Operation timeout error."""

    def __init__(self, message: str, details: dict[str, Any] | None = None):
        super().__init__(message, code="TIMEOUT_ERROR", details=details)


def log_error(error: Exception, context: dict[str, Any] | None = None) -> None:
    """
    Log error with context.

    Args:
        error: Exception to log
        context: Additional context information
    """
    context = context or {}
    logger.error(
        f"Error occurred: {str(error)}",
        extra={"error_type": type(error).__name__, "context": context},
        exc_info=True,
    )


def format_error_response(error: Exception) -> dict[str, Any]:
    """
    Format error for API response.

    Args:
        error: Exception to format

    Returns:
        Formatted error dict
    """
    if isinstance(error, ServiceError):
        return {"error": error.message, "code": error.code, "details": error.details}

    return {"error": str(error), "code": "INTERNAL_ERROR", "details": {}}
