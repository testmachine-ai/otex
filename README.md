# otex

Macros and minimal wrappers over the OpenTelemetry API for Rust.

## Overview

`otex` provides convenient macros and utilities to simplify working with OpenTelemetry in Rust applications. It handles the initialization of tracing, logging, and metrics providers while offering ergonomic macros for creating spans, events, and logs.

## Usage

### Initialization

```rust
use otex::init;

let otex = init(None); // Use default logging, or pass Some(custom_logger)

// Your application code here...

otex.shutdown(); // Clean shutdown of all providers
```

### Tracing

```rust
use otex::{span, event};

// Create a span
let _span = span!("my_operation", user_id = 123, action = "create");

// Add events
event!("processing_started", item_count = 42);
```

### Logging

```rust
use otex::log;
use opentelemetry::logs::Severity;

log!("user_action", Severity::Info, "User performed action", user_id = 123);
```

### Key-Value Helpers

```rust
use otex::kvset;

let attributes = kvset!(name = "test", value = 42, enabled = true);
```

## Features

- **stdout**: Enable stdout exporters for development/debugging

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
