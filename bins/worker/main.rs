use redis::{Client, aio::ConnectionManager};
use rust_apalis_test::server::monitor::JobRegistry;
use rust_apalis_test::storage::redis::StorageFactory;
use rust_apalis_test::server::worker_config::WorkerRetryConfig;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Apalis Job Worker...");
    println!();

    let redis_client = Client::open("redis://127.0.0.1:6379")?;
    let conn = ConnectionManager::new(redis_client).await?;

    let storage_factory = StorageFactory::new(conn);

    // Create retry config for default jobs
    let default_retry_config = WorkerRetryConfig::new(
        3,
        Duration::from_secs(2),
        Duration::from_secs(10),
        0.5,
    );

    // Build job registry and configure all workers
    JobRegistry::new()
        .with_order_storage(storage_factory.create_order_storage())
        .with_order_retry_config(default_retry_config.clone())
        .with_email_storage(storage_factory.create_email_storage())
        .with_email_retry_config(default_retry_config)
        .run()
        .await?;

    Ok(())
}
