use once_cell::sync::Lazy;
use prometheus::{Counter, HistogramVec, Opts};

pub static UPSTREAM_METRICS: Lazy<UpstreamMetrics> = Lazy::new(UpstreamMetrics::new);

pub struct UpstreamMetrics {
    requests_total: Counter,
    errors_total: Counter,
    latency: HistogramVec,
}

impl UpstreamMetrics {
    fn new() -> Self {
        let requests_total = Counter::with_opts(Opts::new(
            "slowpokeapi_upstream_requests_total",
            "Total number of upstream API requests",
        ))
        .unwrap();

        let errors_total = Counter::with_opts(Opts::new(
            "slowpokeapi_upstream_errors_total",
            "Total number of upstream API errors",
        ))
        .unwrap();

        let latency = HistogramVec::new(
            prometheus::histogram_opts!(
                "slowpokeapi_upstream_latency_seconds",
                "Upstream API request latency in seconds",
                vec![0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
            ),
            &["upstream"],
        )
        .unwrap();

        let registry = prometheus::default_registry();
        registry.register(Box::new(requests_total.clone())).unwrap();
        registry.register(Box::new(errors_total.clone())).unwrap();
        registry.register(Box::new(latency.clone())).unwrap();

        Self {
            requests_total,
            errors_total,
            latency,
        }
    }

    pub fn record_request(&self, _upstream: &str) {
        self.requests_total.inc_by(1.0);
    }

    pub fn record_error(&self, _upstream: &str) {
        self.errors_total.inc_by(1.0);
    }

    pub fn observe_latency(&self, upstream: &str, duration: std::time::Duration) {
        self.latency
            .with_label_values(&[upstream])
            .observe(duration.as_secs_f64());
    }
}
