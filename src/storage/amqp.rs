use apalis_amqp::AmqpBackend;
use apalis_codec::json::JsonCodec;
use deadpool_lapin::{Manager, Pool};
use lapin::{ConnectionProperties, types::LongString};
use std::sync::Arc;

use crate::domain::jobs::{AlertJob, EmailJob, OrderJob};

type AmqpStorage<T> = AmqpBackend<T, JsonCodec<Vec<u8>>>;

/// Storage factory for creating all job queue storages
pub struct StorageFactory {
    order_storage: Arc<AmqpStorage<OrderJob>>,
    email_storage: Arc<AmqpStorage<EmailJob>>,
    alert_storage: Arc<AmqpStorage<AlertJob>>,
}

impl StorageFactory {
    /// Create a new storage factory with AMQP backends
    pub async fn new(amqp_addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_connection_name(amqp_addr, "rust-apalis-app").await
    }

    /// Create a new storage factory with custom connection name
    pub async fn with_connection_name(
        amqp_addr: &str,
        connection_name: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let conn_props = ConnectionProperties::default()
            .with_connection_name(LongString::from(connection_name));

        let manager = Manager::new(amqp_addr, conn_props);
        let pool = Pool::builder(manager).max_size(10).build()?;

        let order_storage = Arc::new(create_backend(pool.clone(), "order-jobs").await?);
        let email_storage = Arc::new(create_backend(pool.clone(), "email-jobs").await?);
        let alert_storage = Arc::new(create_backend(pool.clone(), "alert-jobs").await?);

        Ok(Self {
            order_storage,
            email_storage,
            alert_storage,
        })
    }

    /// Create order job storage
    pub fn create_order_storage(&self) -> AmqpStorage<OrderJob> {
        (*self.order_storage).clone()
    }

    /// Create alert job storage
    pub fn create_alert_storage(&self) -> AmqpStorage<AlertJob> {
        (*self.alert_storage).clone()
    }

    /// Create email job storage
    pub fn create_email_storage(&self) -> AmqpStorage<EmailJob> {
        (*self.email_storage).clone()
    }
}

async fn create_backend<T>(
    pool: Pool,
    queue_name: &str,
) -> Result<AmqpBackend<T, JsonCodec<Vec<u8>>>, Box<dyn std::error::Error>>
where
    T: Send + 'static,
{
    let conn = pool.get().await?;
    let channel = conn.create_channel().await?;
    let queue = channel.queue_declare(queue_name, Default::default(), Default::default()).await?;
    Ok(AmqpBackend::new(channel, queue))
}
