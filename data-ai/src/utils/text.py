"""Text processing utilities."""

import re
from typing import Any


def truncate_text(text: str, max_length: int = 100, suffix: str = "...") -> str:
    """
    Truncate text to a maximum length.

    Args:
        text: Text to truncate
        max_length: Maximum length before truncation
        suffix: Suffix to add when truncated

    Returns:
        Truncated text
    """
    if len(text) <= max_length:
        return text
    return text[: max_length - len(suffix)] + suffix


def clean_whitespace(text: str) -> str:
    """
    Clean excessive whitespace from text.

    Args:
        text: Text to clean

    Returns:
        Cleaned text
    """
    # Replace multiple spaces with single space
    text = re.sub(r"\s+", " ", text)
    # Remove leading/trailing whitespace
    return text.strip()


def extract_code_blocks(text: str) -> list[dict[str, str]]:
    """
    Extract code blocks from markdown-formatted text.

    Args:
        text: Text containing markdown code blocks

    Returns:
        List of dicts with 'language' and 'code' keys
    """
    pattern = r"```(\w+)?\n(.*?)```"
    matches = re.findall(pattern, text, re.DOTALL)

    return [{"language": lang or "text", "code": code.strip()} for lang, code in matches]


def sanitize_input(text: str, max_length: int = 10000) -> str:
    """
    Sanitize user input by removing potentially harmful content.

    Args:
        text: User input text
        max_length: Maximum allowed length

    Returns:
        Sanitized text
    """
    # Truncate if too long
    text = text[:max_length]

    # Remove null bytes
    text = text.replace("\x00", "")

    # Clean excessive whitespace
    text = clean_whitespace(text)

    return text


def format_prompt(template: str, **kwargs: Any) -> str:
    """
    Format a prompt template with variables.

    Args:
        template: Prompt template with {variable} placeholders
        **kwargs: Variables to fill in

    Returns:
        Formatted prompt
    """
    try:
        return template.format(**kwargs)
    except KeyError as e:
        raise ValueError(f"Missing required variable in prompt template: {e}")
