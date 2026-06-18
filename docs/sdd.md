# Software Design Description
## For Promethea

Version 0.1  
Prepared by Luca Antonelli
2026-06-02

## Table of Contents
<!-- TOC -->
* [1. Introduction](#1-introduction)
  * [1.1 Document Purpose](#11-document-purpose)
  * [1.2 Subject Scope](#12-subject-scope)
  * [1.3 Definitions, Acronyms, and Abbreviations](#13-definitions-acronyms-and-abbreviations)
  * [1.4 References](#14-references)
  * [1.5 Document Overview](#15-document-overview)
* [2. Design Overview](#2-design-overview)
  * [2.1 Stakeholder Concerns](#21-stakeholder-concerns)
  * [2.2 Selected Viewpoints](#22-selected-viewpoints)
* [3. Design Views](#3-design-views)
* [4. Decisions](#4-decisions)
* [5. Appendixes](#5-appendixes)
<!-- TOC -->

## Revision History

| Name | Date | Reason For Changes | Version |
|------|------|--------------------|---------|
|Luca Antonelli      | 2026-06-02     | initial version                   |   0.1      |

## 1. Introduction 
This Software Design Description (SDD) describes the initial proposed architecture and design for Promethea. It translates the requirements baseline in the SRS into design views, major design elements, and architecture decision records. The SDD is prescriptive where requirements already impose implementation constraints and intentionally marked as draft/TBD where the SRS leaves open questions.

### 1.1 Document Purpose
The purpose of this SDD is to give developers, maintainers, operators, security reviewers, and future contributors a shared design baseline for Promethea. It describes the system structure, major runtime flows, data ownership, deployment shape, and significant architecture decisions needed to implement and evolve the product. The SDD complements the SRS: the SRS defines what Promethea must do, while this SDD describes how the system is designed to satisfy those requirements.

### 1.2 Subject Scope
This SDD covers the Promethea v0.1 design baseline for a personal/self-hostable e-book library management and reading-tracking system. The design includes a Rust backend, REST API, React TypeScript frontend, catalog database, managed file assets, EPUB import/editing/versioning, metadata/image provider integration, reading tracking, analytics, background jobs, security controls, and self-hosted deployment support.

The design is intentionally focused on the server-browser and shared-frontend product shape. Native desktop packaging, native mobile applications, e-reader synchronization, audiobook support, multi-medium progress synchronization, public SaaS multi-tenancy, browser-based EPUB reading, and AI/ML-based recommendations are outside the initial design scope unless reprioritized through change management.


### 1.3 Definitions, Acronyms, and Abbreviations


| Term | Definition                                                                                                               |
|------|--------------------------------------------------------------------------------------------------------------------------|
| ADR  | Architecture Decision Record; a record of a significant design decision, including implications, considered alternatives and drawbacks |
| API  | Application Programming Interface - A set of definitions and protocols for building and integrating application software |
| Asset | A managed media file such as an EPUB e-book file, a cover or author image |
| Backend | The hidden, logical part of a software that handles user requests |
| Catalog | The user-accessible representation of the book database, including books, authors, series, reading status, metadata and assets |
| E-book | A book in electronic format, made to be read on an e-reader or otherwise compatible reader |
| EPUB | Common type of e-book file |
| Frontend | The interface a user sees when communicating with a software |
| Metadata | Data pertaining a book, includes title, author(s), number of pages, date of publication, series |
| Metadata Candidate | A selection of proposed metadata for a book; generally automatically fetched from an API |
| SDD  | Software Design Description - A document that describes the intended purpose, requirements, and nature of a software        |
| SRS  | Software Requirements Specification - A document that defines the features of a software, grouped into categories |
| View | A concrete design description from a specific viewpoint, such as context, composition, logical, interface, runtime, deployment or security |
| Viewpoint | A selected perspective that defines the concerns, notation, and expected content for one or more views |

### 1.4 References

| Reference | Owner/Author | Version/Date | Type | Location |
| --------------- | --------------- | --------------- | --------------- | ---- |
| Hexagonal Architecture in Rust| howtocodeit | 1.1.7 | Blog post | https://www.howtocodeit.com/guides/master-hexagonal-architecture-in-rust |
| SRS | Luca Antonelli | 0.1 / 2026-06-07 | Document | [`srs.md`](./srs.md) |


### 1.5 Document Overview
Section 2 identifies stakeholder concerns and the viewpoints selected to address them. Section 3 contains concrete design views for Promethea, each following the view-template pattern of viewpoint, representation, and supporting information. Section 4 records significant design decisions and links them to the affected views and SRS requirements. Section 5 captures appendixes, traceability, and open issues that should be refined as the design matures.

## 2. Design Overview
Promethea is designed as a self-hostable server application with a reusable frontend and a modular backend. The central design strategy is to keep domain logic independent from transport, storage, provider, EPUB-processing, and job execution concerns so that the system can evolve towards desktop, mobile and sync scenarios without duplicating the core logic.

### 2.1 Stakeholder Concerns

| Stakeholder | Main design concerns | Addressed by viewpoints / views |
|-------------|----------------------|----------------------------------|
| Library Owner / Administrator | Self-hosted installation, data ownership, backups, provider configuration, user/content responsibility, safe EPUB modification | Context, Deployment, Security, Persistence and Asset Storage, EPUB Modification and Versioning |
| Reader / Authenticated User | Browsing usability, book/author/series navigation, reading-status updates, analytics, remote access, privacy of reading data | Frontend Application, Logical Domain Model, Reading Tracking and Analytics, Security, REST and External Interface |
| Maintainer / Developer | Modular implementation, testability, Rust/React constraints, database migrations, reusable core, clear dependencies, future extensibility | Composition, Backend Module and Dependency, Logical Domain Model, Interface, Decisions |
| Operator / Self-Hoster | Container deployment, configurable data directory, restart recovery, observability, backup/restore, resource growth visibility | Deployment and Operations, Persistence and Asset Storage, Security, Job Processing and Automation |
| Security Reviewer | Authentication, authorization, external input validation, image URL download risk, secrets, HTTPS deployment, reading-data privacy | Security, REST and External Interface, Persistence and Asset Storage, Deployment and Operations |
| QA / Verifier | Traceability from requirements to design elements, testable runtime flows, failure behavior, deterministic import/editing semantics | Runtime views, Interface View, Traceability Appendix, Decisions |
| Future Extension Developer | Desktop/mobile reuse, e-reader sync, audiobook model, cross-medium progress, provider abstraction | Deferred Extension View, Backend Module and Dependency View, Logical Domain Model, Decisions |

### 2.2 Selected Viewpoints
This SDD uses a tailored subset of the default viewpoints from the SDD template. The selected viewpoints are included because they address specific concerns in the design of the system. The remaining viewpoints are either omitted or merged into a broader view. 

| Viewpoint | Used? | Purpose in this SDD | Concrete views |
|-----------|-------|---------------------|----------------|
| Context | Yes | Establish system boundary, users, external systems, and deferred external environments. | [VIEW-001](./design/VIEW-001.md), [VIEW-014](./design/VIEW-014.md) |
| Composition | Yes | Decompose the system into frontend, backend, persistence, asset, provider, and job elements. | [VIEW-002](./design/VIEW-002.md), [VIEW-003](./design/VIEW-003.md), [VIEW-004](./design/VIEW-004.md) |
| Logical | Yes | Describe stable domain concepts such as work, edition, book file, author, series, reading event, and automation rule. | [VIEW-005](./design/VIEW-005.md), [VIEW-010](./design/VIEW-010.md) |
| Physical | Partial | Only the self-hosted/container deployment has physical relevance at this stage. Physical hardware topology is otherwise deployment-specific. | [VIEW-013](./design/VIEW-013.md) |
| Structure | Yes | Show internal organization of frontend/backend and major connectors. | [VIEW-002](./design/VIEW-002.md), [VIEW-003](./design/VIEW-003.md), [VIEW-004](./design/VIEW-004.md) |
| Dependency | Yes | Make direction of dependencies explicit, especially around reusable core and adapters. | [VIEW-003](./design/VIEW-003.md) |
| Information | Yes | Define persistent catalog data, asset metadata, versioning, reading events, and operation history. | [VIEW-006](./design/VIEW-006.md), [VIEW-010](./design/VIEW-010.md) |
| Interface | Yes | Describe REST API, file import/download, metadata/image providers, and future device-sync interface. | [VIEW-007](./design/VIEW-007.md) |
| Interaction | Yes | Describe import, metadata enrichment, EPUB modification, reading update, and background-job flows. | [VIEW-008](./design/VIEW-008.md), [VIEW-009](./design/VIEW-009.md), [VIEW-010](./design/VIEW-010.md), [VIEW-011](./design/VIEW-011.md) |
| Algorithm | Partial | Used only where operation logic is architecturally significant, such as import checksums and EPUB mutation/versioning. | [VIEW-008](./design/VIEW-008.md), [VIEW-009](./design/VIEW-009.md) |
| State Dynamics | Partial | Used for reading status, job status, and EPUB modification states. | [VIEW-009](./design/VIEW-009.md), [VIEW-010](./design/VIEW-010.md), [VIEW-011](./design/VIEW-011.md) |
| Concurrency | Partial | Used for background jobs, provider calls, and long-running operations. | [VIEW-011](./design/VIEW-011.md) |
| Patterns | Yes | Records architectural style: modular backend, ports/adapters, server-authoritative API, provider abstraction, versioned asset handling. | [VIEW-003](./design/VIEW-003.md), [VIEW-006](./design/VIEW-006.md), [VIEW-011](./design/VIEW-011.md), Section 4 |
| Deployment | Yes | Map software units to self-hosted/server-browser/container deployment. | [VIEW-013](./design/VIEW-013.md) |
| Resources | Yes | Address file assets, database, job workers, storage growth, logs, backup/restore, and external provider limits. | [VIEW-006](./design/VIEW-006.md), [VIEW-011](./design/VIEW-011.md), [VIEW-013](./design/VIEW-013.md) |

## 3. Design Views

| Title | ID |
| -------------- | --------------- |
| System Context View | [VIEW-001](./design/VIEW-001.md) |
| Top-Level Composition View | [VIEW-002](./design/VIEW-002.md) |
| Backend Module and Dependency View | [VIEW-003](./design/VIEW-003.md) |
| Frontend Application View | [VIEW-004](./design/VIEW-004.md) |
| Logical Domain Model View | [VIEW-005](./design/VIEW-005.md) |
| Persistence and Asset Storage View | [VIEW-006](./design/VIEW-006.md) |
| REST and External Interface View | [VIEW-007](./design/VIEW-007.md) |
| EPUB Import and Metadata Runtime View | [VIEW-008](./design/VIEW-008.md) |
| EPUB Modification and Versioning View | [VIEW-009](./design/VIEW-009.md) |
| Reading Tracking and Analytics View | [VIEW-010](./design/VIEW-010.md) |
| Job Processing and Automation View | [VIEW-011](./design/VIEW-011.md) |
| Security View | [VIEW-012](./design/VIEW-012.md) |
| Deployment and Operations View | [VIEW-013](./design/VIEW-013.md) |
| Deferred Extension View | [VIEW-014](./design/VIEW-014.md) |


## 4. Decisions

All decisions are defined in separate architectural decision record files under [`/decisions`](./decisions/), following the [ADR Template](./decisions/adr-template.md).

| Title | ID |
|-------|----|
| Use a Rust Backend exposing a REST API | [DEC-001](./decisions/DEC-001.md) |
| Use a shared React TypeScript frontend | [DEC-002](./decisions/DEC-002.md) |
| Keep the server as the authoritative consistency boundary | [DEC-003](./decisions/DEC-003.md) |
| Structure the backend as a modular reusable core with adapters | [DEC-004](./decisions/DEC-004.md) |
| Store structured catalog data separately from managed binary assets | [DEC-005](./decisions/DEC-005.md) |
| Execute long-running operations as background jobs | [DEC-006](./decisions/DEC-006.md) |
| Preserve previous EPUB versions before file mutation | [DEC-007](./decisions/DEC-007.md) |
| Use provider abstraction for metadata and external image fetching | [DEC-008](./decisions/DEC-008.md) |
| Treat SQLite as a POC candidate and defer the final database-engine decision | [DEC-009](./decisions/DEC-009.md) |
| Defer native desktop/mobile, e-reader sync, audiobook, and cross-medium progress implementation | [DEC-010](./decisions/DEC-010.md) |
| Use hexagonal architecture in Rust backend | [DEC-011](./decisions/DEC-011.md) |

## 5. Appendixes

### Appendix A: Requirement-to-View Traceability

| Requirement group | Primary views |
|-------------------|---------------|
| External Interfaces: [REQ-INT-001](./requirements/interface/REQ-INT-001.md), [REQ-INT-002](./requirements/interface/REQ-INT-002.md), [REQ-INT-003](./requirements/interface/REQ-INT-003.md), [REQ-INT-004](./requirements/interface/REQ-INT-004.md), [REQ-INT-005](./requirements/interface/REQ-INT-005.md), [REQ-INT-006](./requirements/interface/REQ-INT-006.md), [REQ-INT-007](./requirements/interface/REQ-INT-007.md), [REQ-INT-008](./requirements/interface/REQ-INT-008.md) | [VIEW-001](./design/VIEW-001.md), [VIEW-004](./design/VIEW-004.md), [VIEW-007](./design/VIEW-007.md), [VIEW-012](./design/VIEW-012.md), [VIEW-014](./design/VIEW-014.md) |
| Library Management and EPUB Processing: [REQ-FUNC-001](./requirements/functional/REQ-FUNC-001.md), [REQ-FUNC-002](./requirements/functional/REQ-FUNC-002.md), [REQ-FUNC-003](./requirements/functional/REQ-FUNC-003.md), [REQ-FUNC-004](./requirements/functional/REQ-FUNC-004.md), [REQ-FUNC-005](./requirements/functional/REQ-FUNC-005.md), [REQ-FUNC-006](./requirements/functional/REQ-FUNC-006.md), [REQ-FUNC-007](./requirements/functional/REQ-FUNC-007.md), [REQ-FUNC-008](./requirements/functional/REQ-FUNC-008.md), [REQ-FUNC-009](./requirements/functional/REQ-FUNC-009.md), [REQ-FUNC-010](./requirements/functional/REQ-FUNC-010.md), [REQ-FUNC-037](./requirements/functional/REQ-FUNC-037.md), [REQ-FUNC-038](./requirements/functional/REQ-FUNC-038.md), [REQ-FUNC-039](./requirements/functional/REQ-FUNC-039.md) | [VIEW-002](./design/VIEW-002.md), [VIEW-005](./design/VIEW-005.md), [VIEW-006](./design/VIEW-006.md), [VIEW-007](./design/VIEW-007.md), [VIEW-008](./design/VIEW-008.md), [VIEW-009](./design/VIEW-009.md) |
| Browsing, Authors, and Series: [REQ-FUNC-011](./requirements/functional/REQ-FUNC-011.md), [REQ-FUNC-012](./requirements/functional/REQ-FUNC-012.md), [REQ-FUNC-013](./requirements/functional/REQ-FUNC-013.md), [REQ-FUNC-014](./requirements/functional/REQ-FUNC-014.md), [REQ-FUNC-015](./requirements/functional/REQ-FUNC-015.md), [REQ-FUNC-016](./requirements/functional/REQ-FUNC-016.md), [REQ-FUNC-017](./requirements/functional/REQ-FUNC-017.md), [REQ-FUNC-018](./requirements/functional/REQ-FUNC-018.md), [REQ-FUNC-040](./requirements/functional/REQ-FUNC-040.md) | [VIEW-004](./design/VIEW-004.md), [VIEW-005](./design/VIEW-005.md), [VIEW-007](./design/VIEW-007.md) |
| Reading Tracking and Analytics: [REQ-FUNC-019](./requirements/functional/REQ-FUNC-019.md), [REQ-FUNC-020](./requirements/functional/REQ-FUNC-020.md), [REQ-FUNC-021](./requirements/functional/REQ-FUNC-021.md), [REQ-FUNC-022](./requirements/functional/REQ-FUNC-022.md), [REQ-FUNC-023](./requirements/functional/REQ-FUNC-023.md), [REQ-FUNC-024](./requirements/functional/REQ-FUNC-024.md), [REQ-FUNC-025](./requirements/functional/REQ-FUNC-025.md), [REQ-FUNC-026](./requirements/functional/REQ-FUNC-026.md), [REQ-FUNC-027](./requirements/functional/REQ-FUNC-027.md) | [VIEW-005](./design/VIEW-005.md), [VIEW-007](./design/VIEW-007.md), [VIEW-010](./design/VIEW-010.md), [VIEW-012](./design/VIEW-012.md) |
| Automation and Job Processing: [REQ-FUNC-028](./requirements/functional/REQ-FUNC-028.md), [REQ-FUNC-029](./requirements/functional/REQ-FUNC-029.md), [REQ-FUNC-030](./requirements/functional/REQ-FUNC-030.md), [REQ-FUNC-031](./requirements/functional/REQ-FUNC-031.md),  [REQ-FUNC-032](./requirements/functional/REQ-FUNC-032.md) | [VIEW-008](./design/VIEW-008.md), [VIEW-011](./design/VIEW-011.md) |
| Deferred Device and Media Features: [REQ-FUNC-033](./requirements/functional/REQ-FUNC-033.md), [REQ-FUNC-034](./requirements/functional/REQ-FUNC-034.md), [REQ-FUNC-035](./requirements/functional/REQ-FUNC-035.md),  [REQ-FUNC-036](./requirements/functional/REQ-FUNC-036.md), [REQ-INT-007](./requirements/interface/REQ-INT-007.md) | [VIEW-014](./design/VIEW-014.md) |
| Performance: [REQ-PERF-001](./requirements/performance/REQ-PERF-001.md), [REQ-PERF-002](./requirements/performance/REQ-PERF-002.md),  [REQ-PERF-003](./requirements/performance/REQ-PERF-003.md) | [VIEW-006](./design/VIEW-006.md), [VIEW-008](./design/VIEW-008.md), [VIEW-011](./design/VIEW-011.md), [VIEW-013](./design/VIEW-013.md) |
| Security: [REQ-SEC-001](./requirements/security/REQ-SEC-001.md), [REQ-SEC-002](./requirements/security/REQ-SEC-002.md), [REQ-SEC-003](./requirements/security/REQ-SEC-003.md), [REQ-SEC-004](./requirements/security/REQ-SEC-004.md), [REQ-SEC-005](./requirements/security/REQ-SEC-005.md) | [VIEW-007](./design/VIEW-007.md), [VIEW-012](./design/VIEW-012.md), [VIEW-013](./design/VIEW-013.md) |
| Reliability: [REQ-REL-001](./requirements/reliability/REQ-REL-001.md), [REQ-REL-002](./requirements/reliability/REQ-REL-002.md), [REQ-REL-003](./requirements/reliability/REQ-REL-003.md), [REQ-REL-004](./requirements/reliability/REQ-REL-004.md) | [VIEW-006](./design/VIEW-006.md), [VIEW-008](./design/VIEW-008.md), [VIEW-009](./design/VIEW-009.md), [VIEW-011](./design/VIEW-011.md) |
| Availability and Observability: [REQ-AVAIL-001](./requirements/availability/REQ-AVAIL-001.md), [REQ-AVAIL-002](./requirements/availability/REQ-AVAIL-002.md), [REQ-OBS-001](./requirements/observability/REQ-OBS-001.md), [REQ-OBS-002](./requirements/observability/REQ-OBS-002.md) | [VIEW-006](./design/VIEW-006.md), [VIEW-011](./design/VIEW-011.md), [VIEW-013](./design/VIEW-013.md) |
| Compliance: [REQ-COMP-001](./requirements/compliance/REQ-COMP-001.md), [REQ-COMP-002](./requirements/compliance/REQ-COMP-002.md), [REQ-COMP-003](./requirements/compliance/REQ-COMP-003.md) | [VIEW-001](./design/VIEW-001.md), [VIEW-007](./design/VIEW-007.md), [VIEW-010](./design/VIEW-010.md), [VIEW-012](./design/VIEW-012.md) |
| Installation, Build, Distribution, Maintainability, Reusability, Portability, Cost, Deadline, POC, Change Management | [VIEW-002](./design/VIEW-002.md), [VIEW-003](./design/VIEW-003.md), [VIEW-004](./design/VIEW-004.md), [VIEW-013](./design/VIEW-013.md), [VIEW-014](./design/VIEW-014.md), Section 4 decisions |

### Appendix B: Decision-to-View Traceability

| Decision | Affected views |
|----------|----------------|
| [DEC-001](./decisions/DEC-001.md) Use a Rust backend exposing a REST API | [VIEW-001](./design/VIEW-001.md), [VIEW-002](./design/VIEW-002.md), [VIEW-003](./design/VIEW-003.md), [VIEW-007](./design/VIEW-007.md), [VIEW-013](./design/VIEW-013.md) |
| [DEC-002](./decisions/DEC-002.md) Use a shared React TypeScript frontend | [VIEW-002](./design/VIEW-002.md), [VIEW-004](./design/VIEW-004.md), [VIEW-014](./design/VIEW-014.md) |
| [DEC-003](./decisions/DEC-003.md) Keep the server as the authoritative consistency boundary | [VIEW-001](./design/VIEW-001.md), [VIEW-002](./design/VIEW-002.md), [VIEW-003](./design/VIEW-003.md), [VIEW-004](./design/VIEW-004.md), [VIEW-007](./design/VIEW-007.md), [VIEW-012](./design/VIEW-012.md) |
| [DEC-004](./decisions/DEC-004.md) Structure the backend as a modular reusable core with adapters | [VIEW-002](./design/VIEW-002.md), [VIEW-003](./design/VIEW-003.md), [VIEW-014](./design/VIEW-014.md) |
| [DEC-005](./decisions/DEC-005.md) Store structured catalog data separately from managed binary assets | [VIEW-002](./design/VIEW-002.md), [VIEW-005](./design/VIEW-005.md), [VIEW-006](./design/VIEW-006.md), [VIEW-008](./design/VIEW-008.md), [VIEW-009](./design/VIEW-009.md), [VIEW-013](./design/VIEW-013.md) |
| [DEC-006](./decisions/DEC-006.md) Execute long-running operations as background jobs | [VIEW-003](./design/VIEW-003.md), [VIEW-008](./design/VIEW-008.md), [VIEW-011](./design/VIEW-011.md), [VIEW-013](./design/VIEW-013.md) |
| [DEC-007](./decisions/DEC-007.md) Preserve previous EPUB versions before file mutation | [VIEW-006](./design/VIEW-006.md), [VIEW-009](./design/VIEW-009.md) |
| [DEC-008](./decisions/DEC-008.md) Use provider abstractions for metadata and external image fetching | [VIEW-001](./design/VIEW-001.md), [VIEW-003](./design/VIEW-003.md), [VIEW-007](./design/VIEW-007.md), [VIEW-008](./design/VIEW-008.md), [VIEW-011](./design/VIEW-011.md), [VIEW-012](./design/VIEW-012.md) |
| [DEC-009](./decisions/DEC-009.md) Treat SQLite as a POC candidate and defer final database-engine decision | [VIEW-005](./design/VIEW-005.md), [VIEW-006](./design/VIEW-006.md), [VIEW-013](./design/VIEW-013.md) |
| [DEC-010](./decisions/DEC-010.md) Defer native and multi-medium extensions | [VIEW-004](./design/VIEW-004.md), [VIEW-005](./design/VIEW-005.md), [VIEW-010](./design/VIEW-010.md), [VIEW-014](./design/VIEW-014.md) |

### Appendix C: Open Issues Carried From the SRS

The following items require resolution before the design can be treated as a stable implementation baseline:

1. Target deployment model: single-user, household/multi-user, or future public multi-tenant.
2. Authentication model: owner account, local users, OAuth/OIDC, reverse-proxy auth, or another model.
3. Stable database engine and migration tooling.
4. Expected library size and performance-test target.
5. Expected concurrent user/client count.
6. Metadata providers and provider-specific contract requirements.
7. External image URL security rules, media types, maximum sizes, and redirect policy.
8. First supported EPUB editing workflows.
9. Exact reading statuses and allowed transitions.
10. Reread handling in analytics.
11. Multi-author attribution policy.
12. Page-count and reading-speed calculation rules.
13. UI accessibility target and supported browser matrix.
14. Supported server and future desktop operating environments.
15. Mobile strategy: responsive web, PWA, Tauri mobile, Capacitor, or native wrapper.
16. E-reader devices and sync protocols.
17. Backup retention and restore expectations.
18. License and third-party dependency policy.

### Appendix D: Suggested Next ADRs to Finalize

The following ADRs should be split into individual files and reviewed before implementation hardens:

1. Rust crate layout and module boundaries.
2. Database engine selection and migration tool.
3. Work/Edition/File model granularity.
4. Authentication/session strategy.
5. Image URL download security controls.
6. Job queue implementation strategy.
7. EPUB modification/versioning transaction strategy.
8. Backup/restore format and consistency model.
9. OpenAPI generation and API error schema.
10. Frontend routing/state-management approach.
