//! Declarative job registration without macros
//!
//! Provides a simple function to register and run jobs

use apalis::layers::retry::RetryPolicy;
use apalis::prelude::{Monitor, WorkerBuilder};
use apalis::layers::WorkerBuilderExt;
use std::sync::Arc;
use std::time::Duration;
use tower::retry::backoff::{ExponentialBackoffMaker, MakeBackoff};
use tower::util::rng::HasherRng;

use crate::storage::amqp::StorageFactory;
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
            order_concurrency: 2,
            email_concurrency: 2,
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

    println!("Worker Concurrency:");
    println!("  - Order: {} instances", config.order_concurrency);
    println!("  - Email: {} instances", config.email_concurrency);
    println!();

    let order_storage = storage_factory.create_order_storage();
    let order_backoff = ExponentialBackoffMaker::new(
        Duration::from_secs(2),
        Duration::from_secs(10),
        0.5,
        HasherRng::new(),
    )?
    .make_backoff();

    println!("Registering order worker...");
    monitor = monitor.register(move |_| {
        WorkerBuilder::new("order-worker")
            .backend(order_storage.clone())
            .concurrency(config.order_concurrency)
            .data(container.order_service.clone())
            .retry(RetryPolicy::retries(3).with_backoff(order_backoff.clone()))
            .build(order_handler_fn)
    });

    let email_storage = storage_factory.create_email_storage();
    let email_backoff = ExponentialBackoffMaker::new(
        Duration::from_secs(2),
        Duration::from_secs(10),
        0.5,
        HasherRng::new(),
    )?
    .make_backoff();

    println!("Registering email worker...");
    monitor = monitor.register(move |_| {
        WorkerBuilder::new("email-worker")
            .backend(email_storage.clone())
            .concurrency(config.email_concurrency)
            .data(container.email_service.clone())
            .retry(RetryPolicy::retries(3).with_backoff(email_backoff.clone()))
            .build(email_handler_fn)
    });

    println!();
    println!("Starting monitor...");
    println!("All workers registered successfully!");
    println!();

    monitor.run().await?;
    Ok(())
}
