---
date: 2026-06-08
id: VIEW-008
---

# EPUB Import and Metadata Runtime View

## Viewpoint

Interaction, Algorithm, State Dynamics.

## Representation

The EPUB import flow is a long-running or potentially long-running operation. Small imports may complete synchronously, but the design should support execution as a background job to satisfy non-blocking operation requirements.

Happy path:

```text
User
  -> Frontend: select/upload EPUB
Frontend
  -> REST API: POST /books/import
REST API
  -> Import Service: validate request and create import job
Import Service
  -> Asset Store: stage uploaded file
Import Service
  -> EPUB Processor: validate EPUB and extract metadata
Import Service
  -> Database: compute/store checksum, detect duplicates
Import Service
  -> Database: create/update book, edition, file, asset rows
Import Service
  -> Metadata Provider Adapter, optional: search metadata
Metadata Provider Adapter
  -> Database: store metadata candidates
Import Service
  -> Job Service: update job status and operation history
Frontend
  -> REST API: poll or subscribe to job status
Frontend
  -> User: show imported book and candidate review actions
```

Failure paths:

```text
Invalid EPUB
  -> reject import with user-visible error
  -> preserve operation history

Duplicate checksum
  -> return existing record or duplicate candidate
  -> do not create duplicate asset unless policy permits

Metadata provider unavailable
  -> import continues using extracted/manual metadata
  -> provider error shown as recoverable/non-fatal

Asset write succeeds but catalog write fails
  -> cleanup staged asset or mark it orphaned for repair
  -> record failure and avoid exposing partial catalog entry
```

State model:

```text
ImportRequested
  -> FileStaged
  -> EPUBValidated
  -> MetadataExtracted
  -> DuplicateChecked
  -> CatalogPersisted
  -> CandidatesFetched optional
  -> Completed

Any state -> Failed
Any long-running state -> CancelRequested, if cancellation is implemented
```

#### More Information

Implements: [REQ-FUNC-001](./../requirements/functional/REQ-FUNC-001.md), [REQ-FUNC-002](./../requirements/functional/REQ-FUNC-002.md), [REQ-FUNC-003](./../requirements/functional/REQ-FUNC-003.md), [REQ-FUNC-008](./../requirements/functional/REQ-FUNC-008.md), [REQ-FUNC-009](./../requirements/functional/REQ-FUNC-009.md), [REQ-FUNC-010](./../requirements/functional/REQ-FUNC-010.md), [REQ-FUNC-031](./../requirements/functional/REQ-FUNC-031.md), [REQ-FUNC-032](./../requirements/functional/REQ-FUNC-032.md), [REQ-PERF-002](./../requirements/performance/REQ-PERF-002.md), [REQ-REL-003](./../requirements/reliability/REQ-REL-003.md), [REQ-REL-004](./../requirements/reliability/REQ-REL-004.md), [REQ-OBS-002](./../requirements/observability/REQ-OBS-002.md).  
Related decisions: 
Open issues: synchronous vs asynchronous threshold, duplicate handling policy, job cancellation policy, provider retry/backoff rules, and metadata candidate confidence model.
