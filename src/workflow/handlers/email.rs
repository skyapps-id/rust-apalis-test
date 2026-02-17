use crate::domain::jobs::EmailJob;
use super::super::handler::JobHandler;

#[derive(Clone)]
pub struct EmailHandler;

impl EmailHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for EmailHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl JobHandler for EmailHandler {
    type Job = EmailJob;

    async fn handle(
        &self,
        job: Self::Job,
        attempt: usize,
        max_retries: usize,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Sending email to: {}", job.to);
        println!("Subject: {}", job.subject);
        println!("Attempt: {}/{}", attempt + 1, max_retries);

        if attempt < max_retries {
            let error = format!("Failed to send email to {} (attempt {})", job.to, attempt + 1);
            return Err(error.into());
        }

        println!("Email sent successfully to {}", job.to);
        Ok(())
    }
}
