---
status: planned
date: 2026-05-26
---
# REQ-FUNC-008 — Fetch online metadata

## Statement
The system shall allow an authenticated user to fetch online metadata candidates for a book.

## Rationale
Online metadata lookup is a stated Calibre-replacement must-have.

## Acceptance Criteria
- A user can trigger metadata lookup from an import review or book detail/edit view.
- The system submits relevant search inputs such as title, author, and identifier to configured providers.
- The system returns zero or more normalized candidates without overwriting catalog data automatically unless an approved automation rule applies.

## Verification Method
Test

## More Information
Specific providers and provider credentials are TBD.
