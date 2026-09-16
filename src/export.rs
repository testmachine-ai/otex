pub(crate) fn otlp_enabled() -> bool {
    std::env::var("OTEX_EXPORT")
        .map(|value| value.to_lowercase())
        .unwrap_or_else(|_| "true".to_string())
        .parse()
        .unwrap_or(true)
}
