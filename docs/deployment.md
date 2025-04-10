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
    depends_on:
      - ollama

volumes:
  ollama-data:
```
