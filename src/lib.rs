//! Clean architecture job processing library with apalis.
//!
//! # Architecture
//!
//! - **Domain**: Job types and business entities
//! - **Workflow**: Job handlers and use cases
//! - **Server**: Job registration and monitoring
//! - **Storage**: Storage abstractions

pub mod container;
pub mod domain;
pub mod workflow;
pub mod server;
pub mod storage;

// Re-export commonly used types
pub use domain::jobs::*;
pub use container::AppContainer;
