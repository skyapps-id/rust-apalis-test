use apalis::prelude::*;
use apalis_redis::RedisStorage;
use redis::{Client, aio::ConnectionManager};
use rust_apalis_test::domain::jobs::OrderJob;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Apalis Job Producer...");
    println!();

    let redis_client = Client::open("redis://127.0.0.1:6379")?;
    let conn = ConnectionManager::new(redis_client).await?;

    let mut storage: RedisStorage<OrderJob> = RedisStorage::new(conn.clone());

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

    // Example: Schedule an email job
    // let mut email_storage: RedisStorage<EmailJob> = RedisStorage::new(conn);
    // let email_task = Task::builder(EmailJob {
    //     to: "user@example.com".to_string(),
    //     subject: "Test Email".to_string(),
    //     body: "Hello from apalis!".to_string(),
    // })
    // .run_after(Duration::from_secs(60))
    // .build();
    // email_storage.push_task(email_task).await?;
    // println!("Email task scheduled successfully!");

    Ok(())
}
