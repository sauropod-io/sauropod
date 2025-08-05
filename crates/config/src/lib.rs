//! Sauropod configuration.

use std::{collections::HashMap, path::PathBuf};

mod model_source;
pub use model_source::*;

#[derive(Clone, Debug, Default)]
pub struct ClapConfigSource {
    values: config::Map<String, config::Value>,
}

impl config::Source for ClapConfigSource {
    fn clone_into_box(&self) -> Box<dyn config::Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(&self) -> Result<config::Map<String, config::Value>, config::ConfigError> {
        Ok(self.values.clone())
    }
}

impl ClapConfigSource {
    /// Manually add a value to the source.
    pub fn add_value(
        &mut self,
        key: String,
        value: impl Into<config::Value>,
    ) -> Result<(), config::ConfigError> {
        self.values.insert(key, value.into());
        Ok(())
    }
}

/// Configuration for a model.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModelConfig {
    /// The path or Hugging Face repo of the model.
    pub model: ConfigModelSource,
    /// The project to use for multimodal models.
    pub multimodal_projector: Option<ConfigModelSource>,
    /// The system prompt for the model.
    #[serde(default)]
    pub system_prompt: Option<String>,
    /// Sampling temperature for the model.
    pub temperature: Option<f64>,
    /// Sampling top_p for the model.
    pub top_p: Option<f64>,
    /// The maximum number of tokens to generate.
    pub maximum_tokens: Option<i64>,
    /// The top_k sampling parameter for the model.
    pub top_k: Option<i64>,
    /// The minimum probability for the model.
    pub min_p: Option<i64>,
    /// Jinja template for the chat.
    #[serde(default)]
    pub chat_template: Option<String>,
}

/// Voice model configuration.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, tag = "type", rename_all = "snake_case")]
pub enum VoiceConfig {
    Kokoro {
        voice: String,
        #[serde(default = "VoiceConfig::default_kokoro_model")]
        model: ConfigModelSource,
    },
    Orpheus {
        voice: String,
        #[serde(default = "VoiceConfig::default_orpheus_model")]
        model: ConfigModelSource,
    },
}

impl VoiceConfig {
    fn default_kokoro_model() -> ConfigModelSource {
        ConfigModelSource::from_huggingface("onnx-community/Kokoro-82M-v1.0-ONNX", None)
    }

    fn default_orpheus_model() -> ConfigModelSource {
        ConfigModelSource::from_huggingface(
            "unsloth/orpheus-3b-0.1-ft-GGUF:",
            Some(PathOrQuantization::Quantization {
                quantization: "Q4_K_M".to_string(),
            }),
        )
    }

    /// Get the voice name.
    pub fn get_voice(&self) -> Option<&str> {
        match self {
            VoiceConfig::Kokoro { voice, .. } => Some(voice),
            VoiceConfig::Orpheus { voice, .. } => Some(voice),
        }
    }
}

/// Configuration for authentication.
#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, tag = "type", rename_all = "snake_case")]
pub enum AuthenticationConfig {
    /// A single hard-coded API key.
    ApiKey { api_key: String },
    /// Use the users stored in the database.
    Database,
    /// Allow unauthenticated access.
    #[default]
    None,
}

/// Sauropod configuration.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Whether to log verbosely.
    ///
    /// You can also control the log level using the`SAUROPOD_LOG` environment variable, e.g. `SAUROPOD_LOG=debug`.
    #[serde(default = "Config::default_verbose")]
    pub verbose: bool,
    /// The path to the SQLite database.
    #[serde(default)]
    pub database_path: String,
    /// The host address to listen on.
    #[serde(default)]
    pub host: String,
    /// The port to listen on.
    #[serde(default = "Config::default_port")]
    pub port: u16,
    /// The model configurations.
    #[serde(default)]
    pub models: HashMap<String, ModelConfig>,
    /// Map from voice name to voice configuration.
    #[serde(default = "Config::default_voices")]
    pub voices: HashMap<String, VoiceConfig>,
    /// The path to output a Perfetto trace file to.
    #[serde(default)]
    pub trace_output: Option<String>,
    #[serde(default = "Config::default_stt_model")]
    /// The speech-to-text model to use for voice inputs to models without native audio support.
    pub stt_model: Option<ConfigModelSource>,
    #[serde(default = "Config::default_vad_model")]
    /// The voice activity detection model to use for voice inputs.
    pub vad_model: Option<ConfigModelSource>,
    #[serde(default)]
    pub authentication: AuthenticationConfig,
}

impl Config {
    /// The default backend port.
    fn default_port() -> u16 {
        8080
    }

    /// The default verbose value.
    fn default_verbose() -> bool {
        true
    }

    fn default_voices() -> HashMap<String, VoiceConfig> {
        HashMap::from([(
            "default".to_string(),
            VoiceConfig::Kokoro {
                voice: "af_heart".to_string(),
                model: VoiceConfig::default_kokoro_model(),
            },
        )])
    }

    fn default_vad_model() -> Option<ConfigModelSource> {
        Some(ConfigModelSource::from_huggingface(
            "sauropod/Frame_VAD_Multilingual_MarbleNet_v2.0",
            None,
        ))
    }

    fn default_stt_model() -> Option<ConfigModelSource> {
        Some(ConfigModelSource::from_huggingface(
            "sauropod/parakeet-tdt-0.6b-v2",
            None,
        ))
    }
}

impl Config {
    /// Load the configuration from a file.
    pub fn load_from_file(
        file_path: PathBuf,
        cli_overrides: ClapConfigSource,
    ) -> anyhow::Result<Self> {
        let dirs = directories::ProjectDirs::from("io", "sauropod", "sauropod");
        let data_dir = dirs.as_ref().map(|dirs| dirs.data_dir());
        let default_database_path = data_dir.map(|path: &std::path::Path| {
            path.join("database.sqlite").to_string_lossy().to_string()
        });

        let settings_builder = config::Config::builder()
            .add_source(config::File::from(file_path))
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
    pub fn load(cli_overrides: ClapConfigSource) -> anyhow::Result<Self> {
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

        Self::load_from_file(config, cli_overrides)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            verbose: Self::default_verbose(),
            database_path: "".to_string(),
            host: "".to_string(),
            port: Self::default_port(),
            models: HashMap::new(),
            voices: Self::default_voices(),
            trace_output: None,
            stt_model: Self::default_stt_model(),
            vad_model: Self::default_vad_model(),
            authentication: AuthenticationConfig::default(),
        }
    }
}
