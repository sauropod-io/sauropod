# Sauropod inference platform

Sauropod's inference platform.

- Compatible with OpenAI's Responses API and their Realtime WebSocket API

## Dependencies

- [Rust](https://www.rust-lang.org/tools/install) >= 1.85
  - Debian or Ubuntu: `sudo apt-get install rustup; rustup install stable`
- [Clang](https://clang.llvm.org/)
- [CMake](https://cmake.org/)
- OpenSSL

For development dependencies - see [`./CONTRIBUTING.md`](./CONTRIBUTING.md)

### Backend dependencies

Either Vulkan, CUDA, or Metal can be used as the inference backend.

- [`libvulkan`](https://www.vulkan.org/) and [glslc](https://github.com/google/shaderc/tree/main/glslc) - required when building with `--features=vulkan`
  - Debian or Ubuntu: `sudo apt-get install build-essential clang lld cmake glslc libssl-dev libvulkan-dev pkg-config`
- [CUDA](https://docs.nvidia.com/cuda/cuda-toolkit-release-notes/index.html) - required when building with `--features=cuda`
  - Debian or Ubuntu: `sudo apt-get install build-essential clang lld cmake glslc libssl-dev nvidia-cuda-toolkit pkg-config`

## Build

```bash
# A normal release build
cargo build --locked --profile=optimized-release --features=vulkan --package=sauropod-inference-server

# For systems with Nvidia GPUs
cargo build --locked --profile=optimized-release --no-default-features --features=cuda --package=sauropod-inference-server
```

The built binary will be available at `./target/optimized-release/sauropod-inference-server`.

## Quick start

Now that the code is built run `./target/optimized-release/sauropod-inference-server --verbose --config-file examples/gemma/config.toml`.

For more info see the [`configuration`](./docs/Configuration.md) docs and the [`./examples`](./examples).

## Configuration

See [`./docs/Configuration.md`](./docs/Configuration.md)

## Examples

See [`./examples`](./examples)
