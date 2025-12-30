"""Main FastAPI application."""

import logging
from contextlib import asynccontextmanager

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse

from .api.routes import agent_router, session_router
from .core.config import get_settings
from .core.logging import setup_logging
from .schemas.requests import HealthResponse

logger = logging.getLogger(__name__)


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan events."""
    # Startup
    setup_logging()
    logger.info("Starting Data AI Agent API...")

    settings = get_settings()
    logger.info(f"API Version: {settings.app_version}")
    logger.info(f"OpenAI Model: {settings.openai_model}")

    yield

    # Shutdown
    logger.info("Shutting down Data AI Agent API...")


def create_app() -> FastAPI:
    """Create and configure the FastAPI application."""
    settings = get_settings()

    app = FastAPI(
        title=settings.app_name,
        version=settings.app_version,
        description="AI Agent for Data Analysis powered by FastAPI and LangGraph",
        lifespan=lifespan,
        docs_url=f"{settings.api_prefix}/docs",
        redoc_url=f"{settings.api_prefix}/redoc",
        openapi_url=f"{settings.api_prefix}/openapi.json",
    )

    # CORS middleware
    app.add_middleware(
        CORSMiddleware,
        allow_origins=["*"],  # Configure appropriately for production
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )

    # Health check endpoint
    @app.get("/health", response_model=HealthResponse, tags=["health"])
    async def health_check():
        """Health check endpoint."""
        return HealthResponse(
            status="healthy",
            version=settings.app_version,
        )

    # Include routers
    app.include_router(agent_router, prefix=settings.api_prefix)
    app.include_router(session_router, prefix=settings.api_prefix)

    # Global exception handler
    @app.exception_handler(Exception)
    async def global_exception_handler(request, exc):
        """Handle uncaught exceptions."""
        logger.error(f"Unhandled exception: {str(exc)}", exc_info=True)
        return JSONResponse(
            status_code=500,
            content={
                "detail": "Internal server error",
                "error": str(exc) if settings.debug else "An error occurred",
            },
        )

    return app


# Create the app instance
app = create_app()
