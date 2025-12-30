"""Time and date utilities."""

from datetime import datetime, timezone
from typing import Any


def utc_now() -> datetime:
    """
    Get current UTC timestamp.

    Returns:
        Current UTC datetime
    """
    return datetime.now(timezone.utc)


def format_timestamp(dt: datetime, format_str: str = "%Y-%m-%d %H:%M:%S") -> str:
    """
    Format datetime to string.

    Args:
        dt: Datetime to format
        format_str: Format string

    Returns:
        Formatted datetime string
    """
    return dt.strftime(format_str)


def parse_timestamp(timestamp_str: str, format_str: str = "%Y-%m-%d %H:%M:%S") -> datetime | None:
    """
    Parse timestamp string to datetime.

    Args:
        timestamp_str: Timestamp string to parse
        format_str: Format string

    Returns:
        Parsed datetime or None if invalid
    """
    try:
        return datetime.strptime(timestamp_str, format_str)
    except ValueError:
        return None


def get_elapsed_seconds(start_time: datetime) -> float:
    """
    Get elapsed seconds since start time.

    Args:
        start_time: Start datetime

    Returns:
        Elapsed seconds
    """
    return (utc_now() - start_time).total_seconds()


def timestamp_to_iso(dt: datetime) -> str:
    """
    Convert datetime to ISO format string.

    Args:
        dt: Datetime to convert

    Returns:
        ISO format string
    """
    return dt.isoformat()
