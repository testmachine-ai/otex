mod logging;
mod macros;
mod metrics;
mod tracing;

pub use opentelemetry::trace::FutureExt;

pub(crate) mod init {

    use opentelemetry::trace::{TraceContextExt, TracerProvider};
    use opentelemetry::metrics::{MeterProvider, Meter};
    use opentelemetry::logs::LoggerProvider;
    use opentelemetry_sdk::{self as sdk};
    use std::sync::OnceLock;

    use lazy_static::lazy_static;

    const APPLICATION_NAME: &str = "otex";
    lazy_static! {

        pub static ref TRACER_PROVIDER: OnceLock<sdk::trace::SdkTracerProvider> = OnceLock::new();
        pub static ref LOGGER_PROVIDER: OnceLock<sdk::logs::SdkLoggerProvider> = OnceLock::new();
        pub static ref METER_PROVIDER: OnceLock<sdk::metrics::SdkMeterProvider> = OnceLock::new();
    }

    pub struct Otex;

    impl Otex {
        pub fn shutdown(&self) {
            opentelemetry::context::Context::current().span().end();
            // Flush otel traces
            let tracer_provider = TRACER_PROVIDER.get().unwrap();
            let _ = tracer_provider.force_flush().inspect_err(|e| {
                log::error!("{}", e)
            });
            tracer_provider
                .shutdown()
                .expect("shutdown errors");


            // Flush otel logs
            let logger_provider = LOGGER_PROVIDER.get().unwrap();
            let _ = logger_provider.force_flush().inspect_err(|e| {
                log::error!("{}", e)
            });
            logger_provider
                .shutdown()
                .expect("shutdown errors");



            // Flush log implementation
            log::logger().flush();


            // Flush otel metrics
            let meter_provider = METER_PROVIDER.get().unwrap();
            let _ = meter_provider.force_flush().inspect_err(|e| {
                log::error!("{}", e)
            });
            meter_provider
                .shutdown()
                .expect("shutdown errors");

        }
    }

    pub fn init(log_impl: Option<Box<dyn log::Log>>) -> Otex {
        let trace_provider = crate::tracing::init_tracing();
        crate::init::TRACER_PROVIDER
            .set(trace_provider)
            .expect("failed to set tracer provider");

        let log_provider = crate::logging::init_logging(log_impl);
        crate::init::LOGGER_PROVIDER
            .set(log_provider)
            .expect("failed to set logger provider");

        let meter_provider = crate::metrics::init_metrics();
        crate::init::METER_PROVIDER
            .set(meter_provider)
            .expect("failed to set logger provider");

        Otex
    }

    pub fn app_name() -> &'static str {
        APPLICATION_NAME
    }

    pub fn tracer() -> sdk::trace::SdkTracer {
        TRACER_PROVIDER
            .get()
            .expect("application not initialized")
            .tracer(app_name())
    }

    pub fn logger() -> sdk::logs::SdkLogger {
        LOGGER_PROVIDER
            .get()
            .expect("application not initialized")
            .logger(app_name())
    }

    pub fn meter() -> Meter {
        METER_PROVIDER
            .get()
            .expect("application not initialized")
            .meter(app_name())
    }
}

pub use init::{init, meter};
