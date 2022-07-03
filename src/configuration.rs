#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn get_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize config reader
    let mut settings = config::Config::default();

    // Add config value from file named "configuration".
    // Will look for any top-level file with an extension config knows
    // (json, yaml, etc.)
    settings.merge(config::File::with_name("configuration"))?;

    // Try to convert config values into Settings type
    settings.try_into()
}