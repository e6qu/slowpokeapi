use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<crate::config::Settings>,
    pub start_time: Instant,
}

impl AppState {
    pub fn new(config: crate::config::Settings) -> Self {
        Self {
            config: Arc::new(config),
            start_time: Instant::now(),
        }
    }

    pub fn db_health(&self) -> Option<bool> {
        Some(true)
    }
}
