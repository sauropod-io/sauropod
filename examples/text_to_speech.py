# Script header for `uv run`:
# /// script
# requires-python = ">=3.9"
# dependencies = ["openai"]
# ///
import wave

from openai import OpenAI  # Install with `pip install openai`

client = OpenAI(
    base_url="http://localhost:3000/v1",  # Change to your Sauropod server URL
    api_key="hardcoded_api_key",
)

input = """
The first time Elias heard the world crack, he was twelve. It wasn’t a sound that came from outside—no thunderclap, no landslide—but from the marrow of the earth itself, a deep and aching fracture, as though reality had strained past its limits. Everyone else in the marketplace kept walking, haggling, laughing. Only Elias froze, heart hammering, staring at the cobblestones beneath his feet.
The first world he entered was a desert made of glass. The sand had melted long ago, fused into rippling panes that stretched to every horizon. Above, two suns dragged pale heat across the sky. Shattered towers stood like broken teeth. Elias found no living things, only echoes: shadows burned into walls where people had once stood before a great fire. He took notes in his journal until the fracture pulsed again, tugging him home.
"""
with client.audio.speech.with_streaming_response.create(
    model="default",
    voice="default",
    input=input,
    response_format="pcm",
    stream_format="audio",
) as response, wave.open("tts.wav", "wb") as wf:
    data = response.read()
    wf.setnchannels(1)
    wf.setsampwidth(2)
    wf.setframerate(24_000)
    wf.writeframes(data)

print("Wrote audio to tts.wav")
