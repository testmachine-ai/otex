use opentelemetry_sdk::{self as sdk};

pub(crate) fn init_metrics() -> sdk::metrics::SdkMeterProvider {
    let mut builder = sdk::metrics::MeterProviderBuilder::default();

    let export_enabled: bool = std::env::var("OTEX_EXPORT")
        .map(|s| s.to_lowercase())
        .unwrap_or("true".to_string())
        .parse()
        .unwrap_or(true);

    if export_enabled {
        let exporter = opentelemetry_otlp::MetricExporterBuilder::new()
            .with_tonic()
            .build()
            .expect("failed to build exporter");

        builder = builder.with_periodic_exporter(exporter);
    }

    #[cfg(feature = "stdout")]
    {
        let stdout_exporter = opentelemetry_stdout::MetricExporter::default();
        builder = builder.with_periodic_exporter(stdout_exporter);
    }

    builder.build()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_meter() {
        let otex = crate::init();

        {
            let meter = crate::init::meter();
            let counter = meter
                .u64_counter("test")
                .with_unit("mb")
                .with_description("test up")
                .build();
            let hist = meter
                .u64_histogram("test")
                .with_unit("mb")
                .with_description("test up")
                .build();

            let attributes = crate::kvset!(testas = 100);
            hist.record(5, &attributes);

            counter.add(1, &[]);
        }
        {
            let meter = crate::init::meter();
            let counter = meter
                .u64_counter("test")
                .with_unit("mb")
                .with_description("test up")
                .build();

            counter.add(1, &[]);
        }

        otex.shutdown()
    }
}
