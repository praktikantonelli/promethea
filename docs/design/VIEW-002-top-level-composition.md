---
date: 2026-06-08
id: VIEW-002-top-level-composition
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

Implements: REQ-INT-001, REQ-INT-003, REQ-INT-008, REQ-FUNC-001 through REQ-FUNC-032, REQ-BUILD-001 through REQ-BUILD-003, REQ-MAINT-001, REQ-REUSE-001, REQ-REUSE-002.  
Related decisions: DEC-001, DEC-002, DEC-003, DEC-004, DEC-005.  
This view is intentionally higher-level than the backend module view. It should be updated if desktop, mobile, e-reader sync, or audiobook support becomes part of the active release scope.
