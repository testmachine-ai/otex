use opentelemetry_sdk::{self as sdk};

pub(crate) fn init_metrics() -> sdk::metrics::SdkMeterProvider {
    let exporter = opentelemetry_otlp::MetricExporterBuilder::new()
        .with_http()
        .build()
        .expect("failed to build exporter");

    let builder = sdk::metrics::MeterProviderBuilder::default()
        .with_periodic_exporter(exporter);

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
        let otex = crate::init(None);

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
            hist.record(5, attributes);

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
