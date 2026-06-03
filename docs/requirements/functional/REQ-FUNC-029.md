---
status: deferred
date: 2026-05-26
---
# REQ-FUNC-029 — Automation action for metadata fetching

## Statement 
The system shall support an automation action that fetches metadata for newly imported books.

## Rationale 
Looking up and then inserting metadata for each book is a tedious task that needs to happen for each book.

## Acceptance Criteria
- An enabled rule can start a metadata-fetch job after import.
- The metadata result is either queued for review or applied according to rule configuration.
- Failures are recorded and visible in job or automation history.

## Verification Method 
Test

## More Information 
Human-review defaults should be conservative until provider confidence rules are defined.
