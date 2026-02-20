//! Declarative job registration without macros
//!
//! Provides a simple function to register and run jobs

use apalis::layers::retry::RetryPolicy;
use apalis::prelude::{Monitor, ParallelizeExt, WorkerBuilder};
use apalis::layers::WorkerBuilderExt;
use std::sync::Arc;
use std::time::Duration;
use tower::retry::backoff::{ExponentialBackoffMaker, MakeBackoff};
use tower::util::rng::HasherRng;

use crate::storage::redis::StorageFactory;
use crate::handler::workflow::{email::email_handler_fn, order::order_handler_fn};
use crate::AppContainer;

/// Worker configuration
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
    let mut monitor = Monitor::new();

    // Generate unique worker ID based on timestamp
    let worker_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    println!("Worker ID: {}", worker_id);
    println!("Worker Concurrency:");
    println!("  - Order: {}", config.order_concurrency);
    println!("  - Email: {}", config.email_concurrency);
    println!();

    // Register ORDER workers
    println!("Registering order worker...");
    let order_storage = storage_factory.create_order_storage();
    let order_backoff = ExponentialBackoffMaker::new(
        Duration::from_secs(2),
        Duration::from_secs(10),
        0.5,
        HasherRng::new(),
    )?
    .make_backoff();

    monitor = monitor.register({
        let order_service = container.order_service.clone();
        let worker_id = worker_id;

        move |_count| {
            WorkerBuilder::new(format!("order-worker-{}", worker_id))
                .backend(order_storage.clone())
                .data(order_service.clone())
                .retry(RetryPolicy::retries(3).with_backoff(order_backoff.clone()))
                .concurrency(config.order_concurrency)
                .parallelize(tokio::task::spawn)
                .build(order_handler_fn)
        }
    });

    // Register EMAIL workers
    println!("Registering email worker...");
    let email_storage = storage_factory.create_email_storage();
    let email_backoff = ExponentialBackoffMaker::new(
        Duration::from_secs(2),
        Duration::from_secs(10),
        0.5,
        HasherRng::new(),
    )?
    .make_backoff();

    monitor = monitor.register({
        let email_service = container.email_service.clone();
        let worker_id = worker_id;

        move |_count| {
            WorkerBuilder::new(format!("email-worker-{}", worker_id))
                .backend(email_storage.clone())
                .data(email_service.clone())
                .retry(RetryPolicy::retries(3).with_backoff(email_backoff.clone()))
                .concurrency(config.email_concurrency)
                .parallelize(tokio::task::spawn)
                .build(email_handler_fn)
        }
    });

    println!();
    println!("Starting monitor...");
    println!("All workers registered successfully!");
    println!();

    monitor.run().await?;
    Ok(())
}
