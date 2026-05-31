---
status: proposed
date: 2026-05-26
---
# REQ-ML-001 — No autonomous AI/ML catalog mutation without approval

## Statement 
The system shall not use AI/ML or heuristic metadata matching to make irreversible catalog, file, or reading-data changes without explicit user approval or an enabled automation rule that documents the allowed action.

## Rationale 
The current product description does not require AI/ML, and metadata matching can be wrong; human control protects the library.

## Acceptance Criteria
- Metadata candidates are reviewable before application unless automation explicitly permits application.
- Automation rules identify the actions they may perform.
- Irreversible or destructive operations retain rollback/versioning where applicable.

## Verification Method 
Inspection

## More Information 
No core AI/ML model is specified for the initial release; if added, additional ML lifecycle requirements must be written.
