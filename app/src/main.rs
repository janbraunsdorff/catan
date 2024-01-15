use std::time::Duration;

use axum::Error;

use opentelemetry::logs::LogError;
use opentelemetry::{global, KeyValue};
use opentelemetry::trace::TraceError;
use opentelemetry_otlp::{WithExportConfig, ExportConfig};
use opentelemetry_sdk::logs::Config;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::resource::{OsResourceDetector, ProcessResourceDetector, SdkProvidedResourceDetector, EnvResourceDetector, TelemetryResourceDetector, ResourceDetector};
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::trace::config;
use tokio::net::TcpListener;

use opentelemetry_sdk::{metrics::MeterProvider, Resource};
use tracing::level_filters::LevelFilter;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Registry, Layer};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:4000").await.unwrap();

    let _ = init_tracer_subscriber();

    axum::serve(listener, api::create_main_rounter())
        .await
        .unwrap();

    Ok(())
}

fn init_tracer_subscriber(){
   // trace: Single Request
    let layer_logging = tracing_subscriber::fmt::layer()
        .with_target(false)
        .log_internal_errors(true)
        .with_line_number(false)
        .with_file(false)
        //.json()
        .with_filter(LevelFilter::INFO);

    let tracer = init_tracer().unwrap();
    let otel_layer = OpenTelemetryLayer::new(tracer).with_filter(LevelFilter::INFO);

    let subscriber = Registry::default()
        .with(layer_logging)
        .with(otel_layer);

    tracing::subscriber::set_global_default(subscriber).expect("unable to set global subscriber");

}
fn init_tracer() -> Result<opentelemetry_sdk::trace::Tracer, TraceError> {
   // trace: Single Request

    global::set_text_map_propagator(TraceContextPropagator::new());
    let os_resource = OsResourceDetector.detect(Duration::from_secs(0));
    let process_resource = ProcessResourceDetector.detect(Duration::from_secs(0));
    let sdk_resource = SdkProvidedResourceDetector.detect(Duration::from_secs(0));
    let env_resource = EnvResourceDetector::new().detect(Duration::from_secs(0));
    let telemetry_resource = TelemetryResourceDetector.detect(Duration::from_secs(0));
    let service_name = Resource::new(vec![KeyValue::new(
        opentelemetry_semantic_conventions::resource::SERVICE_NAME,
        "basic-otlp-service",
    )]);

    let exporter =  opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://localhost:4317".to_string());

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            config().with_resource(
                os_resource.merge(&process_resource)
                .merge(&sdk_resource)
                .merge(&env_resource)
                .merge(&telemetry_resource)
                .merge(&service_name)
            ),
        )
        .install_batch(runtime::Tokio);

    tracer
}

fn init_metrics() -> opentelemetry::metrics::Result<MeterProvider> {
    let export_config = ExportConfig {
        endpoint: "http://localhost:4317".to_string(),
        ..ExportConfig::default()
    };
    opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
        )
        .with_resource(Resource::new(vec![KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            "basic-otlp-metrics-example",
        )]))
        .build()
}


fn init_logs() -> Result<opentelemetry_sdk::logs::Logger, LogError> {
    let service_name = env!("CARGO_BIN_NAME");
    opentelemetry_otlp::new_pipeline()
        .logging()
        .with_log_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                service_name,
            )])),
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .install_batch(runtime::Tokio)
}