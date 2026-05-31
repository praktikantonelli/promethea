---
status: proposed
date: 2026-05-26
---
# REQ-AVAIL-001 — Restart recovery

## Statement
The system shall recover its catalog, configuration, and job state sufficiently to resume normal operation after a controlled server restart.

## Rationale
Self-hosted users need predictable behavior across restarts and upgrades.

## Acceptance Criteria
-  After controlled restart, the server starts without manual database repair.
- Previously completed imports and catalog edits are visible.
- Incomplete jobs are marked in a documented state such as failed, canceled, or resumable.

## Verification Method
Test

## More Information
Availability percentage/SLA is TBD.
