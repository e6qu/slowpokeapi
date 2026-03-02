use once_cell::sync::Lazy;
use prometheus::{Counter, Gauge, HistogramVec, Opts};

pub static CACHE_METRICS: Lazy<CacheMetrics> = Lazy::new(CacheMetrics::new);

pub struct CacheMetrics {
    hits: Counter,
    misses: Counter,
    sets: Counter,
    deletes: Counter,
    evictions: Counter,
    size: Gauge,
    latency: HistogramVec,
}

impl CacheMetrics {
    fn new() -> Self {
        let hits = Counter::with_opts(Opts::new(
            "slowpokeapi_cache_hits_total",
            "Total number of cache hits",
        ))
        .unwrap();

        let misses = Counter::with_opts(Opts::new(
            "slowpokeapi_cache_misses_total",
            "Total number of cache misses",
        ))
        .unwrap();

        let sets = Counter::with_opts(Opts::new(
            "slowpokeapi_cache_sets_total",
            "Total number of cache set operations",
        ))
        .unwrap();

        let deletes = Counter::with_opts(Opts::new(
            "slowpokeapi_cache_deletes_total",
            "Total number of cache delete operations",
        ))
        .unwrap();

        let evictions = Counter::with_opts(Opts::new(
            "slowpokeapi_cache_evictions_total",
            "Total number of cache evictions",
        ))
        .unwrap();

        let size = Gauge::with_opts(Opts::new(
            "slowpokeapi_cache_size",
            "Current number of entries in cache",
        ))
        .unwrap();

        let latency = HistogramVec::new(
            prometheus::histogram_opts!(
                "slowpokeapi_cache_latency_seconds",
                "Cache operation latency in seconds",
                vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]
            ),
            &["operation"],
        )
        .unwrap();

        let registry = prometheus::default_registry();
        registry.register(Box::new(hits.clone())).unwrap();
        registry.register(Box::new(misses.clone())).unwrap();
        registry.register(Box::new(sets.clone())).unwrap();
        registry.register(Box::new(deletes.clone())).unwrap();
        registry.register(Box::new(evictions.clone())).unwrap();
        registry.register(Box::new(size.clone())).unwrap();
        registry.register(Box::new(latency.clone())).unwrap();

        Self {
            hits,
            misses,
            sets,
            deletes,
            evictions,
            size,
            latency,
        }
    }

    pub fn record_hit(&self) {
        self.hits.inc();
    }

    pub fn record_miss(&self) {
        self.misses.inc();
    }

    pub fn record_set(&self) {
        self.sets.inc();
    }

    pub fn record_delete(&self) {
        self.deletes.inc();
    }

    pub fn record_eviction(&self) {
        self.evictions.inc();
    }

    pub fn set_size(&self, size: f64) {
        self.size.set(size);
    }

    pub fn observe_latency(&self, operation: &str, duration: std::time::Duration) {
        self.latency
            .with_label_values(&[operation])
            .observe(duration.as_secs_f64());
    }
}
