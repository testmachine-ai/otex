//! W3C Trace Context propagation utilities.
//!
//! This module provides functions to extract and inject trace context
//! from/to HTTP headers using the W3C Trace Context format.

use http::HeaderMap;
use opentelemetry::{
    trace::{SpanContext, SpanId, TraceContextExt, TraceFlags, TraceId, TraceState},
    Context,
};

/// Extract trace context from HTTP headers.
///
/// Parses the `traceparent` header according to W3C Trace Context specification.
/// Returns the parent context if valid trace context is found, otherwise returns
/// an empty context (which will start a new trace).
///
/// # Example
/// ```ignore
/// let parent_context = otex::propagation::extract_context_from_headers(request.headers());
/// let span_context = otex::new_span_with_parent("my_span", SpanKind::Server, &[], parent_context);
/// ```
pub fn extract_context_from_headers(headers: &HeaderMap) -> Context {
    let traceparent = match headers.get("traceparent") {
        Some(value) => match value.to_str() {
            Ok(s) => s,
            Err(_) => return Context::new(),
        },
        None => return Context::new(),
    };

    match parse_traceparent(traceparent) {
        Some(span_context) => Context::new().with_remote_span_context(span_context),
        None => Context::new(),
    }
}

/// Inject trace context into HTTP headers.
///
/// Writes the `traceparent` header from the current span context.
/// If there is no active span, no headers are modified.
///
/// # Example
/// ```ignore
/// let mut headers = HeaderMap::new();
/// otex::propagation::inject_context_to_headers(&mut headers);
/// // headers now contains traceparent
/// ```
pub fn inject_context_to_headers(headers: &mut HeaderMap) {
    let context = Context::current();
    let span = context.span();
    let span_context = span.span_context();

    if span_context.is_valid() {
        let traceparent = format_traceparent(span_context);
        if let Ok(value) = traceparent.parse() {
            headers.insert("traceparent", value);
        }
    }
}

/// Inject a specific context into HTTP headers.
///
/// Writes the `traceparent` header from the provided context's span.
pub fn inject_context_to_headers_with_context(headers: &mut HeaderMap, context: &Context) {
    let span = context.span();
    let span_context = span.span_context();

    if span_context.is_valid() {
        let traceparent = format_traceparent(span_context);
        if let Ok(value) = traceparent.parse() {
            headers.insert("traceparent", value);
        }
    }
}

/// Parse a W3C traceparent header value.
///
/// Format: `{version}-{trace-id}-{parent-id}-{trace-flags}`
/// Example: `00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01`
fn parse_traceparent(value: &str) -> Option<SpanContext> {
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() != 4 {
        return None;
    }

    let version = parts[0];
    let trace_id_hex = parts[1];
    let span_id_hex = parts[2];
    let flags_hex = parts[3];

    // Only support version 00
    if version != "00" {
        return None;
    }

    // Parse trace ID (32 hex chars = 16 bytes)
    if trace_id_hex.len() != 32 {
        return None;
    }
    let trace_id_bytes: [u8; 16] = hex_to_bytes(trace_id_hex)?;
    let trace_id = TraceId::from_bytes(trace_id_bytes);

    // Parse span ID (16 hex chars = 8 bytes)
    if span_id_hex.len() != 16 {
        return None;
    }
    let span_id_bytes: [u8; 8] = hex_to_bytes(span_id_hex)?;
    let span_id = SpanId::from_bytes(span_id_bytes);

    // Parse flags (2 hex chars = 1 byte)
    if flags_hex.len() != 2 {
        return None;
    }
    let flags_byte = u8::from_str_radix(flags_hex, 16).ok()?;
    let trace_flags = TraceFlags::new(flags_byte);

    Some(SpanContext::new(
        trace_id,
        span_id,
        trace_flags,
        true, // is_remote = true since it came from headers
        TraceState::default(),
    ))
}

/// Format a SpanContext as a W3C traceparent header value.
fn format_traceparent(span_context: &SpanContext) -> String {
    format!(
        "00-{}-{}-{:02x}",
        hex::encode(&span_context.trace_id().to_bytes()),
        hex::encode(&span_context.span_id().to_bytes()),
        span_context.trace_flags().to_u8()
    )
}

/// Convert a hex string to a fixed-size byte array.
fn hex_to_bytes<const N: usize>(hex: &str) -> Option<[u8; N]> {
    if hex.len() != N * 2 {
        return None;
    }

    let mut bytes = [0u8; N];
    for i in 0..N {
        bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(bytes)
}

/// Helper module for hex encoding (inline implementation to avoid extra dependency)
mod hex {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

    pub fn encode(bytes: &[u8]) -> String {
        let mut result = String::with_capacity(bytes.len() * 2);
        for &byte in bytes {
            result.push(HEX_CHARS[(byte >> 4) as usize] as char);
            result.push(HEX_CHARS[(byte & 0x0f) as usize] as char);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_traceparent() {
        let traceparent = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01";
        let span_context = parse_traceparent(traceparent).unwrap();

        assert!(span_context.is_valid());
        assert!(span_context.is_remote());
        assert_eq!(span_context.trace_flags(), TraceFlags::SAMPLED);
    }

    #[test]
    fn test_parse_invalid_traceparent() {
        // Wrong version
        assert!(parse_traceparent("01-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01").is_none());

        // Wrong format
        assert!(parse_traceparent("invalid").is_none());

        // Missing parts
        assert!(parse_traceparent("00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331").is_none());
    }

    #[test]
    fn test_extract_no_header() {
        let headers = HeaderMap::new();
        let context = extract_context_from_headers(&headers);
        // Should return empty context (no span)
        assert!(!context.span().span_context().is_valid());
    }

    #[test]
    fn test_extract_valid_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "traceparent",
            "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01".parse().unwrap(),
        );

        let context = extract_context_from_headers(&headers);
        let span_context = context.span().span_context();

        assert!(span_context.is_valid());
        assert!(span_context.is_remote());
    }

    #[test]
    fn test_roundtrip() {
        // Parse a traceparent
        let original = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01";
        let span_context = parse_traceparent(original).unwrap();

        // Format it back
        let formatted = format_traceparent(&span_context);

        assert_eq!(original, formatted);
    }
}
