import logging
import sys
import queue
import threading
import time
from typing import AsyncIterable


import livekit.agents.utils
import livekit.rtc
import numpy as np
import serial
import serial.tools.list_ports
from livekit.agents import (
    Agent,
    AgentSession,
    JobContext,
    WorkerOptions,
    cli,
    ModelSettings,
)
from livekit.plugins import openai
from livekit.agents.voice.room_io import RoomInputOptions, RoomOutputOptions

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
