//! Declarative job registration without macros
//!
//! Provides a simple function to register and run jobs

use apalis::layers::retry::RetryPolicy;
use apalis::prelude::{Monitor, WorkerBuilder};
use apalis::layers::WorkerBuilderExt;
use std::time::Duration;
use tower::retry::backoff::{ExponentialBackoffMaker, MakeBackoff};
use tower::util::rng::HasherRng;

use crate::storage::redis::StorageFactory;
use crate::workflow::handler::{email::email_handler_fn, order::order_handler_fn};
use crate::AppContainer;

pub async fn run_jobs(
    storage_factory: &StorageFactory,
    container: AppContainer,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = Monitor::new();

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
        move |count| {
            println!("Starting order worker instance {}", count);
            WorkerBuilder::new(format!("order-worker-{}", count))
                .backend(order_storage.clone())
                .data(order_service.clone())
                .retry(RetryPolicy::retries(3).with_backoff(order_backoff.clone()))
                .build(order_handler_fn)
        }
    });

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
        move |count| {
            println!("Starting email worker instance {}", count);
            WorkerBuilder::new(format!("email-worker-{}", count))
                .backend(email_storage.clone())
                .data(email_service.clone())
                .retry(RetryPolicy::retries(3).with_backoff(email_backoff.clone()))
                .build(email_handler_fn)
        }
    });

    println!("Starting monitor...");
    monitor.run().await?;
    Ok(())
}
