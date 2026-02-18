//! Storage layer - Storage abstractions for job queues.
//!
//! This module provides storage implementations and factory functions
//! for creating job queue storages.

pub mod postgres;

pub use postgres::StorageFactory;
