# Domain Model
This library contains the domain model of the project. Its purpose is to define business logic without any dependencies to anything else, apart from very few low-level utility libraries (e.g., error handling, date and time etc.).

The goal is that the domain model forms the core of the application. Inbound and outbound requests can and should depend on it, but not vice versa!

# Key Goals
## Format
The model should support multiple formats of the same book. For example, a book may be in the library in the form of an ebook _and_ and audiobook. Both files represent the same literary work, but they are clearly distinct.

## Omnibus
Ebooks often come in collections where one "digital book" contains multiple literary works. This should be reflected in the model. 

## Multiple Authors
A literary work may be a collaboration of multiple authors, so that should be captured by the model.

## Multiple Series
Some authors define multiple series where one may contain the other, for example when an overarching series is split into multiple smaller sub-series. Also, when a book is an omnibus collection, it will even need to support multiple installments within the same series.
This means that something like a hash map cannot be used to model a book's affiliation in a series; an omnibus collection would need to be a member of the same series multiple times with different series indexes.

## Uniqueness
Each book, author and series should be unique. But given that each of those entities is usually known by its name or title, and that especially titles of books and series are arbitrary, the entities should be distinguished by a unique ID. 

## Physical Books
The library should allow the tracking and representing of physical books. While they're not directly as useful as the other formats, the library should be able to track information about them too.

# Ubiquitous Language
A literary work (represented as `LiteraryWork` in the code) represents one unique piece of literature, generally defined uniquely by a title and a list of authors. This representation is meant on a high level, meaning there is no distinction here regarding multiple editions due to revisions, different formats published by different publishers, different editions (e.g., anniversary, collector's editions etc.). 

The `Format` represents a concrete entry in the library that maps to one (or more) `LiteraryWork`(s). In general, the mapping is one-to-one, but it may also be a one-to-many mapping due to omnibus collections. Each `Format` represents a different way in which the book may be available. An `EBook` is a digital version of a book, an `Audiobook` is a read-out version and a `PhysicalBook` is a representation of a physical book. Each `LiteraryWork` may be present in any number of each `Format`, but it will generally only be present in at most one of each (e.g., one `EBook`, one `Audiobook` and one `PhysicalBook`).

# Open Points
## Book vs. Formats
From the key goals, it's clear that a book has to be separated from its formats. A distinction between a unique piece of literature and its concrete formats is needed, but it's not clear yet which exact parts belong to which side of this distinction. 

A model may say that each piece of literature is modeled as a work, and that work has a one-to-many mapping to each available format. For example, a piece of literature may be available as an e-book, an audiobook and a physical book. But that causes problems with omnibus collections because there, the relationship would have to be inverted: 

A format that contains multiple literary works would need to have a one-to-many mapping too, which would turn into a many-to-one mapping from the literary work's perspective. Maybe a many-to-many mapping?

# Sources
Some sources for domain-driven design, Rust web servers and hexagonal architecture:
[The Hexagonal (Ports & Adapters) Architecture](https://alistair.cockburn.us/hexagonal-architecture): the original article by Alastair Cockburn on hexagonal architecture.

[Hexagonal Architecture in Rust - From Zero to Ports and Adapters](https://chirath.hashnode.dev/hexagonal-architecture-in-rust-from-zero-to-ports-and-adapters): a blog post by Chirath Nissanka built around a real codebase. 

[Domain-Driven Design Reference](https://www.domainlanguage.com/wp-content/uploads/2016/05/DDD_Reference_2015-03.pdf): a reference of definitions and pattern summaries by Eric Evans (author of Domain-Driven Design). 

[Hexagonal Rust Template](https://github.com/llmgraph-ai/hexagonal-rust-template): a GitHub template repository for a Rust backend following the ports and adapters model.

[Axum Clean Architecture](https://github.com/codemountains/axum-ddd-explicit-architecture): a GitHub repository of an example Rust project using Axum with the clean architecture (very similar if not identical to hexagonal). 

[Hexagonal Sandwich Recipes](https://github.com/firegloves/Hexagonal-Sandwich-Recipes): a GitHub repository following [this](https://medium.com/@lucorset/hexagonal-architecture-in-rust-72f8958eb26d) Medium article series on hexagonal architecture in Rust.

[Axum Diesel Real-World Example](https://github.com/Quentin-Piot/axum-diesel-real-world): a real-world example of a GitHub repository doing domain-driven design with the Diesel SQL crate.
