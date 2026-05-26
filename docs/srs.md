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

##### REQ-INT-001 — Browser-based library user interface
- ID: REQ-INT-001
- Status: planned
- Date: 2026-05-26
- Title: Browser-based library user interface
- Statement: The system shall provide a browser-based user interface for browsing, searching, importing, and editing the e-book library.
- Rationale: The product is intended to replace calibre-web and support access from devices with an internet connection.
- Acceptance Criteria:
  - A user can open the application in a supported browser and view the library landing page.
  - The UI exposes navigation to books, authors, series, reading tracker, imports, analytics, and settings.
  - The UI communicates with the backend only through supported application interfaces.
- Verification Method: Demonstration
- More Information: Supported browser list is TBD; initial assumption is current evergreen desktop and mobile browsers.

##### REQ-INT-002 — Responsive user interface
- ID: REQ-INT-002
- Status: proposed
- Date: 2026-05-26
- Title: Responsive user interface
- Statement: The system shall provide responsive layouts for desktop/laptop screens and mobile-width screens.
- Rationale: The project includes a nice-to-have goal of availability across desktop/laptop, server browser, and mobile contexts.
- Acceptance Criteria:
  - Book, author, series, and reading-status views remain usable at 390 px, 768 px, and 1280 px viewport widths.
  - Primary navigation, search, and edit actions are reachable without horizontal scrolling at the tested viewport widths.
  - Automated or manual UI checks cover at least one mobile-width and one desktop-width viewport.
- Verification Method: Test
- More Information: Native mobile applications are deferred; this requirement covers web responsiveness only.


#### 3.1.2 Hardware Interfaces

##### REQ-INT-007 — Mounted-folder e-reader sync interface
- ID: REQ-INT-007
- Status: deferred
- Date: 2026-05-26
- Title: Mounted-folder e-reader sync interface
- Statement: The system shall support, when the device-sync feature is enabled, a folder-based sync target suitable for mounted e-readers or export directories.
- Rationale: The project includes a nice-to-have Tolino-cloud replacement direction and automatic syncing between a PC device and an e-reader.
- Acceptance Criteria:
  - A user can configure a local or mounted folder as a sync target.
  - The system can copy selected EPUB files to the configured target.
  - The system records sync success or failure for each attempted file.
- Verification Method: Demonstration
- More Information: Deferred until the e-reader/device sync milestone. Exact Tolino capabilities and protocols are TBD.


#### 3.1.3 Software Interfaces

##### REQ-INT-003 — REST API interface
- ID: REQ-INT-003
- Status: planned
- Date: 2026-05-26
- Title: REST API interface
- Statement: The system shall expose a REST API for client-server communication.
- Rationale: The agreed server direction is a REST server, and remote clients must communicate through the server rather than directly touching persistent storage.
- Acceptance Criteria:
  - API endpoints use HTTP methods consistently for create, read, update, delete, import, job, and sync operations.
  - Requests and responses use a documented JSON schema except where binary file transfer is explicitly specified.
  - Mutating endpoints return deterministic success and error responses that can be handled by clients.
- Verification Method: Inspection
- More Information: The API specification should be published as OpenAPI or equivalent before v0.6.

##### REQ-INT-004 — File import and download interface
- ID: REQ-INT-004
- Status: planned
- Date: 2026-05-26
- Title: File import and download interface
- Statement: The system shall provide interfaces for importing EPUB files and downloading stored EPUB files where the authenticated user has access.
- Rationale: Managing an e-book library requires ingesting book files and retrieving them for reading, backup, export, or device sync.
- Acceptance Criteria:
  - A valid EPUB file can be uploaded or imported through the supported UI/API flow.
  - The system rejects non-EPUB files for EPUB-only import flows with a user-visible error.
  - An authenticated user can download a stored EPUB file without corrupting the file contents.
- Verification Method: Test
- More Information: Additional formats are deferred until multi-medium support is approved.

##### REQ-INT-005 — External metadata provider interface
- ID: REQ-INT-005
- Status: planned
- Date: 2026-05-26
- Title: External metadata provider interface
- Statement: The system shall support integration with one or more external metadata providers through a provider abstraction.
- Rationale: The project requires online metadata fetching while keeping the product independent from a single provider.
- Acceptance Criteria:
  - A metadata search request can be routed through at least one configured provider.
  - Provider responses are normalized into a common candidate format.
  - Provider errors are surfaced without crashing the import or edit workflow.
- Verification Method: Test
- More Information: Specific provider names, API keys, and rate limits are TBD.

##### REQ-INT-006 — External image URL download interface
- ID: REQ-INT-006
- Status: planned
- Date: 2026-05-26
- Title: External image URL download interface
- Statement: The system shall allow an authenticated user to paste an image URL for an author image and have the system download and store the image as an author asset.
- Rationale: The draft explicitly asks for author images that can be fetched automatically or by pasted URL.
- Acceptance Criteria:
  - The UI accepts an image URL on the author edit/detail view.
  - The backend downloads only supported image media types and rejects unsupported responses.
  - The resulting image is displayed on the author detail page after successful download.
  - The original URL is recorded as asset source metadata when available.
- Verification Method: Test
- More Information: Allowed image types and maximum download size are TBD and tracked as security/performance details.

##### REQ-INT-008 — Client database isolation
- ID: REQ-INT-008
- Status: proposed
- Date: 2026-05-26
- Title: Client database isolation
- Statement: The system shall not require desktop, mobile, or browser clients to access the server database file directly.
- Rationale: The server should remain the authoritative consistency boundary for all clients, enabling SQLite or another local database to be used safely on the server.
- Acceptance Criteria:
  - All supported client mutations are expressed as API requests or local embedded-backend commands.
  - No supported remote client workflow requires direct filesystem access to the server database.
  - Integration tests verify that remote changes go through the server application layer.
- Verification Method: Inspection
- More Information: This requirement does not mandate SQLite; it preserves the option to use SQLite or another database behind the server.


### 3.2 Functional

This section defines externally observable product behavior grouped by library management, browsing, reading tracking, automation, and future media support.

#### 3.2.1 Library Management and EPUB Processing

##### REQ-FUNC-001 — Import EPUB files
- ID: REQ-FUNC-001
- Status: planned
- Date: 2026-05-26
- Title: Import EPUB files
- Statement: The system shall import EPUB files into the library catalog.
- Rationale: The Calibre-replacement goal depends on bringing existing e-book files under catalog management.
- Acceptance Criteria:
  - Given a valid EPUB file, the import flow creates a catalog record and a file asset record.
  - The system stores enough import metadata to identify the source file, import time, and file checksum.
  - The user receives a success or failure result for each imported file.
