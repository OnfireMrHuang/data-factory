"""Agent API routes."""

import json
import logging
from typing import Annotated

from fastapi import APIRouter, Depends, HTTPException
from fastapi.responses import StreamingResponse

from ...schemas.requests import AgentRequest, AgentResponse
from ...services import AgentService, SessionManager
from ...utils import ValidationError, format_error_response
from ..dependencies import get_agent_service_dependency, get_session_manager_dependency

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/agent", tags=["agent"])


@router.post("/chat", response_model=AgentResponse)
async def chat(
    request: AgentRequest,
    service: Annotated[AgentService, Depends(get_agent_service_dependency)],
    session_manager: Annotated[SessionManager, Depends(get_session_manager_dependency)],
) -> AgentResponse:
    """
    Chat with the AI agent.

    The agent can help with:
    - Mathematical calculations
    - Data analysis and summaries
    - SQL query generation
    - General data-related questions
    """
    try:
        if request.stream:
            raise HTTPException(
                status_code=400,
                detail="Streaming not supported in this endpoint. Use /agent/stream instead.",
            )

        # Update session if provided
        if request.session_id:
            session_manager.get_or_create_session(request.session_id)
            session_manager.increment_message_count(request.session_id)

        # Process message through service layer
        result = await service.process_message(
            message=request.message,
            session_id=request.session_id,
        )

        return AgentResponse(**result)

    except ValidationError as e:
        logger.warning(f"Validation error: {e.message}")
        raise HTTPException(status_code=400, detail=format_error_response(e))

    except Exception as e:
        logger.error(f"Error processing chat request: {str(e)}", exc_info=True)
        raise HTTPException(status_code=500, detail=format_error_response(e))


@router.post("/stream")
async def stream_chat(
    request: AgentRequest,
    service: Annotated[AgentService, Depends(get_agent_service_dependency)],
    session_manager: Annotated[SessionManager, Depends(get_session_manager_dependency)],
):
    """
    Stream responses from the AI agent.

    This endpoint streams the agent's thinking process and responses in real-time.
    """
    try:
        # Update session if provided
        if request.session_id:
            session_manager.get_or_create_session(request.session_id)
            session_manager.increment_message_count(request.session_id)

        async def event_generator():
            """Generate SSE events from the agent stream."""
            try:
                async for event in service.stream_message(
                    message=request.message,
                    session_id=request.session_id,
                ):
                    # Format as SSE
                    yield f"data: {json.dumps(event)}\n\n"

                # Send completion event
                yield "data: [DONE]\n\n"

            except ValidationError as e:
                logger.warning(f"Validation error in stream: {e.message}")
                error_msg = format_error_response(e)
                yield f"data: {json.dumps(error_msg)}\n\n"

            except Exception as e:
                logger.error(f"Error in stream: {str(e)}", exc_info=True)
                error_msg = format_error_response(e)
                yield f"data: {json.dumps(error_msg)}\n\n"

        return StreamingResponse(
            event_generator(),
            media_type="text/event-stream",
        )

    except ValidationError as e:
        logger.warning(f"Validation error: {e.message}")
        raise HTTPException(status_code=400, detail=format_error_response(e))

    except Exception as e:
        logger.error(f"Error setting up stream: {str(e)}", exc_info=True)
        raise HTTPException(status_code=500, detail=format_error_response(e))
