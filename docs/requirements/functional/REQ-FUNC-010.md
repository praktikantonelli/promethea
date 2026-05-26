# REQ-FUNC-010 — Detect duplicate imported books
- ID: REQ-FUNC-010
- Status: proposed
- Date: 2026-05-26
- Title: Detect duplicate imported books
- Statement: The system shall detect likely duplicate imports using at least file checksum and available identifier/title-author comparisons.
- Rationale: Duplicate detection prevents library clutter and accidental redundant storage.
- Acceptance Criteria:
  - An import with an identical checksum to an existing file is flagged as an exact duplicate.
  - An import with matching ISBN or matching normalized title-author values is flagged as a possible duplicate.
  - The user can choose whether to skip, merge, or keep a flagged possible duplicate.
- Verification Method: Test
- More Information: Merge semantics are TBD and should be refined before implementation.

