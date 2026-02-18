use redis::{Client, aio::ConnectionManager};
use rust_apalis_test::server::worker::register::run_jobs;
use rust_apalis_test::storage::redis::StorageFactory;
use rust_apalis_test::AppContainer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Apalis Job Worker...");
    println!();

    let redis_client = Client::open("redis://127.0.0.1:6379")?;
    let conn = ConnectionManager::new(redis_client).await?;

    let storage_factory = StorageFactory::new(conn);
    let container = AppContainer::default();

    run_jobs(&storage_factory, container).await?;

    Ok(())
}
