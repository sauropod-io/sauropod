import base64
import urllib.request
from pathlib import Path

from openai import OpenAI  # Install with `pip install openai`


GIRAFFE_IMAGE = Path(__file__).parent / "giraffe.jpg"
if not GIRAFFE_IMAGE.exists():
    urllib.request.urlretrieve(
        "https://upload.wikimedia.org/wikipedia/commons/thumb/9/9e/Giraffe_Mikumi_National_Park.jpg/960px-Giraffe_Mikumi_National_Park.jpg",
        GIRAFFE_IMAGE,
    )

image_base64 = base64.b64encode(GIRAFFE_IMAGE.read_bytes()).decode("utf-8")

client = OpenAI(
    base_url="http://localhost:3000/v1",  # Change to your Sauropod server URL
    api_key="hardcoded_api_key",
)

# Query the default model with image
response = client.responses.create(
    model="default",
    input=[
        {
            "role": "user",
            "content": [
                {
                    "type": "input_text",
                    "text": "What animal is in this image?",
                },
                {
                    "type": "input_image",
                    "image_url": f"data:image/jpeg;base64,{image_base64}",
                },
            ],
        }
    ],
)

if response.error:
    print(f"Error: {response.error}")
else:
    print(f"Model response: {response.output_text}")
