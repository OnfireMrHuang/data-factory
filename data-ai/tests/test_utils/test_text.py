"""Tests for text utility functions."""

import pytest

from src.utils.text import (
    clean_whitespace,
    extract_code_blocks,
    format_prompt,
    sanitize_input,
    truncate_text,
)


class TestTruncateText:
    """Tests for truncate_text function."""

    def test_truncate_long_text(self):
        """Test truncating text that exceeds max length."""
        text = "This is a very long text that needs to be truncated"
        result = truncate_text(text, max_length=20)
        assert len(result) == 20
        assert result.endswith("...")
        assert result == "This is a very lo..."

    def test_no_truncate_short_text(self):
        """Test that short text is not truncated."""
        text = "Short text"
        result = truncate_text(text, max_length=100)
        assert result == text

    def test_custom_suffix(self):
        """Test truncation with custom suffix."""
        text = "Long text here"
        result = truncate_text(text, max_length=10, suffix="~")
        assert result.endswith("~")
        assert len(result) == 10

    def test_exact_length(self):
        """Test text that is exactly max length."""
        text = "Exact"
        result = truncate_text(text, max_length=5)
        assert result == text


class TestCleanWhitespace:
    """Tests for clean_whitespace function."""

    def test_remove_multiple_spaces(self):
        """Test removing multiple consecutive spaces."""
        text = "This  has   multiple    spaces"
        result = clean_whitespace(text)
        assert result == "This has multiple spaces"

    def test_remove_leading_trailing_whitespace(self):
        """Test removing leading and trailing whitespace."""
        text = "  text with spaces  "
        result = clean_whitespace(text)
        assert result == "text with spaces"

    def test_replace_tabs_and_newlines(self):
        """Test replacing tabs and newlines with single space."""
        text = "text\twith\ttabs\nand\nnewlines"
        result = clean_whitespace(text)
        assert result == "text with tabs and newlines"

    def test_empty_string(self):
        """Test cleaning empty string."""
        result = clean_whitespace("")
        assert result == ""


class TestExtractCodeBlocks:
    """Tests for extract_code_blocks function."""

    def test_extract_single_code_block(self):
        """Test extracting a single code block."""
        text = """
Here is some code:
```python
print("Hello, World!")
```
"""
        result = extract_code_blocks(text)
        assert len(result) == 1
        assert result[0]["language"] == "python"
        assert 'print("Hello, World!")' in result[0]["code"]

    def test_extract_multiple_code_blocks(self):
        """Test extracting multiple code blocks."""
        text = """
```python
x = 1
```
Some text
```javascript
const y = 2;
```
"""
        result = extract_code_blocks(text)
        assert len(result) == 2
        assert result[0]["language"] == "python"
        assert result[1]["language"] == "javascript"

    def test_code_block_without_language(self):
        """Test code block without specified language."""
        text = """
```
generic code
```
"""
        result = extract_code_blocks(text)
        assert len(result) == 1
        assert result[0]["language"] == "text"

    def test_no_code_blocks(self):
        """Test text with no code blocks."""
        text = "Just plain text here"
        result = extract_code_blocks(text)
        assert len(result) == 0


class TestSanitizeInput:
    """Tests for sanitize_input function."""

    def test_remove_null_bytes(self):
        """Test removing null bytes from input."""
        text = "text\x00with\x00nulls"
        result = sanitize_input(text)
        assert "\x00" not in result
        # sanitize_input also cleans whitespace, so expect no spaces
        assert result == "textwithnulls"

    def test_truncate_long_input(self):
        """Test truncating input that exceeds max length."""
        text = "a" * 20000
        result = sanitize_input(text, max_length=5000)
        assert len(result) == 5000

    def test_clean_whitespace_in_sanitize(self):
        """Test that sanitize also cleans whitespace."""
        text = "  text  with   spaces  "
        result = sanitize_input(text)
        assert result == "text with spaces"


class TestFormatPrompt:
    """Tests for format_prompt function."""

    def test_format_basic_prompt(self):
        """Test formatting a basic prompt."""
        template = "Hello {name}, you are {age} years old."
        result = format_prompt(template, name="Alice", age=30)
        assert result == "Hello Alice, you are 30 years old."

    def test_format_with_missing_variable(self):
        """Test formatting with missing variable raises error."""
        template = "Hello {name}, you are {age} years old."
        with pytest.raises(ValueError, match="Missing required variable"):
            format_prompt(template, name="Alice")

    def test_format_with_extra_variables(self):
        """Test formatting ignores extra variables."""
        template = "Hello {name}!"
        result = format_prompt(template, name="Alice", age=30)
        assert result == "Hello Alice!"
