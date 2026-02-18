use apalis::prelude::*;
use redis::{Client, aio::ConnectionManager};
use rust_apalis_test::domain::jobs::OrderJob;
use rust_apalis_test::storage::redis::StorageFactory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Apalis Job Producer...");
    println!();

    let redis_client = Client::open("redis://127.0.0.1:6379")?;
    let conn = ConnectionManager::new(redis_client).await?;

    let storage_factory = StorageFactory::new(conn);
    let mut storage = storage_factory.create_order_storage();

    let task = Task::builder(OrderJob {
        event_id: "EVT-001".into(),
        device_uuid: "DEV-123".into(),
    })
    .run_in_seconds(5)
    .build();

    storage.push_task(task).await?;

    println!("Order task scheduled successfully!");
    println!("Event ID: EVT-001");
    println!("Device UUID: DEV-123");
    println!();

    Ok(())
}
