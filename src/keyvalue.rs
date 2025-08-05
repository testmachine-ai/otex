use std::fmt::Debug;

use serde::Serialize;

pub(crate) struct Key {
    inner: String,
}

impl Key {
    pub fn new(key: &str) -> Self {
        Self {
            inner: key.to_string(),
        }
    }
}

impl From<Key> for opentelemetry::Key {
    fn from(value: Key) -> Self {
        opentelemetry::Key::from(value.inner)
    }
}

#[derive(Debug)]
pub(crate) enum Value {
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

// OTEL compatibility

impl From<Value> for opentelemetry::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(b) => b.into(),
            Value::Integer(i) => i.into(),
            Value::Float(f) => f.into(),
            Value::String(s) => s.into(),
        }
    }
}

impl From<Value> for opentelemetry::logs::AnyValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(b) => b.into(),
            Value::Integer(i) => i.into(),
            Value::Float(f) => f.into(),
            Value::String(s) => s.into(),
        }
    }
}

// RUST compatibility

impl Value {
    pub fn debug<T: Debug>(value: &T) -> Self {
        Value::String(format!("{:?}", value))
    }

    pub fn json<T: Serialize>(value: &T) -> Self {
        Value::String(serde_json::to_string(value).expect("failed to serialize"))
    }
}

#[cfg(test)]
mod test {

    use serde::Serialize;

    use super::{Value, Key};
    #[test]
    fn from_impl() {
        let test: opentelemetry::Value = Value::Bool(false).into();
        let test_key: opentelemetry::Key = Key {
            inner: "test".to_string()
        }.into();

        #[derive(Debug, Serialize)]
        pub struct Test {
            inner: bool
        }

        let value = Test { inner: true };

        let value = Value::json(&value);
        dbg!(value);
    }
}
