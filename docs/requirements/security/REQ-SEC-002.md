---
status: draft
date: 2026-05-26
---
# REQ-SEC-002 — Authorize user operations

## Statement 
The system shall enforce authorization checks for administrative, catalog-editing, file-management, and reading-tracker operations.

## Rationale 
Even a single-user deployment benefits from explicit authorization boundaries.

## Acceptance Criteria
- Protected mutating endpoints check authorization before applying changes.
- Authorization failures are logged without exposing secrets.

## Verification Method 
Test

## More Information 
Role model is TBD; may be simplified to owner/admin for initial release.
