//! `shared`
//!
//! Core library for the platform-independent logic of Promethea.

/// domain models, defines platform-agnostic types, errors and entities
pub mod domain;
/// hexagonal ports (traits) that define interactions between a sub-part of the system and the rest
pub mod ports;
/// use cases compose all necessary adapters to form a logical order of operations
pub mod usecases;
