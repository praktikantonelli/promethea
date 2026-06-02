---
status: planned
date: 2026-05-26
---
# REQ-REUSE-001 — Shared frontend reuse

## Statement 
The system shall maximize reuse of the React TypeScript frontend across browser, desktop, and mobile-oriented deployments.

## Rationale 
Reusing the frontend reduces the amount of work required. It also guarantees that workflows stay identical regardless of platform. 

## Acceptance Criteria
- Common book, author, series, reading, import, and analytics views are implemented in shared frontend modules.
- Platform-specific shells do not duplicate core UI feature logic except where necessary.
- Any platform-specific UI forks are documented with rationale.

## Verification Method 
Inspection

## More Information 
This is a design and maintainability requirement, not a promise of identical UI on every platform.
