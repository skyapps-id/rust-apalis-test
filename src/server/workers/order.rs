use apalis::prelude::*;
use apalis_core::task::attempt::Attempt;
use std::sync::Arc;

use crate::domain::jobs::OrderJob;
use crate::workflow::{JobHandler, handlers::OrderHandler};

pub async fn order_handler_fn(
    job: OrderJob,
    handler: Data<Arc<OrderHandler>>,
    attempt: Attempt,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let event_id = job.event_id.clone();
    let now = chrono::Local::now();
    println!(
        "[Attempt {}/3] Event: {} at {}",
        attempt.current(),
        event_id,
        now.format("%H:%M:%S%.3f")
    );

    handler.handle(job, attempt.current(), 3).await
}
