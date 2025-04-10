# Configuration file

Configuration can be provided via a config file, environment variables, or command line arguments.

## `backend`

- **Environment variable**: `SAUROPOD_BACKEND`
- **Type**: `string`
- **Default**: `"http://localhost:11434"`

The backend to use.

This is expected to be a URL that points to an OpenAPI-compatible backend like [Ollama](https://ollama.com/) or [llama-cpp](https://github.com/ggml-org/llama.cpp).

## `backend_api_key`

- **Environment variable**: `SAUROPOD_BACKEND_API_KEY`
- **Default**: `null`

The API key to use to access the backend.

## `database_path`

- **Environment variable**: `SAUROPOD_DATABASE_PATH`
- **Default**: `null`
- **Example Value**: `"/data/database.sqlite"`

The path to the SQLite database.

## `host`

- **Environment variable**: `SAUROPOD_HOST`
- **Default**: `null`

The host address to listen on.

## `[[mcp_servers]]`

- **Type**: `array`

`mcp_servers` supports 2 different options:

### Option 1

- **Type**: `object`

Spawn a process and communicate with the MCP server over stdio.

#### `[[command]]`

- **Type**: `array`

##### `command`

- **Type**: `string`

### Option 2

- **Type**: `object`

Communicate with the MCP server over HTTP.

#### `url`

- **Type**: `string`

## `[models]`

- **Type**: `object`
- **Default**: `{"strong":null,"weak":null}`

The model configuration.

### `[strong]`

A model to use for more complex tasks.

#### `model`

- **Environment variable**: `SAUROPOD_MODELS_STRONG_MODEL`
- **Type**: `string`
- **Example Value**: `"hf.co/unsloth/gemma-3-27b-it-GGUF:Q6_K"`

The name of the model.

#### `type`

- **Environment variable**: `SAUROPOD_MODELS_STRONG_TYPE`
- **Type**: `"Default"` | `"Gemma3"` | `"Phi4"`
- **Default**: `"Default"`

The type of model.

### `[weak]`

A model to use for simple tasks.

#### `model`

- **Environment variable**: `SAUROPOD_MODELS_WEAK_MODEL`
- **Type**: `string`
- **Example Value**: `"hf.co/unsloth/gemma-3-27b-it-GGUF:Q6_K"`

The name of the model.

#### `type`

- **Environment variable**: `SAUROPOD_MODELS_WEAK_TYPE`
- **Type**: `"Default"` | `"Gemma3"` | `"Phi4"`
- **Default**: `"Default"`

The type of model.

## `port`

- **Environment variable**: `SAUROPOD_PORT`
- **Default**: `null`
- **Example Value**: `80`

The port to listen on.
