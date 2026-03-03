//! Configuration management

mod settings;

pub use settings::{CacheConfig, DatabaseConfig, LoggingConfig, ServerConfig, Settings};

use config::{Config, ConfigError, Environment, File};

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("logging.level", "info")?
            .set_default("logging.format", "json")?
            .set_default("database.url", "sqlite::memory:")?
            .set_default("cache.max_capacity", 10_000)?
            .set_default("cache.ttl_seconds", 3600)?
            .add_source(File::with_name("config").required(false))
            .add_source(Environment::with_prefix("SLOWPOKEAPI").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}
