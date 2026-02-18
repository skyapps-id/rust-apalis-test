use apalis::prelude::*;

use crate::domain::jobs::EmailJob;
use crate::usecase::EmailSender;

pub async fn email_handler_fn(
    job: EmailJob,
    ctx: Data<std::sync::Arc<dyn EmailSender>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ctx.send_email(job).await?;

    Ok(())
}
