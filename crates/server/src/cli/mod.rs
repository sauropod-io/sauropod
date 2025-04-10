pub(crate) mod generated;

#[derive(Debug, Clone)]
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
        value: config::Value,
    ) -> Result<(), config::ConfigError> {
        self.values.insert(key, value);
        Ok(())
    }
}
