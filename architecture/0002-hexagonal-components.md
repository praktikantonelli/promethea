# Hexagonal Architecture

## Status
Accepted

## Context

Because the scope of this project is already quite big and will potentially only get bigger, a defined architecture on how to structure the different components is needed. This reduces coupling between components and ensures that the code remains readable and maintainable.

## Decision

A hexagonal architecture (ports and adapters) is pursued.

The **desktop-host** and (future) **server-host** each define a **composition root** responsible for wiring the application:
- The composition root constructs the concrete **outbound adapters** (e.g., DB, filesystem, metadata providers, clock).
- The composition root constructs the **application services** (use case implementations) and injects the required **ports** (traits/interfaces) as shared references.
- The composition root injects a handle to these application services into the **inbound adapters** (Tauri commands/IPC and future HTTP handlers).

The **shared-core** crate/module contains:
- the **domain model**
- the **inbound ports** (`CatalogUseCases`, `MetadataUseCases`, `ReadingUseCases`, `AnalyticsUseCases`, `DeviceSyncUseCases`)
- the **outbound ports** (`DatabasePort`, `FileSystemPort`, `MetadataProviderPort`, `ClockPort`, `SearchIndexPort`, …)
- the **application services** that implement the inbound ports and depend only on outbound ports (not on concrete adapter implementations)

## Architecture Overview

![Architecture](./Diagrams/hexagonal.drawio.svg)

## Consequences

### Positive

- Components are decoupled via ports, enabling unit testing and refactoring without pulling in UI, DB, or transport concerns.
- A future server implementation can reuse the same core by adding only thin HTTP inbound adapters and a server composition root.
- Implementation details (DB/filesystem/HTTP clients) are hidden behind ports by default.

### Negative

- The architecture adds upfront complexity (ports, adapters, and wiring) and requires discipline to maintain the dependency direction.
- If an extra HTTP backend is added, some Tauri-specific capabilities may not apply directly and may require alternative implementations in the server-host.
