# Software Design Description
## For Promethea

Version 0.1  
Prepared by Luca Antonelli
2026-06-02

## Table of Contents
<!-- TOC -->
* [1. Introduction](#1-introduction)
  * [1.1 Document Purpose](#11-document-purpose)
  * [1.2 Subject Scope](#12-subject-scope)
  * [1.3 Definitions, Acronyms, and Abbreviations](#13-definitions-acronyms-and-abbreviations)
  * [1.4 References](#14-references)
  * [1.5 Document Overview](#15-document-overview)
* [2. Design Overview](#2-design-overview)
  * [2.1 Stakeholder Concerns](#21-stakeholder-concerns)
  * [2.2 Selected Viewpoints](#22-selected-viewpoints)
* [3. Design Views](#3-design-views)
* [4. Decisions](#4-decisions)
* [5. Appendixes](#5-appendixes)
<!-- TOC -->

## Revision History

| Name | Date | Reason For Changes | Version |
|------|------|--------------------|---------|
|Luca Antonelli      | 2026-06-02     | initial version                   |   0.1      |

## 1. Introduction 
This Software Design Description (SDD) describes the initial proposed architecture and design for Promethea. It translates the requirements baseline in the SRS into design views, major design elements, and architecture decision records. The SDD is prescriptive where requirements already impose implementation constraints and intentionally marked as draft/TBD where the SRS leaves open questions.

### 1.1 Document Purpose
The purpose of this SDD is to give developers, maintainers, operators, security reviewers, and future contributors a shared design baseline for Promethea. It describes the system structure, major runtime flows, data ownership, deployment shape, and significant architecture decisions needed to implement and evolve the product. The SDD complements the SRS: the SRS defines what Promethea must do, while this SDD describes how the system is designed to satisfy those requirements.

### 1.2 Subject Scope
This SDD covers the Promethea v0.1 design baseline for a personal/self-hostable e-book library management and reading-tracking system. The design includes a Rust backend, REST API, React TypeScript frontend, catalog database, managed file assets, EPUB import/editing/versioning, metadata/image provider integration, reading tracking, analytics, background jobs, security controls, and self-hosted deployment support.

The design is intentionally focused on the server-browser and shared-frontend product shape. Native desktop packaging, native mobile applications, e-reader synchronization, audiobook support, multi-medium progress synchronization, public SaaS multi-tenancy, browser-based EPUB reading, and AI/ML-based recommendations are outside the initial design scope unless reprioritized through change management.


### 1.3 Definitions, Acronyms, and Abbreviations


| Term | Definition                                                                                                               |
|------|--------------------------------------------------------------------------------------------------------------------------|
| ADR  | Architecture Decision Record; a record of a segnificant design decision, including implications, considered alternatives and drawbacks |
| API  | Application Programming Interface - A set of definitions and protocols for building and integrating application software |
| Asset | A managed media file such as an EPUB e-book file, a cover or author image |
| Backend | The hidden, logical part of a software that handles user requests |
| Catalog | The user-accessible representation of the book database, including books, authors, series, reading status, metadata and assets |
| E-book | A book in electronic format, made to be read on an e-reader or otherwise compatible reader |
| EPUB | Common type of e-book file |
| Frontend | The interface a user sees when communicating with a software |
| Metadata | Data pertaining a book, includes title, author(s), number of pages, date of publication, series |
| Metadata Candidate | A selection of proposed metadata for a book; generally automatically fetched from an API |
| SDD  | Software Design Description - A document that describes the intended purpose, requirements, and nature of a software        |
| SRS  | Software Requirements Specification - A document that defines the features of a software, grouped into categories |
| View | A concrete design description from a specific viewpoint, such as context, composition, logical, interface, runtime, deployment or security |
| Viewpoint | A selected perspective that defines the concerns, notation, and expected content for one or more views |

### 1.4 References

| Reference | Owner/Author | Version/Date | Type | Location |
| --------------- | --------------- | --------------- | --------------- | ---- |
| Hexagonal Architecture in Rust| howtocodeit | 1.1.7 | Blog post | https://www.howtocodeit.com/guides/master-hexagonal-architecture-in-rust |
| SRS | Luca Antonelli | 0.1 / 2026-06-07 | Document | [`srs.md`](./srs.md) |


### 1.5 Document Overview
Section 2 identifies stakeholder concerns and the viewpoints selected to address them. Section 3 contains concrete design views for Promethea, each following the view-template pattern of viewpoint, representation, and supporting information. Section 4 records significant design decisions and links them to the affected views and SRS requirements. Section 5 captures appendixes, traceability, and open issues that should be refined as the design matures.

## 2. Design Overview
Promethea is designed as a self-hostable server application with a reusable frontend and a modular backend. The central design strategy is to keep domain logic independent from transport, storage, provider, EPUB-processing, and job execution concerns so that the system can evolve towards desktop, mobile and sync scenarios without duplicating the core logic.

### 2.1 Stakeholder Concerns

| Stakeholder | Main design concerns | Addressed by viewpoints / views |
|-------------|----------------------|----------------------------------|
| Library Owner / Administrator | Self-hosted installation, data ownership, backups, provider configuration, user/content responsibility, safe EPUB modification | Context, Deployment, Security, Persistence and Asset Storage, EPUB Modification and Versioning |
| Reader / Authenticated User | Browsing usability, book/author/series navigation, reading-status updates, analytics, remote access, privacy of reading data | Frontend Application, Logical Domain Model, Reading Tracking and Analytics, Security, REST and External Interface |
| Maintainer / Developer | Modular implementation, testability, Rust/React constraints, database migrations, reusable core, clear dependencies, future extensibility | Composition, Backend Module and Dependency, Logical Domain Model, Interface, Decisions |
| Operator / Self-Hoster | Container deployment, configurable data directory, restart recovery, observability, backup/restore, resource growth visibility | Deployment and Operations, Persistence and Asset Storage, Security, Job Processing and Automation |
| Security Reviewer | Authentication, authorization, external input validation, image URL download risk, secrets, HTTPS deployment, reading-data privacy | Security, REST and External Interface, Persistence and Asset Storage, Deployment and Operations |
| QA / Verifier | Traceability from requirements to design elements, testable runtime flows, failure behavior, deterministic import/editing semantics | Runtime views, Interface View, Traceability Appendix, Decisions |
| Future Extension Developer | Desktop/mobile reuse, e-reader sync, audiobook model, cross-medium progress, provider abstraction | Deferred Extension View, Backend Module and Dependency View, Logical Domain Model, Decisions |

### 2.2 Selected Viewpoints
This SDD uses a tailored subset of the default viewpoints from the SDD template. The selected viewpoints are included because they address specific concerns in the design of the system. The remaining viewpoints are either omitted or merged into a broader view. 

| Viewpoint | Used? | Purpose in this SDD | Concrete views |
|-----------|-------|---------------------|----------------|
| Context | Yes | Establish system boundary, users, external systems, and deferred external environments. | 3.1, 3.14 |
| Composition | Yes | Decompose the system into frontend, backend, persistence, asset, provider, and job elements. | 3.2, 3.3, 3.4 |
| Logical | Yes | Describe stable domain concepts such as work, edition, book file, author, series, reading event, and automation rule. | 3.5, 3.10 |
| Physical | Partial | Only the self-hosted/container deployment has physical relevance at this stage. Physical hardware topology is otherwise deployment-specific. | 3.13 |
| Structure | Yes | Show internal organization of frontend/backend and major connectors. | 3.2, 3.3, 3.4 |
| Dependency | Yes | Make direction of dependencies explicit, especially around reusable core and adapters. | 3.3 |
| Information | Yes | Define persistent catalog data, asset metadata, versioning, reading events, and operation history. | 3.6, 3.10 |
| Interface | Yes | Describe REST API, file import/download, metadata/image providers, and future device-sync interface. | 3.7 |
| Interaction | Yes | Describe import, metadata enrichment, EPUB modification, reading update, and background-job flows. | 3.8, 3.9, 3.10, 3.11 |
| Algorithm | Partial | Used only where operation logic is architecturally significant, such as import checksums and EPUB mutation/versioning. | 3.8, 3.9 |
| State Dynamics | Partial | Used for reading status, job status, and EPUB modification states. | 3.9, 3.10, 3.11 |
| Concurrency | Partial | Used for background jobs, provider calls, and long-running operations. | 3.11 |
| Patterns | Yes | Records architectural style: modular backend, ports/adapters, server-authoritative API, provider abstraction, versioned asset handling. | 3.3, 3.6, 3.11, Section 4 |
| Deployment | Yes | Map software units to self-hosted/server-browser/container deployment. | 3.13 |
| Resources | Yes | Address file assets, database, job workers, storage growth, logs, backup/restore, and external provider limits. | 3.6, 3.11, 3.13 |

#### 2.2.1 Context
💬 _Defines the system as a black box, identifying its boundaries and its environment._

**Addresses:** System boundaries, environment actors (users, external systems) and offered services (use cases).  
**Typical Languages:** UML Use Case Diagram, C4 Context Diagram.

#### 2.2.2 Composition
💬 _Describes how the system is recursively assembled from major constituent parts (subsystems, components, or modules), and how those are organized and relate to one another_

**Addresses:** Identify the major design elements; allocation of responsibilities, and localization of functionality; modularity (reuse, buy-vs-build) and integration.  
**Typical Languages:** UML Component Diagram, Hierarchical Decomposition Diagram, UML Package (functional), Deployment (runtime) Diagram.

💡 Tips:
- Focus on how components fit together and where external, reused, or third-party components integrate.
- Consider organizing into subcategories for clarity: Functional (logical) decomposition and Runtime (physical) decomposition.

#### 2.2.3 Logical
💬 _Captures the static design structure of the system in terms of types and their implementation (class, interface) and their relationships._

**Addresses:** Development and reuse of appropriate abstractions and their implementations; encapsulation and dependencies among entities.  
**Typical Languages:** UML Class Diagram, UML Object Diagram.

💡 Tips: 
- Focus on the static and stable abstractions that collaborate to fulfill system goals. 
- Complements Composition (assembly) by clarifying the abstractions that underlie it.

#### 2.2.4 Physical
💬 _Depicts the tangible system infrastructure._

**Addresses:** Hardware configuration, physical topology, and physical constraints.  
**Typical Languages:** Hardware Block Diagram, Network Topology Diagram, Rack Layout, Cloud Infrastructure Diagram.

💡 Tips:
- Complements Deployment by showing the platform topology on which software is mapped.

#### 2.2.5 Structure
💬 _Documents internal organization of components and their parts, ports, and connectors_.

**Addresses:** Internal composition of complex entities; reusability of fine-grained components.  
**Typical Languages:** UML composite structure diagram, UML class diagram, UML package diagram, C4 Container diagram.

💡 Tips: 
- Complements Composition by focusing on interfaces and connectors.

#### 2.2.6 Dependency
💬 _Shows how design elements interconnect and access each other, illustrating import, service, or build-time relationships._

**Addresses:** Integration needs and prioritization; coupling and dependencies; root cause and change impact analysis.  
**Typical Languages:** UML Package Diagram, Dependency Graph, UML Component Diagram

💡 Tips: 
- Draw dependencies directionally (“uses”, “requires”, “provides”).

#### 2.2.7 Information
💬 _Models the persistent data structure, its relationships, and the mechanisms for access and management._

**Addresses:** Data structure and semantics; persistence; metadata; data integrity; data management and access schemes.  
**Typical Languages:** Entity-Relationship Diagram, UML Class Diagram, Logical Data Model.

💡 Tips: 
- Use consistent naming with the Logical viewpoint to maintain type alignment.

#### 2.2.8 Interface
💬 _Specifies the externally visible interfaces among components, subsystems, or with external systems._

**Addresses:** Interoperability through contract definition; encapsulation, and integration risks.  
**Typical Languages:** API specifications, IDLs, function/method signature, UML Component Diagram

#### 2.2.9 Interaction
💬 _Illustrates how entities collaborate at runtime via messages: who talks to whom, in what order, and under which conditions._

**Addresses:** Allocation of responsibilities; message sequencing, timing, and synchronization; error propagation; distributed components state transition logic and concurrency.  
**Typical Languages:** UML Sequence Diagram, UML Collaboration Diagram, BPMN Process Flows.

💡 Tips:
- Provide representative “happy-path” and “failure-path” scenarios.
- If concurrency affects ordering, annotate lifelines/regions and reference the Concurrency viewpoint.

#### 2.2.10 Algorithm
💬 _Details the internal processing logic of an operation: steps, decisions, loops, and error handling, emphasizing critical or novel algorithms within the design._

**Addresses:** Computational complexity; time-space processing logic; performance, determinism, and reproducibility.  
**Typical Languages:** Pseudocode, flowchart, Decision Table mathematical formulation.

💡 Tips: 
- Tie each algorithm to its owning class/component.
- Consider referencing Interface contracts to link invariants and pre/postconditions.
- Consider referencing Resource impacts if performance or space is critical.

#### 2.2.11 State Dynamics
💬 _Details how system or component states evolve in response to events or stimuli over time._

**Addresses:** Modes/states, transitions, events/triggers, guards, entry/exit effects, concurrency regions, synchronization.  
**Typical Languages:** UML State Machine Diagram, State Transition Table, Automata, Petri Net.

💡 Tips: 
- Complements Interaction/Algorithm when behavior differs by state.

#### 2.2.12 Concurrency
💬 _Describes how the design handles parallelism, synchronization, and coordination among concurrent entities._

**Addresses:** Thread/process structure; synchronization and locking; concurrency control; event ordering; parallel execution and race conditions.  
**Typical Languages:** UML Activity Diagram, UML Sequence and State Diagram, actor model.

💡 Tips:
- Complements other dynamic viewpoints when parallelism, synchronization, or ordering guarantees are first-class concerns that would clutter those views.

#### 2.2.13 Patterns
💬 _Identifies reusable design ideas and collaborations—design patterns, architectural styles, or framework templates—that guide or constrain the system’s structure and behavior._

**Addresses:** Reuse of proven solutions; consistency of architectural style; collaboration roles and connectors; template-based component structures.  
**Typical Languages:** UML Composite Structure Diagram, UML Package/Class Diagram, Architecture Description Language.

💡 Tips: 
- Record which patterns are applied and where.

#### 2.2.14 Deployment
💬 _Describes how software entities are mapped onto the physical execution environment, what runs where and how nodes are connected_

**Addresses:** Component-to-node allocation; deployment topology; communication mechanisms; distribution, replication, and scaling; operational constraints.  
**Typical Languages:** UML Deployment Diagram, Infrastructure-as-Code topology, Network Diagram, CI/CD pipeline diagrams.

💡 Tips: 
- Include environment tiers and deployment sequencing if relevant.

#### 2.2.15 Resources
💬 _Specifies use and management of shared or limited resources, such as memory, bandwidth, threads, or file handles._

**Addresses:** Resource utilization and allocation; contention and availability; performance bottlenecks; locks and priorities; resource management strategies.  
**Typical Languages:** UML Class Diagram (for resource entities), UML Real-Time Profile, UML Object Constraint Language (OCL), Resource Allocation Table.

💡 Tips: 
- Cross-reference with Concurrency (timing) and Deployment (placement) views for a full runtime picture..

## 3. Design Views

All views are defined in separate files under [`/design`](./design/), following the [view template](./design/view-template.md).

💬 _Documents the main architectural and design elements that define the system._

➥ Define design views to a level of detail sufficient to implement the system (prescriptive architecture) or to understand how to operate or maintain the existing product (descriptive architecture). Use unique identifiers, keep elements concise and modular, and include diagrams or links where applicable. Reference relevant design decisions from Section 4 that this view represents. Include applicable SRS requirement IDs that this element implements when available.

📃 Template:
```markdown
- ID: [NNN]-{title}
- Title: Short, descriptive name of the view.
- Viewpoint: The viewpoint of which this view is an instance.
- Representation: The design view representation per the viewpoint and language selected, e.g., natural language description or a diagram or a combination thereof.
- More Information: Additional context. Links to related artifacts.
```

💡 Tips:
- This section should contain enough information to implement the system (prescriptive architecture) or to understand how to operate or maintain the existing product (descriptive architecture).
- If available, include references to the SRS requirement IDs that the design view implements. This demonstrates how requirements are addressed by the design.
- Reference relevant design decisions from Section 4 that influenced or resulted from this design element.

## 4. Decisions

All decisions are defined in separate architectural decision record files under [`/decisions`](./decisions/), following the [ADR Template](./decisions/adr-template.md).

💬 Captures significant architectural or design decisions and their rationale.

➥ Document significant architectural decisions that have substantial long-term impact on the system's structure,
behavior, or quality attributes.

```markdown
- ID: [NNN]-{title}
- Title: short title, representative of solved problem and found solution.
- Context: Describe the context and problem statement.
- Options: Enumerate considered alternatives.
- Outcome: Chosen option: "{title of option 1}", because {justification}.
- More Information: Additional context. Links to related artifacts.
```

💡 Tips:
- Keep one decision per record.
- Consider adopting MADR (Markdown Architecture Decision Record) pattern directly to document decisions. 

## 5. Appendixes
💬 _Optional supporting material that aids understanding without being normative._

➥ Include glossaries, data dictionaries, models/diagrams, sample datasets, or change-impact analyses that support the main sections. Reference rather than duplicate content when possible.

💡 Tips:
- Keep appendixes organized and referenced from the main text.

