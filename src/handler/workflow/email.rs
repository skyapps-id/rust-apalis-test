use apalis::prelude::*;
use apalis_core::task::attempt::Attempt;

use crate::domain::jobs::EmailJob;
use crate::usecase::EmailSender;

pub async fn email_handler_fn(
    job: EmailJob,
    ctx: Data<std::sync::Arc<dyn EmailSender>>,
    attempt: Attempt,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[email_handler_fn] Attempt: {}", attempt.current());

    ctx.send_email(job).await?;

    Ok(())
}
