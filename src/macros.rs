// GENERAL
#[macro_export]
macro_rules! kvset {
    // key = value form
    ($( $attr_key:ident = $attr_value:expr ),+ $(,)?) => {{
        use opentelemetry::KeyValue;
        &[
            $( KeyValue::new(stringify!($attr_key), $attr_value) ),*
        ]
    }};

    // shorthand: just ident
    ($( $attr:ident ),+ $(,)?) => {{
        use opentelemetry::KeyValue;
        &[
            $( KeyValue::new(stringify!($attr), $attr) ),*
        ]
    }};
}

#[macro_export]
macro_rules! anykvset {
    // key = value form
    ($( $attr_key:ident = $attr_value:expr ),+ $(,)?) => {{
        use opentelemetry::{Key, logs::AnyValue};
        &[
            $( (Key::new(stringify!($attr_key)), AnyValue::from($attr_value)) ),*
        ]
    }};

    // shorthand: just ident
    ($( $attr:ident ),+ $(,)?) => {{
        use opentelemetry::{Key, logs::AnyValue};
        &[
            $( (Key::new(stringify!($attr), AnyValue::from($attr))) ),*
        ]
    }};
}
// LOGGING
#[macro_export]
macro_rules! log {
    // No attributes
    ($name:expr, $severity:expr, $body:expr) => {{
        $crate::logging::create_log_record($severity, module_path!(), Some($name), Some($body.into()), &[])
    }};

    // key = value form
    ($name:expr, $severity:expr, $body:expr, $( $attr_key:ident = $attr_value:expr ),+ $(,)?) => {{
        let attrs = $crate::anykvset!($( $attr_key = $attr_value ),*);
        $crate::logging::create_log_record($severity, module_path!(), Some($name), Some($body.into()), attrs)
    }};

    // shorthand: ident only
    ($name:expr, $severity:expr, $body:expr, $( $attr:ident ),+ $(,)?) => {{
        let attrs = $crate::anykvset!($( $attr ),*);
        $crate::logging::crate_log_record($severity, module_path!(), Some($name), Some($body.into()), attrs)
    }};
}

// TRACING
#[macro_export]
macro_rules! event {
    // No attributes
    ($name:expr) => {{
        use $crate::tracing::new_event;
        new_event($name, &[])
    }};

    // key = value form
    ($name:expr, $( $attr_key:ident = $attr_value:expr ),+ $(,)?) => {{
        use $crate::{kvset, tracing::new_event};
        let attrs = kvset!($( $attr_key = $attr_value ),*);
        new_event($name, attrs)
    }};

    // shorthand: ident only
    ($name:expr, $( $attr:ident ),+ $(,)?) => {{
        use $crate::{kvset, tracing::new_event};
        let attrs = kvset!($( $attr ),*);
        new_event($name, attrs)
    }};
}

#[macro_export]
macro_rules! span {
    // No attributes
    ($name:expr) => {{
        use $crate::tracing::new_span;
        new_span($name, &[])
    }};

    // key = value form
    ($name:expr, $( $attr_key:ident = $attr_value:expr ),+ $(,)?) => {{
        use $crate::{kvset, tracing::new_span};
        let attrs = kvset!($( $attr_key = $attr_value ),*);
        new_span($name, attrs)
    }};

    // shorthand: ident only
    ($name:expr, $( $attr:ident ),+ $(,)?) => {{
        use $crate::{kvset, tracing::new_span};
        let attrs = kvset!($( $attr ),*);
        new_span($name, attrs)
    }};
}

