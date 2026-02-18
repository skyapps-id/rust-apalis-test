use apalis::prelude::*;
use apalis_core::task::attempt::Attempt;

use crate::domain::jobs::OrderJob;
use crate::workflow::usecase::OrderService;

pub async fn order_handler_fn(
    job: OrderJob,
    ctx: Data<std::sync::Arc<OrderService>>,
    attempt: Attempt,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== ORDER HANDLER CALLED ===");
    println!("Attempt: {}", attempt.current());
    
    ctx.process_order(job).await?;
    
    Ok(())
}
