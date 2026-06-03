---
status: planned
date: 2026-05-26
---
# REQ-FUNC-005 — Write metadata to EPUB files

## Statement
The system shall support writing accepted metadata changes back into EPUB files.

## Rationale
A Calibre replacement needs to modify the actual e-book file, not only the application catalog.

## Acceptance Criteria
- A user can request that catalog metadata be written to an EPUB file.
- After the operation, re-importing or re-reading the EPUB shows the updated metadata for supported fields.
- The operation reports unsupported fields rather than silently pretending to write them.

## Verification Method
Test

## More Information
Safe file versioning requirements apply; see [REQ-FUNC-007](./REQ-FUNC-007.md) and [REQ-REL-002](./../reliability/REQ-REL-002.md).
