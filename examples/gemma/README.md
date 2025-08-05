# Gemma example

## Running the server

```bash
sauropod-inference-server --verbose --config-file examples/gemma/config.toml
```

## Text client example

```bash
pip install openai
python3 examples/gemma/query.py
```

## Voice client example

```bash
# On Ubuntu: apt-get install libportaudio2
pip install "livekit-agents[openai]"
python3 examples/gemma/voice.py console
```
