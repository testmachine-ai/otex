//! TestMachine actionable-log schema primitives.
//!
//! The complete v1 contract and migration policy live in
//! `docs/actionable-log-schema-v1.md`.

/// Current actionable-log schema version.
pub const VERSION: i64 = 1;

/// Canonical schema attribute names.
pub mod keys {
    pub const SCHEMA_VERSION: &str = "log.schema.version";
    pub const TRANSPORT: &str = "log.transport";
    pub const EVENT_NAME: &str = "event.name";
    pub const EVENT_OUTCOME: &str = "event.outcome";
    pub const ERROR_TYPE: &str = "error.type";
    pub const ERROR_RETRYABLE: &str = "error.retryable";
    pub const DIAGNOSTIC_ACTION: &str = "diagnostic.action";
    pub const RUNBOOK_URL: &str = "runbook.url";
    pub const REQUEST_ID: &str = "request.id";
    pub const JOB_ID: &str = "job.id";
    pub const SCAN_ID: &str = "scan.id";
}

/// Bounded outcome values for actionable events.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    Started,
    Succeeded,
    Failed,
    Degraded,
    Retried,
    Skipped,
    Cancelled,
}

impl Outcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Degraded => "degraded",
            Self::Retried => "retried",
            Self::Skipped => "skipped",
            Self::Cancelled => "cancelled",
        }
    }
}

/// Safe, bounded error classes. Domain-specific detail belongs in a separate
/// low-cardinality attribute, never in `error.type`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorType {
    Authentication,
    Authorization,
    Validation,
    NotFound,
    Conflict,
    RateLimited,
    Timeout,
    DependencyUnavailable,
    Connection,
    Protocol,
    DataIntegrity,
    ResourceExhausted,
    Internal,
    Cancelled,
    Unknown,
}

impl ErrorType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authentication => "authentication",
            Self::Authorization => "authorization",
            Self::Validation => "validation",
            Self::NotFound => "not_found",
            Self::Conflict => "conflict",
            Self::RateLimited => "rate_limited",
            Self::Timeout => "timeout",
            Self::DependencyUnavailable => "dependency_unavailable",
            Self::Connection => "connection",
            Self::Protocol => "protocol",
            Self::DataIntegrity => "data_integrity",
            Self::ResourceExhausted => "resource_exhausted",
            Self::Internal => "internal",
            Self::Cancelled => "cancelled",
            Self::Unknown => "unknown",
        }
    }
}
