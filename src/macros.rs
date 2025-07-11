// GENERAL
#[macro_export]
macro_rules! kvset {
    // dot-separated key = value form (e.g., work_order.request_id = value)
    ($( $($attr_key_part:ident).+ = $attr_value:expr ),+ $(,)?) => {{
        use $crate::KeyValue;
        [
            $( KeyValue::new(stringify!($($attr_key_part).+), $attr_value) ),*
        ]
    }};

    // key = value form
    ($( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        use $crate::KeyValue;
        [
            $( KeyValue::new(stringify!($attr_key), $attr_value) ),*
        ]
    }};

    // shorthand: just ident
    ($( $attr:ident ),+ $(,)?) => {{
        use $crate::KeyValue;
        [
            $( KeyValue::new(stringify!($attr), $attr) ),*
        ]
    }};
}

#[macro_export]
macro_rules! anykvset {
    // dot-separated key = value form (e.g., work_order.request_id = value)
    ($( $($attr_key_part:ident).+ = $attr_value:expr ),+ $(,)?) => {{
        use $crate::{Key, logs::AnyValue};
        [
            $( (Key::new(stringify!($($attr_key_part).+)), AnyValue::from($attr_value)) ),*
        ]
    }};

    // key = value form
    ($( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        use $crate::{Key, logs::AnyValue};
        [
            $( (Key::new(stringify!($attr_key)), AnyValue::from($attr_value)) ),*
        ]
    }};

    // shorthand: just ident
    ($( $attr:ident ),+ $(,)?) => {{
        use $crate::{Key, logs::AnyValue};
        [
            $( (Key::new(stringify!($attr)), AnyValue::from($attr)) ),*
        ]
    }};
}
// LOGGING
#[macro_export]
macro_rules! log {
    // No attributes
    ($name:expr, $severity:expr, $body:expr) => {{
        $crate::create_log_record($severity, module_path!(), $name, Some($body.into()), &[])
    }};

    // key = value form
    ($name:expr, $severity:expr, $body:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        let attrs = $crate::anykvset!($( $attr_key = $attr_value ),*);
        $crate::create_log_record($severity, module_path!(), $name, Some($body.into()), &attrs)
    }};

    // shorthand: ident only
    ($name:expr, $severity:expr, $body:expr, $( $attr:ident ),+ $(,)?) => {{
        let attrs = $crate::anykvset!($( $attr ),*);
        $crate::create_log_record($severity, module_path!(), $name, Some($body.into()), &attrs)
    }};
}
#[macro_export]
macro_rules! error_log {

    // anonymous forms
    ($body:expr) => {{
        $crate::log!(None, $crate::logs::Severity::Error, $body);
    }};

    // key = value form
    ($body:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        $crate::log!(None, $crate::logs::Severity::Error, $body, $( $attr_key = $attr_value ),*);
    }};

    // shorthand: ident only
    ($body:expr, $( $attr:ident ),+ $(,)?) => {{
        $crate::log!(None, $crate::logs::Severity::Error, $body, $( $attr ),*);
    }};
    // No attributes
    ($name:expr, $body:expr) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Error, $body);
    }};

    // key = value form
    ($name:expr, $body:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Error, $body, $( $attr_key = $attr_value ),*);
    }};

    // shorthand: ident only
    ($name:expr, $body:expr, $( $attr:ident ),+ $(,)?) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Error, $body, $( $attr ),*);
    }};

}

#[macro_export]
macro_rules! warn_log {
    // anonymous forms
    ($body:expr) => {{
        $crate::log!(None, $crate::logs::Severity::Warn, $body);
    }};

    // key = value form
    ($body:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        $crate::log!(None, $crate::logs::Severity::Warn, $body, $( $attr_key = $attr_value ),*);
    }};

    // shorthand: ident only
    ($body:expr, $( $attr:ident ),+ $(,)?) => {{
        $crate::log!(None, $crate::logs::Severity::Warn, $body, $( $attr ),*);
    }};

    // No attributes
    ($name:expr, $body:expr) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Warn, $body);
    }};

    // key = value form
    ($name:expr, $body:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Warn, $body, $( $attr_key = $attr_value ),*);
    }};

    // shorthand: ident only
    ($name:expr, $body:expr, $( $attr:ident ),+ $(,)?) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Warn, $body, $( $attr ),*);
    }};

}

