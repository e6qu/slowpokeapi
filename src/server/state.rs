use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<crate::config::Settings>,
    pub start_time: Instant,
    pub db_pool: Option<crate::storage::SqlitePool>,
}

impl AppState {
    pub fn new(config: crate::config::Settings) -> Self {
        Self {
            config: Arc::new(config),
            start_time: Instant::now(),
            db_pool: None,
        }
    }

    pub fn with_db(mut self, pool: crate::storage::SqlitePool) -> Self {
        self.db_pool = Some(pool);
        self
    }

    pub fn db_health(&self) -> Option<bool> {
        if let Some(pool) = &self.db_pool {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current()
                    .block_on(async { crate::storage::sqlite::health_check(pool).await.ok() })
            })
        } else {
            None
        }
    }
}
