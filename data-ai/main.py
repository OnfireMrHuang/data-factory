"""Entry point for the Data AI Agent application."""

import uvicorn

from src.core.config import get_settings


def main():
    """Run the FastAPI application."""
    settings = get_settings()

    uvicorn.run(
        "src.app:app",
        host=settings.api_host,
        port=settings.api_port,
        reload=settings.debug,
        log_level="debug" if settings.debug else "info",
    )


if __name__ == "__main__":
    main()

