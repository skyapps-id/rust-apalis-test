//! Storage layer - Storage abstractions for job queues.
//!
//! This module provides storage implementations and factory functions
//! for creating job queue storages.

pub mod redis;

pub use redis::StorageFactory;
