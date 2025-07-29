# Sauropod inference platform

Sauropod's inference platform.

- Compatible with OpenAI's Responses API and their Realtime WebSocket API

## Build

```bash
# A normal release build
cargo build --locked --profile=optimized-release --package=sauropod-inference-server

# For systems with Nvidia GPUs
cargo build --locked --profile=optimized-release --features=cuda --package=sauropod-inference-server
```

## Dependencies

- [Rust](https://www.rust-lang.org/tools/install)
- [CMake](https://cmake.org/)

### Optional dependencies

- [`vulkan-tools`](https://www.vulkan.org/) - required when building with `--features=vulkan`
- [CUDA](https://docs.nvidia.com/cuda/cuda-toolkit-release-notes/index.html) - required when building with `--features=cuda`

## Configuration

Sauropod uses a TOML configuration file located at `$CONFIG_DIR/config.toml`:

- Linux: `~/.config/sauropod/config.toml`
- macOS: `~/Library/Application Support/io.sauropod.sauropod/config.toml`
- Windows: `%APPDATA%\sauropod\io\config\config.toml`

You can override the config file location by setting the `SAUROPOD_CONFIG_FILE` environment variable or passing the `--config-file` flag.

See the examples in `./examples`.

### Configuration options

| Option           | Description                           | Default                     |
| ---------------- | ------------------------------------- | --------------------------- |
| `verbose`        | Whether to log verbosely              | `true`                      |
| `database_path`  | Path to the SQLite database           | `$DATA_DIR/database.sqlite` |
| `host`           | Host address to listen on             | `""`                        |
| `port`           | Port to listen on                     | `8080`                      |
| `models`         | Map of model configurations           | See below                   |
| `voices`         | Map of voice configurations           | See below                   |
| `trace_output`   | Path to output a Perfetto trace file  | `null` (disabled)           |
| `stt_model`      | Speech-to-text model to use           | See below                   |
| `vad_model`      | Voice activity detection model to use | See below                   |
| `authentication` | Authentication settings               | See below                   |

#### Model configuration

Each model in the `models` map has the following options:

| Option           | Description                            | Default  |
| ---------------- | -------------------------------------- | -------- |
| `model`          | Path or Hugging Face repo of the model | Required |
| `system_prompt`  | System prompt for the model            | `null`   |
| `temperature`    | Sampling temperature                   | `null`   |
| `top_p`          | Top-p sampling parameter               | `null`   |
| `maximum_tokens` | Maximum number of tokens to generate   | `null`   |
| `top_k`          | Top-k sampling parameter               | `null`   |
| `min_p`          | Minimum probability parameter          | `null`   |

#### Voice configuration

Each entry in the `voices` map has the following options:

| Option  | Description                                     | Default  |
| ------- | ----------------------------------------------- | -------- |
| `type`  | The voice engine to use (`kokoro` or `orpheus`) | Required |
| `voice` | The name of the voice to use                    | Required |
| `model` | The voice synthesis model to use                | Required |

#### STT & VAD configuration

These top-level keys select the speech-to-text and voice activity detection models.

| Option      | Description                           | Default                                                         |
| ----------- | ------------------------------------- | --------------------------------------------------------------- |
| `stt_model` | Speech-to-text model to use           | `huggingface.co/sauropod/parakeet-tdt-0.6b-v2`                  |
| `vad_model` | Voice activity detection model to use | `huggingface.co/sauropod/Frame_VAD_Multilingual_MarbleNet_v2.0` |

#### Authentication configuration

Controls API access.

| Option                  | Description                               | Default |
| ----------------------- | ----------------------------------------- | ------- |
| `allow_unauthenticated` | Allow API requests without authentication | `true`  |

### Example configuration

```toml
verbose = true
database_path = "/path/to/database.sqlite"
host = "127.0.0.1"
port = 8080

[voices.default]
type = "orpheus"
voice = "tara"

[voices.fast]
type = "kokoro"
voice = "af_heart"

[models.default]
model = "huggingface.co/unsloth/gemma-3-27b-it-qat-GGUF:Q4_K_M"
temperature = 0.7
top_p = 0.9
maximum_tokens = 2048

[models.small]
model = "huggingface.co/unsloth/SmolLM3-3B-128K-GGUF:Q4_K_M"
system_prompt = "You are a helpful dinosaur assistant."
temperature = 0.6
top_p = 0.95
maximum_tokens = 1024

[authentication]
type = "none"
```
