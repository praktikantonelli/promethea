---
status: proposed
date: 2026-05-26
---
# REQ-REL-004 — Idempotent import by checksum
## Statement 
The system shall identify repeated import attempts of an identical EPUB file by checksum and avoid creating unintended duplicate file records without user confirmation.

## Rationale 
Retrying failed or uncertain imports should not accidentally duplicate the library.

## Acceptance Criteria
- An identical file checksum is detected during import.
- The import response indicates that an identical file already exists.
- The system does not create a new active duplicate record unless the user explicitly confirms keeping a duplicate.

## Verification Method 
Test

## More Information 
Related to [REQ-FUNC-010](./../functional/REQ-FUNC-010.md).
