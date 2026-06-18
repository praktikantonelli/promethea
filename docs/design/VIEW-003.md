---
date: 2026-06-08
id: VIEW-003
---

# Backend Module and Dependency View

## Viewpoint

Composition, Dependency, Structure, Patterns.

## Representation

The backend should be organized around a reusable Rust core with adapters at the edges. The exact crate layout is an ADR-level decision, but the dependency rule is: transport and infrastructure depend on the core; the core must not depend on HTTP routing, database drivers, provider SDKs, or deployment-specific code.

```text
promethea-server
├── api / transport layer
│   ├── routes
│   ├── request/response DTOs
│   └── auth/session middleware
│        depends on
│        v
├── application services
│   ├── import service
│   ├── catalog service
│   ├── metadata service
│   ├── reading service
│   ├── analytics service
│   ├── asset service
│   └── job service
│        depends on ports / traits
│        v
├── domain core
│   ├── book/work/edition/file model
│   ├── author and series model
│   ├── reading model
│   ├── metadata candidate model
│   ├── job/operation model
│   └── validation and invariants
│        used by
│        ^
├── infrastructure adapters
│   ├── database repository implementation
│   ├── migration runner
│   ├── filesystem/object-store asset implementation
│   ├── EPUB parser/writer implementation
│   ├── metadata provider implementation
│   ├── image downloader implementation
│   └── job executor implementation
└── observability/configuration
    ├── structured logging
    ├── configuration loading
    ├── secret/provider credential handling
    └── operational metrics/status hooks
```

The backend exposes application operations rather than raw database operations. The persistence layer stores catalog state and metadata, while the asset adapter stores binary files and preserves previous EPUB versions when destructive modification is attempted.

#### More Information

| Relation | Requirement / Decision |
| -------------- | --------------- |
| Implements | [REQ-BUILD-001](./../requirements/build/REQ-BUILD-001.md), [REQ-BUILD-002](./../requirements/build/REQ-BUILD-002.md), [REQ-MAINT-001](./../requirements/maintainability/REQ-MAINT-001.md), [REQ-MAINT-002](./../requirements/maintainability/REQ-MAINT-002.md), [REQ-REUSE-002](./../requirements/reusability/REQ-REUSE-002.md), [REQ-INT-003](./../requirements/interface/REQ-INT-003.md), [REQ-INT-008](./../requirements/interface/REQ-INT-008.md), [REQ-REL-001](./../requirements/reliability/REQ-REL-001.md), [REQ-REL-002](./../requirements/reliability/REQ-REL-002.md), [REQ-OBS-001](./../requirements/observability/REQ-OBS-001.md)  |
| Related decisions | [DEC-001](./../decisions/DEC-001.md), [DEC-003](./../decisions/DEC-003.md), [DEC-004](./../decisions/DEC-004.md), [DEC-005](./../decisions/DEC-005.md), [DEC-006](./../decisions/DEC-006.md), [DEC-007](./../decisions/DEC-007.md)|
| Open issues | final Rust crate split, trait boundaries, database library, migration tool, and test strategy.|

