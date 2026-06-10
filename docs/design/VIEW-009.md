---
date: 2026-06-08
id: VIEW-009
---

# EPUB Modification and Versioning View

## Viewpoint

Interaction, Algorithm, State Dynamics, Information.

## Representation

Promethea supports selected EPUB modification workflows, including writing supported metadata, editing selected EPUB content, and updating cover images. The design must preserve previous EPUB versions before applying changes that alter stored files.

Proposed modification flow:

```text
User
  -> Frontend: submit metadata/content/cover change
Frontend
  -> REST API: mutation request
REST API
  -> Authorization + validation
Application Service
  -> Database: load current file and catalog state
Application Service
  -> Asset Store: copy current EPUB to versions/{version_id}.epub
Application Service
  -> EPUB Processor: apply modification to working copy
Application Service
  -> EPUB Processor: validate resulting EPUB
Application Service
  -> Asset Store: replace current EPUB atomically or via staged move
Application Service
  -> Database: record new version, update metadata, operation history
Frontend
  -> User: show success and access to current/previous version info
```

Failure handling:

```text
Modification fails before current replacement
  -> keep current EPUB unchanged
  -> keep or clean temporary files according to repair policy
  -> show user-visible error

Modification fails after previous version copied but before DB update
  -> preserve previous version and record repair-needed state if possible

Database update fails after file replacement
  -> prefer staged file replacement inside an application-level transaction boundary
  -> repair process reconciles asset and DB state
```

The design must distinguish catalog-only metadata edits from EPUB-file mutations. Catalog-only edits update database metadata. EPUB metadata writing updates both the catalog and EPUB file according to a documented consistency policy.

#### More Information

Implements: [REQ-FUNC-004](./../requirements/functional/REQ-FUNC-004.md), [REQ-FUNC-005](./../requirements/functional/REQ-FUNC-005.md), [REQ-FUNC-006](./../requirements/functional/REQ-FUNC-006.md), [REQ-FUNC-007](./../requirements/functional/REQ-FUNC-007.md), [REQ-FUNC-037](./../requirements/functional/REQ-FUNC-037.md), [REQ-FUNC-038](./../requirements/functional/REQ-FUNC-038.md), [REQ-FUNC-039](./../requirements/functional/REQ-FUNC-039.md), [REQ-REL-001](./../requirements/reliability/REQ-REL-001.md), [REQ-REL-002](./../requirements/reliability/REQ-REL-002.md), [REQ-OBS-002](./../requirements/observability/REQ-OBS-002.md).  
Related decisions: [DEC-005](./../decisions/DEC-005.md), [DEC-007](./../decisions/DEC-007.md)
Open issues: exact first EPUB editing workflows, EPUB validation library, atomic file replacement technique, rollback/repair strategy, cover image constraints, and user access to previous versions.
