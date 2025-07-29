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
print(f"Big model response: {response.output_text}\n\n")

# Query the small model
response = client.responses.create(
    model="tiny",
    input="What are interesting landmarks I should visit in France?",
)
print(f"Tiny model response: {response.output_text}")
