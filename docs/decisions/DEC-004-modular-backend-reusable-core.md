---
id: DEC-004-modular-backend-reusable-core
status: "proposed"
date: 2026-06-08
---

# Structure the backend as a modular reusable core with adapters

## Context and Problem Statement

Promethea must support catalog management, EPUB processing, metadata provider integration, reading tracking, jobs, assets, and future desktop or local workflows. These concerns should be testable and evolvable without mixing transport, storage, provider, and domain behavior into one undifferentiated server implementation.

## Decision Drivers

* [REQ-MAINT-001](./../requirements/maintainability/REQ-MAINT-001.md) requires modular backend organization.
* [REQ-REUSE-002](./../requirements/reusability/REQ-REUSE-002.md) proposes a reusable Rust core.
* [REQ-BUILD-001](./../requirements/build/REQ-BUILD-001.md) requires Rust backend implementation.
* Future desktop/local workflows should be possible without rewriting domain logic.
* Provider and storage choices remain partially open and should be isolated behind adapters.

## Considered Options

* Modular core plus transport/storage/provider/job adapters
* Single monolithic server crate with all concerns mixed
* Initial microservices split

## Decision Outcome

Chosen option: "Modular core plus transport/storage/provider/job adapters", because it supports maintainability, testability, and future reuse without imposing microservice deployment complexity on a self-hosted product.

### Consequences

* Good, because domain behavior can be tested without running the HTTP server.
* Good, because storage, provider, and job infrastructure can evolve behind interfaces.
* Good, because a reusable Rust core can serve future desktop/local contexts.
* Bad, because module boundaries and dependency rules must be designed and enforced.
* Bad, because excessive abstraction could slow early development if not kept pragmatic.

### Confirmation

Confirm by reviewing the backend crate/module graph and dependency direction. Domain/core modules should not depend on HTTP framework details or concrete provider implementations. Code review should reject direct cross-layer shortcuts that bypass the intended module boundaries.

## Pros and Cons of the Options

### Modular core plus transport/storage/provider/job adapters

* Good, because it satisfies maintainability and reusable-core requirements.
* Good, because it keeps external services and persistence replaceable.
* Good, because it gives clear ownership of domain logic.
* Neutral, because the exact crate/package layout can evolve.
* Bad, because it requires architecture discipline from the beginning.

### Single monolithic server crate with all concerns mixed

* Good, because it is initially simple to create.
* Bad, because it weakens testability and future desktop/local reuse.
* Bad, because provider, storage, and HTTP concerns would become entangled.
* Bad, because future refactoring would be more expensive.

### Initial microservices split

* Good, because service boundaries would be explicit.
* Neutral, because some domains such as metadata fetching or jobs could later be split.
* Bad, because it adds deployment and operational complexity unsuitable for the initial self-hosted scope.
* Bad, because it would create distributed consistency problems too early.

## More Information

Affects [VIEW-002](./../design/VIEW-002-top-level-composition.md), [VIEW-003](./../design/VIEW-003-backend-module-dependency.md), and [VIEW-014](./../design/VIEW-014-deferred-extension.md). Implements [REQ-MAINT-001](./../requirements/maintainability/REQ-MAINT-001.md), [REQ-REUSE-002](./../requirements/reusability/REQ-REUSE-002.md), and [REQ-BUILD-001](./../requirements/build/REQ-BUILD-001.md).
