/// Generic trait for job handlers.
///
/// All job handlers should implement this trait for consistency
/// and to make it easy to add new job types.
pub trait JobHandler: Clone + Send + Sync + 'static {
    type Job: Send + Sync;

    /// Process the job with the given attempt information
    fn handle(
        &self,
        job: Self::Job,
        attempt: usize,
        max_retries: usize,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send;
}
