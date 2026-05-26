# REQ-FUNC-015 — Support related series
- ID: REQ-FUNC-015
- Status: planned
- Date: 2026-05-26
- Title: Support related series
- Statement: The system shall support relationships between series, including overarching series and contained series.
- Rationale: The draft requires series views to include related series such as overarching or contained series.
- Acceptance Criteria:
  - A user can define a relationship between two series.
  - The series detail page displays related series grouped or labeled by relationship type.
  - The system prevents or reports invalid self-relations and relation cycles where cycles are disallowed by the chosen relation type.
- Verification Method: Test
- More Information: Allowed relation types should be finalized during domain-model design.

