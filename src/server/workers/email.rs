use apalis::prelude::*;
use apalis_core::task::attempt::Attempt;
use std::sync::Arc;

use crate::domain::jobs::EmailJob;
use crate::workflow::{JobHandler, handlers::EmailHandler};

pub async fn email_handler_fn(
    job: EmailJob,
    handler: Data<Arc<EmailHandler>>,
    attempt: Attempt,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let recipient = job.to.clone();
    let now = chrono::Local::now();
    println!(
        "[Attempt {}/3] Sending email to: {} at {}",
        attempt.current(),
        recipient,
        now.format("%H:%M:%S%.3f")
    );

    handler.handle(job, attempt.current(), 3).await
}