- Verification Method: Test
- More Information: Batch import and folder watching are later desktop/server enhancements.

##### REQ-FUNC-002 — Persist catalog records and assets
- ID: REQ-FUNC-002
- Status: planned
- Date: 2026-05-26
- Title: Persist catalog records and assets
- Statement: The system shall persist book metadata, author metadata, series metadata, reading data, and file/cover/author-image asset references across server restarts.
- Rationale: The product is a long-lived personal library system and must not lose catalog state after normal restart.
- Acceptance Criteria:
  - After restart, previously imported books, authors, series, reading states, and asset links remain available.
  - Stored assets remain addressable by stable identifiers.
  - Catalog records and asset records remain referentially consistent after normal shutdown and restart.
- Verification Method: Test
- More Information: Exact database engine is TBD; initial design assumption is one authoritative server-side database plus asset directory.

##### REQ-FUNC-003 — Extract EPUB metadata
- ID: REQ-FUNC-003
- Status: planned
- Date: 2026-05-26
- Title: Extract EPUB metadata
- Statement: The system shall extract available title, author, identifier, language, publication, publisher, and cover metadata from imported EPUB files.
- Rationale: Automatic extraction reduces manual entry and establishes the baseline catalog record.
- Acceptance Criteria:
  - For a test EPUB containing standard metadata, the imported catalog fields match the source metadata.
  - If a metadata field is missing from the EPUB, the import still succeeds and marks the field blank or unknown.
  - The import result identifies which fields were extracted and which were unavailable.
- Verification Method: Test
- More Information: Exact EPUB metadata fields may expand over time; current fields are a minimum set.

##### REQ-FUNC-004 — Edit catalog metadata
- ID: REQ-FUNC-004
- Status: planned
- Date: 2026-05-26
- Title: Edit catalog metadata
- Statement: The system shall allow an authenticated user to edit book, edition, author, and series metadata stored in the catalog.
- Rationale: Manual correction is necessary because imported and provider-supplied metadata may be incomplete or incorrect.
- Acceptance Criteria:
  - The UI allows editing of at least title, sort title, author associations, series associations, description, language, publication date, identifiers, and cover association.
  - Submitted edits are validated and persisted.
  - After saving, the updated values are visible in the relevant book, author, and series views.
- Verification Method: Test
- More Information: Field-level validation rules are TBD and should be refined during UI/API design.

##### REQ-FUNC-005 — Write metadata to EPUB files
- ID: REQ-FUNC-005
- Status: planned
- Date: 2026-05-26
- Title: Write metadata to EPUB files
- Statement: The system shall support writing accepted metadata changes back into EPUB files.
- Rationale: A Calibre replacement needs to modify the actual e-book file, not only the application catalog.
- Acceptance Criteria:
  - A user can request that catalog metadata be written to an EPUB file.
  - After the operation, re-importing or re-reading the EPUB shows the updated metadata for supported fields.
  - The operation reports unsupported fields rather than silently pretending to write them.
- Verification Method: Test
- More Information: Safe file versioning requirements apply; see REQ-FUNC-007 and REQ-REL-002.

##### REQ-FUNC-006 — Edit EPUB content
- ID: REQ-FUNC-006
- Status: planned
- Date: 2026-05-26
- Title: Edit EPUB content
- Statement: The system shall support authenticated editing of EPUB textual content for selected books.
- Rationale: The draft explicitly requires editing e-book content, including later automation such as search-and-replace.
- Acceptance Criteria:
  - A user can open an EPUB content-edit workflow for an imported EPUB.
  - The system can save a textual content change to a new EPUB file version.
  - The user receives validation feedback if the edited EPUB cannot be saved in a usable package structure.
- Verification Method: Demonstration
- More Information: The granularity and UX of content editing are TBD; initial scope may be search-and-replace and limited XHTML editing.

##### REQ-FUNC-007 — Version EPUB modifications
- ID: REQ-FUNC-007
- Status: planned
- Date: 2026-05-26
- Title: Version EPUB modifications
- Statement: The system shall retain a recoverable previous EPUB file version before overwriting or replacing an EPUB through metadata, cover, or content edits.
- Rationale: EPUB editing can damage files; preserving prior versions mitigates data-loss risk.
- Acceptance Criteria:
  - Before a write operation changes an EPUB, the previous version remains recoverable.
  - The system records version creation time, operation type, and checksum.
  - A user or administrator can identify the current version and at least one previous version.
- Verification Method: Test
- More Information: Retention limits are TBD; backup policy should define whether old versions are pruned.

##### REQ-FUNC-008 — Fetch online metadata
- ID: REQ-FUNC-008
- Status: planned
- Date: 2026-05-26
- Title: Fetch online metadata
- Statement: The system shall allow an authenticated user to fetch online metadata candidates for a book.
- Rationale: Online metadata lookup is a stated Calibre-replacement must-have.
- Acceptance Criteria:
  - A user can trigger metadata lookup from an import review or book detail/edit view.
  - The system submits relevant search inputs such as title, author, and identifier to configured providers.
  - The system returns zero or more normalized candidates without overwriting catalog data automatically unless an approved automation rule applies.
- Verification Method: Test
- More Information: Specific providers and provider credentials are TBD.

##### REQ-FUNC-009 — Review metadata candidates
- ID: REQ-FUNC-009
- Status: planned
- Date: 2026-05-26
- Title: Review metadata candidates
- Statement: The system shall present online metadata candidates for user review before applying them unless an enabled automation rule explicitly authorizes automatic application.
- Rationale: Provider metadata may be incorrect, so a human review point prevents accidental catalog corruption.
- Acceptance Criteria:
  - Candidate values are shown alongside current catalog values.
  - The user can accept, reject, or selectively apply candidate fields.
  - The system records the selected source for applied candidate fields where practical.
- Verification Method: Demonstration
- More Information: Automation exceptions are governed by REQ-FUNC-028 and REQ-ML-001.

##### REQ-FUNC-010 — Detect duplicate imported books
- ID: REQ-FUNC-010
- Status: proposed
- Date: 2026-05-26
- Title: Detect duplicate imported books
- Statement: The system shall detect likely duplicate imports using at least file checksum and available identifier/title-author comparisons.
- Rationale: Duplicate detection prevents library clutter and accidental redundant storage.
- Acceptance Criteria:
  - An import with an identical checksum to an existing file is flagged as an exact duplicate.
  - An import with matching ISBN or matching normalized title-author values is flagged as a possible duplicate.
  - The user can choose whether to skip, merge, or keep a flagged possible duplicate.
