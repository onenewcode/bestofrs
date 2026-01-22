use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub github_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        let mut builder = ConfigBuilder::builder();

        let path = format!("{}/src/config/toml/{env}.toml", env!("CARGO_MANIFEST_DIR"));
        let contents = std::fs::read_to_string(&path).map_err(|e| {
            ConfigError::Message(format!("Failed to read config file {}: {}", path, e))
        })?;

        builder = builder.add_source(File::from_str(&contents, config::FileFormat::Toml));
        builder = builder.add_source(Environment::with_prefix("APP").separator("__"));

        let config = builder.build()?.try_deserialize()?;

        Ok(config)
    }

    pub fn server_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server.host, self.server.port)
            .parse()
            .expect("Invalid server address")
    }
}
