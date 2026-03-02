use axum_prometheus::{
    metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle},
    PrometheusMetricLayer,
};
use once_cell::sync::Lazy;

static METRICS: Lazy<(PrometheusMetricLayer<'static>, PrometheusHandle)> = Lazy::new(|| {
    axum_prometheus::PrometheusMetricLayerBuilder::new()
        .with_prefix("slowpokeapi")
        .with_metrics_from_fn(|| {
            PrometheusBuilder::new()
                .install_recorder()
                .expect("Failed to install recorder")
        })
        .build_pair()
});

pub fn get_layer() -> PrometheusMetricLayer<'static> {
    METRICS.0.clone()
}

pub fn metrics_handler() -> String {
    METRICS.1.render()
}

pub static PROMETHEUS_LAYER: Lazy<PrometheusMetricLayer<'static>> = Lazy::new(get_layer);
