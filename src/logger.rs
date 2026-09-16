use opentelemetry::logs::{LogRecord, Logger};
use opentelemetry_sdk::{self as sdk};

use crate::schema::{self, Outcome};

pub(crate) fn init_logging() -> sdk::logs::SdkLoggerProvider {
    let mut builder = sdk::logs::LoggerProviderBuilder::default();

    let export_enabled = crate::export::otlp_enabled();

    if export_enabled {
        let exporter = opentelemetry_otlp::LogExporterBuilder::default()
            .with_http()
            .build()
            .expect("failed to build exporter");

        builder = builder.with_batch_exporter(exporter);
    }

    #[cfg(feature = "stdout")]
    if !export_enabled {
        let stdout_exporter = opentelemetry_stdout::LogExporter::default();
        builder = builder.with_batch_exporter(stdout_exporter);
    }

    builder.build()
}

pub fn create_log_record(
    severity: opentelemetry::logs::Severity,
    _module_path: &'static str,
    name: Option<&'static str>,
    body: Option<opentelemetry::logs::AnyValue>,
    attributes: &[(opentelemetry::Key, opentelemetry::logs::AnyValue)],
) {
    let logger = crate::init::logger();

    let mut record = logger.create_log_record();

    let mut record_attributes = Vec::with_capacity(attributes.len() + 1);

    if let Some(name) = name {
        record.set_event_name(name);
        record_attributes.push((
            opentelemetry::Key::new(schema::keys::EVENT_NAME),
            opentelemetry::logs::AnyValue::String(name.into()),
        ));
    }

    if let Some(body) = &body {
        record.set_body(body.clone());
    }

    record_attributes.extend(attributes.iter().filter_map(|(key, value)| {
        let owned_by_library = name.is_some() && key.as_str() == schema::keys::EVENT_NAME;
        (!owned_by_library).then(|| (key.clone(), value.clone()))
    }));
    record.add_attributes(record_attributes.clone());

    record.set_severity_number(severity);
    record.set_severity_text(severity.name());

    // Emit otel record
    logger.emit(record);
}

/// Emit a schema-v1 actionable event.
///
/// The event name must be a stable, low-cardinality identifier. Human-readable
/// detail belongs in `body`; queryable context belongs in `attributes`.
#[track_caller]
pub fn create_actionable_log_record(
    severity: opentelemetry::logs::Severity,
    module_path: &'static str,
    name: &'static str,
    outcome: Outcome,
    body: Option<opentelemetry::logs::AnyValue>,
    attributes: &[(opentelemetry::Key, opentelemetry::logs::AnyValue)],
) {
    let mut actionable_attributes = Vec::with_capacity(attributes.len() + 2);
    actionable_attributes.push((
        opentelemetry::Key::new(schema::keys::SCHEMA_VERSION),
        opentelemetry::logs::AnyValue::Int(schema::VERSION),
    ));
    actionable_attributes.push((
        opentelemetry::Key::new(schema::keys::EVENT_OUTCOME),
        opentelemetry::logs::AnyValue::String(outcome.as_str().into()),
    ));
    actionable_attributes.extend(attributes.iter().filter_map(|(key, value)| {
        (!matches!(
            key.as_str(),
            schema::keys::SCHEMA_VERSION | schema::keys::EVENT_OUTCOME
        ))
        .then(|| (key.clone(), value.clone()))
    }));

    create_log_record(
        severity,
        module_path,
        Some(name),
        body,
        &actionable_attributes,
    );
}

#[cfg(test)]
mod test {

    #[test]
    fn test_log_provider() {
        let logger = env_logger::Builder::from_default_env().build();
        log::set_max_level(logger.filter());

        log::set_boxed_logger(Box::new(logger));

        let otex = crate::init();

        let span = crate::context!("test", value = "attach").attach();

        crate::event!("test", attr = "name");
        crate::log!(
            Some("test log"),
            opentelemetry::logs::Severity::Error,
            "error!",
            test_key = "hello"
        );

        otex.shutdown();
    }

    #[test]
    fn test_info() {
        let logger = env_logger::Builder::from_default_env().build();
        log::set_max_level(logger.filter());
        log::set_boxed_logger(Box::new(logger));

        let otex = crate::init();

        crate::info_log!("test log", "test!");

        crate::info_log!("test log", "test!", key = 1);

        crate::info_log!("test!");

        otex.shutdown();
    }
}
