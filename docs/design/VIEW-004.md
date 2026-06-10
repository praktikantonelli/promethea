---
date: 2026-06-08
id: VIEW-004
---

# Frontend Application View

## Viewpoint

Composition, Structure, Interface.

## Representation

The frontend is a React TypeScript application organized around reusable feature modules. It communicates with Promethea only through the REST API. Platform-specific shells may be added later, but core UI features should remain shared.

```text
React TypeScript Frontend
├── app shell
│   ├── routing
│   ├── navigation
│   ├── authentication/session state
│   └── error boundaries
├── API client layer
│   ├── generated or typed REST client
│   ├── request/response types
│   └── error mapping
├── features
│   ├── books
│   ├── authors
│   ├── series
│   ├── import and metadata review
│   ├── EPUB edit/version workflows
│   ├── reading tracker
│   ├── analytics
│   ├── jobs and operation history
│   └── settings/admin
├── shared UI components
│   ├── cards, tables, filters, forms
│   ├── image/cover components
│   └── status/error components
└── platform shells, future
    ├── browser
    ├── desktop wrapper
    └── mobile/PWA/native wrapper
```

The initial UI must support browser-based library access. Responsive layouts should support mobile-width and desktop-width usage where required, but native mobile and native desktop packaging are deferred until later milestones.

#### More Information

Implements: [REQ-INT-001](./../requirements/interface/REQ-INT-001.md), [REQ-INT-002](./../requirements/interface/REQ-INT-002.md), [REQ-BUILD-003](./../requirements/build/REQ-BUILD-003.md), [REQ-REUSE-001](./../requirements/reusability/REQ-REUSE-001.md), [REQ-DIST-001](./../requirements/distribution/REQ-DIST-001.md), [REQ-DIST-002](./../requirements/distribution/REQ-DIST-002.md), [REQ-DIST-003](./../requirements/distribution/REQ-DIST-003.md), [REQ-FUNC-011](./../requirements/functional/REQ-FUNC-011.md) through [REQ-FUNC-018](./../requirements/functional/REQ-FUNC-018.md), [REQ-FUNC-020](./../requirements/functional/REQ-FUNC-020.md), [REQ-FUNC-023](./../requirements/functional/REQ-FUNC-023.md) through [REQ-FUNC-026](./../requirements/functional/REQ-FUNC-026.md), [REQ-FUNC-032](./../requirements/functional/REQ-FUNC-032.md).  
Related decisions: [DEC-002](./../decisions/DEC-002.md), [DEC-003](./../decisions/DEC-003.md), [DEC-010](./../decisions/DEC-010.md)
Open issues: UI design system, accessibility target, supported browser matrix, and mobile strategy.
