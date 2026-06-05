---
status: proposed
date: 2026-05-26
---
# REQ-PERF-002 — Non-blocking long-running operations

## Statement 
The system shall keep ordinary catalog browsing and reading-status update endpoints available while import, metadata fetch, image fetch, EPUB edit, or sync jobs are running.

## Rationale 
Long-running jobs must not make the application unusable for normal library operations.

## Acceptance Criteria
- A test can submit a long-running job and still successfully call catalog and reading-status endpoints.
- The long-running job status remains queryable while the job is running.
- Failures in one job do not prevent unrelated normal API requests from completing.

## Verification Method 
Test

## More Information 
Exact latency thresholds are covered by [REQ-PERF-001](./REQ-PERF-001.md).