- Verification Method: Test
- More Information: Merge semantics are TBD and should be refined before implementation.


#### 3.2.2 Browsing, Authors, and Series

##### REQ-FUNC-011 — Display book card view
- ID: REQ-FUNC-011
- Status: planned
- Date: 2026-05-26
- Title: Display book card view
- Statement: The system shall display a card-based overview of books with cover images when available.
- Rationale: This is a stated calibre-web replacement requirement.
- Acceptance Criteria:
  - The book overview renders each visible book as a card.
  - Each card displays title and cover image, or a generated placeholder when no cover is available.
  - Selecting a card opens the book detail view.
- Verification Method: Demonstration
- More Information: Sorting and filtering controls are covered by REQ-FUNC-018.

##### REQ-FUNC-012 — Display author detail view
- ID: REQ-FUNC-012
- Status: planned
- Date: 2026-05-26
- Title: Display author detail view
- Statement: The system shall display all cataloged books associated with a selected author.
- Rationale: Users need to browse a personal library by author.
- Acceptance Criteria:
  - An author detail page lists all books linked to the author.
  - Books on the author detail page link to their book detail pages.
  - The page displays the author image when an image asset exists.
- Verification Method: Demonstration
- More Information: Author roles such as translator/editor are TBD but should be represented in the data model.

##### REQ-FUNC-013 — Manage author images
- ID: REQ-FUNC-013
- Status: planned
- Date: 2026-05-26
- Title: Manage author images
- Statement: The system shall support displaying, automatically fetching, and manually setting author images.
- Rationale: The draft requires author images and allows fetching either automatically or by pasted URL.
- Acceptance Criteria:
  - An author without an image can be assigned an image through a supported fetch or upload/download flow.
  - An author with an image displays that image on author overview and detail views.
  - The user can replace or remove an author image.
- Verification Method: Test
- More Information: Automatic source selection is provider-dependent and TBD.

##### REQ-FUNC-014 — Display series detail view
- ID: REQ-FUNC-014
- Status: planned
- Date: 2026-05-26
- Title: Display series detail view
- Statement: The system shall display all books belonging to a selected series.
- Rationale: Series browsing is a stated calibre-web replacement requirement.
- Acceptance Criteria:
  - A series detail page lists all books assigned to the series.
  - Series entries display ordering information when available.
  - Books on the series detail page link to book detail pages.
- Verification Method: Demonstration
- More Information: Numbering/position model must support decimals or labels; exact allowed values are TBD.

##### REQ-FUNC-015 — Support related series
- ID: REQ-FUNC-015
- Status: planned
- Date: 2026-05-26
- Title: Support related series
- Statement: The system shall support relationships between series, including overarching series and contained series.
- Rationale: The draft requires series views to include related series such as overarching or contained series.
- Acceptance Criteria:
  - A user can define a relationship between two series.
  - The series detail page displays related series grouped or labeled by relationship type.
  - The system prevents or reports invalid self-relations and relation cycles where cycles are disallowed by the chosen relation type.
- Verification Method: Test
- More Information: Allowed relation types should be finalized during domain-model design.

##### REQ-FUNC-016 — Display author overview
- ID: REQ-FUNC-016
- Status: planned
- Date: 2026-05-26
- Title: Display author overview
- Statement: The system shall provide a card-based overview of all authors whose books are in the library.
- Rationale: The draft requires an overview of authors with card view.
- Acceptance Criteria:
  - The author overview includes every author linked to at least one cataloged book.
  - Each author card displays the author name and image or placeholder.
  - Selecting an author card opens the author detail view.
- Verification Method: Demonstration
- More Information: None.

##### REQ-FUNC-017 — Display series overview
- ID: REQ-FUNC-017
- Status: planned
- Date: 2026-05-26
- Title: Display series overview
- Statement: The system shall provide a card-based overview of all series whose books are in the library.
- Rationale: The draft requires an overview of series with card view.
- Acceptance Criteria:
  - The series overview includes every series linked to at least one cataloged book.
  - Each series card displays the series title and representative image or placeholder.
  - Selecting a series card opens the series detail view.
- Verification Method: Demonstration
- More Information: None.

##### REQ-FUNC-018 — Search and filter library
- ID: REQ-FUNC-018
- Status: proposed
- Date: 2026-05-26
- Title: Search and filter library
- Statement: The system shall allow users to search and filter the catalog by at least title, author, and series.
- Rationale: Search and filtering are necessary for practical library browsing once the catalog grows.
- Acceptance Criteria:
  - Search by title returns books with matching title fields.
  - Search by author returns books linked to matching authors.
  - Search by series returns books linked to matching series.
  - No-result states are shown clearly.
- Verification Method: Test
- More Information: Advanced search over EPUB full text is out of initial scope unless explicitly added.


#### 3.2.3 Reading Tracking and Analytics

##### REQ-FUNC-019 — Track owned books
- ID: REQ-FUNC-019
- Status: planned
- Date: 2026-05-26
- Title: Track owned books
- Statement: The system shall track whether a book or edition is owned by the user.
- Rationale: The Goodreads-replacement goal includes tracking owned books.
- Acceptance Criteria:
  - A user can mark a book or edition as owned or not owned.
  - Ownership status is persisted and visible on the book detail page.
  - Owned books can be filtered or listed separately.
- Verification Method: Test
- More Information: Ownership granularity across work/edition/file is a domain-model decision to finalize.

##### REQ-FUNC-020 — Track reading status
- ID: REQ-FUNC-020
- Status: planned
- Date: 2026-05-26
- Title: Track reading status
- Statement: The system shall track reading status for books using defined statuses including at least unread, currently reading, read, and abandoned.
- Rationale: The Goodreads-replacement goal requires reading-status tracking.
- Acceptance Criteria:
  - A user can set and update a book reading status.
  - The current status is visible on the book detail page and in reading views.
  - Changing a status records the change time or creates a reading event.
- Verification Method: Test
- More Information: Additional statuses such as want-to-read or paused are TBD.

##### REQ-FUNC-021 — Track completed reads
- ID: REQ-FUNC-021
- Status: planned
- Date: 2026-05-26
- Title: Track completed reads
- Statement: The system shall record completed-reading events for books marked as read.
- Rationale: The system needs completion history to support analytics by period and rereads.
- Acceptance Criteria:
  - A user can record a finished date for a book.
  - The system can store multiple completed-read events for the same book to represent rereads.
  - Completed reads appear in analytics for the corresponding period.
