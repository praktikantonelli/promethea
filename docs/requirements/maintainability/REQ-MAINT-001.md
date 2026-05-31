---
status: proposed
date: 2026-05-26
---
# REQ-MAINT-001 — Modular backend organization

## Statement 
The backend shall separate domain logic from transport, storage, metadata-provider, EPUB-processing, and job-execution concerns.

## Rationale 
Modularity supports reuse across server, desktop, and future mobile or sync contexts.

## Acceptance Criteria
- Core domain logic can be used without importing the HTTP routing layer.
- Storage and provider integrations are isolated behind interfaces or modules.
- Unit tests can exercise domain logic without starting the full server.

## Verification Method 
Inspection

## More Information 
Exact Rust crate layout is an architectural decision.
