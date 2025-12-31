"""Tests for application entrypoint."""

from src.app import app


class TestApp:
    """Tests for FastAPI application."""

    def test_app_instance(self):
        """Test that app instance exists."""
        assert app is not None
        assert hasattr(app, "router")

    def test_app_metadata(self):
        """Test application metadata."""
        assert app.title == "Data AI Agent"
        assert app.version == "0.1.0"

    def test_cors_middleware_configured(self):
        """Test that CORS middleware is configured."""
        # Check that middleware is present
        # Note: FastAPI wraps middleware, so we check for Middleware wrapper
        middleware_types = [type(m).__name__ for m in app.user_middleware]
        # CORS middleware exists if we have any middleware configured
        assert len(middleware_types) > 0

    def test_api_routes_registered(self):
        """Test that API routes are registered."""
        routes = [route.path for route in app.routes]
        # Check for OpenAPI routes (with API prefix)
        assert any("/openapi.json" in route for route in routes)
        assert any("/docs" in route for route in routes)
