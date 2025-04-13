pub fn add_config_flags(parser: clap::Command) -> clap::Command {
    parser.args([
        clap::Arg::new("backend")
            .long("backend")
            .env("SAUROPOD_BACKEND")
            .help(r#"The backend to use"#),
        clap::Arg::new("backend-api-key")
            .long("backend-api-key")
            .env("SAUROPOD_BACKEND_API_KEY")
            .help(r#"The API key to use to access the backend"#),
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
        clap::Arg::new("verbose")
            .long("verbose")
            .env("SAUROPOD_VERBOSE")
            .help(r#"Whether to log verbosely"#)
            .value_parser(clap::value_parser!(bool)),
    ])
}

pub fn clap_to_config_source(matches: &clap::ArgMatches) -> Box<super::ClapConfigSource> {
    let mut values = config::Map::new();

    if let Some(value) = matches
        .get_one::<String>("backend")
        .cloned()
        .map(|x| config::Value::new(None, x))
    {
        values.insert("backend".to_string(), value);
    }
    if let Some(value) = matches
        .get_one::<String>("backend-api-key")
        .cloned()
        .map(|x| config::Value::new(None, x))
    {
        values.insert("backend_api_key".to_string(), value);
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
    if let Some(value) = matches
        .get_one::<bool>("verbose")
        .cloned()
        .map(|x| config::Value::new(None, x))
    {
        values.insert("verbose".to_string(), value);
    }
    Box::new(super::ClapConfigSource { values })
}
