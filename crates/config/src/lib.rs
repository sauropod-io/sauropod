//! Sauropod configuration.

use std::{collections::BTreeMap, path::PathBuf};

/// The type of a model.
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, Default)]
pub enum ModelType {
    /// The default model type.
    #[default]
    Default,
    /// Google's Gemma 3 model.
    Gemma3,
    /// Microsoft's Phi 4.
    Phi4,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ModelConfig {
    /// The name of the model.
    pub model: String,
    /// The type of model.
    #[serde(default, rename = "type")]
    pub model_type: ModelType,
}

/// Configuration for models.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
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
#[serde(deny_unknown_fields)]
pub struct Config {
    /// The cache directory.
    #[serde(default)]
    pub cache_directory: Option<String>,
    /// The path to the SQLite database.
    #[serde(default)]
    pub database_path: Option<String>,
    /// The host address to listen on.
    #[serde(default)]
    pub host: Option<String>,
    /// The port to listen on.
    #[serde(default)]
    pub port: Option<u16>,
    /// The backend to use.
    #[serde(default = "default_backend")]
    pub backend: String,
    /// The model configuration.
    #[serde(default)]
    pub models: Models,
}

macro_rules! set_cli_override {
    ($settings_builder:ident , $cli_overrides:ident . $field:ident) => {
        if let Some(override_val) = $cli_overrides.$field {
            $settings_builder.set_override(stringify!($field), override_val)?
        } else {
            $settings_builder
        }
    };
}

impl Config {
    /// Load the configuration from a file.
    pub fn load_from_file(
        file_path: PathBuf,
        cli_overrides: Option<Config>,
    ) -> anyhow::Result<Self> {
        let dirs = directories::ProjectDirs::from("io", "sauropod", "sauropod");
        let cache_dir = dirs
            .as_ref()
            .map(|dirs| dirs.cache_dir().to_string_lossy().to_string());
        let data_dir = dirs.as_ref().map(|dirs| dirs.data_dir());
        let default_database_path =
            data_dir.map(|path| path.join("database.sqlite").to_string_lossy().to_string());

        let settings_builder = config::Config::builder()
            .add_source(config::File::from(file_path))
            .add_source(config::Environment::with_prefix("SAUROPOD"));

        let settings_builder = if let Some(default_cache_directory) = cache_dir {
            settings_builder.set_default("cache_directory", default_cache_directory)?
        } else {
            settings_builder
        };
        let settings_builder = if let Some(default_database_path) = default_database_path {
            settings_builder.set_default("database_path", default_database_path)?
        } else {
            settings_builder
        };

        let cli_overrides = cli_overrides.unwrap_or_default();
        let settings_builder = set_cli_override!(settings_builder, cli_overrides.cache_directory);
        let settings_builder = set_cli_override!(settings_builder, cli_overrides.host);
        let settings_builder = set_cli_override!(settings_builder, cli_overrides.port);

        let settings = settings_builder.build()?;
        Ok(settings.try_deserialize::<Config>()?)
    }

    /// Load the configuration.
    pub fn load(cli_overrides: Option<Config>) -> anyhow::Result<Self> {
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
            cache_directory: None,
            database_path: None,
            host: None,
            port: None,
            backend: default_backend(),
            models: Models::default(),
        }
    }
}
