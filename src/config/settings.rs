//! Application settings

use serde::Deserialize;

use crate::auth::AuthConfig;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    #[serde(default)]
    pub sync: SyncConfig,
    #[serde(default)]
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CacheConfig {
    pub max_capacity: u64,
    pub ttl_seconds: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_capacity: 10_000,
            ttl_seconds: 3600,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SyncConfig {
    pub enabled: bool,
    pub peer_id: String,
    pub sync_interval_ms: u64,
    pub peer_timeout_ms: u64,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            peer_id: uuid::Uuid::new_v4().to_string(),
            sync_interval_ms: 5000,
            peer_timeout_ms: 60000,
        }
    }
}
