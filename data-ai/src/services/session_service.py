"""Session management service."""

import logging
from datetime import datetime
from typing import Any

from ..utils import utc_now

logger = logging.getLogger(__name__)


class SessionManager:
    """
    Simple in-memory session manager.

    For production, replace with Redis or a database.
    """

    def __init__(self, max_sessions: int = 1000, session_ttl: int = 3600):
        """
        Initialize session manager.

        Args:
            max_sessions: Maximum number of sessions to store
            session_ttl: Session time-to-live in seconds
        """
        self._sessions: dict[str, dict[str, Any]] = {}
        self.max_sessions = max_sessions
        self.session_ttl = session_ttl

    def create_session(self, session_id: str, metadata: dict[str, Any] | None = None) -> dict[str, Any]:
        """
        Create a new session.

        Args:
            session_id: Unique session identifier
            metadata: Optional session metadata

        Returns:
            Session data
        """
        # Clean up old sessions if at max capacity
        if len(self._sessions) >= self.max_sessions:
            self._cleanup_expired_sessions()

        session_data = {
            "session_id": session_id,
            "created_at": utc_now(),
            "last_accessed": utc_now(),
            "message_count": 0,
            "metadata": metadata or {},
        }

        self._sessions[session_id] = session_data
        logger.info(f"Created session: {session_id}")

        return session_data

    def get_session(self, session_id: str) -> dict[str, Any] | None:
        """
        Get session data.

        Args:
            session_id: Session identifier

        Returns:
            Session data or None if not found
        """
        session = self._sessions.get(session_id)

        if session:
            # Check if expired
            elapsed = (utc_now() - session["last_accessed"]).total_seconds()
            if elapsed > self.session_ttl:
                logger.info(f"Session expired: {session_id}")
                self.delete_session(session_id)
                return None

            # Update last accessed
            session["last_accessed"] = utc_now()

        return session

    def update_session(self, session_id: str, **updates: Any) -> dict[str, Any] | None:
        """
        Update session data.

        Args:
            session_id: Session identifier
            **updates: Fields to update

        Returns:
            Updated session data or None if not found
        """
        session = self.get_session(session_id)

        if session:
            session.update(updates)
            logger.debug(f"Updated session: {session_id}")

        return session

    def increment_message_count(self, session_id: str) -> int:
        """
        Increment message count for a session.

        Args:
            session_id: Session identifier

        Returns:
            New message count
        """
        session = self.get_session(session_id)

        if session:
            session["message_count"] = session.get("message_count", 0) + 1
            return session["message_count"]

        return 0

    def delete_session(self, session_id: str) -> bool:
        """
        Delete a session.

        Args:
            session_id: Session identifier

        Returns:
            True if deleted, False if not found
        """
        if session_id in self._sessions:
            del self._sessions[session_id]
            logger.info(f"Deleted session: {session_id}")
            return True

        return False

    def get_or_create_session(self, session_id: str, metadata: dict[str, Any] | None = None) -> dict[str, Any]:
        """
        Get existing session or create new one.

        Args:
            session_id: Session identifier
            metadata: Optional metadata for new session

        Returns:
            Session data
        """
        session = self.get_session(session_id)

        if not session:
            session = self.create_session(session_id, metadata)

        return session

    def _cleanup_expired_sessions(self) -> int:
        """
        Remove expired sessions.

        Returns:
            Number of sessions removed
        """
        expired_ids = []
        current_time = utc_now()

        for session_id, session in self._sessions.items():
            elapsed = (current_time - session["last_accessed"]).total_seconds()
            if elapsed > self.session_ttl:
                expired_ids.append(session_id)

        for session_id in expired_ids:
            del self._sessions[session_id]

        if expired_ids:
            logger.info(f"Cleaned up {len(expired_ids)} expired sessions")

        return len(expired_ids)

    def get_stats(self) -> dict[str, Any]:
        """
        Get session statistics.

        Returns:
            Statistics dictionary
        """
        return {
            "total_sessions": len(self._sessions),
            "max_sessions": self.max_sessions,
            "session_ttl": self.session_ttl,
        }


# Singleton instance
_session_manager: SessionManager | None = None


def get_session_manager() -> SessionManager:
    """Get or create the session manager instance."""
    global _session_manager
    if _session_manager is None:
        _session_manager = SessionManager()
    return _session_manager
