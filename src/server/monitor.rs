use apalis::prelude::{Monitor, WorkerBuilder};
use apalis::layers::WorkerBuilderExt;
use apalis_redis::RedisStorage;
use std::sync::Arc;

use crate::domain::jobs::{OrderJob, AlertJob, EmailJob};
use crate::workflow::handlers::{EmailHandler, OrderHandler};
use crate::server::worker_config::WorkerRetryConfig;

/// Job registry - holds all storage and handler configurations
pub struct JobRegistry {
    // Storage instances
    pub order_storage: Option<RedisStorage<OrderJob>>,
    pub alert_storage: Option<RedisStorage<AlertJob>>,
    pub email_storage: Option<RedisStorage<EmailJob>>,
    
    // Handlers
    pub order_handler: Arc<OrderHandler>,
    pub email_handler: Arc<EmailHandler>,
    
    // Retry configurations
    pub order_retry_config: WorkerRetryConfig,
    pub email_retry_config: WorkerRetryConfig,
}

impl Default for JobRegistry {
    fn default() -> Self {
        Self {
            order_storage: None,
            alert_storage: None,
            email_storage: None,
            order_handler: Arc::new(OrderHandler),
            email_handler: Arc::new(EmailHandler),
            order_retry_config: WorkerRetryConfig::default(),
            email_retry_config: WorkerRetryConfig::default(),
        }
    }
}

impl JobRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set order storage
    pub fn with_order_storage(mut self, storage: RedisStorage<OrderJob>) -> Self {
        self.order_storage = Some(storage);
        self
    }

    /// Set alert storage
    pub fn with_alert_storage(mut self, storage: RedisStorage<AlertJob>) -> Self {
        self.alert_storage = Some(storage);
        self
    }

    /// Set email storage
    pub fn with_email_storage(mut self, storage: RedisStorage<EmailJob>) -> Self {
        self.email_storage = Some(storage);
        self
    }

    /// Set order retry config
    pub fn with_order_retry_config(mut self, config: WorkerRetryConfig) -> Self {
        self.order_retry_config = config;
        self
    }

    /// Set email retry config
    pub fn with_email_retry_config(mut self, config: WorkerRetryConfig) -> Self {
        self.email_retry_config = config;
        self
    }

    /// Build and run the monitor with all registered workers
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut monitor = Monitor::new();

        // Register order worker if storage is configured
        if let Some(storage) = self.order_storage {
            let handler = self.order_handler;
            let config = self.order_retry_config;

            use crate::server::workers::order::{order_handler_fn};
            use apalis::layers::retry::RetryPolicy;
            use tower::retry::backoff::{ExponentialBackoffMaker, MakeBackoff};
            use tower::util::rng::HasherRng;

            monitor = monitor.register(move |count| {
                let backoff = ExponentialBackoffMaker::new(
                    config.min_delay,
                    config.max_delay,
                    config.jitter,
                    HasherRng::new(),
                )
                .unwrap()
                .make_backoff();

                WorkerBuilder::new(format!("order-worker-{}-{}", count, std::process::id()))
                    .backend(storage.clone())
                    .retry(RetryPolicy::retries(config.max_retries).with_backoff(backoff))
                    .data(handler.clone())
                    .build(order_handler_fn)
            });
        }

        // Register email worker if storage is configured
        if let Some(storage) = self.email_storage {
            let handler = self.email_handler;
            let config = self.email_retry_config;

            use crate::server::workers::email::{email_handler_fn};
            use apalis::layers::retry::RetryPolicy;
            use tower::retry::backoff::{ExponentialBackoffMaker, MakeBackoff};
            use tower::util::rng::HasherRng;

            monitor = monitor.register(move |count| {
                let backoff = ExponentialBackoffMaker::new(
                    config.min_delay,
                    config.max_delay,
                    config.jitter,
                    HasherRng::new(),
                )
                .unwrap()
                .make_backoff();

                WorkerBuilder::new(format!("email-worker-{}-{}", count, std::process::id()))
                    .backend(storage.clone())
                    .retry(RetryPolicy::retries(config.max_retries).with_backoff(backoff))
                    .data(handler.clone())
                    .build(email_handler_fn)
            });
        }

        // Future: Add other workers here
        // if let Some(storage) = self.alert_storage { ... }

        monitor.run().await?;
        Ok(())
    }
}
