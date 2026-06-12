---
date: 2026-06-08
id: VIEW-002
---

# Top-Level Composition View

## Viewpoint

Composition, Structure, Dependency.

## Representation

Promethea is decomposed into the following major design elements:

```text
Promethea
├── React TypeScript Frontend
│   ├── Library browsing UI
│   ├── Book/author/series detail UI
│   ├── Import and metadata review UI
│   ├── Reading tracker and analytics UI
│   ├── Job status and operation history UI
│   └── Settings/admin UI
├── Rust REST API Backend
│   ├── HTTP routing and API contracts
│   ├── Domain/application services
│   ├── Persistence adapters
│   ├── Asset storage adapters
│   ├── EPUB processing services
│   ├── Metadata/image provider adapters
│   ├── Background job executor
│   └── Security, configuration, logging
├── Catalog Database
│   ├── Books, works, editions, files
│   ├── Authors, series, relationships
│   ├── Reading events and analytics source data
│   ├── Jobs and operation history
│   └── Configuration and provider metadata
├── Managed Asset Store
│   ├── EPUB files
│   ├── Covers and author images
│   └── Backups/export artifacts
└── External Services
    ├── Metadata providers
    └── Image URL sources
```

The frontend does not directly access the catalog database or asset store. All supported client mutations pass through the REST API or a future approved embedded-backend interface. The backend owns consistency boundaries, validation, persistence, asset writes, provider interactions, and job execution.

#### More Information

| Relation | Requirement / Decision |
| -------------- | --------------- |
| Implements | [DEC-001](./../decisions/DEC-001.md), [DEC-002](./../decisions/DEC-002.md), [DEC-003](./../decisions/DEC-003.md), [DEC-004](./../decisions/DEC-004.md), [DEC-005](./../decisions/DEC-005.md) |
| Related decisions | [REQ-INT-001](./../requirements/interface/REQ-INT-001.md), [REQ-INT-003](./../requirements/interface/REQ-INT-003.md), [REQ-INT-008](./../requirements/interface/REQ-INT-008.md), [REQ-FUNC-001](./../requirements/functional/REQ-FUNC-001.md) through [REQ-FUNC-032](./../requirements/functional/REQ-FUNC-032.md), [REQ-BUILD-001](./../requirements/build/REQ-BUILD-001.md) through [REQ-BUILD-003](./../requirements/build/REQ-BUILD-003.md), [REQ-MAINT-001](./../requirements/maintainability/REQ-MAINT-001.md), [REQ-REUSE-001](./../requirements/reusability/REQ-REUSE-001.md), [REQ-REUSE-002](./../requirements/reusability/REQ-REUSE-002.md) |
| Notes | This view is intentionally higher-level than the backend module view. It will be updated if desktop, mobile, e-reader sync, or audiobook support becomes part of the active release scope. |
