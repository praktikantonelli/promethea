# Software Requirements Specification
## For Promethea

Version 0.1  
Prepared by Luca Antonelli 
Date Modified: 2026-05-26

## Table of Contents
<!-- TOC -->
* [1. Introduction](#1-introduction)
    * [1.1 Document Purpose](#11-document-purpose)
    * [1.2 Product Scope](#12-product-scope)
    * [1.3 Definitions, Acronyms, and Abbreviations](#13-definitions-acronyms-and-abbreviations)
    * [1.4 References](#14-references)
    * [1.5 Document Overview](#15-document-overview)
* [2. Product Overview](#2-product-overview)
    * [2.1 Product Perspective](#21-product-perspective)
    * [2.2 Product Functions](#22-product-functions)
    * [2.3 Product Constraints](#23-product-constraints)
    * [2.4 User Characteristics](#24-user-characteristics)
    * [2.5 Assumptions and Dependencies](#25-assumptions-and-dependencies)
    * [2.6 Apportioning of Requirements](#26-apportioning-of-requirements)
* [3. Requirements](#3-requirements)
    * [3.1 External Interfaces](#31-external-interfaces)
    * [3.2 Functional](#32-functional)
    * [3.3 Quality of Service](#33-quality-of-service)
    * [3.4 Compliance](#34-compliance)
    * [3.5 Design and Implementation](#35-design-and-implementation)
    * [3.6 AI/ML](#36-aiml)
* [4. Verification](#4-verification)
* [5. Appendixes](#5-appendixes)
<!-- TOC -->

## Revision History

| Name | Date | Reason For Changes | Version |
|------|------|--------------------|---------|
| Luca Antonelli | 2026-05-26 | Initial SRS draft | 0.1 |

## 1. Introduction

This Software Requirements Specification (SRS) defines the initial requirements for Promethea, a personal e-book library and reading-tracking system. The document states what the system must, should, or may do; it avoids detailed implementation design except where technology constraints have already been provided. Section 2 gives product context and assumptions, Section 3 contains verifiable requirements using the supplied requirement template, Section 4 maps requirements to verification methods, and Section 5 records open questions and supporting material.

### 1.1 Document Purpose

The purpose of this SRS is to translate the project description into a structured, testable requirements baseline for product planning, engineering, QA, security, operations, and future maintainers. Product and engineering can use it to scope releases and design the system; QA can use it to derive test cases; security and operations can use it to identify deployment, authentication, backup, and observability expectations. This SRS defines required system behavior and constraints, not a final internal architecture.

### 1.2 Product Scope

Promethea version 0.1 is a proposed personal/self-hostable library management system intended to replace major personal workflows from Calibre, Goodreads, and calibre-web. Its primary capabilities include importing and managing EPUB files, editing metadata and selected EPUB content, fetching metadata online, browsing books/authors/series, tracking ownership and reading status, and producing reading analytics. Nice-to-have and future capabilities include desktop/laptop distribution, mobile availability, e-reader synchronization, audiobook support, book-medium tracking, and cross-medium progress synchronization.

This SRS covers the requirements for the Promethea product family and apportions them across proposed milestones. It does not define visual design mockups, provider-specific contracts, legal advice for copyrighted material, exact hosting topology, or final database technology until those decisions are confirmed.

```text
Supported user-facing shape:

Browser / Desktop / Mobile-oriented UI
                |
                v
        Promethea REST API Server
                |
        +-------+--------+
        |                |
   Catalog database   Managed assets
                    EPUBs, covers,
                    author images,
                    backups/versions
```

### 1.3 Definitions, Acronyms, and Abbreviations

| Term | Definition |
|------|------------|
| API | Application Programming Interface; a defined way for software components to communicate. |
| Asset | A managed binary file such as an EPUB, cover image, author image, original import, or previous EPUB version. |
| Book | A catalog entity representing a readable work or title in the library. Final work/edition/file modeling details are TBD. |
| Calibre | Existing e-book management application whose personal library-management workflows Promethea aims to replace. |
| calibre-web | Existing web UI for Calibre libraries whose browsing workflows Promethea aims to replace. |
| EPUB | Electronic Publication file format used as the initial supported e-book file format. |
| Metadata Candidate | Metadata returned by an external source and presented for review or automation before being applied. |
| POC | Proof of Concept. |
| REST | Representational State Transfer; the API style selected for client-server communication. |
| SRS | Software Requirements Specification. |
| Series | A named collection, sequence, universe, or related group of books. |
| UI | User Interface. |
| Work / Edition / File | Possible domain split where a work is the intellectual content, an edition is a specific publication, and a file is a stored digital artifact. Exact model is TBD. |

### 1.4 References

| Reference | Owner/Author | Version/Date | Type | Location |
|-----------|--------------|--------------|------|----------|
| Project requirements draft (`srs.md`) | Luca Antonelli | 2026-05-26 | Normative for product intent | [`/docs/srs.md`](./srs.md) |
| Requirement template (`req-template.md`) | Luca Antonelli/template source | 2026-05-26 | Normative for individual requirement format | [`/docs/requirements/req-template.md`](./requirements/req-template.md) |

### 1.5 Document Overview

Section 2 describes Promethea in context, including user classes, assumptions, dependencies, constraints, and milestone apportioning. Section 3 defines all requirements using unique IDs, status, rationale, acceptance criteria, verification method, and supporting notes. Section 4 provides a verification matrix so tests and other evidence can be traced back to requirements. Section 5 captures open questions and a requirement index for future refinement.

## 2. Product Overview

### 2.1 Product Perspective

Promethea is a new product intended to consolidate personal e-book management, web library browsing, and reading-status tracking into one self-hostable product. It is positioned as a replacement for selected personal workflows from Calibre, Goodreads, and calibre-web, with future possible expansion into Tolino-cloud-like e-reader sync and Audiobookshelf-like audiobook management.

The expected architecture is a Rust backend exposing a REST API, a React TypeScript frontend reused across as many deployment contexts as practical, and persistent storage for catalog data plus managed file assets. The server is expected to be authoritative for remote clients; browser, desktop, and mobile-oriented clients should not directly modify the server database. Ownership, service-level agreements, and support model are TBD; the current assumption is a personal or small-household single-user self-hosted product rather than a public multi-tenant SaaS.

### 2.2 Product Functions

Promethea enables users to:

- Import EPUB files and manage them as a personal library.
- View books in card-based catalog views with cover images.
- View authors, author images, and all books associated with an author.
- View series, all books in a series, and related/contained/overarching series.
- Edit catalog metadata and write supported metadata back into EPUB files.
- Edit selected EPUB content, with safe file versioning.
- Fetch metadata, covers, and author images from online sources or pasted image URLs.
- Track owned books, reading status, completed reads, progress events, and reading history.
- Produce analytics for books/pages read by period, books grouped by author, and reading speed grouped by author.
- Define future automation rules for book-import events, metadata fetching, and EPUB search-and-replace.
- Support future desktop, mobile-oriented, e-reader sync, audiobook, and cross-medium progress workflows.

### 2.3 Product Constraints

The following constraints shape the requirements in Section 3:

- The backend must be implemented in Rust.
- The server API must be REST-based.
- The frontend must be implemented with React and TypeScript.
- The frontend should be reused across browser, desktop, and mobile-oriented contexts as much as practical.
- EPUB is the initial required e-book file format.
- Clients should communicate changes through the server API or approved local embedded-backend interface, not by directly editing the server database file.
- The system should not require a paid third-party service for core self-hosted use.
- Provider-specific metadata and image fetching depends on external services that may be unavailable, rate-limited, or subject to terms of service.
- Exact authentication model, database engine, target scale, supported operating systems, accessibility standard, and e-reader protocol support remain open decisions.

### 2.4 User Characteristics

| User Class | Characteristics | Primary Goals | Access Level |
|------------|-----------------|---------------|--------------|
| Library Owner / Administrator | Technical enough to install or configure a self-hosted application; may also be the main reader. | Import books, maintain metadata, configure providers, back up data, manage sync/settings. | Full administrative access. |
| Reader / Authenticated User | Uses the library through browser or future client; may be less technical. | Browse books, update reading status, view reading analytics, download/sync books. | Access to own or permitted library data; exact role model TBD. |
| Maintainer / Developer | Contributes to implementation, testing, release, and operations. | Understand requirements, implement features, verify changes, maintain migrations. | Repository and development environment access. |
| Future Device-Sync User | Uses desktop/server workflows to copy books to e-readers or sync targets. | Send selected books to e-reader or folder target and inspect sync history. | Authenticated user or administrator; exact permission model TBD. |

Accessibility needs are not yet specified. At minimum, UI requirements should be refined to include keyboard accessibility, readable error states, and an agreed accessibility target before a stable release.

### 2.5 Assumptions and Dependencies

| ID | Assumption or Dependency | Impact if False | Mitigation / Follow-up |
|----|--------------------------|-----------------|------------------------|
| A-001 | Initial deployment is personal/self-hosted or small-household, not public multi-tenant SaaS. | Security, scalability, and admin requirements would need expansion. | Confirm target deployment model. |
| A-002 | EPUB is the first required e-book format. | If PDF/MOBI/AZW are required early, import, metadata, and editing requirements expand. | Confirm file-format roadmap. |
| A-003 | Users have rights to upload, modify, and sync their own e-book files. | Legal/compliance documentation may need stricter controls. | Add user-content responsibility notice. |
| A-004 | External metadata/image providers may be unavailable or rate-limited. | Core workflows must continue manually without providers. | Keep manual editing mandatory; provider integrations optional. |
| A-005 | Mobile support can initially be responsive web/PWA rather than native app. | Native packaging scope would increase. | Confirm mobile strategy before v0.7/v0.8. |
| A-006 | Browser reading is not required for Tolino-cloud replacement scope. | Adding browser reading would create a major reader UI/rendering feature set. | Keep browser reading out of initial scope unless approved. |
| A-007 | A single authoritative server database may be sufficient if clients communicate through REST. | If multi-instance or high-write deployment is required, database architecture may need Postgres or similar. | Run SQLite POC and confirm expected scale. |
| A-008 | Exact metadata providers are not yet chosen. | Provider-specific fields, credentials, rate limits, and attribution cannot be fully specified. | Select providers and add provider interface specs. |
| A-009 | Exact e-reader sync target/protocol is not yet chosen. | Device sync requirements remain deferred and high-level. | Validate target devices and protocols before v0.8. |
| A-010 | No core AI/ML model is required initially. | If AI/ML matching or recommendations are added, Section 3.6 must be expanded. | Require ML change review before adding models. |

### 2.6 Apportioning of Requirements

| Milestone | Scope Summary | Primary Requirement IDs | Status |
|-----------|---------------|-------------------------|--------|
| v0.1 Foundation & Data Model | Backend/frontend skeleton, REST API baseline, database/storage setup, migration approach, core model | REQ-INT-003, REQ-INT-008, REQ-FUNC-002, REQ-BUILD-001, REQ-BUILD-002, REQ-BUILD-003, REQ-MAINT-001, REQ-MAINT-002, REQ-POC-001 | planned/draft |
| v0.2 Web Library Alpha | EPUB import, catalog, book/author/series browsing, card views, basic search | REQ-FUNC-001 through REQ-FUNC-018, REQ-INT-001, REQ-INT-004 | planned/proposed |
| v0.3 Reading Tracker Alpha | Owned books, reading status, completed reads, progress/events, analytics foundation | REQ-FUNC-019 through REQ-FUNC-027, REQ-COMP-003 | planned/proposed |
| v0.4 EPUB Management Beta | EPUB metadata writing, content editing, file versioning, safe modification, related series | REQ-FUNC-005 through REQ-FUNC-007, REQ-FUNC-014, REQ-FUNC-015, REQ-REL-002 | planned |
| v0.5 Metadata, Import & Automation | Metadata provider abstraction, candidate review, author images, duplicate detection, automation, jobs | REQ-INT-005, REQ-INT-006, REQ-FUNC-008 through REQ-FUNC-010, REQ-FUNC-028 through REQ-FUNC-032 | planned/deferred |
| v0.6 Self-Hosted Server Release | Authentication, HTTPS deployment support, container install, backup/restore, observability | REQ-SEC-001 through REQ-SEC-005, REQ-INST-001 through REQ-INST-003, REQ-AVAIL-001, REQ-AVAIL-002, REQ-OBS-001, REQ-OBS-002 | planned/proposed |
| v0.7 Desktop App Alpha | Desktop/laptop distribution, shared UI reuse, local workflows | REQ-DIST-002, REQ-REUSE-001, REQ-REUSE-002, REQ-PORT-001 | deferred/proposed |
| v0.8 E-reader / Device Sync | Folder/mounted device sync target and sync job history | REQ-INT-007, REQ-FUNC-033 | deferred |
| v1.0 Stable Personal Library Release | Stabilization across library, reading, server, backup, security, docs | All non-deferred planned/proposed requirements accepted for v1.0 | planned |
| v1.1+ Audiobooks & Multi-Medium Support | Audiobook support, medium tracking, e-book/audiobook progress sync | REQ-FUNC-034, REQ-FUNC-035, REQ-FUNC-036 | deferred |

## 3. Requirements

Requirement entries in this section use the supplied requirement template fields: ID, status, date, title, statement, rationale, acceptance criteria, verification method, and more information. Status values use the supplied status vocabulary: `draft`, `proposed`, `deferred`, `planned`, `in-progress`, `blocked`, `passed`, `failed`, or `waived`.

### 3.1 External Interfaces

This section defines user, hardware, and software-facing interfaces at a logical level.

#### 3.1.1 User Interfaces

| ID | Title | 
|----|-------|
| [REQ-INT-001](./requirements/interface/REQ-INT-001.md) | Browser-based library user interface |
| [REQ-INT-002](./requirements/interface/REQ-INT-002.md) | Responsive user interface |


#### 3.1.2 Hardware Interfaces

| ID | Title |
|----|-------|
| [REQ-INT-007](./requirements/interface/REQ-INT-007.md) | Mounted-folder e-reader sync interface |
#### 3.1.3 Software Interfaces
| ID | Title |
|----|-------|
| [REQ-INT-003](./requirements/interface/REQ-INT-003.md) | REST API interface |
| [REQ-INT-004](./requirements/interface/REQ-INT-004.md) | File import and download interface |
| [REQ-INT-005](./requirements/interface/REQ-INT-005.md) | External metadata provider interface |
| [REQ-INT-006](./requirements/interface/REQ-INT-006.md) | External image URL download interface |
| [REQ-INT-008](./requirements/interface/REQ-INT-008.md) | Client database isolation |

### 3.2 Functional

This section defines externally observable product behavior grouped by library management, browsing, reading tracking, automation, and future media support.

#### 3.2.1 Library Management and EPUB Processing
| ID | Title |
|----|-------|
| [REQ-FUNC-001](./requirements/functional/REQ-FUNC-001.md) | Import EPUB files |
| [REQ-FUNC-002](./requirements/functional/REQ-FUNC-002.md) | Persist catalog records and assets |
| [REQ-FUNC-003](./requirements/functional/REQ-FUNC-003.md) | Extract EPUB metadata |
| [REQ-FUNC-004](./requirements/functional/REQ-FUNC-004.md) | Edit catalog metadata |
| [REQ-FUNC-005](./requirements/functional/REQ-FUNC-005.md) | Write metadata to EPUB files |
| [REQ-FUNC-006](./requirements/functional/REQ-FUNC-006.md) | Edit EPUB content |
| [REQ-FUNC-007](./requirements/functional/REQ-FUNC-007.md) | Version EPUB modification |
| [REQ-FUNC-008](./requirements/functional/REQ-FUNC-008.md) | Fetch online metadata |
| [REQ-FUNC-009](./requirements/functional/REQ-FUNC-009.md) | Review metadata candidates |
| [REQ-FUNC-010](./requirements/functional/REQ-FUNC-010.md) | Detect duplicate imported books |


#### 3.2.2 Browsing, Authors, and Series
| ID | Title |
|----|-------|
| [REQ-FUNC-011](./requirements/functional/REQ-FUNC-011.md) | Display book card view |
| [REQ-FUNC-012](./requirements/functional/REQ-FUNC-012.md) | Display author detail view |
| [REQ-FUNC-013](./requirements/functional/REQ-FUNC-013.md) | Manage author images |
| [REQ-FUNC-014](./requirements/functional/REQ-FUNC-014.md) | Display series detail view |
| [REQ-FUNC-015](./requirements/functional/REQ-FUNC-015.md) | Support related series |
| [REQ-FUNC-016](./requirements/functional/REQ-FUNC-016.md) | Display author overview |
| [REQ-FUNC-017](./requirements/functional/REQ-FUNC-017.md) | Display series overview |
| [REQ-FUNC-018](./requirements/functional/REQ-FUNC-018.md) | Search and filter library |

#### 3.2.3 Reading Tracking and Analytics
| ID | Title |
|----|-------|
| [REQ-FUNC-019](./requirements/functional/REQ-FUNC-019.md) | Track owned books |
| [REQ-FUNC-020](./requirements/functional/REQ-FUNC-020.md) | Track reading status |
| [REQ-FUNC-021](./requirements/functional/REQ-FUNC-021.md) | Track completed reads |
| [REQ-FUNC-022](./requirements/functional/REQ-FUNC-022.md) | Track reading progress events |
| [REQ-FUNC-023](./requirements/functional/REQ-FUNC-023.md) | Analyze books read per period |
| [REQ-FUNC-024](./requirements/functional/REQ-FUNC-024.md) | Analyze pages read per period |
| [REQ-FUNC-025](./requirements/functional/REQ-FUNC-025.md) | Analyze books by author |
| [REQ-FUNC-026](./requirements/functional/REQ-FUNC-026.md) | Analyze reading speed by author |
| [REQ-FUNC-027](./requirements/functional/REQ-FUNC-027.md) | Support remote updates through server |

#### 3.2.4 Automation and Job Processing
| ID | Title |
|----|-------|
| [REQ-FUNC-028](./requirements/functional/REQ-FUNC-028.md) | Define automation rules on book import |
| [REQ-FUNC-029](./requirements/functional/REQ-FUNC-029.md) | Automation action for metadata fetching |
| [REQ-FUNC-030](./requirements/functional/REQ-FUNC-030.md) | Automation action for EPUB search-and-replace |
| [REQ-FUNC-031](./requirements/functional/REQ-FUNC-031.md) | Run long operations as background jobs |
| [REQ-FUNC-032](./requirements/functional/REQ-FUNC-032.md) | Display job status |

#### 3.2.5 Deferred Device and Media Features
| ID | Title |
|----|-------|
| [REQ-FUNC-033](./requirements/functional/REQ-FUNC-033.md) | Synchronize books to e-reader targets |
| [REQ-FUNC-034](./requirements/functional/REQ-FUNC-034.md) | Support audiobook records |
| [REQ-FUNC-035](./requirements/functional/REQ-FUNC-035.md) | Track book medium |
| [REQ-FUNC-036](./requirements/functional/REQ-FUNC-036.md) | Sync progress between e-book and audiobook |

### 3.3 Quality of Service

This section defines performance, security, reliability, availability, and observability expectations. Requirements with status `draft` need owner confirmation before they can be treated as release gates.

#### 3.3.1 Performance
| ID | Title |
| -------------- | --------------- |
| [REQ-PERF-001](./requirements/performance/REQ-PERF-001.md) | Catalog browsing latency target |
| [REQ-PERF-002](./requirements/performance/REQ-PERF-002.md) | Non-blocking long-running operations |
| [REQ-PERF-003](./requirements/performance/REQ-PERF-003.md) | Asset storage growth visibility |


#### 3.3.2 Security
| ID | Title |
|--------------- | --------------- |
| [REQ-SEC-001](./requirements/security/REQ-SEC-001.md)   | Authenticate protected operations  |
| [REQ-SEC-002](./requirements/security/REQ-SEC-002.md)   | Authorize user operations   |
| [REQ-SEC-003](./requirements/security/REQ-SEC-003.md)   | Validate external inputs   |
| [REQ-SEC-004](./requirements/security/REQ-SEC-004.md)   |  Protect secrets and provider credentials  |
| [REQ-SEC-005](./requirements/security/REQ-SEC-005.md) | Support secure deployment over HTTPS |


#### 3.3.3 Reliability
| ID   | Title |
|--------------- | --------------- |
| [REQ-REL-001](./requirements/reliability/REQ-REL-001.md)   | Transactional catalog mutations   |
| [REQ-REL-002](./requirements/reliability/REQ-REL-002.md)   | Recover from failed EPUB mutation   |
| [REQ-REL-003](./requirements/reliability/REQ-REL-003.md)   | Handle metadata provider failure gracefully   |
| [REQ-REL-004](./requirements/reliability/REQ-REL-004.md)   |  Idempotent import by checksum  |


#### 3.3.4 Availability
| ID | Title |
| -------------- | --------------- |
| [REQ-AVAIL-001](./requirements/availability/REQ-AVAIL-001.md) | Restart recovery |
| [REQ-AVAIL-002](./requirements/availability/REQ-AVAIL-002.md) | Backup and restore support |


#### 3.3.5 Observability
| ID | Title |
| -------------- | --------------- |
| [REQ-OBS-001](./requirements/observability/REQ-OBS-001.md) | Structured server logs |
| [REQ-OBS-002](./requirements/observability/REQ-OBS-002.md) | User-visible operation history |


### 3.4 Compliance
This section defines compliance-oriented requirements derived from user-content handling, external metadata/image sources, and privacy of reading data.
| ID | Title |
| -------------- | --------------- |
| [REQ-COMP-001](./requirements/compliance/REQ-COMP-001.md) | External source attribution |
| [REQ-COMP-002](./requirements/compliance/REQ-COMP-002.md) | User-content responsibility notice |
| [REQ-COMP-003](./requirements/compliance/REQ-COMP-003.md) | Privacy of reading data |


### 3.5 Design and Implementation

This section captures binding implementation constraints and delivery expectations supplied by the project description or architecture discussion.

#### 3.5.1 Installation
| ID | Title |
| -------------- | --------------- |
| [REQ-INST-001](./requirements/installation/REQ-INST-001.md) | Self-hosted server installation |
| [REQ-INST-002](./requirements/installation/REQ-INST-002.md) | Configurable data directory |
| [REQ-INST-003](./requirements/installation/REQ-INST-003.md) | Containerized deployment package |


#### 3.5.2 Build and Delivery
| ID   | Title |
|--------------- | --------------- |
| [REQ-BUILD-001](./requirements/build/REQ-BUILD-001.md)   | Rust backend   |
| [REQ-BUILD-002](./requirements/build/REQ-BUILD-002.md)   | REST server   |
| [REQ-BUILD-003](./requirements/build/REQ-BUILD-003.md)   | React TypeScript frontend   |
| [REQ-BUILD-004](./requirements/build/REQ-BUILD-004.md)   | Continuous integration checks  |


#### 3.5.3 Distribution
| ID | Title |
| -------------- | --------------- |
| [REQ-DIST-001](./requirements/distribution/REQ-DIST-001.md) | Server-browser deployment topology |
| [REQ-DIST-002](./requirements/distribution/REQ-DIST-002.md) | Desktop application distribution |
| [REQ-DIST-003](./requirements/distribution/REQ-DIST-003.md) | Mobile availability |


#### 3.5.4 Maintainability
| ID | Title |
| -------------- | --------------- |
| [REQ-MAINT-001](./requirements/maintainability/REQ-MAINT-001.md) | Modular backend organization |
| [REQ-MAINT-002](./requirements/maintainability/REQ-MAINT-002.md) | Database migrations |


#### 3.5.5 Reusability
| ID | Title |
| -------------- | --------------- |
| [REQ-REUSE-001](./requirements/reusability/REQ-REUSE-001.md) | Shared frontend reuse |
| [REQ-REUSE-002](./requirements/reusability/REQ-REUSE-002.md) | Reusable Rust core |


#### 3.5.6 Portability
| ID | Title |
| -------------- | --------------- |
| [REQ-PORT-001](./requirements/portability/REQ-PORT-001.md) | Supported operating environments |


#### 3.5.7 Cost

##### REQ-COST-001 — No mandatory paid third-party service for core use
- ID: REQ-COST-001
- Status: proposed
- Date: 2026-05-26
- Title: No mandatory paid third-party service for core use
- Statement: The system shall not require a mandatory paid third-party service for core self-hosted library management, reading tracking, and browsing features.
- Rationale: A personal self-hosted library should remain usable without recurring external service dependencies.
- Acceptance Criteria:
  - Core import, browse, edit-catalog, and reading-tracker features operate without paid external services.
  - Optional metadata providers may require user-supplied credentials without blocking manual operation.
  - Documentation identifies optional paid or rate-limited integrations if any are added.
- Verification Method: Inspection
- More Information: Hosting infrastructure cost is outside application control.


#### 3.5.8 Deadline

##### REQ-DEAD-001 — Milestone release sequence
- ID: REQ-DEAD-001
- Status: proposed
- Date: 2026-05-26
- Title: Milestone release sequence
- Statement: The project shall organize delivery using the milestone sequence documented in Section 2.6 unless changed through project change management.
- Rationale: The project has multiple product goals; milestones keep scope incremental and verifiable.
- Acceptance Criteria:
  - GitHub milestones or equivalent planning artifacts exist for the documented sequence.
  - Deferred requirements are assigned to a future milestone or explicitly left unscheduled.
  - Each milestone has release-readiness criteria before work begins.
- Verification Method: Inspection
- More Information: No calendar deadlines have been provided; this is a sequence requirement, not a date commitment.


#### 3.5.9 Proof of Concept

##### REQ-POC-001 — SQLite server data-store proof of concept
- ID: REQ-POC-001
- Status: draft
- Date: 2026-05-26
- Title: SQLite server data-store proof of concept
- Statement: The project shall validate whether a single authoritative server using a SQLite database satisfies expected concurrency and reliability needs before committing to SQLite for stable server releases.
- Rationale: The architecture discussion identified SQLite as likely sufficient but exact workload and operational requirements remain unspecified.
- Acceptance Criteria:
  - The POC uses the same server-only write boundary required by REQ-INT-008.
  - The POC includes concurrent read and write scenarios representative of expected use.
  - The POC records whether SQLite is accepted, rejected, or accepted with constraints for the stable release.
- Verification Method: Analysis
- More Information: Clarification needed: expected concurrent users, write rate, background job load, and backup expectations.


#### 3.5.10 Change Management

##### REQ-CM-001 — Versioned change management
- ID: REQ-CM-001
- Status: proposed
- Date: 2026-05-26
- Title: Versioned change management
- Statement: The project shall document breaking, additive, and bugfix changes in release notes and migration notes for released versions.
- Rationale: Users managing a long-lived library need to understand upgrade effects and data migrations.
- Acceptance Criteria:
  - Each release includes release notes.
  - Breaking changes identify user-visible impact and migration steps.
  - Database or asset-format migrations are documented with rollback or backup guidance where applicable.
- Verification Method: Inspection
- More Information: Formal approval workflow is TBD.


### 3.6 AI/ML

The current product description does not require a core AI/ML model. If future metadata matching, recommendation, or classification capabilities use AI/ML, additional model specification, data management, ethics, and lifecycle requirements must be added before release.

#### 3.6.1 Model Specification

No AI/ML model is specified for the initial release.

#### 3.6.2 Data Management

No AI/ML training, validation, or inference dataset is specified for the initial release.

#### 3.6.3 Guardrails

##### REQ-ML-001 — No autonomous AI/ML catalog mutation without approval
- ID: REQ-ML-001
- Status: proposed
- Date: 2026-05-26
- Title: No autonomous AI/ML catalog mutation without approval
- Statement: The system shall not use AI/ML or heuristic metadata matching to make irreversible catalog, file, or reading-data changes without explicit user approval or an enabled automation rule that documents the allowed action.
- Rationale: The current product description does not require AI/ML, and metadata matching can be wrong; human control protects the library.
- Acceptance Criteria:
  - Metadata candidates are reviewable before application unless automation explicitly permits application.
  - Automation rules identify the actions they may perform.
  - Irreversible or destructive operations retain rollback/versioning where applicable.
- Verification Method: Inspection
- More Information: No core AI/ML model is specified for the initial release; if added, additional ML lifecycle requirements must be written.


#### 3.6.4 Ethics

No AI/ML-specific ethics requirements are applicable until an AI/ML component is proposed. General privacy and authorization requirements still apply.

#### 3.6.5 Human-in-the-Loop

Human review is required for metadata candidate application unless a user-enabled automation rule permits a narrower automatic action. See REQ-FUNC-009, REQ-FUNC-028, and REQ-ML-001.

#### 3.6.6 Model Lifecycle and Operations

No AI/ML model lifecycle or operations requirements are applicable until an AI/ML component is proposed.

## 4. Verification

Verification artifacts should be created as the implementation matures. The paths below are proposed traceability placeholders, not existing evidence unless later populated.

| Requirement ID | Verification Method | Test/Artifact Link | Status | Evidence |
|----------------|---------------------|--------------------|--------|----------|
| REQ-INT-001 | Demonstration | tests/requirements/req-int-001.md | planned | TBD |
| REQ-INT-002 | Test | tests/requirements/req-int-002.md | proposed | TBD |
| REQ-INT-003 | Inspection | tests/requirements/req-int-003.md | planned | TBD |
| REQ-INT-004 | Test | tests/requirements/req-int-004.md | planned | TBD |
| REQ-INT-005 | Test | tests/requirements/req-int-005.md | planned | TBD |
| REQ-INT-006 | Test | tests/requirements/req-int-006.md | planned | TBD |
| REQ-INT-007 | Demonstration | tests/requirements/req-int-007.md | deferred | TBD |
| REQ-INT-008 | Inspection | tests/requirements/req-int-008.md | proposed | TBD |
| REQ-FUNC-001 | Test | tests/requirements/req-func-001.md | planned | TBD |
| REQ-FUNC-002 | Test | tests/requirements/req-func-002.md | planned | TBD |
| REQ-FUNC-003 | Test | tests/requirements/req-func-003.md | planned | TBD |
| REQ-FUNC-004 | Test | tests/requirements/req-func-004.md | planned | TBD |
| REQ-FUNC-005 | Test | tests/requirements/req-func-005.md | planned | TBD |
| REQ-FUNC-006 | Demonstration | tests/requirements/req-func-006.md | planned | TBD |
| REQ-FUNC-007 | Test | tests/requirements/req-func-007.md | planned | TBD |
| REQ-FUNC-008 | Test | tests/requirements/req-func-008.md | planned | TBD |
| REQ-FUNC-009 | Demonstration | tests/requirements/req-func-009.md | planned | TBD |
| REQ-FUNC-010 | Test | tests/requirements/req-func-010.md | proposed | TBD |
| REQ-FUNC-011 | Demonstration | tests/requirements/req-func-011.md | planned | TBD |
| REQ-FUNC-012 | Demonstration | tests/requirements/req-func-012.md | planned | TBD |
| REQ-FUNC-013 | Test | tests/requirements/req-func-013.md | planned | TBD |
| REQ-FUNC-014 | Demonstration | tests/requirements/req-func-014.md | planned | TBD |
| REQ-FUNC-015 | Test | tests/requirements/req-func-015.md | planned | TBD |
| REQ-FUNC-016 | Demonstration | tests/requirements/req-func-016.md | planned | TBD |
| REQ-FUNC-017 | Demonstration | tests/requirements/req-func-017.md | planned | TBD |
| REQ-FUNC-018 | Test | tests/requirements/req-func-018.md | proposed | TBD |
| REQ-FUNC-019 | Test | tests/requirements/req-func-019.md | planned | TBD |
| REQ-FUNC-020 | Test | tests/requirements/req-func-020.md | planned | TBD |
| REQ-FUNC-021 | Test | tests/requirements/req-func-021.md | planned | TBD |
| REQ-FUNC-022 | Test | tests/requirements/req-func-022.md | proposed | TBD |
| REQ-FUNC-023 | Test | tests/requirements/req-func-023.md | planned | TBD |
| REQ-FUNC-024 | Analysis | tests/requirements/req-func-024.md | planned | TBD |
| REQ-FUNC-025 | Test | tests/requirements/req-func-025.md | planned | TBD |
| REQ-FUNC-026 | Analysis | tests/requirements/req-func-026.md | planned | TBD |
| REQ-FUNC-027 | Demonstration | tests/requirements/req-func-027.md | planned | TBD |
| REQ-FUNC-028 | Test | tests/requirements/req-func-028.md | deferred | TBD |
| REQ-FUNC-029 | Test | tests/requirements/req-func-029.md | deferred | TBD |
| REQ-FUNC-030 | Test | tests/requirements/req-func-030.md | deferred | TBD |
| REQ-FUNC-031 | Test | tests/requirements/req-func-031.md | planned | TBD |
| REQ-FUNC-032 | Demonstration | tests/requirements/req-func-032.md | planned | TBD |
| REQ-FUNC-033 | Demonstration | tests/requirements/req-func-033.md | deferred | TBD |
| REQ-FUNC-034 | Inspection | tests/requirements/req-func-034.md | deferred | TBD |
| REQ-FUNC-035 | Inspection | tests/requirements/req-func-035.md | deferred | TBD |
| REQ-FUNC-036 | Analysis | tests/requirements/req-func-036.md | deferred | TBD |
| REQ-PERF-001 | Analysis | tests/requirements/req-perf-001.md | draft | TBD |
| REQ-PERF-002 | Test | tests/requirements/req-perf-002.md | proposed | TBD |
| REQ-PERF-003 | Demonstration | tests/requirements/req-perf-003.md | proposed | TBD |
| REQ-SEC-001 | Test | tests/requirements/req-sec-001.md | planned | TBD |
| REQ-SEC-002 | Test | tests/requirements/req-sec-002.md | draft | TBD |
| REQ-SEC-003 | Test | tests/requirements/req-sec-003.md | planned | TBD |
| REQ-SEC-004 | Inspection | tests/requirements/req-sec-004.md | proposed | TBD |
| REQ-SEC-005 | Inspection | tests/requirements/req-sec-005.md | proposed | TBD |
| REQ-REL-001 | Test | tests/requirements/req-rel-001.md | planned | TBD |
| REQ-REL-002 | Test | tests/requirements/req-rel-002.md | planned | TBD |
| REQ-REL-003 | Test | tests/requirements/req-rel-003.md | planned | TBD |
| REQ-REL-004 | Test | tests/requirements/req-rel-004.md | proposed | TBD |
| REQ-AVAIL-001 | Test | tests/requirements/req-avail-001.md | proposed | TBD |
| REQ-AVAIL-002 | Demonstration | tests/requirements/req-avail-002.md | planned | TBD |
| REQ-OBS-001 | Inspection | tests/requirements/req-obs-001.md | proposed | TBD |
| REQ-OBS-002 | Demonstration | tests/requirements/req-obs-002.md | proposed | TBD |
| REQ-COMP-001 | Test | tests/requirements/req-comp-001.md | proposed | TBD |
| REQ-COMP-002 | Inspection | tests/requirements/req-comp-002.md | proposed | TBD |
| REQ-COMP-003 | Test | tests/requirements/req-comp-003.md | planned | TBD |
| REQ-INST-001 | Demonstration | tests/requirements/req-inst-001.md | planned | TBD |
| REQ-INST-002 | Test | tests/requirements/req-inst-002.md | planned | TBD |
| REQ-INST-003 | Demonstration | tests/requirements/req-inst-003.md | planned | TBD |
| REQ-BUILD-001 | Inspection | tests/requirements/req-build-001.md | planned | TBD |
| REQ-BUILD-002 | Inspection | tests/requirements/req-build-002.md | planned | TBD |
| REQ-BUILD-003 | Inspection | tests/requirements/req-build-003.md | planned | TBD |
| REQ-BUILD-004 | Inspection | tests/requirements/req-build-004.md | proposed | TBD |
| REQ-DIST-001 | Demonstration | tests/requirements/req-dist-001.md | planned | TBD |
| REQ-DIST-002 | Demonstration | tests/requirements/req-dist-002.md | deferred | TBD |
| REQ-DIST-003 | Demonstration | tests/requirements/req-dist-003.md | deferred | TBD |
| REQ-MAINT-001 | Inspection | tests/requirements/req-maint-001.md | proposed | TBD |
| REQ-MAINT-002 | Test | tests/requirements/req-maint-002.md | planned | TBD |
| REQ-REUSE-001 | Inspection | tests/requirements/req-reuse-001.md | planned | TBD |
| REQ-REUSE-002 | Inspection | tests/requirements/req-reuse-002.md | proposed | TBD |
| REQ-PORT-001 | Inspection | tests/requirements/req-port-001.md | draft | TBD |
| REQ-COST-001 | Inspection | tests/requirements/req-cost-001.md | proposed | TBD |
| REQ-DEAD-001 | Inspection | tests/requirements/req-dead-001.md | proposed | TBD |
| REQ-POC-001 | Analysis | tests/requirements/req-poc-001.md | draft | TBD |
| REQ-CM-001 | Inspection | tests/requirements/req-cm-001.md | proposed | TBD |
| REQ-ML-001 | Inspection | tests/requirements/req-ml-001.md | proposed | TBD |

## 5. Appendixes

### Appendix A: Open Questions and Clarifications Needed

The following items are not supplied in the source material and should be answered before the affected requirements are treated as final release gates:

1. What should appear as the official author and organization on the SRS cover page?
2. Is Promethea the final project name?
3. Is the target deployment single-user, household/multi-user, or future public multi-tenant?
4. What authentication model is desired: single owner account, local users, OAuth/OIDC, reverse-proxy auth, or something else?
5. What database should the initial stable release target, and should SQLite be a formal requirement or only an allowed implementation?
6. What is the expected library size for performance tests: hundreds, thousands, tens of thousands, or more books?
7. How many concurrent users or clients should the self-hosted server support?
8. Which metadata providers should be supported first?
9. Which image providers or URL restrictions should apply to cover and author-image downloads?
10. Which EPUB content-editing workflows are required first: full editor, OPF metadata only, cover replacement, XHTML search-and-replace, or another subset?
11. What exact reading statuses are required beyond unread/currently reading/read/abandoned?
12. Should reading analytics count rereads separately from first reads?
13. How should books with multiple authors be counted in analytics?
14. What page-count and reading-speed calculation rules should be used?
15. Which accessibility standard or target should the UI meet?
16. Which operating systems are release targets for server and desktop builds?
17. Should mobile be responsive web only, PWA, Tauri mobile, Capacitor, or another native wrapper?
18. Which e-reader devices and sync protocols are in scope for the first device-sync milestone?
19. What backup retention and restore expectations are required?
20. What license should the project use, and what third-party dependency license policy should apply?

### Appendix B: Requirement Index

| Requirement ID | Title | Status | Section |
|----------------|-------|--------|---------|
| REQ-INT-001 | Browser-based library user interface | planned | 3.1 External Interfaces |
| REQ-INT-002 | Responsive user interface | proposed | 3.1 External Interfaces |
| REQ-INT-003 | REST API interface | planned | 3.1 External Interfaces |
| REQ-INT-004 | File import and download interface | planned | 3.1 External Interfaces |
| REQ-INT-005 | External metadata provider interface | planned | 3.1 External Interfaces |
| REQ-INT-006 | External image URL download interface | planned | 3.1 External Interfaces |
| REQ-INT-007 | Mounted-folder e-reader sync interface | deferred | 3.1 External Interfaces |
| REQ-INT-008 | Client database isolation | proposed | 3.1 External Interfaces |
| REQ-FUNC-001 | Import EPUB files | planned | 3.2 Functional Requirements |
| REQ-FUNC-002 | Persist catalog records and assets | planned | 3.2 Functional Requirements |
| REQ-FUNC-003 | Extract EPUB metadata | planned | 3.2 Functional Requirements |
| REQ-FUNC-004 | Edit catalog metadata | planned | 3.2 Functional Requirements |
| REQ-FUNC-005 | Write metadata to EPUB files | planned | 3.2 Functional Requirements |
| REQ-FUNC-006 | Edit EPUB content | planned | 3.2 Functional Requirements |
| REQ-FUNC-007 | Version EPUB modifications | planned | 3.2 Functional Requirements |
| REQ-FUNC-008 | Fetch online metadata | planned | 3.2 Functional Requirements |
| REQ-FUNC-009 | Review metadata candidates | planned | 3.2 Functional Requirements |
| REQ-FUNC-010 | Detect duplicate imported books | proposed | 3.2 Functional Requirements |
| REQ-FUNC-011 | Display book card view | planned | 3.2 Functional Requirements |
| REQ-FUNC-012 | Display author detail view | planned | 3.2 Functional Requirements |
| REQ-FUNC-013 | Manage author images | planned | 3.2 Functional Requirements |
| REQ-FUNC-014 | Display series detail view | planned | 3.2 Functional Requirements |
| REQ-FUNC-015 | Support related series | planned | 3.2 Functional Requirements |
| REQ-FUNC-016 | Display author overview | planned | 3.2 Functional Requirements |
| REQ-FUNC-017 | Display series overview | planned | 3.2 Functional Requirements |
| REQ-FUNC-018 | Search and filter library | proposed | 3.2 Functional Requirements |
| REQ-FUNC-019 | Track owned books | planned | 3.2 Functional Requirements |
| REQ-FUNC-020 | Track reading status | planned | 3.2 Functional Requirements |
| REQ-FUNC-021 | Track completed reads | planned | 3.2 Functional Requirements |
| REQ-FUNC-022 | Track reading progress events | proposed | 3.2 Functional Requirements |
| REQ-FUNC-023 | Analyze books read per period | planned | 3.2 Functional Requirements |
| REQ-FUNC-024 | Analyze pages read per period | planned | 3.2 Functional Requirements |
| REQ-FUNC-025 | Analyze books by author | planned | 3.2 Functional Requirements |
| REQ-FUNC-026 | Analyze reading speed by author | planned | 3.2 Functional Requirements |
| REQ-FUNC-027 | Support remote updates through server | planned | 3.2 Functional Requirements |
| REQ-FUNC-028 | Define automation rules on book import | deferred | 3.2 Functional Requirements |
| REQ-FUNC-029 | Automation action for metadata fetching | deferred | 3.2 Functional Requirements |
| REQ-FUNC-030 | Automation action for EPUB search-and-replace | deferred | 3.2 Functional Requirements |
| REQ-FUNC-031 | Run long operations as background jobs | planned | 3.2 Functional Requirements |
| REQ-FUNC-032 | Display job status | planned | 3.2 Functional Requirements |
| REQ-FUNC-033 | Synchronize books to e-reader targets | deferred | 3.2 Functional Requirements |
| REQ-FUNC-034 | Support audiobook records | deferred | 3.2 Functional Requirements |
| REQ-FUNC-035 | Track book medium | deferred | 3.2 Functional Requirements |
| REQ-FUNC-036 | Sync progress between e-book and audiobook | deferred | 3.2 Functional Requirements |
| REQ-PERF-001 | Catalog browsing latency target | draft | 3.3 Quality of Service Requirements |
| REQ-PERF-002 | Non-blocking long-running operations | proposed | 3.3 Quality of Service Requirements |
| REQ-PERF-003 | Asset storage growth visibility | proposed | 3.3 Quality of Service Requirements |
| REQ-SEC-001 | Authenticate protected operations | planned | 3.3 Quality of Service Requirements |
| REQ-SEC-002 | Authorize user operations | draft | 3.3 Quality of Service Requirements |
| REQ-SEC-003 | Validate external inputs | planned | 3.3 Quality of Service Requirements |
| REQ-SEC-004 | Protect secrets and provider credentials | proposed | 3.3 Quality of Service Requirements |
| REQ-SEC-005 | Support secure deployment over HTTPS | proposed | 3.3 Quality of Service Requirements |
| REQ-REL-001 | Transactional catalog mutations | planned | 3.3 Quality of Service Requirements |
| REQ-REL-002 | Recover from failed EPUB modification | planned | 3.3 Quality of Service Requirements |
| REQ-REL-003 | Handle metadata provider failure gracefully | planned | 3.3 Quality of Service Requirements |
| REQ-REL-004 | Idempotent import by checksum | proposed | 3.3 Quality of Service Requirements |
| REQ-AVAIL-001 | Restart recovery | proposed | 3.3 Quality of Service Requirements |
| REQ-AVAIL-002 | Backup and restore support | planned | 3.3 Quality of Service Requirements |
| REQ-OBS-001 | Structured server logs | proposed | 3.3 Quality of Service Requirements |
| REQ-OBS-002 | User-visible operation history | proposed | 3.3 Quality of Service Requirements |
| REQ-COMP-001 | External source attribution | proposed | 3.4 Compliance Requirements |
| REQ-COMP-002 | User-content responsibility notice | proposed | 3.4 Compliance Requirements |
| REQ-COMP-003 | Privacy of reading data | planned | 3.4 Compliance Requirements |
| REQ-INST-001 | Self-hosted server installation | planned | 3.5 Design and Implementation Requirements |
| REQ-INST-002 | Configurable data directory | planned | 3.5 Design and Implementation Requirements |
| REQ-INST-003 | Containerized deployment package | planned | 3.5 Design and Implementation Requirements |
| REQ-BUILD-001 | Rust backend | planned | 3.5 Design and Implementation Requirements |
| REQ-BUILD-002 | REST server | planned | 3.5 Design and Implementation Requirements |
| REQ-BUILD-003 | React TypeScript frontend | planned | 3.5 Design and Implementation Requirements |
| REQ-BUILD-004 | Continuous integration checks | proposed | 3.5 Design and Implementation Requirements |
| REQ-DIST-001 | Server-browser deployment topology | planned | 3.5 Design and Implementation Requirements |
| REQ-DIST-002 | Desktop application distribution | deferred | 3.5 Design and Implementation Requirements |
| REQ-DIST-003 | Mobile availability | deferred | 3.5 Design and Implementation Requirements |
| REQ-MAINT-001 | Modular backend organization | proposed | 3.5 Design and Implementation Requirements |
| REQ-MAINT-002 | Database migrations | planned | 3.5 Design and Implementation Requirements |
| REQ-REUSE-001 | Shared frontend reuse | planned | 3.5 Design and Implementation Requirements |
| REQ-REUSE-002 | Reusable Rust core | proposed | 3.5 Design and Implementation Requirements |
| REQ-PORT-001 | Supported operating environments | draft | 3.5 Design and Implementation Requirements |
| REQ-COST-001 | No mandatory paid third-party service for core use | proposed | 3.5 Design and Implementation Requirements |
| REQ-DEAD-001 | Milestone release sequence | proposed | 3.5 Design and Implementation Requirements |
| REQ-POC-001 | SQLite server data-store proof of concept | draft | 3.5 Design and Implementation Requirements |
| REQ-CM-001 | Versioned change management | proposed | 3.5 Design and Implementation Requirements |
| REQ-ML-001 | No autonomous AI/ML catalog mutation without approval | proposed | 3.6 AI/ML Requirements |

### Appendix C: Initial Domain Concepts

The following conceptual entities are implied by the requirements and should be refined during domain-model design:

| Concept | Description |
|---------|-------------|
| Work | Abstract intellectual work, useful for grouping e-book, print, and audiobook editions in future releases. |
| Edition | Specific publication or medium-specific expression of a work. |
| Book File | Stored digital file, initially EPUB. |
| Asset | Cover image, author image, imported original, generated thumbnail, or previous EPUB version. |
| Author | Person or entity associated with books, potentially with role metadata. |
| Series | Named sequence, universe, group, or related collection of books. |
| Reading Event | Event such as started, progress update, completed, abandoned, or status change. |
| Automation Rule | Trigger/condition/action definition executed by the job system. |
| Sync Target | Folder, mounted device, WebDAV endpoint, or other future destination for copied/synced books. |

### Appendix D: Out of Scope for Initial Stable Release Unless Reprioritized

- Browser-based reading/rendering of EPUB content.
- Public SaaS multi-tenancy.
- Native mobile applications beyond responsive or wrapped shared frontend.
- Full Audiobookshelf replacement functionality.
- Cross-medium e-book/audiobook progress synchronization.
- AI/ML-based recommendations or autonomous metadata decisions.
- DRM removal, circumvention, or unauthorized redistribution workflows.
