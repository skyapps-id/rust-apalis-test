//! Clean architecture job processing library with apalis.
//!
//! # Architecture
//!
//! - **Domain**: Job types and business entities
//! - **Workflow**: Job handlers and business logic
//! - **Server**: Worker implementations and monitoring
//! - **Storage**: Storage abstractions

pub mod domain;
pub mod workflow;
pub mod server;
pub mod storage;

// Re-export commonly used types
pub use domain::jobs::*;