- Verification Method: Test
- More Information: Exact reread UX is TBD.

##### REQ-FUNC-022 — Track reading progress events
- ID: REQ-FUNC-022
- Status: proposed
- Date: 2026-05-26
- Title: Track reading progress events
- Statement: The system shall allow recording reading progress events containing at least date/time and one progress measure such as page, percentage, or location.
- Rationale: Reading progress supports reading-speed analytics and history beyond a single status field.
- Acceptance Criteria:
  - A user can create a progress event for a book.
  - Progress events are associated with the correct book or edition.
  - Invalid progress values are rejected or reported to the user.
- Verification Method: Test
- More Information: Which progress measures are mandatory depends on available page/location data and is TBD.

##### REQ-FUNC-023 — Analyze books read per period
- ID: REQ-FUNC-023
- Status: planned
- Date: 2026-05-26
- Title: Analyze books read per period
- Statement: The system shall provide analytics for number of books read per selected period.
- Rationale: The draft explicitly lists books read per period as an analytics example.
- Acceptance Criteria:
  - The user can select or view at least monthly and yearly periods.
  - The analytics result counts completed-reading events in the selected period.
  - The result is visible as a table or chart.
- Verification Method: Test
- More Information: Specific chart design is outside this SRS.

##### REQ-FUNC-024 — Analyze pages read per period
- ID: REQ-FUNC-024
- Status: planned
- Date: 2026-05-26
- Title: Analyze pages read per period
- Statement: The system shall provide analytics for number of pages read per selected period when page-count or progress data is available.
- Rationale: The draft explicitly lists pages read per period as an analytics example.
- Acceptance Criteria:
  - The system computes page totals from page counts and/or reading progress events according to documented rules.
  - The analytics view indicates when page totals are unavailable due to missing page data.
  - The user can view at least monthly and yearly page totals.
- Verification Method: Analysis
- More Information: Page calculation rules are TBD and should account for books without reliable page counts.

##### REQ-FUNC-025 — Analyze books by author
- ID: REQ-FUNC-025
- Status: planned
- Date: 2026-05-26
- Title: Analyze books by author
- Statement: The system shall provide analytics grouping read books by author.
- Rationale: The draft explicitly lists books read grouped by author.
- Acceptance Criteria:
  - The analytics view displays counts of read books grouped by author.
  - Books with multiple authors are counted according to documented attribution rules.
  - The grouping can be inspected for at least the full library history.
- Verification Method: Test
- More Information: Multi-author attribution policy is TBD.

##### REQ-FUNC-026 — Analyze reading speed by author
- ID: REQ-FUNC-026
- Status: planned
- Date: 2026-05-26
- Title: Analyze reading speed by author
- Statement: The system shall provide analytics for reading speed grouped by author when sufficient progress and completion data exists.
- Rationale: The draft explicitly lists reading speed grouped by author.
- Acceptance Criteria:
  - The analytics view displays reading-speed values grouped by author for books with sufficient data.
  - The system indicates when speed cannot be calculated due to missing start, progress, finish, or page data.
  - The calculation formula is documented and consistently applied.
- Verification Method: Analysis
- More Information: Reading speed unit is TBD, e.g. pages/day or books/month.

##### REQ-FUNC-027 — Support remote updates through server
- ID: REQ-FUNC-027
- Status: planned
- Date: 2026-05-26
- Title: Support remote updates through server
- Statement: The system shall allow authenticated users to update library metadata and reading data through the server from devices with internet connectivity.
- Rationale: The draft requires updates from anywhere with internet connection.
- Acceptance Criteria:
  - A supported remote browser or client can authenticate to the server.
  - The user can perform at least reading-status updates remotely.
  - The server applies remote changes transactionally and returns success or failure.
- Verification Method: Demonstration
- More Information: Network exposure method, domain, VPN, or reverse proxy setup is deployment-specific and TBD.


#### 3.2.4 Automation and Job Processing

##### REQ-FUNC-028 — Define automation rules on book import
- ID: REQ-FUNC-028
- Status: deferred
- Date: 2026-05-26
- Title: Define automation rules on book import
- Statement: The system shall allow users to define automatic actions triggered when a book is added.
- Rationale: The draft lists automatic actions when a book is added as a nice-to-have.
- Acceptance Criteria:
  - A user can create, enable, disable, and delete an import-triggered automation rule.
  - A rule has a trigger, optional conditions, and one or more actions.
  - The system records rule executions and failures.
- Verification Method: Test
- More Information: Deferred until metadata/import/automation milestone.

##### REQ-FUNC-029 — Automation action for metadata fetching
- ID: REQ-FUNC-029
- Status: deferred
- Date: 2026-05-26
- Title: Automation action for metadata fetching
- Statement: The system shall support an automation action that fetches metadata for newly imported books.
- Rationale: The draft gives fetching metadata as an example automatic action.
- Acceptance Criteria:
  - An enabled rule can start a metadata-fetch job after import.
  - The metadata result is either queued for review or applied according to rule configuration.
  - Failures are recorded and visible in job or automation history.
- Verification Method: Test
- More Information: Human-review defaults should be conservative until provider confidence rules are defined.

##### REQ-FUNC-030 — Automation action for EPUB search-and-replace
- ID: REQ-FUNC-030
- Status: deferred
- Date: 2026-05-26
- Title: Automation action for EPUB search-and-replace
- Statement: The system shall support an automation action that performs configured search-and-replace operations in EPUB text.
- Rationale: The draft gives search-and-replace in EPUB text as an example automatic action.
- Acceptance Criteria:
  - A user can configure a search pattern and replacement value within an automation rule.
  - The system applies the replacement to a new EPUB version, not the only stored copy.
  - The system reports the number of replacements or an error.
- Verification Method: Test
- More Information: Regex support, scope controls, and preview requirements are TBD.

##### REQ-FUNC-031 — Run long operations as background jobs
- ID: REQ-FUNC-031
- Status: planned
- Date: 2026-05-26
- Title: Run long operations as background jobs
- Statement: The system shall execute long-running operations such as import, metadata lookup, image download, EPUB modification, and device sync as trackable background jobs.
- Rationale: Long operations should not block normal API responsiveness or leave users uncertain about progress.
- Acceptance Criteria:
  - Submitting a long operation returns a job identifier.
  - The job has a status of queued, running, succeeded, failed, or canceled.
  - Job failure includes a user-visible error message.
- Verification Method: Test
- More Information: Job cancellation behavior is TBD.

