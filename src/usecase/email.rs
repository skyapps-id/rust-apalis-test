use async_trait::async_trait;
use crate::domain::jobs::EmailJob;

#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send_email(&self, job: EmailJob) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

pub struct EmailService;

#[async_trait]
impl EmailSender for EmailService {
    async fn send_email(&self, job: EmailJob) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("=== SENDING EMAIL ===");
        println!("To: {}", job.to);
        println!("Subject: {}", job.subject);
        println!("Body: {}", job.body);
        
        Ok(())
    }
}
