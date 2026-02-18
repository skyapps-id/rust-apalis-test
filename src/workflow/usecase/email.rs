use crate::domain::jobs::EmailJob;

pub struct EmailService;

impl EmailService {
    pub async fn send_email(&self, job: EmailJob) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("=== SENDING EMAIL ===");
        println!("To: {}", job.to);
        println!("Subject: {}", job.subject);
        println!("Body: {}", job.body);
        
        Ok(())
    }
}
