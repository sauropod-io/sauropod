# Examples

## gpt-oss

```bash
sauropod-inference-server --verbose --config-file examples/gpt-oss.toml
```

## Gemma

```bash
sauropod-inference-server --verbose --config-file examples/gemma.toml
```

### Memory efficienct gemma

To run a very small Gemma model.

```bash
sauropod-inference-server --verbose --config-file examples/memory-efficient.toml
```


## SmolVLM 2

```bash
sauropod-inference-server --verbose --config-file examples/smolvlm.toml
```

## Client example

### Text

```bash
# Using pip:
pip install openai
python3 examples/query.py

# Using Astral uv (will automatically install dependencies):
uv run examples/query.py
```

### Image

```bash
# Using pip:
pip install openai
python3 examples/query_with_image.py

# Using Astral uv (will automatically install dependencies):
uv run examples/query_with_image.py
```

### Voice

```bash
# On Ubuntu: apt-get install libportaudio2

# Using pip:
pip install "livekit-agents[openai]"
python3 examples/voice.py console

# Using Astral uv (will automatically install dependencies):
uv run examples/voice.py console
```
