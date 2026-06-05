---
status: proposed
date: 2026-05-26
---
# REQ-INT-008 — Client database isolation

## Statement 
The system shall not require desktop, mobile, or browser clients to access the server database file directly.

## Rationale 
The server should remain the authoritative consistency boundary for all clients, enabling a local database to be used safely on the server.

## Acceptance Criteria
- All supported client mutations are expressed as API requests or local embedded-backend commands.
- No supported remote client workflow requires direct filesystem access to the server database.
- Integration tests verify that remote changes go through the server application layer.

## Verification Method 
Inspection

## More Information 
The current potential candidate for a database is SQLite, as tracked in [REQ-POC-001](./../proof-of-concept/REQ-POC-001.md). If that POC fails, other options shall be explored.
