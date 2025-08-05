use opentelemetry::trace::{TraceContextExt, Tracer};
use opentelemetry_sdk as sdk;

pub(crate) fn init_tracing() -> sdk::trace::SdkTracerProvider {
    let mut builder = sdk::trace::TracerProviderBuilder::default();

    let export_enabled: bool = std::env::var("OTEX_EXPORT")
        .map(|s| s.to_lowercase())
        .unwrap_or("true".to_string())
        .parse()
        .unwrap_or(true);

    if export_enabled {
        let exporter = opentelemetry_otlp::SpanExporterBuilder::default()
            .with_http()
            .build()
            .expect("failed to build exporter");

        builder = builder.with_batch_exporter(exporter);
    }

    #[cfg(feature = "stdout")]
    {
        let stdout_exporter = opentelemetry_stdout::SpanExporter::default();
        builder = builder.with_batch_exporter(stdout_exporter);
    }

    builder.build()
}

/// Creates a new span with the current context as its parent
pub fn new_span(
    name: &str,
    kind: opentelemetry::trace::SpanKind,
    attributes: &[opentelemetry::KeyValue],
) -> opentelemetry::Context {
    let tracer = crate::init::tracer();
    let span_builder = tracer
        .span_builder(name.to_string())
        .with_attributes(attributes.to_owned());


    let span = tracer.build(span_builder);
    opentelemetry::Context::current_with_span(span)
}

pub fn new_event(name: &str, attributes: &[opentelemetry::KeyValue]) {
    let context = opentelemetry::Context::current();
    context
        .span()
        .add_event(name.to_string(), attributes.to_vec());
}

pub fn new_error_event(name: &str, description: &str, attributes: &[opentelemetry::KeyValue]) {
    let context = opentelemetry::Context::current();
    context
        .span()
        .add_event(name.to_string(), attributes.to_vec());

    context.span().set_status(opentelemetry::trace::Status::error(description.to_string()));
}

#[cfg(test)]
mod test {

    use crate::{FutureExt, event, context};

    #[test]
    fn span_macro() {
        let otex = crate::init();
        {
            let parent = context!("hello", crate::trace::SpanKind::Internal, test_attr = "value");
            event!("parent event");

            let child_attr = 123;
            let mut child = context!("world", crate::trace::SpanKind::Internal, child_attr);

            event!("child event");
        }
        otex.shutdown();
    }

    #[tokio::test]
    async fn async_span() {
        let otex = crate::init();
        {
            let _parent = context!("hello", crate::trace::SpanKind::Internal, test_attr = "value").attach();
            event!("parent event");

            let child_attr = 123;
            let child = context!("world", crate::trace::SpanKind::Internal, child_attr);

            let task = async {
                event!("child event", child_name = "childevent");
            }
            .with_context(child);

            tokio::spawn(task).await.unwrap();
        }
        otex.shutdown();
    }
}