#[macro_export]
macro_rules! info_log {
    // anonymous forms
    ($body:expr) => {{
        $crate::log!(None, $crate::logs::Severity::Info, $body);
    }};

    // key = value form
    ($body:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        $crate::log!(None, $crate::logs::Severity::Info, $body, $( $attr_key = $attr_value ),*);
    }};

    // shorthand: ident only
    ($body:expr, $( $attr:ident ),+ $(,)?) => {{
        $crate::log!(None, $crate::logs::Severity::Info, $body, $( $attr ),*);
    }};

    // No attributes
    ($name:expr, $body:expr) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Info, $body);
    }};

    // key = value form
    ($name:expr, $body:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Info, $body, $( $attr_key = $attr_value ),*);
    }};

    // shorthand: ident only
    ($name:expr, $body:expr, $( $attr:ident ),+ $(,)?) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Info, $body, $( $attr ),*);
    }};
}

#[macro_export]
macro_rules! debug_log {
    // anonymous forms
    ($body:expr) => {{
        $crate::log!(None, $crate::logs::Severity::Debug, $body);
    }};

    // key = value form
    ($body:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        $crate::log!(None, $crate::logs::Severity::Debug, $body, $( $attr_key = $attr_value ),*);
    }};

    // shorthand: ident only
    ($body:expr, $( $attr:ident ),+ $(,)?) => {{
        $crate::log!(None, $crate::logs::Severity::Debug, $body, $( $attr ),*);
    }};

    // No attributes
    ($name:expr, $body:expr) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Debug, $body);
    }};

    // key = value form
    ($name:expr, $body:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Debug, $body, $( $attr_key = $attr_value ),*);
    }};

    // shorthand: ident only
    ($name:expr, $body:expr, $( $attr:ident ),+ $(,)?) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Debug, $body, $( $attr ),*);
    }};
}

#[macro_export]
macro_rules! trace_log {
    // anonymous forms
    ($body:expr) => {{
        $crate::log!(None, $crate::logs::Severity::Trace, $body);
    }};

    // key = value form
    ($body:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        $crate::log!(None, $crate::logs::Severity::Trace, $body, $( $attr_key = $attr_value ),*);
    }};

    // shorthand: ident only
    ($body:expr, $( $attr:ident ),+ $(,)?) => {{
        $crate::log!(None, $crate::logs::Severity::Trace, $body, $( $attr ),*);
    }};

    // No attributes
    ($name:expr, $body:expr) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Trace, $body);
    }};

    // key = value form
    ($name:expr, $body:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Trace, $body, $( $attr_key = $attr_value ),*);
    }};

    // shorthand: ident only
    ($name:expr, $body:expr, $( $attr:ident ),+ $(,)?) => {{
        $crate::log!(Some($name), $crate::logs::Severity::Trace, $body, $( $attr ),*);
    }};
}

// TRACING
#[macro_export]
macro_rules! event {
    // No attributes
    ($name:expr) => {{
        $crate::new_event($name, &[])
    }};

    // key = value form
    ($name:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        let attrs = $crate::kvset!($( $attr_key = $attr_value ),*);
        $crate::new_event($name, &attrs)
    }};

    // shorthand: ident only
    ($name:expr, $( $attr:ident ),+ $(,)?) => {{
        let attrs = $crate::kvset!($( $attr ),*);
        $crate::new_event($name, &attrs)
    }};
}

#[macro_export]
macro_rules! error_event {
    // No attributes
    ($name:expr, $desc:expr) => {{
        $crate::new_error_event($name, $desc &[])
    }};

    // key = value form
    ($name:expr, $desc:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        let attrs = $crate::kvset!($( $attr_key = $attr_value ),*);
        $crate::new_error_event($name, &attrs)
    }};

    // shorthand: ident only
    ($name:expr, $desc:expr, $( $attr:ident ),+ $(,)?) => {{
        let attrs = $crate::kvset!($( $attr ),*);
        $crate::new_error_event($name, &attrs)
    }};
}

