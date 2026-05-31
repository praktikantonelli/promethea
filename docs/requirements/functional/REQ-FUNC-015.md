---
status: planned
date: 2026-05-26
---
# REQ-FUNC-015 — Support related series

## Statement
The system shall support relationships between series, including overarching series and contained series.

## Rationale
The draft requires series views to include related series such as overarching or contained series.

## Acceptance Criteria
- A link between series gets created automatically whenever a book's metadata lists more than one series.
- The default relationship between two series is that the overarching one contains the other, or is contained by the other. 
- The series detail page displays related series grouped or labeled by relationship type.
- The system prevents or reports invalid self-relations and relation cycles where cycles are disallowed by the chosen relation type.

## Verification Method
Test

## More Information
Allowed relation types should be finalized during domain-model design. A potential, additional relationship is that between two series contained in the same spanning series.
