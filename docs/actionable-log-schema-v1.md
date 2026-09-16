# Actionable log schema v1

This contract is shared by TestMachine services regardless of language or logging library. It extends the OpenTelemetry log data model rather than replacing it. `schema/actionable-log-v1.schema.json` is the language-neutral validation artifact; it represents a flattened logical projection of OTLP intrinsic fields and attributes.

## Record contract

Every migrated event has these fields:

| Field | Rule |
| --- | --- |
| OTel timestamp, observed timestamp | Emitter/SDK managed; UTC at transport boundaries. |
| OTel severity number and text | Follow the severity rules below. |
| `log.schema.version` | Integer `1`. Do not stamp legacy records at the collector. |
| OTel event name and `event.name` | The same stable dotted identifier. |
| `event.outcome` | One of `started`, `succeeded`, `failed`, `degraded`, `retried`, `skipped`, `cancelled`. |
| `log.transport` | `otlp` in the normalized production record; collector-managed. |
| body | A short, safe human summary. It must remain useful without interpolated payloads. |
| trace/span IDs | SDK-managed when an active context exists. |
| service resource | `service.name`; production must also provide deployment environment and service version/revision when available. |

Add the affected entity using a typed, namespaced identifier such as `request.id`, `job.id`, `scan.id`, `batch.id`, or a domain-specific equivalent. Add `error.type` and `error.retryable` to failures. Add `diagnostic.action` when the next step is not obvious and `runbook.url` only when a maintained runbook exists. Owners belong on service resources/deployment metadata, not repeated in every record.

Emit `event.name` explicitly even when an SDK supports the intrinsic OTel event-name field. The deployed collector version cannot normalize that intrinsic field into a searchable Loki attribute.

Event names are lowercase dot-separated identifiers in the form `<domain>.<entity>.<transition>`, for example `queue.job.failed` or `report.delivery.succeeded`. Names must not contain IDs, URLs, status text, error messages, or other unbounded values. Changing an event's meaning requires a new name; adding optional fields does not.

## Transport boundary

OTLP is the sole production transport for normal application events. Producers must not mirror a normal event to stdout/stderr, directly or through a console logger. Local-development console output is permitted only with OTLP export disabled.

Kubernetes stdout collection is an exception path for container/runtime and collector diagnostics, nginx until a supported OTLP bridge exists, workloads not yet migrated, and failures that occur before telemetry initialization. These records are not schema-v1 application events and do not pass through OTLP normalization or sensitive-attribute filtering. Applications must keep pre-initialization output minimal and safe because collector-side OTLP defenses do not apply.

## Severity and signal selection

- `TRACE`: temporary, opt-in local diagnostics; disabled in deployed environments.
- `DEBUG`: bounded engineering detail that is not required for routine operation.
- `INFO`: meaningful lifecycle transitions and completed outcomes. Do not log polling, healthy probes, or per-item progress when a metric answers the question.
- `WARN`: degraded but continuing operation, exhausted retry that falls back safely, or operator attention that is not yet an outage.
- `ERROR`: requested work failed, data may be incorrect/lost, or a dependency failure cannot be handled locally.
- `FATAL`: process cannot continue safely. Preserve the OTLP fatal severity.

Use metrics for rates, counts, latency distributions, saturation, and health state. Use traces for call graphs and high-volume per-operation timing. Log state transitions, decisions, and failures that require diagnosis. A retry attempt is normally a metric or `DEBUG`; emit `WARN` only for meaningful degradation and one terminal `ERROR` when the operation fails.

## Error taxonomy

`error.type` is one of: `authentication`, `authorization`, `validation`, `not_found`, `conflict`, `rate_limited`, `timeout`, `dependency_unavailable`, `connection`, `protocol`, `data_integrity`, `resource_exhausted`, `internal`, `cancelled`, `unknown`.

Set `error.retryable` from the caller's policy, not merely from the exception class. Preserve HTTP/RPC/database status in typed fields such as `http.response.status_code` or a bounded vendor code. Stack traces are permitted only for unexpected internal failures, as structured exception fields, after sensitive inputs are excluded. Never use a raw exception dump as the sole diagnosis.

## Correlation

Continue W3C `traceparent` across synchronous HTTP and asynchronous messages. Preserve the original trace as a link when a queue consumer starts independent work. Use OTel trace/span IDs for telemetry correlation and domain IDs (`request.id`, `job.id`, `scan.id`) for business workflow correlation; do not overload one as the other. Correlation IDs must be opaque, non-secret, and exact-match queryable. Email addresses, repository URLs, wallet addresses, and free-form subjects are entities, not safe correlation IDs.

## Privacy, security, and cardinality

Never emit authorization/cookie headers, credentials, tokens, connection strings, signed URLs, request/response bodies, SQL parameters, prompts, source code, email addresses/subjects/bodies, or raw third-party payloads. Record safe metadata instead: byte/item counts, media type, status, duration, bounded error class, and opaque internal IDs. Query strings and fragments are excluded from URLs; prefer route templates and dependency names over hosts.

Keep Loki stream labels low-cardinality. `service.name`, deployment environment, namespace, severity, and schema version are suitable. Event names and outcomes remain structured metadata until their fleet-wide cardinality is measured. Domain IDs, trace/span IDs, URLs, messages, error details, and runbook URLs must never be stream labels.

## Deduplication and rate limits

Emit one event per state transition, not once per observer. Aggregate identical failures at the source where possible. Health/poll success belongs in metrics and is filtered at the collector during migration. Alerting deduplicates terminal failures by `event.name` plus the strongest available domain correlation ID, falling back to trace ID; never deduplicate by message text. Rate limiting must retain a suppressed-count metric and the first/last occurrence metadata.

## Compatibility and migration

1. Ship collector normalization first. The OTLP path marks transport, removes known-dangerous attributes, normalizes safe aliases, and drops low-value health logs, but never marks legacy records as v1. Kubernetes stdout remains opaque and bypasses OTLP normalization; reserve it for the explicit exception path above and do not treat collector filtering as a privacy boundary.
2. Release `otex` 0.5.0. Existing macros remain source-compatible but stop mirroring records through the host `log` facade; named records gain the searchable `event.name` alias but are not stamped as v1. Callers migrate meaningful transitions to `actionable_log!`, which adds `log.schema.version=1` and a bounded outcome.
3. TypeScript adapters should expose the same field names and enums, enrich from active OTel context, and export normal production events only through OTLP. Console output must be gated to local development with OTLP disabled. They must not depend on the Rust crate.
4. Rust repositories upgrade `otex`, convert highest-value failure/state-transition sites first, and preserve current message text where safe. Do not bulk-convert debug/noise logs.
5. Update dashboards and Sentinel queries to prefer `log.schema.version=1`, `event.name`, and typed fields while retaining legacy query arms for one release window. Remove aliases only after all deployed revisions emit v1 and rollback windows expire.

Acceptance for a migrated event: a Loki query can select it by service and `event.name`; `log.transport=otlp`; no duplicate normal application event appears in Kubernetes stdout; a trace link works when context exists; the affected domain entity and outcome are present; failures have safe type/retryability and a next action or maintained runbook when needed; no prohibited data appears in body or attributes; repeated normal operation does not create unbounded volume or labels.
