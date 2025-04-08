# Sauropod

A local AI orchestrator.

## Running Sauropod

### Example configuration

`~/.config/sauropod/config.toml`

```toml
# Run the server on port 8080
port = 8080
# Point the backend to an OpenAI-compatible server like Ollama.
backend = "http://localhost:11434"

[models.strong]
model = "hf.co/unsloth/gemma-3-27b-it-GGUF:Q6_K"
type = "Gemma3"

[[mcp_servers]]
# Spawn an MCP server as a subprocess controlled by the server
command = "docker run -it --rm markitdown-mcp:latest"

[[mcp_servers]]
# Connect to a remote MCP server
url = "http://localhost:1234"
```

## Build from source

### Dependencies

- [Clang](https://clang.llvm.org/)
- [Node.js](https://nodejs.org/)
- [Rust and Cargo](https://www.rust-lang.org/learn/get-started)
- `libssl`
- `make`
- `pkg-config`

### Build a release

```bash
make release
```

The binary will be created in `target/optimized-release/sauropod-server`.

## License

Most of the code is licensed under [AGPL](https://opensource.org/license/agpl-v3).

The code required to build custom clients - such as the schemas, client APIs, and OpenAPI specification - is licensed under [Apache-2.0](https://opensource.org/license/apache-2-0).
