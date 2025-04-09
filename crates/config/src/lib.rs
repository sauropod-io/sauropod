//! Sauropod configuration.

use std::{collections::BTreeMap, path::PathBuf};

pub const ENV_VAR_PREFIX: &str = "SAUROPOD";

/// The type of a model.
#[derive(Clone, Copy, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub enum ModelType {
    /// The default model type.
    #[default]
    Default,
    /// Google's Gemma 3 model.
    Gemma3,
    /// Microsoft's Phi 4.
    Phi4,
}

/// Model Context Protocol server definitions.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum McpServer {
    /// Spawn a process and communicate with the MCP server over stdio.
    Process { command: String },
    /// Communicate with the MCP server over HTTP.
    Http { url: String },
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ModelConfig {
    /// The name of the model.
    #[cfg_attr(
        feature = "json_schema",
        schemars(example = "hf.co/unsloth/gemma-3-27b-it-GGUF:Q6_K")
    )]
    pub model: String,
    /// The type of model.
    #[serde(default, rename = "type")]
    pub model_type: ModelType,
}

/// Configuration for models.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Models {
    /// A model to use for simple tasks.
    pub weak: Option<ModelConfig>,
    /// A model to use for more complex tasks.
    pub strong: Option<ModelConfig>,
}

impl Models {
    /// Get a map from model strength to model configuration.
    pub fn to_map(&self) -> BTreeMap<sauropod_schemas::task::ModelStrength, ModelConfig> {
        let mut models = BTreeMap::new();
        if let Some(weak) = &self.weak {
            models.insert(sauropod_schemas::task::ModelStrength::Weak, weak.clone());
        }
        if let Some(strong) = &self.strong {
            models.insert(
                sauropod_schemas::task::ModelStrength::Strong,
                strong.clone(),
            );
        }
        models
    }
}

/// The default backend URL.
fn default_backend() -> String {
    "http://localhost:11434".to_string()
}

/// Sauropod configuration.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// The path to the SQLite database.
    #[serde(default)]
    #[cfg_attr(feature = "json_schema", schemars(example = "/data/database.sqlite"))]
    pub database_path: Option<String>,
    /// The host address to listen on.
    #[serde(default)]
    pub host: Option<String>,
    /// The port to listen on.
    #[serde(default)]
    #[cfg_attr(feature = "json_schema", schemars(example = 80))]
    pub port: Option<u16>,
    /// The backend to use.
    ///
    /// This is expected to be a URL that points to an OpenAPI-compatible backend like Ollama or llama-cpp.
    #[serde(default = "default_backend")]
    pub backend: String,
    /// The API key to use to access the backend.
    #[serde(default)]
    pub backend_api_key: Option<String>,
    /// The model configuration.
    #[serde(default)]
    pub models: Models,
    /// The MCP servers.
    #[serde(default)]
    pub mcp_servers: Vec<McpServer>,
}

impl Config {
    /// Load the configuration from a file.
    pub fn load_from_file(
        file_path: PathBuf,
        cli_overrides: Box<dyn config::Source + Send + Sync>,
    ) -> anyhow::Result<Self> {
        let dirs = directories::ProjectDirs::from("io", "sauropod", "sauropod");
        let data_dir = dirs.as_ref().map(|dirs| dirs.data_dir());
        let default_database_path = data_dir.map(|path: &std::path::Path| {
            path.join("database.sqlite").to_string_lossy().to_string()
        });

        let settings_builder = config::Config::builder()
            .add_source(config::File::from(file_path))
            .add_source(config::Environment::with_prefix(ENV_VAR_PREFIX))
            .add_source(vec![cli_overrides]);

        let settings_builder = if let Some(default_database_path) = default_database_path {
            settings_builder.set_default("database_path", default_database_path)?
        } else {
            settings_builder
        };

        let settings = settings_builder.build()?;
        Ok(settings.try_deserialize::<Config>()?)
    }

    /// Load the configuration.
    pub fn load(cli_overrides: Box<dyn config::Source + Send + Sync>) -> anyhow::Result<Self> {
        let dirs = match directories::ProjectDirs::from("io", "sauropod", "sauropod") {
            Some(dirs) => dirs,
            None => {
                anyhow::bail!("Failed to determine configuration directories.");
            }
        };

        let config = dirs.config_dir().join("config.toml");

        // Create the configuration directory.
        if !dirs.config_dir().exists() {
            std::fs::create_dir_all(dirs.config_dir())?;
        }

        // Create the configuration file if it does not exist.
        if !config.exists() {
            let default_config = Config::default();
            std::fs::write(&config, toml::to_string(&default_config)?)?;
        }

        Self::load_from_file(config, cli_overrides)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_path: None,
            host: None,
            port: None,
            backend: default_backend(),
            backend_api_key: None,
            models: Models::default(),
            mcp_servers: vec![],
        }
    }
}
