"""Validation utilities."""

import re
from typing import Any


def is_valid_session_id(session_id: str) -> bool:
    """
    Validate session ID format.

    Args:
        session_id: Session ID to validate

    Returns:
        True if valid, False otherwise
    """
    if not session_id:
        return False

    # Allow alphanumeric, hyphens, and underscores, 1-100 chars
    pattern = r"^[a-zA-Z0-9_-]{1,100}$"
    return bool(re.match(pattern, session_id))


def validate_message_length(message: str, min_length: int = 1, max_length: int = 10000) -> tuple[bool, str | None]:
    """
    Validate message length.

    Args:
        message: Message to validate
        min_length: Minimum allowed length
        max_length: Maximum allowed length

    Returns:
        Tuple of (is_valid, error_message)
    """
    length = len(message.strip())

    if length < min_length:
        return False, f"Message too short (minimum {min_length} characters)"

    if length > max_length:
        return False, f"Message too long (maximum {max_length} characters)"

    return True, None


def is_safe_sql(query: str) -> bool:
    """
    Basic check if SQL query appears safe (no DROP, DELETE, etc.).

    Note: This is NOT a comprehensive SQL injection prevention.
    Use parameterized queries and proper ORM for real safety.

    Args:
        query: SQL query to check

    Returns:
        True if appears safe, False otherwise
    """
    dangerous_keywords = [
        "DROP",
        "DELETE",
        "TRUNCATE",
        "ALTER",
        "INSERT",
        "UPDATE",
        "EXEC",
        "EXECUTE",
    ]

    query_upper = query.upper()

    for keyword in dangerous_keywords:
        if keyword in query_upper:
            return False

    return True


def validate_dict_schema(data: dict[str, Any], required_keys: list[str]) -> tuple[bool, str | None]:
    """
    Validate that a dictionary contains required keys.

    Args:
        data: Dictionary to validate
        required_keys: List of required keys

    Returns:
        Tuple of (is_valid, error_message)
    """
    missing_keys = [key for key in required_keys if key not in data]

    if missing_keys:
        return False, f"Missing required keys: {', '.join(missing_keys)}"

    return True, None
