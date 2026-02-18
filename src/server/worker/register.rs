//! Declarative job registration without macros
//!
//! Provides a simple function to register and run jobs

use apalis::prelude::{Monitor, WorkerBuilder};
use apalis::layers::WorkerBuilderExt;
use std::sync::Arc;

use crate::storage::postgres::StorageFactory;
use crate::handler::workflow::{email::email_handler_fn, order::order_handler_fn};
use crate::AppContainer;

/// Worker configuration
#[derive(Clone, Debug)]
pub struct WorkerConfig {
    pub order_concurrency: usize,
    pub email_concurrency: usize,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            order_concurrency: 2,  // Default: 2 concurrent workers
            email_concurrency: 2,  // Default: 2 concurrent workers
        }
    }
}

pub async fn run_jobs(
    storage_factory: &Arc<StorageFactory>,
    container: AppContainer,
) -> Result<(), Box<dyn std::error::Error>> {
    run_jobs_with_config(storage_factory, container, WorkerConfig::default()).await
}

pub async fn run_jobs_with_config(
    storage_factory: &Arc<StorageFactory>,
    container: AppContainer,
    config: WorkerConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    // Generate unique worker ID based on timestamp
    let worker_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    println!("Worker ID: {}", worker_id);
    println!("Worker Concurrency:");
    println!("  - Order: {} worker instances", config.order_concurrency);
    println!("  - Email: {} worker instances", config.email_concurrency);
    println!();

    let order_storage = storage_factory.create_order_storage();
    let email_storage = storage_factory.create_email_storage();

    println!();
    println!("Starting monitor...");
    println!("All workers registered successfully!");
    println!();

    Monitor::new()
        .register({
            let config = config.clone();
            move |count| {
                let name = format!("order-worker-{}-{}", worker_id, count);
                WorkerBuilder::new(name)
                    .backend(order_storage.clone())
                    .data(container.order_service.clone())
                    .concurrency(config.order_concurrency)
                    .build(order_handler_fn)
            }
        })
        .register({
            let config = config.clone();
            move |count| {
                let name = format!("email-worker-{}-{}", worker_id, count);
                WorkerBuilder::new(name)
                    .backend(email_storage.clone())
                    .data(container.email_service.clone())
                    .concurrency(config.email_concurrency)
                    .build(email_handler_fn)
            }
        })
        .run()
        .await?;
    Ok(())
}
