use std::{fmt::Arguments, panic::Location};

use opentelemetry::logs::{LogRecord, Logger};
use opentelemetry_sdk::{self as sdk};

pub(crate) fn init_logging(logger: Option<Box<dyn log::Log>>) -> sdk::logs::SdkLoggerProvider {
    if let Some(logger) = logger {
        log::set_boxed_logger(logger).expect("failed to set global logger");
    }

    let mut builder = sdk::logs::LoggerProviderBuilder::default();

    #[cfg(not(feature = "stdout"))]
    {
        let exporter = opentelemetry_otlp::LogExporterBuilder::default()
            .with_http()
            .build()
            .expect("failed to build exporter");

        builder = builder.with_batch_exporter(exporter);
    }

    #[cfg(feature = "stdout")]
    {
        let stdout_exporter = opentelemetry_stdout::LogExporter::default();
        builder = builder.with_batch_exporter(stdout_exporter);
    }

    builder.build()
}

pub(crate) fn create_log_record(
    severity: opentelemetry::logs::Severity,
    module_path: &'static str,
    name: Option<&'static str>,
    body: Option<opentelemetry::logs::AnyValue>,
    attributes: &[(opentelemetry::Key, opentelemetry::logs::AnyValue)],
) {
    let logger = crate::init::logger();

    let location = Location::caller();
    let mut record = logger.create_log_record();

    if let Some(name) = name {
        record.set_event_name(name);
    }

    if let Some(body) = &body {
        record.set_body(body.clone());
    }

    record.add_attributes(attributes.to_owned());

    record.set_severity_number(severity);

    // Emit otel record
    logger.emit(record);

    // Emit log impl record
    let log_attributes = attributes
        .iter()
        .map(|(key, value)| {
            (
                log::kv::Key::from_str(key.as_str()),
                log::kv::Value::from_debug(value),
            )
        })
        .collect::<Vec<(_, _)>>();

    if let Some(body) = body.as_ref() {
        use opentelemetry::logs::AnyValue;
        let formatted_body = match body {
            AnyValue::Int(i) => format!("{}", i),
            AnyValue::Double(d) => format!("{}", d),
            AnyValue::String(string_value) => format!("{}", string_value),
            AnyValue::Boolean(b) => format!("{}", b),
            AnyValue::Bytes(vec) => format!("{:?}", vec),
            AnyValue::ListAny(list) => format!("{:?}", list),
            AnyValue::Map(map) => format!("{:?}", map),
            _ => todo!(),
        };
        emit_log_impl_record(
            severity,
            module_path,
            location,
            &log_attributes,
            &format_args!("{}", formatted_body),
        );
    }
}

fn emit_log_impl_record<'a>(
    severity: opentelemetry::logs::Severity,
    module_path: &'static str,
    location: &'a Location<'a>,
    attributes: &'a [(log::kv::Key, log::kv::Value)],
    arguments: &'a Arguments<'a>,
) {
    // Emit log impl record
    let mut log_builder = log::RecordBuilder::new();
    log_builder
        .module_path(Some(module_path))
        .file(Some(location.file()))
        .line(Some(location.line()));

    log_builder.key_values(&attributes);

    use opentelemetry::logs::Severity;
    let log_level = match severity {
        Severity::Trace | Severity::Trace2 | Severity::Trace3 | Severity::Trace4 => {
            log::Level::Trace
        }
        Severity::Debug | Severity::Debug2 | Severity::Debug3 | Severity::Debug4 => {
            log::Level::Debug
        }
        Severity::Info | Severity::Info2 | Severity::Info3 | Severity::Info4 => log::Level::Info,
        Severity::Warn | Severity::Warn2 | Severity::Warn3 | Severity::Warn4 => log::Level::Warn,
        Severity::Error | Severity::Error2 | Severity::Error3 | Severity::Error4 => {
            log::Level::Error
        }
        Severity::Fatal | Severity::Fatal2 | Severity::Fatal3 | Severity::Fatal4 => {
            log::Level::Error
        }
    };

    log_builder.level(log_level);

    log_builder.args(*arguments); // Borrow is safe — body_str lives long enough
    let log_record = log_builder.build();
    log::logger().log(&log_record);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_log_provider() {
        let logger = env_logger::Builder::from_default_env().build();
        log::set_max_level(logger.filter());

        let otex = crate::init(Some(Box::new(logger)));

        let span = crate::span!("test", value = "attach").attach();

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

        let otex = crate::init(Some(Box::new(logger)));

        crate::info_log!(
            "test log",
            "test!"
        );

        crate::info_log!(
            "test log",
            "test!",
            key=1
        );

        crate::info_log!("test!");

        otex.shutdown();
    }
}
