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
| ADR  | Architecture Decision Record; a record of a segnificant design decision, including implications, considered alternatives and drawbacks |
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
| Context | Yes | Establish system boundary, users, external systems, and deferred external environments. | 3.1, 3.14 |
| Composition | Yes | Decompose the system into frontend, backend, persistence, asset, provider, and job elements. | 3.2, 3.3, 3.4 |
| Logical | Yes | Describe stable domain concepts such as work, edition, book file, author, series, reading event, and automation rule. | 3.5, 3.10 |
| Physical | Partial | Only the self-hosted/container deployment has physical relevance at this stage. Physical hardware topology is otherwise deployment-specific. | 3.13 |
| Structure | Yes | Show internal organization of frontend/backend and major connectors. | 3.2, 3.3, 3.4 |
| Dependency | Yes | Make direction of dependencies explicit, especially around reusable core and adapters. | 3.3 |
| Information | Yes | Define persistent catalog data, asset metadata, versioning, reading events, and operation history. | 3.6, 3.10 |
| Interface | Yes | Describe REST API, file import/download, metadata/image providers, and future device-sync interface. | 3.7 |
| Interaction | Yes | Describe import, metadata enrichment, EPUB modification, reading update, and background-job flows. | 3.8, 3.9, 3.10, 3.11 |
| Algorithm | Partial | Used only where operation logic is architecturally significant, such as import checksums and EPUB mutation/versioning. | 3.8, 3.9 |
| State Dynamics | Partial | Used for reading status, job status, and EPUB modification states. | 3.9, 3.10, 3.11 |
| Concurrency | Partial | Used for background jobs, provider calls, and long-running operations. | 3.11 |
| Patterns | Yes | Records architectural style: modular backend, ports/adapters, server-authoritative API, provider abstraction, versioned asset handling. | 3.3, 3.6, 3.11, Section 4 |
| Deployment | Yes | Map software units to self-hosted/server-browser/container deployment. | 3.13 |
| Resources | Yes | Address file assets, database, job workers, storage growth, logs, backup/restore, and external provider limits. | 3.6, 3.11, 3.13 |

## 3. Design Views

| Title | ID |
| -------------- | --------------- |
| System Context View | [VIEW-001-system-context](./design/VIEW-001-system-context.md) |
| Top-Level Composition View | [VIEW-002-top-level-composition](./design/VIEW-002-top-level-composition.md) |
| Backend Module and Dependency View | [VIEW-003-backend-module-dependency](./design/VIEW-003-backend-module-dependency.md) |
| Frontend Application View | [VIEW-004-frontend-application](./design/VIEW-004-frontend-application.md) |
| Logical Domain Model View | [VIEW-005-logical-domain-model](./design/VIEW-005-logical-domain-model.md) |
| Persistence and Asset Storage View | [VIEW-006-persistence-asset-storage](./design/VIEW-006-persistence-asset-storage.md) |
| REST and External Interface View | [VIEW-007-rest-external-interface](./design/VIEW-007-rest-external-interface.md) |
| EPUB Import and Metadata Runtime View | [VIEW-008-epub-import-metadata](./design/VIEW-008-epub-import-metadata-runtime.md) |
| EPUB Modification and Versioning View | [VIEW-009-epub-modification-versioning](./design/VIEW-009-epub-modification-versioning.md) |
| Reading Tracking and Analytics View | [VIEW-010-reading-tracking-analytics](./design/VIEW-010-reading-tracking-analytics.md) |
| Job Processing and Automation View | [VIEW-011-job-processing-automation](./design/VIEW-011-job-processing-automation.md) |
| Security View | [VIEW-012-security](./design/VIEW-012-security.md) |
| Deployment and Operations View | [VIEW-013-deployment-operation](./design/VIEW-013-deployment-operations.md) |
| Deferred Extension View | [VIEW-014-deferred-extension](./design/VIEW-014-deferred-extension.md) |


## 4. Decisions

All decisions are defined in separate architectural decision record files under [`/decisions`](./decisions/), following the [ADR Template](./decisions/adr-template.md).

| Title | ID |
|-------|----|
| Use a Rust Backend exposing a REST API | [DEC-001-rust-rest-backend](./decisions/adr-template.md) |
| Use a shared React Typescript frontend | [DEC-002-shared-react-typescript-frontend](./decisions/DEC-002-shared-react-typescript-frontend.md) |
| Keep the server as the authoritative consistency boundary | [DEC-003-server-authoritative-consistency-boundary](./decisions/DEC-003-server-authoritative-consistency-boundary.md) |
| Structure the backend as a modular reusable core with adapters | [DEC-004-modular-backend-reusable-core](./decisions/DEC-004-modular-backend-reusable-core.md) |
| Store structured catalog data separately from managed binary assets | [DEC-005-separate-catalog-database-and-managed-assets](./decisions/DEC-005-separate-catalog-database-and-managed-assets.md) |
| Execute long-running operations as background jobs | [DEC-006-background-jobs-for-long-operations](./decisions/DEC-006-background-jobs-for-long-operations.md) |
| Preserve previous EPUB versions before file mutation | [DEC-007-version-epub-before-mutation](./decisions/DEC-007-version-epub-before-mutation.md) |
| Use provider abstraction for metadata and external image fetching | [DEC-008-provider-abstraction-for-metadata-and-images](./decisions/DEC-008-provider-abstraction-for-metadata-and-images.md) |
| Treat SQLite as a POC candiate and defer the final database-engine decision | [DEC-009-database-engine-poc-deferred-final-choice](./decisions/DEC-009-database-engine-poc-deferred-final-choice.md) |
| Defer native desktop/mobile, e-reader sync, audiobook, and cross-medium progress implementation | [DEC-010-defer-native-and-multi-medium-extension](./decisions/DEC-010-defer-native-and-multi-medium-extensions.md) |

## 5. Appendixes
💬 _Optional supporting material that aids understanding without being normative._

➥ Include glossaries, data dictionaries, models/diagrams, sample datasets, or change-impact analyses that support the main sections. Reference rather than duplicate content when possible.

💡 Tips:
- Keep appendixes organized and referenced from the main text.

