use std::sync::Arc;
use std::time::Instant;

use crate::cache::RateCache;
use crate::storage::SqlitePool;
use crate::upstream::UpstreamManager;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<crate::config::Settings>,
    pub start_time: Instant,
    pub db_pool: Option<SqlitePool>,
    pub rate_cache: Option<Arc<RateCache>>,
    pub upstream_manager: Option<Arc<UpstreamManager>>,
}

impl AppState {
    pub fn new(config: crate::config::Settings) -> Self {
        Self {
            config: Arc::new(config),
            start_time: Instant::now(),
            db_pool: None,
            rate_cache: None,
            upstream_manager: None,
        }
    }

    pub fn with_db(mut self, pool: SqlitePool) -> Self {
        self.db_pool = Some(pool.clone());
        let cache = crate::cache::create_rate_cache(&self.config.cache, pool);
        self.rate_cache = Some(Arc::new(cache));
        self
    }

    pub fn with_upstream(mut self, manager: UpstreamManager) -> Self {
        self.upstream_manager = Some(Arc::new(manager));
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
