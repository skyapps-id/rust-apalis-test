use crate::domain::jobs::OrderJob;
use super::super::handler::JobHandler;

#[derive(Clone)]
pub struct OrderHandler;

impl OrderHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OrderHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl JobHandler for OrderHandler {
    type Job = OrderJob;

    async fn handle(
        &self,
        job: Self::Job,
        attempt: usize,
        max_retries: usize,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let event_id = job.event_id.clone();
        println!("Processing order for event: {}", event_id);
        println!("Device UUID: {}", job.device_uuid);

        if attempt < max_retries {
            let error = "Simulated error for testing retry!";
            return Err(error.into());
        }

        println!("Order processed successfully!");
        Ok(())
    }
}
