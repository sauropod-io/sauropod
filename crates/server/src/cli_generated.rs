pub fn add_config_flags(parser: clap::Command) -> clap::Command {
    parser.args([
        clap::Arg::new("backend")
            .long("backend")
            .env("SAUROPOD_BACKEND")
            .help(r#"The backend to use"#),
        clap::Arg::new("cache-directory")
            .long("cache-directory")
            .env("SAUROPOD_CACHE_DIRECTORY")
            .help(r#"The cache directory"#),
        clap::Arg::new("database-path")
            .long("database-path")
            .env("SAUROPOD_DATABASE_PATH")
            .help(r#"The path to the SQLite database"#),
        clap::Arg::new("host")
            .long("host")
            .env("SAUROPOD_HOST")
            .help(r#"The host address to listen on"#),
        clap::Arg::new("port")
            .long("port")
            .env("SAUROPOD_PORT")
            .help(r#"The port to listen on"#)
            .value_parser(clap::value_parser!(i64)),
    ])
}

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

pub fn clap_to_config_source(matches: clap::ArgMatches) -> Box<ClapConfigSource> {
    let mut values = config::Map::new();

    if let Some(value) = matches
        .get_one::<String>("backend")
        .cloned()
        .map(|x| config::Value::new(None, x))
    {
        values.insert("backend".to_string(), value);
    }
    if let Some(value) = matches
        .get_one::<String>("cache-directory")
        .cloned()
        .map(|x| config::Value::new(None, x))
    {
        values.insert("cache_directory".to_string(), value);
    }
    if let Some(value) = matches
        .get_one::<String>("database-path")
        .cloned()
        .map(|x| config::Value::new(None, x))
    {
        values.insert("database_path".to_string(), value);
    }
    if let Some(value) = matches
        .get_one::<String>("host")
        .cloned()
        .map(|x| config::Value::new(None, x))
    {
        values.insert("host".to_string(), value);
    }
    if let Some(value) = matches
        .get_one::<i64>("port")
        .cloned()
        .map(|x| config::Value::new(None, x))
    {
        values.insert("port".to_string(), value);
    }
    Box::new(ClapConfigSource { values })
}
