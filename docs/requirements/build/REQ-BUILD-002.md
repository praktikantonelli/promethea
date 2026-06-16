---
status: planned
date: 2026-05-26
---
# REQ-BUILD-002 — REST server

## Statement
The server REST API shall be implemented using a Rust library.

## Rationale
The backend shall be written in Rust, so naturally the REST API shall be likewise.

## Acceptance Criteria
- The server routing layer uses a Rust crate for REST API.
- REST endpoints are exposed by the library's server application.
- The project documents any future change away from a 3rd party REST API library as a breaking design decision.

## Verification Method
Inspection

## More Information
This is a design constraint because the user specified the server direction.
