---
status: draft
date: 2026-05-26
---
# REQ-POC-001 — SQLite server data-store

## Statement 
The project shall validate whether a single authoritative server using a SQLite database satisfies expected concurrency and reliability needs before committing to SQLite for stable server releases.

## Rationale 
The architecture discussion identified SQLite as likely sufficient but exact workload and operational requirements remain unspecified.

## Acceptance Criteria
- The POC uses the same server-only write boundary required by [REQ-INT-008](./../interface/REQ-INT-008.md).
- The POC includes concurrent read and write scenarios representative of expected use.
- The POC records whether SQLite is accepted, rejected, or accepted with constraints for the stable release.

## Verification Method 
Analysis

## More Information 
Clarification needed: expected concurrent users, write rate, background job load, and backup expectations.
