# Sauropod inference platform

Sauropod's inference platform.

- Compatible with OpenAI's Responses API and their Realtime WebSocket API

## Dependencies

- [Rust](https://www.rust-lang.org/tools/install) >= 1.85
- [Clang](https://clang.llvm.org/)
- [CMake](https://cmake.org/)
- OpenSSL

### Backend dependencies

Either Vulkan, CUDA, or Metal can be used as the inference backend.

- [`libvulkan`](https://www.vulkan.org/) and [glslc](https://github.com/google/shaderc/tree/main/glslc) - required when building with `--features=vulkan`
  - Debian or Ubuntu: `sudo apt-get install clang lld cmake glslc libssl-dev libvulkan-dev pkg-config`
- [CUDA](https://docs.nvidia.com/cuda/cuda-toolkit-release-notes/index.html) - required when building with `--features=cuda`
  - Debian or Ubuntu: `sudo apt-get install clang lld cmake glslc libssl-dev nvidia-cuda-toolkit pkg-config`

## Build

```bash
# A normal release build
cargo build --locked --profile=optimized-release --features=vulkan --package=sauropod-inference-server

# For systems with Nvidia GPUs
cargo build --locked --profile=optimized-release --no-default-features --features=cuda --package=sauropod-inference-server
```

The built binary will be available at `./target/optimized-release/sauropod-inference-server`.

## Configuration

Sauropod uses a TOML configuration file located at `$CONFIG_DIR/config.toml`:

- Linux: `~/.config/sauropod/config.toml`
- macOS: `~/Library/Application Support/io.sauropod.sauropod/config.toml`
- Windows: `%APPDATA%\sauropod\io\config\config.toml`

You can override the config file location by setting the `SAUROPOD_CONFIG_FILE` environment variable or passing the `--config-file` flag.

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

| Option                 | Description                                         | Default  |
| ---------------------- | --------------------------------------------------- | -------- |
| `model`                | Path or Hugging Face repo of the model              | Required |
| `multimodal_projector` | Path or Hugging Face repo of the multimodal project | `null`   |
| `system_prompt`        | System prompt for the model                         | `null`   |
| `temperature`          | Sampling temperature                                | `null`   |
| `top_p`                | Top-p sampling parameter                            | `null`   |
| `maximum_tokens`       | Maximum number of tokens to generate                | `null`   |
| `top_k`                | Top-k sampling parameter                            | `null`   |
| `min_p`                | Minimum probability parameter                       | `null`   |

##### Model source formats

The `model` field (and `multimodal_projector`) accepts three formats:

**Local path:**

```toml
model = "/path/to/local/model.gguf"
```

**Hugging Face with quantization:**

```toml
model = { repo = "unsloth/gemma-3-27b-it-qat-GGUF", quantization = "Q4_K_M" }
```

**Hugging Face with specific file:**

```toml
model = { repo = "unsloth/gemma-3-27b-it-qat-GGUF", file = "gemma-3-27b-it-qat-Q4_K_M.gguf" }
```

#### Voice configuration

Each entry in the `voices` map has the following options:

| Option  | Description                                     | Default                 |
| ------- | ----------------------------------------------- | ----------------------- |
| `type`  | The voice engine to use (`kokoro` or `orpheus`) | Required                |
| `voice` | The name of the voice to use                    | Required                |
| `model` | The voice synthesis model to use                | Optional (has defaults) |

##### Kokoro voice configuration

```toml
[voices.my_voice]
type = "kokoro"
voice = "af_heart"  # or other available voices
model = "huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX"  # optional, uses default if not specified
```

##### Orpheus voice configuration

```toml
[voices.my_voice]
type = "orpheus"
voice = "tara"  # or other available voices
model = "huggingface.co/unsloth/orpheus-3b-0.1-ft-GGUF:Q4_K_M"  # optional, uses default if not specified
```

#### STT & VAD configuration

These top-level keys select the speech-to-text and voice activity detection models.

| Option      | Description                           | Default                                                         |
| ----------- | ------------------------------------- | --------------------------------------------------------------- |
| `stt_model` | Speech-to-text model to use           | `huggingface.co/sauropod/parakeet-tdt-0.6b-v2`                  |
| `vad_model` | Voice activity detection model to use | `huggingface.co/sauropod/Frame_VAD_Multilingual_MarbleNet_v2.0` |

#### Authentication configuration

Controls API access using a tagged enum structure:

##### No authentication (default)

```toml
[authentication]
type = "none"
```

##### API key authentication

```toml
[authentication]
type = "api_key"
api_key = "your-secret-api-key"
```

##### Database-based authentication

```toml
[authentication]
type = "database"
```

### Example configuration

See the examples in `./examples`.
