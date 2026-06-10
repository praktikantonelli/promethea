---
date: 2026-06-08
id: VIEW-011
---

# Job Processing and Automation View

## Viewpoint

Interaction, Concurrency, State Dynamics, Resources.

## Representation

Promethea uses background jobs for long-running operations such as import, metadata fetching, bulk metadata application, EPUB search-and-replace, cover fetching, backup/restore, and future sync operations.

```text
Frontend
  -> REST API: request long operation
REST API
  -> Job Service: create job record
Job Service
  -> Job Queue / Worker: enqueue execution
Worker
  -> Application Service: execute operation
Application Service
  -> Database / Asset Store / Provider Adapter
Worker
  -> Job Service: update progress, result, failure
Frontend
  -> REST API: poll job status / operation history
```

Job state baseline:

```text
Queued -> Running -> Succeeded
Queued -> Running -> Failed
Queued -> CancelRequested -> Cancelled, if supported
Running -> RetryScheduled -> Running, for retryable provider/job failures
```

Automation rules are deferred but should use the same job execution and operation-history model:

```text
AutomationRule
├── trigger: book_imported, metadata_candidate_available, TBD
├── conditions
├── actions
├── enabled flag
└── execution history
```

Concurrency rules:

- Jobs must not corrupt shared catalog or asset state.
- Multiple imports may run concurrently only if duplicate detection and asset paths are safe under concurrency.
- EPUB modification jobs for the same book file should be serialized or guarded by per-file locking.
- Provider calls should respect provider-specific rate limits once known.
- User-visible job status should survive server restart where feasible.

#### More Information

Implements: [REQ-FUNC-028](./../requirements/functional/REQ-FUNC-028.md), [REQ-FUNC-029](./../requirements/functional/REQ-FUNC-029.md), [REQ-FUNC-030](./../requirements/functional/REQ-FUNC-030.md), [REQ-FUNC-031](./../requirements/functional/REQ-FUNC-031.md), [REQ-FUNC-032](./../requirements/functional/REQ-FUNC-032.md), [REQ-PERF-002](./../requirements/performance/REQ-PERF-002.md), [REQ-REL-003](./../requirements/reliability/REQ-REL-003.md), [REQ-OBS-001](./../requirements/observability/REQ-OBS-001.md), [REQ-OBS-002](./../requirements/observability/REQ-OBS-002.md), [REQ-AVAIL-001](./../requirements/availability/REQ-AVAIL-001.md).  
Related decisions: [DEC-006](./../decisions/DEC-006.md), [DEC-008](./../decisions/DEC-008.md)
Open issues: in-process vs external queue, cancellation model, retry policy, concurrency limits, and persistence of in-flight job state.

