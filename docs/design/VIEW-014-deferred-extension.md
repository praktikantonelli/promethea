---
date: 2026-06-08
id: VIEW-014-deferred-extension
---

# Deferred Extension View

## Viewpoint

Context, Interface, Deployment, Logical.

## Representation

The SRS identifies several future or deferred capabilities. The current architecture should not implement all of them immediately, but it should avoid blocking them unnecessarily.

Deferred extension areas:

```text
Desktop application
  -> reuse React frontend
  -> reuse Rust core/application services where practical
  -> may use local embedded backend or configured remote server

Mobile availability
  -> initially responsive web/PWA-compatible frontend
  -> native wrapper only if later approved

E-reader / mounted folder sync
  -> future sync target abstraction
  -> background job execution
  -> sync operation history

Audiobooks and multi-medium support
  -> preserve Work / Edition / File / Medium conceptual separation
  -> avoid hard-coding EPUB-only assumptions into domain core

Cross-medium progress synchronization
  -> reading state/events should be medium-aware in future
  -> exact sync policy deferred
```

The design principle is to treat these as extension vectors, not current implementation obligations. Deferred capabilities should be activated through future requirements and ADRs.

#### More Information

Implements or preserves future compatibility for: [REQ-DIST-002](./../requirements/distribution/REQ-DIST-002.md), [REQ-DIST-003](./../requirements/distribution/REQ-DIST-003.md), [REQ-INT-007](./../requirements/interface/REQ-INT-007.md), [REQ-FUNC-033](./../requirements/functional/REQ-FUNC-033.md), [REQ-FUNC-034](./../requirements/functional/REQ-FUNC-034.md), [REQ-FUNC-035](./../requirements/functional/REQ-FUNC-035.md), [REQ-FUNC-036](./../requirements/functional/REQ-FUNC-036.md), [REQ-REUSE-001](./../requirements/reusability/REQ-REUSE-001.md), [REQ-REUSE-002](./../requirements/reusability/REQ-REUSE-002.md), [REQ-PORT-001](./../requirements/portability/REQ-PORT-001.md).  
Related decisions: 
Open issues: desktop packaging technology, native mobile strategy, sync target/protocol, audiobook model, and cross-medium progress semantics.
