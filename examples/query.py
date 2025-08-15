# Script header for `uv run`:
# /// script
# requires-python = ">=3.9"
# dependencies = ["openai"]
# ///
import datetime

from openai import OpenAI  # Install with `pip install openai`

client = OpenAI(
    base_url="http://localhost:3000/v1",  # Change to your Sauropod server URL
    api_key="hardcoded_api_key",
)

# Query the default model
current_time = datetime.datetime.now()
response = client.responses.create(
    model="default",
    input="What are interesting landmarks I should visit in France?",
)
duration = datetime.datetime.now() - current_time
if response.error:
    print(f"Error: {response.error}")
else:
    print(f"Model response (output tokens {response.usage.output_tokens}, tokens/s {response.usage.output_tokens / duration.total_seconds():.2f}):\n{response.output_text}")
