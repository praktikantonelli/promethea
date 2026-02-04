# Hexagonal Architecture

## Status
In progress

## Context

Because the scope of this project is already quite big and will potentially only get bigger, a defined architecture on how to structure the different components is needed. This reduces coupling between components and ensures that the code remains readable and maintainable. 

## Decision

In order to ensure that each component truly stays separate, a hexagonal design is used, where each component handles only one responsibility, and any interaction with that area will only be possible through its component. An orchestrator is responsible for aggregating all components, and both the Tauri framework and a potential future server-only framework will only be thin wrappers around said orchestrator.

## Architecture Overview

Ports are defined as interfaces between the main application and each component. Each component implements its specific interface, thereby providing an adapter that hides business logic behind an API. Currently, there are three components: database access, file system access and metadata scraping. Each component will come with a trait that defines the necessary interface, and a struct for each component will implement the necessary trait. 

```mermaid
classDiagram
direction LR

%% =========================
%% Hosts / Transports
%% =========================
class TauriHost
class ServerHost

%% =========================
%% Core API
%% =========================
class CoreApp {
  +addNewFile(input) Result
}

TauriHost o-- CoreApp : holds
ServerHost o-- CoreApp : holds

%% =========================
%% Use Cases (Orchestrators)
%% =========================
class UC_AddNewFile {
  +execute(input) Result
}


CoreApp --> UC_AddNewFile : uses

%% =========================
%% Common Ports (shared)
%% =========================
class Port_Database {
  <<interface>>
}
class Port_FileSystem {
  <<interface>>
}
class Port_Scraper {
  <<interface>>
}
class Port_Progress {
  <<interface>>
}
class Port_Library {
    <<interface>>
}

UC_AddNewFile ..> Port_Database
UC_AddNewFile ..> Port_FileSystem
UC_AddNewFile ..> Port_Scraper
UC_AddNewFile ..> Port_Progress : optional

%% =========================
%% Composition Roots (wiring)
%% =========================
class DesktopServices {
  +db: Port_Database
  +fs: Port_FileSystem
  +scraper: Port_Scraper
  +progress: Port_Progress
}

class ServerServices {
  +db: Port_Database
  +fs: Port_FileSystem
  +scraper: Port_Scraper
  +library: Port_Library
  +progress: Port_Progress
}

CoreApp *-- DesktopServices : can be built with
CoreApp *-- ServerServices : can be built with

%% =========================
%% Adapters (Implementations)
%% =========================
class Adapter_SqlDatabase
class Adapter_NativeFs
class Adapter_ReqwestScraper
class Adapter_TauriProgress
class Adapter_ServerProgress

Port_Database <|.. Adapter_SqlDatabase
Port_FileSystem <|.. Adapter_NativeFs 
Port_Scraper <|.. Adapter_ReqwestScraper
Port_Progress <|.. Adapter_TauriProgress
Port_Progress <|.. Adapter_ServerProgress
Port_Library <|.. Adapter_LibraryHost

%% Wiring intent (dashed dependencies)
DesktopServices ..> Port_FileSystem : uses
DesktopServices ..> Port_Database : uses
DesktopServices ..> Port_Scraper : uses
DesktopServices ..> Port_Progress : uses

ServerServices ..> Port_FileSystem : uses
ServerServices ..> Port_Database : uses
ServerServices ..> Port_Scraper : uses
ServerServices ..> Port_Progress : uses
ServerServices ..> Port_Library : uses


```

## Consequences

### Positive

- Each component is completely independent and therefore allows testing and modifying without affecting other components.
- If the project ever reaches a state where a server is implemented, only a very thin wrapper will have to be implemented.
- Implementation details are hidden by default.

### Negative

- The architecture is quite complex, and will require a lot of work to set up.
- For an extra HTTP backend, some Tauri features cannot be used out of the box.
- The architecture is a as-of-yet unknown concept; understanding and applying it will take time.
