use apalis::prelude::*;
use apalis_core::task::attempt::Attempt;

use crate::domain::jobs::OrderJob;
use crate::usecase::OrderUsecase;

pub async fn order_handler_fn(
    job: OrderJob,
    ctx: Data<std::sync::Arc<dyn OrderUsecase>>,
    attempt: Attempt,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[order_handler_fn] Attempt: {}", attempt.current());

    ctx.process_order(job).await?;

    Ok(())
}
