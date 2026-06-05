---
status: planned
date: 2026-05-26
---
# REQ-FUNC-003 — Extract EPUB metadata

## Statement
The system shall extract available title, author, identifier, language, publication, publisher, and cover metadata from imported EPUB files.

## Rationale
Automatic extraction reduces manual entry and establishes the baseline catalog record.

## Acceptance Criteria
- For a test EPUB containing standard metadata, the imported catalog fields match the source metadata.
- If a metadata field is missing from the EPUB, the import still succeeds and marks the field blank or unknown.
- The import result identifies which fields were extracted and which were unavailable.

## Verification Method
Test

## More Information
Exact EPUB metadata fields may expand over time; current fields are a minimum set.
