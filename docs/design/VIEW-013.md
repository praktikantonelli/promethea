---
date: 2026-06-08
id: VIEW-013
---

# Deployment and Operations View

## Viewpoint

Deployment, Physical, Resources.

## Representation

The initial deployment design supports a self-hosted server-browser topology. The server runs headlessly and exposes the frontend and/or REST API to browser clients. Persistent data is stored in a configurable data directory or mounted volume.

```text
Browser Client
  |
  | HTTPS recommended for exposed deployments
  v
Reverse Proxy, optional
  |
  v
Promethea Server Container / Binary
├── Rust REST API backend
├── frontend static assets, if served by backend
├── background worker execution
├── configuration loader
└── structured logging
  |
  +--> Catalog database file/service, engine TBD
  |
  +--> Managed asset directory / volume
  |
  +--> Backup/export directory
```

Operational baseline:

- The server can run without a local GUI.
- The deployment package should support containerized installation.
- A configurable data directory is required for database, assets, configuration, and backups.
- Restart recovery must preserve catalog and job/operation history where possible.
- Backup and restore support must cover catalog and managed assets consistently.
- Structured logs should include operation identifiers, job identifiers, and error categories.
- Storage growth visibility should report managed asset size and relevant database size.

#### More Information

Implements: [REQ-DIST-001](./../requirements/distribution/REQ-DIST-001.md), [REQ-INST-001](./../requirements/installation/REQ-INST-001.md), [REQ-INST-002](./../requirements/installation/REQ-INST-002.md), [REQ-INST-003](./../requirements/installation/REQ-INST-003.md), [REQ-AVAIL-001](./../requirements/availability/REQ-AVAIL-001.md), [REQ-AVAIL-002](./../requirements/availability/REQ-AVAIL-002.md), [REQ-OBS-001](./../requirements/observability/REQ-OBS-001.md), [REQ-OBS-002](./../requirements/observability/REQ-OBS-002.md), [REQ-PERF-003](./../requirements/performance/REQ-PERF-003.md), [REQ-PORT-001](./../requirements/portability/REQ-PORT-001.md).  
Related decisions: [DEC-001](./../decisions/DEC-001.md), [DEC-005](./../decisions/DEC-005.md), [DEC-006](./../decisions/DEC-006.md), [DEC-009](./../decisions/DEC-009.md), [DEC-010](./../decisions/DEC-010.md)
Open issues: container base image, reverse-proxy guide, database engine, backup format and retention, supported OS/container environments, and monitoring/metrics interface.
