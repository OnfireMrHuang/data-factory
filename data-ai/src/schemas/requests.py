"""API request and response schemas."""

from pydantic import BaseModel, Field


class ChatMessage(BaseModel):
    """A single chat message."""

    role: str = Field(..., description="Message role (user, assistant, system)")
    content: str = Field(..., description="Message content")


class AgentRequest(BaseModel):
    """Request to run the agent."""

    message: str = Field(..., description="User message to process", min_length=1)
    session_id: str | None = Field(None, description="Optional session ID for conversation context")
    stream: bool = Field(False, description="Whether to stream the response")


class AgentResponse(BaseModel):
    """Response from the agent."""

    response: str = Field(..., description="Agent's response")
    session_id: str = Field(..., description="Session ID for this conversation")
    intermediate_steps: list[dict] | None = Field(
        None, description="Intermediate steps taken by the agent"
    )
    metadata: dict | None = Field(None, description="Additional metadata")


class HealthResponse(BaseModel):
    """Health check response."""

    status: str = Field(..., description="Service status")
    version: str = Field(..., description="API version")
