---
date: 2026-06-08
id: VIEW-006
---

# Persistence and Asset Storage View

## Viewpoint

Information, Resources, Patterns.

## Representation

Promethea stores structured catalog state in a database and binary content in a managed asset store. The database is the source of truth for entity relationships, metadata, reading state, job status, operation history, asset references, and migration state. The asset store is the source of truth for EPUB files, covers, author images, previous EPUB versions, and export/backup artifacts.

```text
Catalog Database
├── schema_migrations
├── works
├── editions
├── book_files
├── authors
├── book_authors / author_roles
├── series
├── series_memberships
├── series_relationships
├── assets
├── file_versions
├── metadata_candidates
├── reading_states
├── reading_events
├── jobs
├── operation_history
├── provider_config
└── users / auth entities, model TBD

Managed Asset Store
├── originals/
│   └── {book_file_id}/{checksum}.epub
├── current/
│   └── {book_file_id}/current.epub
├── versions/
│   └── {book_file_id}/{version_id}.epub
├── covers/
├── author-images/
├── thumbnails/
└── backups/
```

The database and asset store must be coordinated when importing or modifying files. The design should avoid leaving catalog rows pointing to missing files and should avoid overwriting EPUBs without preserving the previous state. File checksums support duplicate detection and idempotent import. The exact database engine is still open; SQLite is treated as a POC candidate rather than an irreversible stable-release decision.

#### More Information

Implements: [REQ-FUNC-001](./../requirements/functional/REQ-FUNC-001.md), [REQ-FUNC-002](./../requirements/functional/REQ-FUNC-002.md), [REQ-FUNC-007](./../requirements/functional/REQ-FUNC-007.md), [REQ-FUNC-010](./../requirements/functional/REQ-FUNC-010.md), [REQ-FUNC-013](./../requirements/functional/REQ-FUNC-013.md), [REQ-FUNC-037](./../requirements/functional/REQ-FUNC-037.md) through [REQ-FUNC-039](./../requirements/functional/REQ-FUNC-039.md), [REQ-PERF-003](./../requirements/performance/REQ-PERF-003.md), [REQ-REL-001](./../requirements/reliability/REQ-REL-001.md), [REQ-REL-002](./../requirements/reliability/REQ-REL-002.md), [REQ-REL-004](./../requirements/reliability/REQ-REL-004.md), [REQ-AVAIL-001](./../requirements/availability/REQ-AVAIL-001.md), [REQ-AVAIL-002](./../requirements/availability/REQ-AVAIL-002.md), [REQ-INST-002](./../requirements/installation/REQ-INST-002.md), [REQ-MAINT-002](./../requirements/maintainability/REQ-MAINT-002.md), [REQ-POC-001](./../requirements/proof-of-concept/REQ-POC-001.md).  
Related decisions: [DEC-005](./../decisions/DEC-005.md), [DEC-007](./../decisions/DEC-007.md), [DEC-009](./../decisions/DEC-009.md)
Open issues: final database engine, migration tool, backup format, restore scope, asset path conventions, max file/image sizes, and cleanup policy for orphaned assets.
