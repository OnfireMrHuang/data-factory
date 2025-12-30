"""Session management API routes."""

import logging
from typing import Annotated

from fastapi import APIRouter, Depends, HTTPException

from ...services import SessionManager
from ...utils import is_valid_session_id
from ..dependencies import get_session_manager_dependency

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/sessions", tags=["sessions"])


@router.get("/{session_id}")
async def get_session(
    session_id: str,
    session_manager: Annotated[SessionManager, Depends(get_session_manager_dependency)],
):
    """
    Get session information.

    Returns session metadata including message count and timestamps.
    """
    if not is_valid_session_id(session_id):
        raise HTTPException(status_code=400, detail="Invalid session ID format")

    session = session_manager.get_session(session_id)

    if not session:
        raise HTTPException(status_code=404, detail="Session not found")

    return session


@router.delete("/{session_id}")
async def delete_session(
    session_id: str,
    session_manager: Annotated[SessionManager, Depends(get_session_manager_dependency)],
):
    """
    Delete a session.

    Removes session data and history.
    """
    if not is_valid_session_id(session_id):
        raise HTTPException(status_code=400, detail="Invalid session ID format")

    deleted = session_manager.delete_session(session_id)

    if not deleted:
        raise HTTPException(status_code=404, detail="Session not found")

    return {"message": "Session deleted successfully", "session_id": session_id}


@router.get("/")
async def get_session_stats(
    session_manager: Annotated[SessionManager, Depends(get_session_manager_dependency)],
):
    """
    Get session statistics.

    Returns information about active sessions and system limits.
    """
    stats = session_manager.get_stats()
    return stats
