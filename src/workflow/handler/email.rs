use apalis::prelude::*;
use apalis_core::task::attempt::Attempt;

use crate::domain::jobs::EmailJob;
use crate::workflow::usecase::EmailService;

pub async fn email_handler_fn(
    job: EmailJob,
    ctx: Data<std::sync::Arc<EmailService>>,
    attempt: Attempt,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== EMAIL HANDLER CALLED ===");
    println!("Attempt: {}", attempt.current());
    
    ctx.send_email(job).await?;
    
    Ok(())
}
