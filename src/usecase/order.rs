use async_trait::async_trait;
use apalis::prelude::*;
use crate::domain::jobs::OrderJob;
use crate::storage::redis::StorageFactory;
use std::sync::Arc;

#[async_trait]
pub trait OrderUsecase: Send + Sync {
    async fn create_order(&self, event_id: String, device_uuid: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    async fn process_order(&self, job: OrderJob) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

pub struct OrderService {
    storage: Arc<StorageFactory>,
}

impl OrderService {
    pub fn new(storage: Arc<StorageFactory>) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl OrderUsecase for OrderService {
    async fn create_order(&self, event_id: String, device_uuid: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.create_order_storage();

        let task = Task::builder(OrderJob {
            event_id: event_id.clone(),
            device_uuid,
        })
        .run_in_seconds(5)
        .build();

        storage.push_task(task).await?;

        println!("Order create successfully!");
        Ok(event_id)
    }

    async fn process_order(&self, job: OrderJob) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let now = chrono::Local::now();
        println!(
            "[ORDER] Event: {} | Device: {} | Time: {}",
            job.event_id,
            job.device_uuid,
            now.format("%H:%M:%S%.3f")
        );

        println!("Order processed successfully!");
        Ok(())
    }
}
