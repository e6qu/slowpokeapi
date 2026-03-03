pub mod api_key;

pub use api_key::validate_api_key;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    #[serde(default = "default_require_api_key")]
    pub require_api_key: bool,

    #[serde(default = "default_public_paths")]
    pub public_paths: Vec<String>,
}

fn default_enabled() -> bool {
    true
}

fn default_require_api_key() -> bool {
    false
}

fn default_public_paths() -> Vec<String> {
    vec![
        "/healthz".to_string(),
        "/readyz".to_string(),
        "/livez".to_string(),
        "/health".to_string(),
        "/metrics".to_string(),
        "/swagger-ui".to_string(),
        "/api-docs".to_string(),
    ]
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            require_api_key: default_require_api_key(),
            public_paths: default_public_paths(),
        }
    }
}