##### REQ-FUNC-032 — Display job status
- ID: REQ-FUNC-032
- Status: planned
- Date: 2026-05-26
- Title: Display job status
- Statement: The system shall display job status and recent job history to authenticated users.
- Rationale: Users need feedback for imports, metadata fetching, image fetching, EPUB editing, and sync workflows.
- Acceptance Criteria:
  - The UI displays current status for active jobs.
  - The UI displays at least the most recent completed and failed jobs.
  - Selecting a failed job shows the failure reason when available.
- Verification Method: Demonstration
- More Information: Retention policy for job history is TBD.


#### 3.2.5 Deferred Device and Media Features

##### REQ-FUNC-033 — Synchronize books to e-reader targets
- ID: REQ-FUNC-033
- Status: deferred
- Date: 2026-05-26
- Title: Synchronize books to e-reader targets
- Statement: The system shall support synchronizing selected EPUB files to configured e-reader or folder targets.
- Rationale: The project has a nice-to-have goal of replacing parts of Tolino cloud with automatic PC-to-e-reader syncing.
- Acceptance Criteria:
  - A user can select books and a configured target for sync.
  - The system copies or updates selected files on the target according to target rules.
  - Sync results are recorded per file.
- Verification Method: Demonstration
- More Information: Deferred until device sync milestone; exact e-reader support is TBD.

##### REQ-FUNC-034 — Support audiobook records
- ID: REQ-FUNC-034
- Status: deferred
- Date: 2026-05-26
- Title: Support audiobook records
- Statement: The system shall, in a future release, support catalog records and file assets for audiobooks in addition to e-books.
- Rationale: The draft lists an Audiobookshelf replacement as a very optional future feature.
- Acceptance Criteria:
  - The data model can represent an audiobook edition or asset.
  - The UI can display an audiobook associated with a work.
  - The system can distinguish audiobook-specific metadata such as narrator and duration when implemented.
- Verification Method: Inspection
- More Information: Future feature; not required for initial stable release.

##### REQ-FUNC-035 — Track book medium
- ID: REQ-FUNC-035
- Status: deferred
- Date: 2026-05-26
- Title: Track book medium
- Statement: The system shall, in a future release, track book medium values including e-book, print, and audiobook.
- Rationale: The draft lists book-medium tracking as a potential future feature.
- Acceptance Criteria:
  - A book or edition can be associated with a medium value.
  - Ownership and reading views can distinguish records by medium.
  - Unknown or unspecified medium can be represented without data loss.
- Verification Method: Inspection
- More Information: Future feature; early data model should avoid blocking this extension.

##### REQ-FUNC-036 — Sync progress between e-book and audiobook
- ID: REQ-FUNC-036
- Status: deferred
- Date: 2026-05-26
- Title: Sync progress between e-book and audiobook
- Statement: The system shall, in a future release, support progress synchronization between an e-book and audiobook representation of the same work.
- Rationale: The draft lists progress sync between audiobook and e-book as a potential future feature.
- Acceptance Criteria:
  - The system can associate e-book and audiobook editions with the same work.
  - The system can store mapping metadata or conversion rules for progress positions.
  - The UI can show synchronized progress where mapping is available.
- Verification Method: Analysis
- More Information: Very optional future feature; exact synchronization model is TBD.


### 3.3 Quality of Service

This section defines performance, security, reliability, availability, and observability expectations. Requirements with status `draft` need owner confirmation before they can be treated as release gates.

#### 3.3.1 Performance

##### REQ-PERF-001 — Catalog browsing latency target
- ID: REQ-PERF-001
- Status: draft
- Date: 2026-05-26
- Title: Catalog browsing latency target
- Statement: The system shall return paginated catalog-browsing API responses within an approved p95 latency target for an approved reference library size and reference server environment.
- Rationale: Performance targets require concrete scale assumptions that have not yet been supplied; the requirement preserves the need for measurable acceptance criteria before performance work begins.
- Acceptance Criteria:
  - Before v0.6, the project owner approves a reference library size, reference hardware/container profile, and p95 latency target.
  - A performance test exists for catalog browse/search endpoints using the approved reference library.
  - The measured p95 latency meets the approved target.
- Verification Method: Analysis
- More Information: Clarification needed: expected library size, concurrent users, server hardware, and acceptable latency.

##### REQ-PERF-002 — Non-blocking long-running operations
- ID: REQ-PERF-002
- Status: proposed
- Date: 2026-05-26
- Title: Non-blocking long-running operations
- Statement: The system shall keep ordinary catalog browsing and reading-status update endpoints available while import, metadata fetch, image fetch, EPUB edit, or sync jobs are running.
- Rationale: Long-running jobs must not make the application unusable for normal library operations.
- Acceptance Criteria:
  - A test can submit a long-running job and still successfully call catalog and reading-status endpoints.
  - The long-running job status remains queryable while the job is running.
  - Failures in one job do not prevent unrelated normal API requests from completing.
- Verification Method: Test
- More Information: Exact latency thresholds are covered by REQ-PERF-001.

##### REQ-PERF-003 — Asset storage growth visibility
- ID: REQ-PERF-003
- Status: proposed
- Date: 2026-05-26
- Title: Asset storage growth visibility
- Statement: The system shall expose total storage usage for the database and managed asset directory to an administrator.
- Rationale: E-book, cover, author-image, original, and versioned files can grow over time; users need visibility to manage storage.
- Acceptance Criteria:
  - An administrator can view total managed storage usage.
  - The storage usage view includes at least book files, covers, author images, originals, and previous versions when those categories exist.
  - The values update after import or deletion operations within a documented refresh interval.
- Verification Method: Demonstration
- More Information: Quotas are TBD and not required by this requirement.


#### 3.3.2 Security

##### REQ-SEC-001 — Authenticate protected operations
- ID: REQ-SEC-001
- Status: planned
- Date: 2026-05-26
- Title: Authenticate protected operations
- Statement: The system shall require authentication before allowing access to mutating operations or private library data over the server interface.
- Rationale: Remote updates from the internet require protection of personal library and reading data.
- Acceptance Criteria:
  - Unauthenticated requests to protected endpoints receive an authentication error.
  - Authenticated requests with valid credentials can access authorized protected endpoints.
  - The login/session/token mechanism is documented before v0.6.
- Verification Method: Test
- More Information: Authentication mechanism and multi-user model are TBD.

##### REQ-SEC-002 — Authorize user operations
- ID: REQ-SEC-002
- Status: draft
- Date: 2026-05-26
- Title: Authorize user operations
- Statement: The system shall enforce authorization checks for administrative, catalog-editing, file-management, and reading-tracker operations.
- Rationale: If the project supports more than one account, users should not gain unintended administrative access; even a single-user deployment benefits from explicit authorization boundaries.
- Acceptance Criteria:
  - Administrative settings are inaccessible to non-admin roles if non-admin roles exist.
  - Protected mutating endpoints check authorization before applying changes.
  - Authorization failures are logged without exposing secrets.
