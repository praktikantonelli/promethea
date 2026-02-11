# Hexagonal Architecture

## Status
In progress

## Context

Because the scope of this project is already quite big and will potentially only get bigger, a defined architecture on how to structure the different components is needed. This reduces coupling between components and ensures that the code remains readable and maintainable. 

## Decision

In order to ensure that each component truly stays separate, a hexagonal design is used, where each component handles only one responsibility, and any interaction with that area will only be possible through its component. An orchestrator is responsible for aggregating all components, and both the Tauri framework and a potential future server-only framework will only be thin wrappers around said orchestrator. Use cases define typical workflows and are responsible for calling and using the adapters appropriately. 

## Architecture Overview

```mermaid
flowchart LR

  subgraph CORE[shared-core]
    direction TB

    APP[Application Services]
    DOMAIN[Domain Model]

    CATALOG[CatalogUseCases]
    META[MetadataUseCases]
    READING[ReadingUseCases]
    ANALYTICS[AnalyticsUseCases]
    SYNC[DeviceSyncUseCases]

    DBP[DatabasePort]
    FSP[FileSystemPort]
    METAPROV[MetadataProviderPort]
    CLOCK[ClockPort]
    IDX[SearchIndexPort]

    APP --> DOMAIN

    APP -.->|implements| CATALOG
    APP -.->|implements| META
    APP -.->|implements| READING
    APP -.->|implements| ANALYTICS
    APP -.->|implements| SYNC

    APP --> DBP
    APP --> FSP
    APP --> METAPROV
    APP --> CLOCK
    APP --> IDX
  end

  %% Blank title + in-subgraph label avoids edges crossing the "adapters" heading
  subgraph ADAPTERS[ ]
    direction TB

    %% Inbound anchors (transparent) to force a bend on entry
    AIN_DBP(( ))
    AIN_FSP(( ))
    AIN_METAPROV(( ))
    AIN_CLOCK(( ))
    AIN_IDX(( ))
    style AIN_DBP fill:transparent,stroke:transparent
    style AIN_FSP fill:transparent,stroke:transparent
    style AIN_METAPROV fill:transparent,stroke:transparent
    style AIN_CLOCK fill:transparent,stroke:transparent
    style AIN_IDX fill:transparent,stroke:transparent

    DBAD[DB Adapter]
    FSAD[FS Adapter]
    METAAD[Metadata Adapter]
    CLOCKAD[Clock Adapter]
    IDXAD[Index Adapter]

    %% Internal edges from anchors to adapters
    AIN_DBP --> DBAD
    AIN_FSP --> FSAD
    AIN_METAPROV --> METAAD
    AIN_CLOCK --> CLOCKAD
    AIN_IDX --> IDXAD

    ADAPTERS_LABEL[adapters]
    style ADAPTERS_LABEL fill:transparent,stroke:transparent,color:#666
  end

  %% Cross-subgraph edges go to anchors first (this creates the "sharp bend")
  DBP --> AIN_DBP
  FSP --> AIN_FSP
  METAPROV --> AIN_METAPROV
  CLOCK --> AIN_CLOCK
  IDX --> AIN_IDX

  subgraph DESKTOP[desktop-host]
    direction TB

    TAURI[Tauri Inbound Adapter]
    DW[Desktop Composition Root]

    TAURI -->|calls| CATALOG
    TAURI -->|calls| META
    TAURI -->|calls| READING
    TAURI -->|calls| ANALYTICS

    DW -->|constructs| APP
    DW -->|injects| TAURI

    DW --> DBAD
    DW --> FSAD
    DW --> METAAD
    DW --> CLOCKAD
    DW --> IDXAD
  end

  subgraph SERVER[server-host future]
    direction TB

    HTTP[HTTP Inbound Adapter]
    SW[Server Composition Root]

    HTTP -->|calls| CATALOG
    HTTP -->|calls| META
    HTTP -->|calls| READING
    HTTP -->|calls| ANALYTICS
    HTTP -->|calls| SYNC

    SW -->|constructs| APP
    SW -->|injects| HTTP

    SW --> DBAD
    SW --> FSAD
    SW --> METAAD
    SW --> CLOCKAD
    SW --> IDXAD
  end

  subgraph FE[Frontend]
    direction TB

    UI[React UI]

    UI -->|IPC| TAURI
    UI -->|REST future| HTTP
  end
```


## Consequences

### Positive

- Each component is completely independent and therefore allows testing and modifying without affecting other components.
- If the project ever reaches a state where a server is implemented, only a very thin wrapper will have to be implemented.
- Implementation details are hidden by default.

### Negative

- The architecture is quite complex, and will require a lot of work to set up.
- For an extra HTTP backend, some Tauri features cannot be used out of the box.
- The architecture is an as-of-yet unknown concept; understanding and applying it will take time.