#[macro_export]
macro_rules! context {
    // No attributes
    ($name:expr) => {{
        $crate::new_span($name, $crate::trace::SpanKind::Internal, &[])
    }};

    // key = value form
    ($name:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        let attrs = $crate::kvset!($( $attr_key = $attr_value ),*);
        $crate::new_span($name, $crate::trace::SpanKind::Internal, &attrs)
    }};

    // shorthand: ident only
    ($name:expr, $( $attr:ident ),+ $(,)?) => {{
        let attrs = $crate::kvset!($( $attr ),*);
        $crate::new_span($name, $crate::trace::SpanKind::Internal, &attrs)
    }};
    // No attributes
    ($name:expr, $kind:expr) => {{
        $crate::new_span($name, $kind, &[])
    }};

    // key = value form
    ($name:expr, $kind:expr, $( $attr_key:tt = $attr_value:expr ),+ $(,)?) => {{
        let attrs = $crate::kvset!($( $attr_key = $attr_value ),*);
        $crate::new_span($name, $kind, &attrs)
    }};

    // shorthand: ident only
    ($name:expr, $kind:expr, $( $attr:tt ),+ $(,)?) => {{
        let attrs = $crate::kvset!($( $attr ),*);
        $crate::new_span($name, $kind, &attrs)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_kvset_macro_syntax() {
        // Test kvset macro with key = value form
        let _test1 = || {
            kvset!(component = "auth", version = "1.0");
        };
        let _test2 = || {
            kvset!(user_id = 123, session_id = "abc", active = true);
        };
        
        // Test kvset macro with shorthand form
        let _test3 = || {
            let component = "logging";
            let version = "2.0";
            kvset!(component, version);
        };
        
        // Test kvset with mixed types - separate calls since mixing styles isn't supported
        let _test4a = || {
            kvset!(service = "api", region = "us-east-1");
        };
        let _test4b = || {
            let count = 42;
            let enabled = false;
            kvset!(count, enabled);
        };
        
        // Test kvset with trailing comma
        let _test5 = || {
            kvset!(name = "test", id = 1,);
        };

        // Test kvset with dot-separated keys
        let _test6 = || {
            kvset!(work_order.request_id = "12345", user.profile.name = "John");
        };
        let _test7 = || {
            kvset!(service.api.version = "v2", database.connection.timeout = 30);
        };
        let _test8 = || {
            kvset!(app.config.debug = true, metrics.counter.requests = 100,);
        };
    }

    #[test]
    fn test_anykvset_macro_syntax() {
        // Test anykvset macro with key = value form
        let _test1 = || {
            anykvset!(level = "error", code = 500);
        };
        let _test2 = || {
            anykvset!(message = "test", timestamp = 1234567890, success = false);
        };
        
        // Test anykvset macro with shorthand form
        let _test3 = || {
            let level = "info";
            let code = 200;
            anykvset!(level, code);
        };
        
        // Test anykvset with mixed types - separate calls since mixing styles isn't supported
        let _test4a = || {
            anykvset!(operation = "query", cached = true);
        };
        let _test4b = || {
            let duration = 1.5;
            let retries = 3;
            anykvset!(duration, retries);
        };
        
        // Test anykvset with trailing comma
        let _test5 = || {
            anykvset!(key = "value", num = 42,);
        };

        // Test anykvset with dot-separated keys
        let _test6 = || {
            anykvset!(work_order.request_id = "67890", event.source.system = "billing");
        };
        let _test7 = || {
            anykvset!(trace.span.id = "abc123", log.level.severity = "warn");
        };
        let _test8 = || {
            anykvset!(http.request.method = "POST", response.status.code = 201,);
        };
    }

    #[test]
    fn test_base_log_macro_syntax() {
        // Test base log macro with no attributes
        let _test1 = || {
            log!(Some("test_log"), crate::logs::Severity::Info, "message");
        };
        let _test2 = || {
            log!(None, crate::logs::Severity::Error, "error occurred");
        };
        
        // Test base log macro with key = value attributes
        let _test3 = || {
            log!(Some("user_action"), crate::logs::Severity::Info, "action performed", user_id = 123, action = "login");
        };
        
        // Test base log macro with shorthand attributes
        let _test4 = || {
            let user_id = 456;
            let session_id = "session123";
            log!(Some("session_log"), crate::logs::Severity::Debug, "session info", user_id, session_id);
        };
        
        // Test with different severity levels
        let _test5 = || {
            log!(None, crate::logs::Severity::Trace, "trace message", component = "auth");
        };
        let _test6 = || {
            log!(Some("warning"), crate::logs::Severity::Warn, "warning message", level = "high");
        };
    }

    #[test]
    fn test_error_log_macro_syntax() {
        // Test anonymous forms
        let _test1 = || {
            error_log!("error message");
        };
        let _test2 = || {
            error_log!("database error", error_code = 500, table = "users");
        };
        let _test3 = || {
            let error_code = 404;
            let resource = "user";
            error_log!("resource not found", error_code, resource);
        };
        
        // Test named forms
        let _test4 = || {
            error_log!("database_error", "connection failed");
        };
        let _test5 = || {
            error_log!("auth_error", "invalid credentials", user_id = 123, attempt = 3);
        };
        let _test6 = || {
            let user_id = 789;
            let ip_address = "192.168.1.1";
            error_log!("login_failure", "login attempt failed", user_id, ip_address);
        };
        
        // Test with various data types
        let _test7 = || {
            error_log!("validation_error", "field validation failed", field = "email", required = true, length = 0);
        };
        let _test8 = || {
            let duration = 30.5;
            let timeout = true;
            error_log!("timeout_error", "operation timed out", duration, timeout);
        };
    }

    #[test]
    fn test_warn_log_macro_syntax() {
        // Test anonymous forms
        let _test1 = || {
            warn_log!("warning message");
        };
        let _test2 = || {
            warn_log!("high memory usage", usage_percent = 85, threshold = 80);
        };
        let _test3 = || {
            let usage_percent = 90;
            let service = "api";
            warn_log!("resource warning", usage_percent, service);
        };
        
        // Test named forms
        let _test4 = || {
            warn_log!("memory_warning", "memory usage high");
        };
        let _test5 = || {
            warn_log!("rate_limit_warning", "approaching rate limit", requests = 950, limit = 1000);
        };
        let _test6 = || {
            let requests = 980;
            let window = "1h";
            warn_log!("rate_limit", "rate limit warning", requests, window);
        };
    }

    #[test]
    fn test_info_log_macro_syntax() {
        // Test anonymous forms
        let _test1 = || {
            info_log!("info message");
        };
        let _test2 = || {
            info_log!("user logged in", user_id = 123, session_duration = 3600);
        };
        let _test3 = || {
            let user_id = 456;
            let action = "logout";
            info_log!("user action", user_id, action);
        };
        
        // Test named forms
        let _test4 = || {
            info_log!("user_session", "session started");
        };
        let _test5 = || {
            info_log!("api_request", "request processed", endpoint = "/users", method = "GET", status = 200);
        };
        let _test6 = || {
            let endpoint = "/orders";
            let response_time = 150;
            info_log!("api_response", "request completed", endpoint, response_time);
        };
    }

    #[test]
    fn test_debug_log_macro_syntax() {
        // Test anonymous forms
        let _test1 = || {
            debug_log!("debug message");
        };
        let _test2 = || {
            debug_log!("cache miss", key = "user:123", ttl = 300);
        };
        let _test3 = || {
            let key = "session:abc";
            let hit = false;
            debug_log!("cache lookup", key, hit);
        };
        
        // Test named forms
        let _test4 = || {
            debug_log!("cache_operation", "cache accessed");
        };
        let _test5 = || {
            debug_log!("db_query", "executing query", table = "users", where_clause = "id = ?", params = 1);
        };
        let _test6 = || {
            let table = "orders";
            let execution_time = 25;
            debug_log!("query_performance", "query executed", table, execution_time);
        };
    }

    #[test]
    fn test_trace_log_macro_syntax() {
        // Test anonymous forms
        let _test1 = || {
            trace_log!("trace message");
        };
        let _test2 = || {
            trace_log!("function entry", function = "process_order", args_count = 3);
        };
        let _test3 = || {
            let function = "validate_user";
            let line = 42;
            trace_log!("execution trace", function, line);
        };
        
        // Test named forms
        let _test4 = || {
            trace_log!("function_trace", "entering function");
        };
        let _test5 = || {
            trace_log!("execution_flow", "processing step", step = 1, total_steps = 5, progress = 20.0);
        };
        let _test6 = || {
            let step = 3;
            let state = "processing";
            trace_log!("workflow_trace", "workflow step", step, state);
        };
    }

    #[test]
    fn test_event_macro_syntax() {
        // Test event macro with no attributes
        let _test1 = || {
            event!("user_login");
        };
        let _test2 = || {
            event!("order_created");
        };
        
        // Test event macro with key = value attributes
        let _test3 = || {
            event!("user_action", action = "click", element = "button", page = "checkout");
        };
        let _test4 = || {
            event!("api_call", endpoint = "/users", method = "POST", duration = 150);
        };
        
        // Test event macro with shorthand attributes
        let _test5 = || {
            let action = "scroll";
            let position = 500;
            event!("user_interaction", action, position);
        };
        let _test6 = || {
            let operation = "delete";
            let resource_id = 123;
            let success = true;
            event!("resource_operation", operation, resource_id, success);
        };
        
        // Test with mixed attribute styles - separate calls since mixing styles isn't supported
        let _test7a = || {
            event!("mixed_event", event_type = "navigation", timestamp = 1234567890);
        };
        let _test7b = || {
            let user_id = 789;
            let session_active = true;
            event!("mixed_event", user_id, session_active);
        };
        
        // Test with trailing comma
        let _test8 = || {
            event!("test_event", attr1 = "value1", attr2 = 42,);
        };
    }

    #[test]
    fn test_span_macro_syntax() {
        // Test span macro with no attributes
        let _test1 = || {
            context!("http_request");
        };
        let _test2 = || {
            context!("database_query");
        };
        
        // Test span macro with key = value attributes
        let _test3 = || {
            context!("api_request", method = "GET", endpoint = "/users", version = "v1");
        };
        let _test4 = || {
            context!("db_transaction", table = "orders", operation = "INSERT", isolation = "READ_COMMITTED");
        };
        
        // Test span macro with shorthand attributes
        let _test5 = || {
            let method = "POST";
            let status_code = 201;
            context!("http_response", method, status_code);
        };
        let _test6 = || {
            let service = "payment";
            let provider = "stripe";
            let amount = 99.99;
            context!("payment_processing", service, provider, amount);
        };
        
        // Test with mixed attribute styles - separate calls since mixing styles isn't supported
        let _test7a = || {
            context!("user_operation", operation_type = "update", timestamp = 1234567890);
        };
        let _test7b = || {
            let user_id = 456;
            let authenticated = true;
            context!("user_operation", user_id, authenticated);
        };
        
        // Test with trailing comma
        let _test8 = || {
            context!("test_span", attr1 = "value1", attr2 = 42,);
        };
    }

    #[test]
    fn test_complex_macro_combinations() {
        // Test macros with complex expressions
        let _test1 = || {
            let user = StructWithId { id: 123, name: "test".to_string() };
            info_log!("user_info", "user data", user_id = user.id, user_name = "test_user");
        };
        
        // Test macros with function calls
        let _test2 = || {
            error_log!("calculation_error", "division by zero", result = calculate_result(), timestamp = "2024-01-01");
        };
        
        // Test macros with conditional expressions
        let _test3 = || {
            let success = true;
            debug_log!("operation_result", "operation completed", status = if success { "ok" } else { "failed" }, code = if success { 200 } else { 500 });
        };
        
        // Test macros with array/vector attributes
        let _test4 = || {
            let tags = vec!["important", "urgent"];
            warn_log!("tagged_warning", "system warning", tag_count = tags.len() as i64, first_tag = *tags.get(0).unwrap_or(&"none"));
        };
        
        // Test macros with nested macro calls
        let _test5 = || {
            let attrs = kvset!(nested = "value", count = 1);
            // Note: This tests that kvset can be used in expressions, though not directly in log macros
            let _combined = attrs;
        };
    }

    // Helper structs and functions for complex tests
    #[allow(dead_code)]
    struct StructWithId {
        id: u32,
        name: String,
    }
    
    #[allow(dead_code)]
    fn calculate_result() -> i32 {
        42
    }
}