- Verification Method: Test
- More Information: Role model is TBD; may be simplified to owner/admin for initial release.

##### REQ-SEC-003 — Validate external inputs
- ID: REQ-SEC-003
- Status: planned
- Date: 2026-05-26
- Title: Validate external inputs
- Statement: The system shall validate and sanitize user-supplied input, uploaded files, provider responses, and external image URLs before processing or persisting them.
- Rationale: The system accepts files and URLs and may be exposed remotely, making input validation essential to protect integrity and availability.
- Acceptance Criteria:
  - Unsupported file types are rejected for EPUB import.
  - External image downloads enforce supported schemes and media types.
  - Text inputs used in database queries or file paths cannot cause injection or path traversal in test cases.
  - Provider response parsing handles missing or malformed fields.
- Verification Method: Test
- More Information: Maximum file sizes and URL restrictions are TBD.

##### REQ-SEC-004 — Protect secrets and provider credentials
- ID: REQ-SEC-004
- Status: proposed
- Date: 2026-05-26
- Title: Protect secrets and provider credentials
- Statement: The system shall store API keys, session secrets, and provider credentials outside source-controlled code and outside client-delivered frontend bundles.
- Rationale: External metadata providers and authentication may require secrets that must not be exposed to clients or repositories.
- Acceptance Criteria:
  - No provider API key is embedded in the compiled frontend bundle.
  - Runtime configuration supports providing secrets through environment variables, config files, or secret stores.
  - Configuration examples use placeholders rather than real secrets.
- Verification Method: Inspection
- More Information: Exact secret-storage mechanism depends on deployment mode.

##### REQ-SEC-005 — Support secure deployment over HTTPS
- ID: REQ-SEC-005
- Status: proposed
- Date: 2026-05-26
- Title: Support secure deployment over HTTPS
- Statement: The system shall support deployment behind HTTPS for all remote browser and API traffic.
- Rationale: Remote access over the internet should protect credentials and personal reading data in transit.
- Acceptance Criteria:
  - Deployment documentation includes an HTTPS-capable reverse-proxy or equivalent configuration pattern.
  - The application does not require mixed-content HTTP calls from an HTTPS frontend.
  - Cookie or token settings are compatible with HTTPS deployment.
- Verification Method: Inspection
- More Information: TLS termination may be handled by a reverse proxy rather than the application process.


#### 3.3.3 Reliability

##### REQ-REL-001 — Transactional catalog mutations
- ID: REQ-REL-001
- Status: planned
- Date: 2026-05-26
- Title: Transactional catalog mutations
- Statement: The system shall apply catalog mutations transactionally so that each mutation either completes fully or leaves persistent catalog state unchanged.
- Rationale: Partial updates could corrupt relationships between books, authors, series, files, and reading records.
- Acceptance Criteria:
  - A failed book metadata update does not leave partially updated related records.
  - A failed import does not create orphaned catalog records that point to missing assets.
  - Integration tests cover at least one failure path for import or metadata update.
- Verification Method: Test
- More Information: File operations and database operations may require compensating actions; exact implementation is design-specific.

##### REQ-REL-002 — Recover from failed EPUB modification
- ID: REQ-REL-002
- Status: planned
- Date: 2026-05-26
- Title: Recover from failed EPUB modification
- Statement: The system shall preserve the previous usable EPUB version if metadata writing, cover replacement, or content editing fails.
- Rationale: EPUB modification is a data-loss risk and must fail safely.
- Acceptance Criteria:
  - When an EPUB modification fails, the previous current version remains downloadable.
  - The failed operation creates a failed job or operation record with an error message.
  - No book record points to a missing current EPUB file after the failure.
- Verification Method: Test
- More Information: Related to REQ-FUNC-007.

##### REQ-REL-003 — Handle metadata provider failure gracefully
- ID: REQ-REL-003
- Status: planned
- Date: 2026-05-26
- Title: Handle metadata provider failure gracefully
- Statement: The system shall continue to support manual catalog operations when external metadata providers are unavailable or return errors.
- Rationale: The application should not depend on third-party service availability for core library management.
- Acceptance Criteria:
  - A provider timeout or error does not crash the server.
  - The user receives a visible provider-error message.
  - Manual metadata editing remains available after provider failure.
- Verification Method: Test
- More Information: Provider timeout durations are TBD.

##### REQ-REL-004 — Idempotent import by checksum
- ID: REQ-REL-004
- Status: proposed
- Date: 2026-05-26
- Title: Idempotent import by checksum
- Statement: The system shall identify repeated import attempts of an identical EPUB file by checksum and avoid creating unintended duplicate file records without user confirmation.
- Rationale: Retrying failed or uncertain imports should not accidentally duplicate the library.
- Acceptance Criteria:
  - An identical file checksum is detected during import.
  - The import response indicates that an identical file already exists.
  - The system does not create a new active duplicate record unless the user explicitly confirms keeping a duplicate.
- Verification Method: Test
- More Information: Related to REQ-FUNC-010.


#### 3.3.4 Availability

##### REQ-AVAIL-001 — Restart recovery
- ID: REQ-AVAIL-001
- Status: proposed
- Date: 2026-05-26
- Title: Restart recovery
- Statement: The system shall recover its catalog, configuration, and job state sufficiently to resume normal operation after a controlled server restart.
- Rationale: Self-hosted users need predictable behavior across restarts and upgrades.
- Acceptance Criteria:
  - After controlled restart, the server starts without manual database repair.
  - Previously completed imports and catalog edits are visible.
  - Incomplete jobs are marked in a documented state such as failed, canceled, or resumable.
- Verification Method: Test
- More Information: Availability percentage/SLA is TBD.

##### REQ-AVAIL-002 — Backup and restore support
- ID: REQ-AVAIL-002
- Status: planned
- Date: 2026-05-26
- Title: Backup and restore support
- Statement: The system shall provide documented backup and restore procedures for catalog data and managed assets.
- Rationale: The library state includes database records and file assets; losing either can compromise the library.
- Acceptance Criteria:
  - Documentation identifies all data that must be backed up.
  - A restore procedure can recreate a usable library from backup artifacts in a test environment.
  - Backup/restore covers both database and managed asset directory.
- Verification Method: Demonstration
- More Information: Automated backup scheduling is TBD.


