# Script header for `uv run`:
# /// script
# requires-python = ">=3.9"
# dependencies = ["livekit-agents[openai]"]
# ///

import logging

import livekit.agents.utils
import livekit.rtc
from livekit.agents import (
    Agent,
    AgentSession,
    JobContext,
    WorkerOptions,
    cli,
)
from livekit.plugins import openai

logger = logging.getLogger("voice-agent")


class VoiceAgent(Agent):
    def __init__(self) -> None:
        super().__init__(
            instructions=("Your name is Voicey."),
        )


async def entrypoint(ctx: JobContext):
    await ctx.connect()
    session = AgentSession(
        llm=openai.realtime.RealtimeModel(
            base_url="http://localhost:3000/v1",
            api_key="hardcoded_api_key",
            model="default",
            voice="default",
        ),
        mcp_servers=[],
    )

    await session.start(agent=VoiceAgent(), room=ctx.room)


if __name__ == "__main__":
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    )
    cli.run_app(WorkerOptions(entrypoint_fnc=entrypoint))
