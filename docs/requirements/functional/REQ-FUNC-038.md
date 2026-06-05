---
status: draft
date: 2026-05-31
---
# REQ-FUNC-038 — Automatic EPUB cover image fetching

## Statement
The system shall allow automatically fetching alternative EPUB cover image candidates. 

## Rationale
When an EPUB file contains an undesirable cover image, the user should be able to replace it with as little effort as possible. 

## Acceptance Criteria
- The user triggers a search for alternative cover images.
- The system provides a list of alternative images.
- The user selects one and it is persisted in the EPUB file.

## Verification Method
Test (E2E)

## More Information
Potential providers for alternative images are the same as for other metadata, listed in [REQ-INT-005](./../interface/REQ-INT-005.md).
