use crate::domain::jobs::{EmailJob, OrderJob};
use crate::storage::redis::StorageFactory;
use apalis::prelude::*;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait OrderUsecase: Send + Sync {
    async fn create_order(
        &self,
        event_id: String,
        device_uuid: String,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    async fn process_order(
        &self,
        job: OrderJob,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

pub struct OrderService {
    storage: Arc<StorageFactory>,
}

impl OrderService {
    pub fn new(storage: Arc<StorageFactory>) -> Self {
        Self { storage }
    }

    async fn send_order_email(
        &self,
        event_id: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut email_storage = self.storage.create_email_storage();

        let email_task = Task::builder(EmailJob {
            to: "customer@example.com".to_string(),
            subject: format!("Order Processed: {}", event_id),
            body: format!("Your order {} has been processed successfully.", event_id),
        })
        .run_in_seconds(5)
        .build();

        email_storage.push_task(email_task).await?;

        println!("Email notification scheduled for order: {}", event_id);
        Ok(())
    }

    async fn schedule_order_task(
        &self,
        event_id: String,
        device_uuid: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.create_order_storage();

        let task = Task::builder(OrderJob {
            event_id: event_id.clone(),
            device_uuid,
        })
        .run_in_seconds(1)
        .build();

        storage.push_task(task).await?;

        println!("Order task scheduled for event: {}", event_id);
        Ok(())
    }
}

#[async_trait]
impl OrderUsecase for OrderService {
    async fn create_order(
        &self,
        event_id: String,
        device_uuid: String,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.schedule_order_task(event_id.clone(), device_uuid)
            .await?;

        println!("Order created successfully!");
        Ok(event_id)
    }

    async fn process_order(
        &self,
        job: OrderJob,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let now = chrono::Local::now();
        println!(
            "[ORDER] Processing - Event: {} | Device: {} | Time: {}",
            job.event_id,
            job.device_uuid,
            now.format("%H:%M:%S%.3f")
        );

        println!("Order processed successfully!");

        self.send_order_email(job.event_id.clone()).await?;

        Ok(())
    }
}
