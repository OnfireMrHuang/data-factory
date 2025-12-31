"""Tests for validation utility functions."""

import pytest

from src.utils.validation import (
    is_safe_sql,
    is_valid_session_id,
    validate_dict_schema,
    validate_message_length,
)


class TestIsValidSessionId:
    """Tests for is_valid_session_id function."""

    def test_valid_session_ids(self):
        """Test valid session ID formats."""
        valid_ids = [
            "session123",
            "test-session-456",
            "user_session_789",
            "abc-123_xyz",
            "a",
            "A1",
        ]
        for session_id in valid_ids:
            assert is_valid_session_id(session_id), f"{session_id} should be valid"

    def test_invalid_session_ids(self):
        """Test invalid session ID formats."""
        invalid_ids = [
            "",
            "session with spaces",
            "session@123",
            "session#456",
            "a" * 101,  # Too long
            "session!",
            "session$",
        ]
        for session_id in invalid_ids:
            assert not is_valid_session_id(session_id), f"{session_id} should be invalid"

    def test_boundary_lengths(self):
        """Test session ID length boundaries."""
        assert is_valid_session_id("a")  # 1 char - min valid
        assert is_valid_session_id("a" * 100)  # 100 chars - max valid
        assert not is_valid_session_id("a" * 101)  # 101 chars - invalid


class TestValidateMessageLength:
    """Tests for validate_message_length function."""

    def test_valid_message(self):
        """Test validation of valid message."""
        is_valid, error = validate_message_length("This is a valid message")
        assert is_valid
        assert error is None

    def test_message_too_short(self):
        """Test message that is too short."""
        is_valid, error = validate_message_length("", min_length=1)
        assert not is_valid
        assert "too short" in error

    def test_message_too_long(self):
        """Test message that is too long."""
        long_message = "x" * 10001
        is_valid, error = validate_message_length(long_message, max_length=10000)
        assert not is_valid
        assert "too long" in error

    def test_custom_length_limits(self):
        """Test with custom length limits."""
        message = "Hello"
        is_valid, error = validate_message_length(message, min_length=3, max_length=10)
        assert is_valid
        assert error is None

    def test_whitespace_only_message(self):
        """Test message with only whitespace."""
        is_valid, error = validate_message_length("   ", min_length=1)
        assert not is_valid
        assert "too short" in error


class TestIsSafeSql:
    """Tests for is_safe_sql function."""

    def test_safe_select_query(self):
        """Test safe SELECT query."""
        safe_queries = [
            "SELECT * FROM users",
            "SELECT name, email FROM customers WHERE active = true",
            "select id from orders",
        ]
        for query in safe_queries:
            assert is_safe_sql(query), f"{query} should be safe"

    def test_unsafe_queries(self):
        """Test queries with dangerous keywords."""
        unsafe_queries = [
            "DROP TABLE users",
            "DELETE FROM customers",
            "TRUNCATE TABLE orders",
            "ALTER TABLE users ADD COLUMN",
            "INSERT INTO users VALUES ('hacker')",
            "UPDATE users SET admin = true",
            "EXEC sp_executesql",
            "EXECUTE stored_procedure",
        ]
        for query in unsafe_queries:
            assert not is_safe_sql(query), f"{query} should be unsafe"

    def test_case_insensitive_detection(self):
        """Test that detection is case insensitive."""
        assert not is_safe_sql("drop table users")
        assert not is_safe_sql("DrOp TaBlE users")
        assert not is_safe_sql("DELETE from data")

    def test_query_with_drop_in_string(self):
        """Test query where DROP appears in a string literal."""
        # Note: This simple check will fail - it's intentionally basic
        query = "SELECT * FROM users WHERE name = 'DROP'"
        # This will return False due to the simple implementation
        assert not is_safe_sql(query)


class TestValidateDictSchema:
    """Tests for validate_dict_schema function."""

    def test_valid_schema(self):
        """Test dictionary with all required keys."""
        data = {"name": "Alice", "age": 30, "email": "alice@example.com"}
        required = ["name", "age", "email"]
        is_valid, error = validate_dict_schema(data, required)
        assert is_valid
        assert error is None

    def test_missing_single_key(self):
        """Test dictionary missing one required key."""
        data = {"name": "Alice", "age": 30}
        required = ["name", "age", "email"]
        is_valid, error = validate_dict_schema(data, required)
        assert not is_valid
        assert "email" in error

    def test_missing_multiple_keys(self):
        """Test dictionary missing multiple required keys."""
        data = {"name": "Alice"}
        required = ["name", "age", "email", "phone"]
        is_valid, error = validate_dict_schema(data, required)
        assert not is_valid
        assert "age" in error
        assert "email" in error
        assert "phone" in error

    def test_extra_keys_allowed(self):
        """Test that extra keys don't cause validation to fail."""
        data = {"name": "Alice", "age": 30, "extra": "value"}
        required = ["name", "age"]
        is_valid, error = validate_dict_schema(data, required)
        assert is_valid
        assert error is None

    def test_empty_required_keys(self):
        """Test with no required keys."""
        data = {"anything": "goes"}
        required = []
        is_valid, error = validate_dict_schema(data, required)
        assert is_valid
        assert error is None
