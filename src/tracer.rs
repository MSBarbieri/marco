use cfg_if::cfg_if;
use thiserror::Error;

use crate::cli::Cli;
cfg_if! {
if #[cfg(feature = "trace")] {
use opentelemetry::sdk::trace::Tracer;
use tracing::{subscriber::SetGlobalDefaultError, Subscriber};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;
use opentelemetry::{
    sdk::{propagation::TraceContextPropagator, trace::Sampler, Resource},
    trace::TraceError,
};

type BoxError = Box<dyn std::error::Error + Send + Sync>;
}
}

#[derive(Error, Debug)]
pub enum TracingError {
    #[cfg(feature = "trace")]
    #[error("Failed to connect with opentelemetry '{0}'")]
    ConnectionFailed(#[from] BoxError),
    #[cfg(feature = "trace")]
    #[error("Failed to set global output'{0}'")]
    SetGlobalDefaultError(#[from] SetGlobalDefaultError),
}

cfg_if! {
if #[cfg(feature = "trace")] {
pub fn build_logger_text<S>() -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    Box::new(
        tracing_subscriber::fmt::layer()
            .json()
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .with_timer(tracing_subscriber::fmt::time::uptime()),
    )
}

pub fn build_loglevel_filter_layer(cli: &Cli) -> tracing_subscriber::filter::EnvFilter {
    // filter what is output on log (fmt)
    // std::env::set_var("RUST_LOG", "warn,axum_tracing_opentelemetry=info,otel=debug");
    let log_level = cli.log_level.to_string();
    std::env::set_var(
        "RUST_LOG",
        format!(
            // `axum_tracing_opentelemetry` should be a level info to emit opentelemetry trace & span
            // `otel::setup` set to debug to log detected resources, configuration read and infered
            "{},axum_tracing_opentelemetry=info,otel=debug",
            log_level
        ),
    );
    EnvFilter::from_default_env()
}

fn infer_protocol_and_endpoint(
    (maybe_protocol, maybe_endpoint): (Option<String>, Option<String>),
) -> (String, String) {
    let protocol = maybe_protocol.unwrap_or_else(|| {
        match maybe_endpoint
            .as_ref()
            .map(|e| e.contains(":4317"))
            .unwrap_or(false)
        {
            true => "grpc",
            false => "http/protobuf",
        }
        .to_string()
    });

    let endpoint = match protocol.as_str() {
        "http/protobuf" => maybe_endpoint.unwrap_or_else(|| "http://localhost:4318".to_string()), //Devskim: ignore DS137138
        _ => maybe_endpoint.unwrap_or_else(|| "http://localhost:4317".to_string()), //Devskim: ignore DS137138
    };

    (protocol, endpoint)
}

pub fn init_tracer<F>(cli: &Cli, resource: Resource, transform: F) -> Result<Tracer, TraceError>
where
    F: FnOnce(opentelemetry_otlp::OtlpTracePipeline) -> opentelemetry_otlp::OtlpTracePipeline,
{
    use opentelemetry_otlp::SpanExporterBuilder;
    use opentelemetry_otlp::WithExportConfig;

    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
    let (protocol, endpoint) = infer_protocol_and_endpoint((None, Some(cli.otlp_url.clone())));
    tracing::debug!(target: "otel::setup", OTEL_EXPORTER_OTLP_ENDPOINT = endpoint);
    tracing::debug!(target: "otel::setup", OTEL_EXPORTER_OTLP_PROTOCOL = protocol);
    let exporter: SpanExporterBuilder = match protocol.as_str() {
        "http/protobuf" => opentelemetry_otlp::new_exporter()
            .http()
            .with_endpoint(endpoint)
            .into(),
        _ => opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(endpoint)
            .into(),
    };
    println!("exporter: {:?}", exporter);

    let mut pipeline = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            opentelemetry::sdk::trace::config()
                .with_resource(resource)
                .with_sampler(Sampler::AlwaysOn),
        );
    pipeline = transform(pipeline);
    pipeline.install_batch(opentelemetry::runtime::Tokio)
}

pub fn build_otel_layer<S>(cli: &Cli) -> Result<OpenTelemetryLayer<S, Tracer>, BoxError>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    use axum_tracing_opentelemetry::{init_propagator, otlp, resource::DetectResource};
    let otel_rsrc = DetectResource::default().build();
    let otel_tracer = init_tracer(cli, otel_rsrc, otlp::identity)?;
    init_propagator()?;
    Ok(tracing_opentelemetry::layer().with_tracer(otel_tracer))
}
}
}
pub fn setup_tracing(cli: &Cli) -> Result<(), TracingError> {
    cfg_if::cfg_if! {
    if #[cfg(feature = "trace")] {
    let subscriber = tracing_subscriber::registry()
        .with(build_loglevel_filter_layer(cli))
        .with(build_logger_text());
    let _guard = tracing::subscriber::set_default(subscriber);
    tracing::info!("init logging & tracing");

    let subscriber = tracing_subscriber::registry()
        .with(build_otel_layer(cli)?)
        .with(build_loglevel_filter_layer(cli))
        .with(build_logger_text());
    tracing::subscriber::set_global_default(subscriber)?;
    } else {
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Into::<tracing::Level>::into(cli.log_level.clone()))
        .init();
    }
    }
    Ok(())
}
