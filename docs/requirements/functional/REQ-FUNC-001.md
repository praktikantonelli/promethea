---
status: planned
date: 2026-05-26
---
# REQ-FUNC-001 — Import EPUB files

## Statement
The system shall import EPUB files into the library catalog.

## Rationale
The Calibre-replacement goal depends on bringing existing e-book files under catalog management.

## Acceptance Criteria
- Given a valid EPUB file, the import flow creates a catalog record and a file asset record.
- The system stores enough import metadata to identify the source file, import time, and file checksum.
- The user receives a success or failure result for each imported file.

## Verification Method
Test

## More Information
Batch import and folder watching are later desktop/server enhancements.
