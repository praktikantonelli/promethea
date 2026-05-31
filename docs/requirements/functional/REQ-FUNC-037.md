---
status: draft
date: 2026-05-31
---
# REQ-FUNC-037 — Update EPUB cover image

## Statement
The system shall allow modifying the cover image of an EPUB file.

## Rationale
The cover image may have less-than-desired quality, or it may not match the edition of other books of a series. 

## Acceptance Criteria
- A user can trigger an action to select a new cover image for a given EPUB file.
- The user's choice is persisted.
- If the choice is invalid (format, unavailable resource), the system informs the user.

## Verification Method
Test
Testable with E2E test.

## More Information
Related requirement: [[REQ-FUNC-038]]