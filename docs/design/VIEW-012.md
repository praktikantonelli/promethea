---
date: 2026-06-08
id: VIEW-012
---

# Security View

## Viewpoint

Interface, Deployment, Information, Resources.

## Representation

Promethea is initially designed as a personal/self-hosted or small-household product, not a public multi-tenant SaaS. Even so, protected operations must require authentication, user operations must be authorized, external inputs must be validated, provider credentials/secrets must be protected, and secure deployment over HTTPS must be supported.

Security boundaries:

```text
Untrusted / semi-trusted inputs
├── browser requests
├── uploaded EPUB files
├── pasted image URLs
├── metadata provider responses
├── future mounted device paths
└── configuration values
        |
        v
REST API boundary
├── authentication
├── authorization
├── validation
├── request size/type limits
├── error normalization
└── audit/operation logging
        |
        v
Trusted application boundary
├── domain services
├── database access
├── asset store access
├── provider credentials
└── background jobs
```

Security design principles:

- Clients do not directly modify the server database.
- Mutating operations require an authenticated user or approved local administrative context.
- Authorization decisions should be centralized enough to support future user roles.
- Uploaded EPUBs and external images are validated before storage/use.
- External image URL fetching must restrict media types, response size, redirects, and internal-network access according to the final security policy.
- Provider credentials are stored outside source code and protected in configuration/secrets storage.
- HTTPS is supported for exposed deployments, possibly through reverse proxy configuration.
- Reading data is treated as private user data.

#### More Information

Implements: [REQ-SEC-001](./../requirements/security/REQ-SEC-001.md), [REQ-SEC-002](./../requirements/security/REQ-SEC-002.md), [REQ-SEC-003](./../requirements/security/REQ-SEC-003.md), [REQ-SEC-004](./../requirements/security/REQ-SEC-004.md), [REQ-SEC-005](./../requirements/security/REQ-SEC-005.md), [REQ-COMP-003](./../requirements/compliance/REQ-COMP-003.md), [REQ-INT-008](./../requirements/interface/REQ-INT-008.md), [REQ-COMP-001](./../requirements/compliance/REQ-COMP-001.md), [REQ-COMP-002](./../requirements/compliance/REQ-COMP-002.md).  
Related decisions:
Open issues: authentication model, role model, reverse-proxy assumptions, HTTPS termination, provider credential storage, image URL SSRF protections, audit requirements, and privacy policy wording.
