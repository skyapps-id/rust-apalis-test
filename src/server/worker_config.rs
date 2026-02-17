use std::time::Duration;

/// Configuration for worker retry policy
#[derive(Clone)]
pub struct WorkerRetryConfig {
    pub max_retries: usize,
    pub min_delay: Duration,
    pub max_delay: Duration,
    pub jitter: f64,
}

impl Default for WorkerRetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            min_delay: Duration::from_secs(2),
            max_delay: Duration::from_secs(10),
            jitter: 0.5,
        }
    }
}

impl WorkerRetryConfig {
    pub fn new(max_retries: usize, min_delay: Duration, max_delay: Duration, jitter: f64) -> Self {
        Self {
            max_retries,
            min_delay,
            max_delay,
            jitter,
        }
    }
}
