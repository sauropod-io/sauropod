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

# Query the default model
response = client.responses.create(
    model="default",
    input="What are interesting landmarks I should visit in France?",
)
if response.error:
    print(f"Error: {response.error}")
else:
    print(f"Model response:\n{response.output_text}")
