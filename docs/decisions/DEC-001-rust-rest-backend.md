---
id: DEC-001-rust-rest-backend
status: "accepted"
date: 2026-06-08
---

# Use a Rust backend with a REST API as the server boundary

## Context and Problem Statement

Promethea must expose a server API for browser, desktop-oriented, and future mobile-oriented clients. The SRS constrains the backend implementation to Rust and the server interface style to REST, so the design must make the Rust REST server the primary boundary for catalog mutations, asset operations, import workflows, and remote client updates.

## Decision Drivers

* [REQ-BUILD-001](./../requirements/build/REQ-BUILD-001.md) requires a Rust backend.
* [REQ-BUILD-002](./../requirements/build/REQ-BUILD-002.md) and REQ-INT-003 require a REST server/API interface.
* [REQ-FUNC-027](./../requirements/functional/REQ-FUNC-027.md) requires remote updates to go through the server.
* [REQ-DIST-001](./../requirements/distribution/REQ-DIST-001.md) expects a server-browser deployment topology.
* The backend must remain suitable for personal/self-hosted operation without mandatory paid services.

## Considered Options

* Rust backend with REST API
* Rust backend with GraphQL or RPC API
* Non-Rust backend with REST API

## Decision Outcome

Chosen option: "Rust backend with REST API", because it is explicitly required by the SRS and provides a clear, technology-constrained server boundary for all clients.

### Consequences

* Good, because the decision directly satisfies the binding Rust and REST constraints.
* Good, because REST keeps the client/server boundary straightforward for browser, desktop-shell, mobile-oriented, and external integration use cases.
* Good, because the Rust implementation can host reusable core logic while still exposing a stable HTTP API.
* Bad, because GraphQL-style flexible querying and strongly typed RPC tooling are not the primary API style.
* Bad, because API versioning and OpenAPI/schema discipline will need to be handled explicitly.

### Confirmation

Confirm by inspecting the backend repository, build configuration, and API surface. The production server must be implemented in Rust and expose documented REST endpoints for catalog, asset, job, metadata, reading-tracking, and administration workflows. CI should fail if the backend cannot build and test as a Rust project.

## Pros and Cons of the Options

### Rust backend with REST API

* Good, because it satisfies [REQ-BUILD-001](./../requirements/build/REQ-BUILD-001.md), [REQ-BUILD-002](./../requirements/build/REQ-BUILD-002.md), and [REQ-INT-003](./../requirements/interface/REQ-INT-003.md).
* Good, because it is compatible with self-hosted server deployment and a browser-based UI.
* Good, because REST APIs are easy to call from React TypeScript and future clients.
* Neutral, because the API contract must still be documented and versioned.
* Bad, because custom endpoint design is required for complex query/filter scenarios.

### Rust backend with GraphQL or RPC API

* Good, because GraphQL or RPC could improve type-specific client ergonomics.
* Neutral, because Rust can support these API styles technically.
* Bad, because this conflicts with the SRS requirement for a REST server.
* Bad, because it would introduce additional protocol/tooling decisions early in the project.

### Non-Rust backend with REST API

* Good, because many ecosystems can implement REST quickly.
* Bad, because it violates the explicit Rust backend requirement.
* Bad, because it would weaken the reuse path for a Rust core in future local/desktop contexts.

## More Information

Affects [VIEW-001](./../design/VIEW-001-system-context.md), [VIEW-002](./../design/VIEW-002-top-level-composition.md), [VIEW-003](./../design/VIEW-003-backend-module-dependency.md), [VIEW-007](./../design/VIEW-007-rest-external-interface.md), and [VIEW-013](./../design/VIEW-013-deployment-operations.md). Implements [REQ-BUILD-001](./../requirements/build/REQ-BUILD-001.md), [REQ-BUILD-002](./../requirements/build/REQ-BUILD-002.md), [REQ-INT-003](./../requirements/interface/REQ-INT-003.md), [REQ-FUNC-027](./../requirements/functional/REQ-FUNC-027.md), and [REQ-DIST-001](./../requirements/distribution/REQ-DIST-001.md).
