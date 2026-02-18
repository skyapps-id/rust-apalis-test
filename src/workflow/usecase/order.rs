use crate::domain::jobs::OrderJob;

pub struct OrderService;

impl OrderService {
    pub async fn process_order(&self, job: OrderJob) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
