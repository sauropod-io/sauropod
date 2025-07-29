//! Global state.

use std::sync::Arc;

use tracing::Instrument as _;

use sauropod_inference_engine::Model;

/// Global state accessor for Axum routes.
pub type AxumGlobalState = axum::extract::State<Arc<GlobalState>>;

/// The global state of the application.
pub struct GlobalState {
    /// The configuration.
    pub config: sauropod_config::Config,
    /// The database instance.
    database: sauropod_database::Database,
    /// The loaded models.
    loaded_models: sauropod_model_loading::LoadedModels,
}

impl GlobalState {
    /// Create a new global state instance.
    pub async fn new(config: &sauropod_config::Config) -> anyhow::Result<Self> {
        Self::new_with_database(
            config,
            sauropod_database::create_database(std::path::Path::new(&config.database_path)).await?,
        )
        .await
    }

    /// Create a new global state instance with an in-memory database.
    ///
    /// # Note
    ///
    /// This is only for unit testing.
    pub async fn new_in_memory(config: sauropod_config::Config) -> anyhow::Result<Self> {
        Self::new_with_database(&config, sauropod_database::create_in_memory().await?).await
    }

    /// Create a new global state instance using a database.
    async fn new_with_database(
        config: &sauropod_config::Config,
        database: sauropod_database::Database,
    ) -> anyhow::Result<Self> {
        let loaded_models = sauropod_model_loading::LoadedModels::new(config)
            .instrument(tracing::info_span!("Load models"))
            .await?;

        Ok(Self {
            config: config.clone(),
            database,
            loaded_models,
        })
    }

    /// Get a reference to the database.
    pub fn database(&self) -> &sauropod_database::Database {
        &self.database
    }

    /// Get a loaded model by name.
    pub async fn get_model(&self, model_name: &str) -> Option<Arc<Model>> {
        self.loaded_models.get_model(model_name).await
    }

    /// Get a loaded voice model by name.
    pub async fn get_voice_model(
        &self,
        model_name: &str,
    ) -> Option<Arc<sauropod_tts::ConfiguredTtsThread>> {
        self.loaded_models.get_voice_model(model_name).await
    }

    /// Get all the loaded models.
    pub async fn get_all_models(
        &self,
    ) -> tokio::sync::RwLockReadGuard<'_, std::collections::HashMap<String, Arc<Model>>> {
        self.loaded_models.get_all_models().await
    }

    /// Get all the loaded models.
    pub fn get_loaded_models(&self) -> &sauropod_model_loading::LoadedModels {
        &self.loaded_models
    }
}
