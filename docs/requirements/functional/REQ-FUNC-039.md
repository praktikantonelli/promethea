---
status: draft
date: 2026-05-31
---
# REQ-FUNC-039 — Manual EPUB Cover Image Fetching

## Statement
The system shall allow a user to manually provide an alternative cover image.

## Rationale
When the user wants a very specific cover image, the system should allow the user to provide one instead of enforcing the usage of automatic image fetchers.

## Acceptance Criteria
- The user triggers an update to a cover image.
- The user either provides a link, path or image file.
- The system persists the new choice and reports whether the operation was successful or not.

## Verification Method
Test (E2E)

## More Information
Related to [REQ-INT-006](./../interface/REQ-INT-006.md). 
