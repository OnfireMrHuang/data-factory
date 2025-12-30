"""LangGraph agent implementation with state management."""

import logging
from typing import Annotated, Sequence, TypedDict

from langchain_core.messages import BaseMessage, HumanMessage, SystemMessage
from langchain_openai import ChatOpenAI
from langgraph.graph import END, StateGraph
from langgraph.graph.message import add_messages
from langgraph.prebuilt import ToolNode

from ..core.config import get_settings
from .tools import get_tools

logger = logging.getLogger(__name__)


class AgentState(TypedDict):
    """State for the agent graph."""

    messages: Annotated[Sequence[BaseMessage], add_messages]
    intermediate_steps: list[dict]


class DataAnalysisAgent:
    """
    LangGraph-based agent for data analysis tasks.

    This agent uses a state graph to manage complex workflows with tool calling.
    """

    def __init__(self):
        """Initialize the agent with LLM and tools."""
        settings = get_settings()

        # Initialize LLM
        self.llm = ChatOpenAI(
            model=settings.openai_model,
            temperature=settings.openai_temperature,
            max_tokens=settings.openai_max_tokens,
            api_key=settings.openai_api_key,
        )

        # Get tools and bind to LLM
        self.tools = get_tools()
        self.llm_with_tools = self.llm.bind_tools(self.tools)

        # Build the graph
        self.graph = self._build_graph()

        logger.info("DataAnalysisAgent initialized with LangGraph")

    def _build_graph(self) -> StateGraph:
        """Build the agent state graph."""
        workflow = StateGraph(AgentState)

        # Define nodes
        workflow.add_node("agent", self._call_model)
        workflow.add_node("tools", ToolNode(self.tools))

        # Set entry point
        workflow.set_entry_point("agent")

        # Add conditional edges
        workflow.add_conditional_edges(
            "agent",
            self._should_continue,
            {
                "continue": "tools",
                "end": END,
            },
        )

        # Add edge from tools back to agent
        workflow.add_edge("tools", "agent")

        return workflow.compile()

    async def _call_model(self, state: AgentState) -> dict:
        """Call the LLM with the current state."""
        messages = state["messages"]

        # Add system message if this is the first call
        if not any(isinstance(msg, SystemMessage) for msg in messages):
            system_message = SystemMessage(
                content="""You are a helpful AI assistant specialized in data analysis.
You have access to tools for calculations, data analysis, and SQL generation.

When helping users:
1. Break down complex tasks into steps
2. Use tools when appropriate
3. Provide clear explanations
4. Ask clarifying questions if needed

Always be concise and helpful."""
            )
            messages = [system_message] + list(messages)

        # Call the LLM
        response = await self.llm_with_tools.ainvoke(messages)

        return {"messages": [response]}

    def _should_continue(self, state: AgentState) -> str:
        """Determine if the agent should continue or end."""
        messages = state["messages"]
        last_message = messages[-1]

        # If there are tool calls, continue
        if hasattr(last_message, "tool_calls") and last_message.tool_calls:
            return "continue"

        # Otherwise, end
        return "end"

    async def run(self, message: str, session_id: str | None = None) -> dict:
        """
        Run the agent with a user message.

        Args:
            message: User's input message
            session_id: Optional session ID for conversation context

        Returns:
            Agent response with metadata
        """
        logger.info(f"Running agent with message: {message[:100]}...")

        try:
            # Create initial state
            initial_state = {
                "messages": [HumanMessage(content=message)],
                "intermediate_steps": [],
            }

            # Run the graph
            result = await self.graph.ainvoke(initial_state)

            # Extract response
            messages = result["messages"]
            last_message = messages[-1]

            response_text = last_message.content if hasattr(last_message, "content") else str(last_message)

            # Collect intermediate steps
            intermediate_steps = []
            for msg in messages[1:-1]:  # Skip first and last
                if hasattr(msg, "tool_calls") and msg.tool_calls:
                    for tool_call in msg.tool_calls:
                        intermediate_steps.append({
                            "tool": tool_call.get("name", "unknown"),
                            "input": tool_call.get("args", {}),
                        })

            logger.info("Agent completed successfully")

            return {
                "response": response_text,
                "session_id": session_id or "default",
                "intermediate_steps": intermediate_steps if intermediate_steps else None,
                "metadata": {
                    "message_count": len(messages),
                    "tools_used": len(intermediate_steps),
                },
            }

        except Exception as e:
            logger.error(f"Error running agent: {str(e)}", exc_info=True)
            raise

    async def stream(self, message: str, session_id: str | None = None):
        """
        Stream agent responses.

        Args:
            message: User's input message
            session_id: Optional session ID

        Yields:
            Chunks of the agent's response
        """
        logger.info(f"Streaming agent response for: {message[:100]}...")

        try:
            initial_state = {
                "messages": [HumanMessage(content=message)],
                "intermediate_steps": [],
            }

            # Stream events from the graph
            async for event in self.graph.astream(initial_state):
                # Yield the event
                yield event

        except Exception as e:
            logger.error(f"Error streaming agent response: {str(e)}", exc_info=True)
            raise


# Singleton instance
_agent_instance: DataAnalysisAgent | None = None


def get_agent() -> DataAnalysisAgent:
    """Get or create the agent instance."""
    global _agent_instance
    if _agent_instance is None:
        _agent_instance = DataAnalysisAgent()
    return _agent_instance