#### 3.3.5 Observability

##### REQ-OBS-001 — Structured server logs
- ID: REQ-OBS-001
- Status: proposed
- Date: 2026-05-26
- Title: Structured server logs
- Statement: The system shall produce structured server logs for API requests, job execution, import operations, metadata provider calls, and errors.
- Rationale: Operators need to diagnose failures in a self-hosted server without inspecting source code.
- Acceptance Criteria:
  - Logs include timestamp, severity, component, and request or job correlation identifier when applicable.
  - Sensitive values such as passwords, tokens, and provider API keys are not logged.
  - Errors include enough context to identify the affected operation without exposing private secrets.
- Verification Method: Inspection
- More Information: Log format and retention are TBD.

##### REQ-OBS-002 — User-visible operation history
- ID: REQ-OBS-002
- Status: proposed
- Date: 2026-05-26
- Title: User-visible operation history
- Statement: The system shall provide user-visible history for imports, EPUB modifications, metadata fetches, image downloads, and sync jobs.
- Rationale: Users need to understand what automated or background actions changed their library.
- Acceptance Criteria:
  - The UI can show recent operations with status and timestamp.
  - Failed operations show an error summary.
  - Completed operations identify the affected book, author, series, or asset when applicable.
- Verification Method: Demonstration
- More Information: History retention duration is TBD.


### 3.4 Compliance

This section defines compliance-oriented requirements derived from user-content handling, external metadata/image sources, and privacy of reading data.

#### REQ-COMP-001 — External source attribution
- ID: REQ-COMP-001
- Status: proposed
- Date: 2026-05-26
- Title: External source attribution
- Statement: The system shall record the source provider or source URL for metadata, cover images, and author images when that information is available.
- Rationale: Attribution supports auditability, troubleshooting, and compliance with provider terms.
- Acceptance Criteria:
  - Applied provider metadata records include provider identity when available.
  - Downloaded image assets record source URL when available.
  - The UI or admin export can inspect source information for an asset or applied metadata field.
- Verification Method: Test
- More Information: Provider-specific license and attribution display rules are TBD.

#### REQ-COMP-002 — User-content responsibility notice
- ID: REQ-COMP-002
- Status: proposed
- Date: 2026-05-26
- Title: User-content responsibility notice
- Statement: The system shall include documentation stating that users are responsible for having the rights necessary to upload, modify, and sync e-book files they manage with the system.
- Rationale: The product manages user-provided copyrighted content, and documentation should not imply a right to redistribute files.
- Acceptance Criteria:
  - Project documentation includes a user-content responsibility notice.
  - The notice is included in installation or first-use documentation.
  - The notice does not block local personal use workflows.
- Verification Method: Inspection
- More Information: Legal review is TBD; this is not legal advice.

#### REQ-COMP-003 — Privacy of reading data
- ID: REQ-COMP-003
- Status: planned
- Date: 2026-05-26
- Title: Privacy of reading data
- Statement: The system shall treat reading status, ownership, reading history, and analytics as private user data protected by authentication in server deployments.
- Rationale: Reading history can be personally sensitive, especially when remote access is enabled.
- Acceptance Criteria:
  - Unauthenticated users cannot retrieve reading history through protected APIs.
  - Reading analytics are not publicly exposed unless an explicit sharing feature is later added.
  - Backups and exports identify that reading data is included.
- Verification Method: Test
- More Information: Regulatory regime, if any, is TBD.


### 3.5 Design and Implementation

This section captures binding implementation constraints and delivery expectations supplied by the project description or architecture discussion.

#### 3.5.1 Installation

##### REQ-INST-001 — Self-hosted server installation
- ID: REQ-INST-001
- Status: planned
- Date: 2026-05-26
- Title: Self-hosted server installation
- Statement: The system shall provide a self-hosted server installation path that serves the backend API and browser frontend.
- Rationale: The product needs a headless server mode for browser access and remote updates.
- Acceptance Criteria:
  - A user can start the server using documented configuration.
  - The server exposes the REST API and frontend from the configured host/port or reverse-proxy path.
  - The installation documentation identifies data directory, configuration, and backup locations.
- Verification Method: Demonstration
- More Information: Container packaging is covered by REQ-INST-003.

##### REQ-INST-002 — Configurable data directory
- ID: REQ-INST-002
- Status: planned
- Date: 2026-05-26
- Title: Configurable data directory
- Statement: The system shall allow operators to configure the managed data directory used for database and asset storage.
- Rationale: Self-hosted and desktop deployments need predictable data placement for backup and storage management.
- Acceptance Criteria:
  - The server starts with a configured data directory path.
  - The system creates or validates required subdirectories at startup.
  - Changing the data directory requires explicit configuration rather than hidden defaults only.
- Verification Method: Test
- More Information: Migration between data directories is TBD.

##### REQ-INST-003 — Containerized deployment package
- ID: REQ-INST-003
- Status: planned
- Date: 2026-05-26
- Title: Containerized deployment package
- Statement: The system shall provide a containerized deployment package for the self-hosted server release.
- Rationale: Container deployment simplifies running a headless server and matches the planned self-hosted release milestone.
- Acceptance Criteria:
  - A container image can start the server with documented environment variables and mounted data volume.
  - An example compose or equivalent file is available.
  - The container does not require embedding secrets into the image.
- Verification Method: Demonstration
- More Information: Exact container registry and image signing policy are TBD.


#### 3.5.2 Build and Delivery

##### REQ-BUILD-001 — Rust backend
- ID: REQ-BUILD-001
- Status: planned
- Date: 2026-05-26
- Title: Rust backend
- Statement: The backend server and core application logic shall be implemented in Rust.
- Rationale: The draft explicitly requires a Rust backend.
- Acceptance Criteria:
  - Backend source code for server and core domain modules is Rust.
  - Backend build steps compile through the Rust toolchain in CI.
  - Non-Rust support scripts do not replace the Rust backend requirement.
- Verification Method: Inspection
- More Information: None.

##### REQ-BUILD-002 — REST server
- ID: REQ-BUILD-002
- Status: planned
- Date: 2026-05-26
- Title: REST server
- Statement: The server REST API shall be implemented using a Rust library.
- Rationale: The specific REST API library has not yet been selected.
- Acceptance Criteria:
  - The server routing layer uses a Rust crate for REST API.
  - REST endpoints are exposed by the library's server application.
  - The project documents any future change away from a 3rd party REST API library as a breaking design decision.
- Verification Method: Inspection
- More Information: This is a design constraint because the user specified the server direction.

