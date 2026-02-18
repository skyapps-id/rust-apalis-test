use apalis::prelude::*;
use crate::domain::jobs::OrderJob;
use crate::usecase::OrderUsecase;

pub async fn order_handler_fn(
    job: OrderJob,
    ctx: Data<std::sync::Arc<dyn OrderUsecase>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ctx.process_order(job).await?;

    Ok(())
}
