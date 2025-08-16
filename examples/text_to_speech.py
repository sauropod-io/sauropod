# Script header for `uv run`:
# /// script
# requires-python = ">=3.9"
# dependencies = ["openai"]
# ///
from openai import OpenAI  # Install with `pip install openai`

client = OpenAI(
    base_url="http://localhost:3000/v1",  # Change to your Sauropod server URL
    api_key="hardcoded_api_key",
)

with client.audio.speech.with_streaming_response.create(
    model="default",
    voice="default",
    input="the quick brown fox jumped over the lazy dogs",
    response_format="pcm",
    stream_format="audio",
) as response:
    print(response.read())
