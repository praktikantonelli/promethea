# Shared Library

This crate contains all platform-agnostic and 3rd-party-independent code for the backend. It is further split into different submodules. Note that there are very few choice exceptions for allowed 3rd-party dependencies in this crate. 

## Domain
The [`domain`](./src/domain) module defines common entities, such as objects like books and shared error types. This should be kept as dependency-free as possible because the entities should just be *logical*. Exceptions are limited to utility that has to be defined on the type level (e.g., derives for `serde`, error traits from `thiserror` and types not natively supported, such as dates and times). 
 

## Ports
The [`ports`](./src/ports/) module defines all ports of the system. A port defines the communication between the system as a whole and the port's specific subsystem (e.g., the database, an HTTP client, the file system, etc.). Each port has a corresponding adapter in [`adapters`](../shared). A port should be defined in a way that makes it completely independent of concrete implementation details. For example, a database port should be written so it can be implemented for any database crate (e.g., `sqlx`, `diesel`, any NoSQL-database crate). 


## Use Cases
The [`usecases`](./src/usecases/) module defines specific actions from a user's point of view. Each use case composes the necessary ports as trait objects and defines a single sequence of actions that need to happen to achieve the user's goal. A use case may be "Add a new book to the library", which would then consist of multiple steps, such as 
1. adding a file to the correct place in the file system
2. automatically fetching metadata from the web
3. adding an entry to the database

It is important that use cases use trait objects so the concrete adapters aren't pulled into the crate as a dependency. The composition itself is then done in the main application.
