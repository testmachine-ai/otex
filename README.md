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
use otex::{actionable_log, logs::Severity, schema::Outcome};

actionable_log!(
    "queue.job.succeeded",
    Severity::Info,
    Outcome::Succeeded,
    "Job completed",
    job.id = job_id,
);
```

`actionable_log!` emits [actionable log schema v1](docs/actionable-log-schema-v1.md),
with a [language-neutral JSON Schema](schema/actionable-log-v1.schema.json) for adapters.
Existing named records gain a searchable `event.name` without being marked as v1;
anonymous log macros remain available as a compatibility path for legacy records.

Production application events use OTLP only (`OTEX_EXPORT=true`, the default).
`otex` does not mirror those records through the Rust `log` facade. For local
development only, compile with the `stdout` feature and set `OTEX_EXPORT=false`;
the stdout exporters are disabled whenever OTLP export is enabled, so the same
normal event is never emitted on both paths.

### Key-Value Helpers

```rust
use otex::kvset;

let attributes = kvset!(name = "test", value = 42, enabled = true);
```

## Features

- **stdout**: Enable non-production stdout exporters when `OTEX_EXPORT=false`

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
