---
status: planned
date: 2026-05-26
---
# REQ-FUNC-002 — Persist catalog records and assets

## Statement
The system shall persist book metadata, author metadata, series metadata, reading data, and file/cover/author-image asset references across server restarts.

## Rationale
The product is a long-lived personal library system and must not lose catalog state after normal restart.

## Acceptance Criteria
- After restart, previously imported books, authors, series, reading states, and asset links remain available.
- Stored assets remain addressable by stable identifiers.
- Catalog records and asset records remain referentially consistent after normal shutdown and restart.

## Verification Method
Test

## More Information
Exact database engine is TBD; initial design assumption is one authoritative server-side database plus asset directory.
