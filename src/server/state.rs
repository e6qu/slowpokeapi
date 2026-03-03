use std::sync::Arc;
use std::time::Instant;

use crate::cache::RateCache;
use crate::ratelimit::RateLimiter;
use crate::storage::{ApiKeyStore, SqlitePool};
use crate::upstream::UpstreamManager;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<crate::config::Settings>,
    pub start_time: Instant,
    pub db_pool: Option<SqlitePool>,
    pub rate_cache: Option<Arc<RateCache>>,
    pub upstream_manager: Option<Arc<UpstreamManager>>,
    pub api_key_store: Option<Arc<ApiKeyStore>>,
    pub rate_limiter: Option<Arc<RateLimiter>>,
}

impl AppState {
    pub fn new(config: crate::config::Settings) -> Self {
        Self {
            config: Arc::new(config),
            start_time: Instant::now(),
            db_pool: None,
            rate_cache: None,
            upstream_manager: None,
            api_key_store: None,
            rate_limiter: None,
        }
    }

    pub fn with_db(mut self, pool: SqlitePool) -> Self {
        self.db_pool = Some(pool.clone());
        let cache = crate::cache::create_rate_cache(&self.config.cache, pool.clone());
        self.rate_cache = Some(Arc::new(cache));
        let api_key_store = Arc::new(ApiKeyStore::new(pool));
        let rate_limiter = Arc::new(RateLimiter::new(
            self.config.rate_limit.clone(),
            api_key_store.clone(),
        ));
        self.rate_limiter = Some(rate_limiter);
        self.api_key_store = Some(api_key_store);
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
