---
date: 2026-06-08
id: VIEW-007
---

# REST and External Interface View

## Viewpoint

Interface.

## Representation

Promethea exposes a REST API for all supported client-server operations. The API should be documented in OpenAPI or an equivalent contract before the self-hosted server release. Mutating client operations should return deterministic success/error responses and should not require direct database or asset-store access by clients.

Representative API groups:

```text
/auth
  POST /auth/login
  POST /auth/logout
  GET  /auth/session

/books
  GET    /books
  POST   /books/import
  GET    /books/{book_id}
  PATCH  /books/{book_id}/metadata
  GET    /books/{book_id}/download
  POST   /books/{book_id}/cover

/authors
  GET    /authors
  GET    /authors/{author_id}
  PATCH  /authors/{author_id}
  POST   /authors/{author_id}/image-url

/series
  GET    /series
  GET    /series/{series_id}
  PATCH  /series/{series_id}
  POST   /series/{series_id}/relationships

/metadata
  POST   /metadata/search
  GET    /metadata/candidates/{candidate_id}
  POST   /metadata/candidates/{candidate_id}/apply

/reading
  GET    /reading/status
  PATCH  /reading/status/{book_or_edition_id}
  POST   /reading/events
  GET    /reading/analytics

/jobs
  GET    /jobs
  GET    /jobs/{job_id}
  POST   /jobs/{job_id}/cancel, if supported

/admin
  GET    /admin/storage
  POST   /admin/backup
  POST   /admin/restore, if supported
```

External interfaces include metadata providers, external image URLs, file import/download, and future mounted-folder e-reader sync. Provider-specific contracts remain TBD. Provider responses should be normalized into metadata candidates before application.

#### More Information

Implements: [REQ-INT-003](./../requirements/interface/REQ-INT-003.md), [REQ-INT-004](./../requirements/interface/REQ-INT-004.md), [REQ-INT-005](./../requirements/interface/REQ-INT-005.md), [REQ-INT-006](./../requirements/interface/REQ-INT-006.md), [REQ-INT-007](./../requirements/interface/REQ-INT-007.md), [REQ-INT-008](./../requirements/interface/REQ-INT-008.md), [REQ-FUNC-008](./../requirements/functional/REQ-FUNC-008.md), [REQ-FUNC-009](./../requirements/functional/REQ-FUNC-009.md), [REQ-FUNC-013](./../requirements/functional/REQ-FUNC-013.md), [REQ-FUNC-027](./../requirements/functional/REQ-FUNC-027.md), [REQ-FUNC-032](./../requirements/functional/REQ-FUNC-032.md), [REQ-SEC-001](./../requirements/security/REQ-SEC-001.md) through [REQ-SEC-005](./../requirements/security/REQ-SEC-005.md).  
Related decisions: [DEC-001](./../decisions/DEC-001.md), [DEC-003](./../decisions/DEC-003.md), [DEC-008](./../decisions/DEC-008.md)
Open issues: final API resource names, authentication/session mechanism, OpenAPI generation approach, error schema, pagination/filtering convention, and provider contracts.
