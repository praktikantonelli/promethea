---
status: planned
date: 2026-05-26
---
# REQ-FUNC-007 — Version EPUB modifications

## Statement
The system shall retain a recoverable previous EPUB file version before overwriting or replacing an EPUB through metadata, cover, or content edits.

## Rationale
EPUB editing can damage files; preserving prior versions mitigates data-loss risk.

## Acceptance Criteria
- Before a write operation changes an EPUB, the previous version remains recoverable.
- The system records version creation time, operation type, and checksum.
- A user or administrator can identify the current version and at least one previous version.

## Verification Method
Test

## More Information
Retention limits are TBD; backup policy should define whether old versions are pruned.
