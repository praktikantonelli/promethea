# REQ-INT-003 — REST API interface
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