##### REQ-BUILD-003 — React TypeScript frontend
- ID: REQ-BUILD-003
- Status: planned
- Date: 2026-05-26
- Title: React TypeScript frontend
- Statement: The frontend shall be implemented with React and TypeScript.
- Rationale: The draft explicitly requires a React TypeScript frontend.
- Acceptance Criteria:
  - Frontend source files use TypeScript/TSX for application code.
  - The frontend build type-checks successfully in CI.
  - The browser UI is produced from the React application.
- Verification Method: Inspection
- More Information: None.

##### REQ-BUILD-004 — Continuous integration checks
- ID: REQ-BUILD-004
- Status: proposed
- Date: 2026-05-26
- Title: Continuous integration checks
- Statement: The project shall provide automated checks for backend build, frontend build, tests, formatting, and linting before release artifacts are produced.
- Rationale: CI helps keep a multi-component Rust/React project maintainable and verifiable.
- Acceptance Criteria:
  - CI runs backend build/tests.
  - CI runs frontend build/type checks.
  - CI reports failures for formatting or linting checks selected by the project.
  - Release artifacts are not promoted from failing CI runs.
- Verification Method: Inspection
- More Information: Specific CI provider is TBD.


#### 3.5.3 Distribution

##### REQ-DIST-001 — Server-browser deployment topology
- ID: REQ-DIST-001
- Status: planned
- Date: 2026-05-26
- Title: Server-browser deployment topology
- Statement: The system shall support a deployment topology in which a headless server exposes the application to users through a browser.
- Rationale: The draft calls for server availability through a browser and calibre-web-style access.
- Acceptance Criteria:
  - The server can run without a local GUI.
  - A browser can access the UI served by or connected to the server.
  - Server data persists independently of browser sessions.
- Verification Method: Demonstration
- More Information: None.

##### REQ-DIST-002 — Desktop application distribution
- ID: REQ-DIST-002
- Status: deferred
- Date: 2026-05-26
- Title: Desktop application distribution
- Statement: The system should provide a desktop/laptop application distribution that reuses the shared frontend and supports local-library workflows.
- Rationale: Desktop/laptop availability is a nice-to-have and aligns with replacing Calibre for local file management.
- Acceptance Criteria:
  - The desktop app reuses the shared React frontend.
  - The desktop app can manage a local or configured library.
  - Desktop-specific workflows such as drag-and-drop or folder import are considered for the desktop milestone.
- Verification Method: Demonstration
- More Information: Deferred until desktop app alpha; packaging technology is TBD.

##### REQ-DIST-003 — Mobile availability
- ID: REQ-DIST-003
- Status: deferred
- Date: 2026-05-26
- Title: Mobile availability
- Statement: The system should provide mobile availability through a responsive web UI, PWA, or native wrapper that reuses the shared frontend where practical.
- Rationale: Mobile availability is a nice-to-have and supports remote status updates from anywhere.
- Acceptance Criteria:
  - A mobile-width browser can perform reading-status updates.
  - The mobile experience reuses the shared frontend codebase where practical.
  - Native mobile packaging is not required unless approved in a later milestone.
- Verification Method: Demonstration
- More Information: Deferred; initial implementation may satisfy this through responsive web only.


#### 3.5.4 Maintainability

##### REQ-MAINT-001 — Modular backend organization
- ID: REQ-MAINT-001
- Status: proposed
- Date: 2026-05-26
- Title: Modular backend organization
- Statement: The backend shall separate domain logic from transport, storage, metadata-provider, EPUB-processing, and job-execution concerns.
- Rationale: Modularity supports reuse across server, desktop, and future mobile or sync contexts.
- Acceptance Criteria:
  - Core domain logic can be used without importing the HTTP routing layer.
  - Storage and provider integrations are isolated behind interfaces or modules.
  - Unit tests can exercise domain logic without starting the full server.
- Verification Method: Inspection
- More Information: Exact Rust crate layout is an architectural decision.

##### REQ-MAINT-002 — Database migrations
- ID: REQ-MAINT-002
- Status: planned
- Date: 2026-05-26
- Title: Database migrations
- Statement: The system shall manage persistent schema changes through versioned database migrations.
- Rationale: Library data must survive upgrades, and schema changes must be traceable.
- Acceptance Criteria:
  - A fresh installation can apply all migrations from empty state.
  - An existing test database can migrate to the current schema.
  - Migration files are version-controlled.
- Verification Method: Test
- More Information: Database engine is TBD; migration tooling should support the chosen engine.


#### 3.5.5 Reusability

##### REQ-REUSE-001 — Shared frontend reuse
- ID: REQ-REUSE-001
- Status: planned
- Date: 2026-05-26
- Title: Shared frontend reuse
- Statement: The system shall maximize reuse of the React TypeScript frontend across browser, desktop, and mobile-oriented deployments.
- Rationale: The draft explicitly requires reusing as much of the frontend as possible.
- Acceptance Criteria:
  - Common book, author, series, reading, import, and analytics views are implemented in shared frontend modules.
  - Platform-specific shells do not duplicate core UI feature logic except where necessary.
  - Any platform-specific UI forks are documented with rationale.
- Verification Method: Inspection
- More Information: This is a design and maintainability requirement, not a promise of identical UI on every platform.

##### REQ-REUSE-002 — Reusable Rust core
- ID: REQ-REUSE-002
- Status: proposed
- Date: 2026-05-26
- Title: Reusable Rust core
- Statement: The system shall keep core library, reading, metadata, EPUB, and automation logic reusable outside the REST API server entry point.
- Rationale: A reusable core reduces duplication when adding desktop or other clients.
- Acceptance Criteria:
  - Core functions are callable from tests or non-server binaries without starting HTTP routes.
  - Server-specific concerns are not required by the core domain module.
  - Desktop or job-runner code can reuse core operations where applicable.
- Verification Method: Inspection
- More Information: This requirement reflects the proposed architecture and can be refined after crate design.


#### 3.5.6 Portability

##### REQ-PORT-001 — Supported operating environments
- ID: REQ-PORT-001
- Status: draft
- Date: 2026-05-26
- Title: Supported operating environments
- Statement: The system shall document supported server, desktop, and mobile operating environments for each release.
- Rationale: Platform breadth is a stated nice-to-have, but exact support must be explicit to avoid false compatibility claims.
- Acceptance Criteria:
  - Release notes list supported server OS/container environments.
  - Desktop release notes list supported desktop OSes when desktop builds exist.
  - Mobile/browser support notes list supported mobile access method when mobile support exists.
- Verification Method: Inspection
- More Information: Exact OS matrix is TBD.


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
