---
status: planned
date: 2026-05-26
---
# REQ-FUNC-008 — Fetch online metadata

## Statement
The system shall allow an authenticated user to fetch online metadata candidates for a book.

## Rationale
E-book files do not contain all metadata. Manually searching and inserting metadata for every book is tedious for the user.

## Acceptance Criteria
- A user can trigger metadata lookup from an import review or book detail/edit view.
- The system submits relevant search inputs such as title, author, and identifier to configured providers.
- The system returns zero or more normalized candidates without overwriting catalog data automatically unless an approved automation rule applies.

## Verification Method
Test

## More Information
Specific providers and provider credentials are TBD.
