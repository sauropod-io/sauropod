# Deploying Sauropod

## Docker compose

The easiest way is to use [Docker Compose](https://docs.docker.com/compose/).

Here's an example

```yaml
services:
  ollama:
    image: ollama/ollama:latest
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: all
              capabilities: [gpu]
    volumes:
      - ollama-data:/root/.ollama
  sauropod:
    image: ghcr.io/sauropod-io/sauropod:latest
    ports:
      - "8080:8080"
    environment:
      - SAUROPOD_HOST=0.0.0.0
      - SAUROPOD_PORT=8080
      - SAUROPOD_BACKEND=http://ollama:11434
      # You can run MCP servers in your compose file as well
      # - SAUROPOD_MCP_SERVERS="http://container-1:port,http://container-2:port"
    depends_on:
      - ollama

volumes:
  ollama-data:
```

## Backend requirements

Sauropod supports any backend that has an OpenAI-compatible API with structured output support, for example:

- [Ollama](https://ollama.com/)
- [llama.cpp](https://github.com/ggml-org/llama.cpp)
- [mistral.rs](https://github.com/EricLBuehler/mistral.rs/)

## Using cloud-hosted LLM providers

If you want to use a hosted LLM providers like OpenAI put your API key in the [`backend_api_key`](./config.md#backend_api_key) field, set the `--backend-api-key` flag, or set `SAUROPOD_BACKEND_API_KEY`.

**NOTE**: The provider must support structured outputs - which providers like Groq and Anthropic don't have support in their OpenAI compatibility layers yet.

### OpenAI example

Example:

```bash
SAUROPOD_DEFAULT_MODEL__MODEL=gpt-4o-mini sauropod --backend=https://api.openai.com --backend-api-key="YOUR_KEY"
```
