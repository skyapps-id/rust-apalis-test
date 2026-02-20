use apalis::prelude::*;
use apalis_core::task::attempt::Attempt;

use crate::domain::jobs::OrderJob;
use crate::usecase::OrderUsecase;

pub async fn order_handler_fn(
    job: OrderJob,
    ctx: Data<std::sync::Arc<dyn OrderUsecase>>,
    attempt: Attempt,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let current_attempt = attempt.current();
    println!("[order_handler_fn] Handler attempt: {}", current_attempt);

    if current_attempt < 3 {
        println!("❌ [ORDER] Simulating failure on attempt {}", current_attempt);
        return Err("Simulated error for testing retry".into());
    }

    println!("✅ [ORDER] Processing on attempt {}", current_attempt);
    ctx.process_order(job).await?;

    Ok(())
}
