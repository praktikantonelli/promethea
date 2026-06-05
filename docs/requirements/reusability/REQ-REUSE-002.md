---
status: proposed
date: 2026-05-26
---
# REQ-REUSE-002 — Reusable Rust core

## Statement 
The system shall keep core library, reading, metadata, EPUB, and automation logic reusable outside the REST API server entry point.

## Rationale 
A reusable core reduces duplication when adding desktop or other clients.

## Acceptance Criteria
- Core functions are callable from tests or non-server binaries without starting HTTP routes.
- Server-specific concerns are not required by the core domain module.
- Desktop or job-runner code can reuse core operations where applicable.

## Verification Method
Inspection

## More Information 
This requirement reflects the proposed architecture and can be refined after crate design.
