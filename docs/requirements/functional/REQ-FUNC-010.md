---
status: proposed
date: 2026-05-26
---
# REQ-FUNC-010 — Detect duplicate imported books

## Statement
The system shall detect likely duplicate imports using at least file checksum and available identifier/title-author comparisons.

## Rationale
Duplicate detection prevents library clutter and accidental redundant storage.

## Acceptance Criteria
- An import with an identical checksum to an existing file is flagged as an exact duplicate.
- An import with matching ISBN or matching normalized title-author values is flagged as a possible duplicate.
- The user can choose whether they want to keep potential duplicates, or either of the two conflicting books.

## Verification Method
Test

## More Information
None
