"""API dependencies and shared utilities."""

import logging

from ..agents.graph import get_agent
from ..core.config import get_settings
from ..services import get_agent_service, get_session_manager

logger = logging.getLogger(__name__)


async def get_agent_dependency():
    """Dependency to get the agent instance."""
    return get_agent()


async def get_agent_service_dependency():
    """Dependency to get the agent service."""
    return get_agent_service()


async def get_session_manager_dependency():
    """Dependency to get the session manager."""
    return get_session_manager()


async def get_settings_dependency():
    """Dependency to get settings."""
    return get_settings()

