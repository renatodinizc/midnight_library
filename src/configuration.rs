#[derive(serde::Deserialize)]
pub struct ApplicationConfigs {
    pub database: DatabaseConfig,
}

#[derive(serde::Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub name: String,
}

impl DatabaseConfig {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }
}

pub fn get_configuration() -> Result<ApplicationConfigs, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<ApplicationConfigs>()
}
