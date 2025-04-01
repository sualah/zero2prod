
use secrecy::{ExposeSecret, SecretString};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub username: String,
    pub password: SecretString,
    pub database_name: String,
    pub port: u16,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretString {
       SecretString::from( format!(
           "postgres://{}:{}@{}:{}/{}",
           self.username, self.password.expose_secret(), self.host, self.port, self.database_name
       ))
    }

    pub fn connection_string_without_db(&self) -> SecretString {
       SecretString::from( format!(
           "postgres://{}:{}@{}:{}",
           self.username, self.password.expose_secret(), self.host, self.port
       ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()?
        .try_deserialize()?;

    Ok(settings)
}
