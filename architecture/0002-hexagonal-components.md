# Hexagonal Architecture

## Status
In progress

## Context

Because the scope of this project is already quite big and will potentially only get bigger, a defined architecture on how to structure the different components is needed. This reduces coupling between components and ensures that the code remains readable and maintainable. 

## Decision

In order to ensure that each component truly stays separate, a hexagonal design is used, where each component handles only one responsibility, and any interaction with that area will only be possible through its component. An orchestrator is responsible for aggregating all components, and both the Tauri framework and a potential future server-only framework will only be thin wrappers around said orchestrator.

## Architecture Overview

Ports are defined as interfaces between the main application and each component. Each component implements its specific interface, thereby providing an adapter that hides business logic behind an API. Currently, there are three components: database access, file system access and metadata scraping. Each component will come with a trait that defines the necessary interface, and a struct for each component will implement the necessary trait. 

```plantuml
@startuml
skinparam componentStyle rectangle
skinparam shadowing false

' =========================
' Shared core (domain + application)
' =========================




package shared-core {
  component "Use Cases" as App
  component "Domain Model" as Domain

  ' Inbound port(s)
  interface  CoreApiPort
  CoreApiPort ..> App : calls use cases

  App --> Domain

  ' Outbound ports (driven)
  interface DatabasePort
  interface FileSystemPort
  interface WebScraperPort

  App ..> DatabasePort
  App ..> FileSystemPort
  App ..> WebScraperPort
}

' =========================
' Shared adapters (identical in desktop + server)
' =========================
package adapters {
  component "DB Adapter" as SharedDbAdapter
  component "FS Adapter" as SharedFsAdapter
  component "Web Scraper Adapter" as SharedScraperAdapter

  SharedDbAdapter ..|> DatabasePort
  SharedFsAdapter ..|> FileSystemPort
  SharedScraperAdapter ..|> WebScraperPort
}

' =========================
' Desktop host: inbound adapter + wiring
' =========================
package desktop-host {
  component "Tauri Inbound Adapter\n(Commands/IPC)" as TauriInbound
  TauriInbound ..|> CoreApiPort

  component "Desktop Composition Root\n(wires core + shared adapters)" as DesktopWiring

  DesktopWiring --> App
  DesktopWiring --> TauriInbound
  DesktopWiring --> SharedDbAdapter
  DesktopWiring --> SharedFsAdapter
  DesktopWiring --> SharedScraperAdapter
}

' =========================
' Server host: inbound adapter + extra services + wiring
' =========================
package server-host {
  component "HTTP Inbound Adapter" as RESTInbound
  RESTInbound ..|> CoreApiPort

  component "Server-Only Services\n(inbound)" as ServerOnly
  component "REST API Routes" as RestRoutes
  component "Kobo Sync REST API" as Ops

  RESTInbound --> RestRoutes
  ServerOnly --> Ops

  component "Server Composition Root\n(wires core + shared adapters + extras)" as ServerWiring

  ServerWiring --> App
  ServerWiring --> RESTInbound
  ServerWiring --> SharedDbAdapter
  ServerWiring --> SharedFsAdapter
  ServerWiring --> SharedScraperAdapter
  ServerWiring --> ServerOnly
}



package "Frontend" {

  component "Desktop Frontend" as DesktopFrontend
  component "Server Frontend" as ServerFrontend
  interface ReactFrontend

  ReactFrontend --> TauriInbound : IPC
  ReactFrontend --> RESTInbound : REST
  DesktopFrontend --|> ReactFrontend
  ServerFrontend --|> ReactFrontend
}

@enduml


```
In general, interfaces are only to be used when necessary, i.e., only when there are multiple implementations that need to shareable between desktop and server. As such, the adapters will not be associated with an interface for now. Those interfaces are currently not required, and if they at some point become a necessity, adding them will still be a manageable refactor. 

## Consequences

### Positive

- Each component is completely independent and therefore allows testing and modifying without affecting other components.
- If the project ever reaches a state where a server is implemented, only a very thin wrapper will have to be implemented.
- Implementation details are hidden by default.

### Negative

- The architecture is quite complex, and will require a lot of work to set up.
- For an extra HTTP backend, some Tauri features cannot be used out of the box.
- The architecture is an as-of-yet unknown concept; understanding and applying it will take time.
