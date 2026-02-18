use apalis_redis::RedisStorage;
use redis::aio::ConnectionManager;
use std::sync::Arc;

use crate::domain::jobs::{AlertJob, EmailJob, OrderJob};

/// Storage factory for creating all job queue storages
pub struct StorageFactory {
    order_storage: Arc<RedisStorage<OrderJob>>,
    email_storage: Arc<RedisStorage<EmailJob>>,
    alert_storage: Arc<RedisStorage<AlertJob>>,
}

impl StorageFactory {
    /// Create a new storage factory with a Redis connection
    pub fn new(conn: ConnectionManager) -> Self {
        let order_storage = Arc::new(RedisStorage::new(conn.clone()));
        let email_storage = Arc::new(RedisStorage::new(conn.clone()));
        let alert_storage = Arc::new(RedisStorage::new(conn.clone()));

        Self {
            order_storage,
            email_storage,
            alert_storage,
        }
    }

    /// Create order job storage
    pub fn create_order_storage(&self) -> RedisStorage<OrderJob> {
        (*self.order_storage).clone()
    }

    /// Create alert job storage
    pub fn create_alert_storage(&self) -> RedisStorage<AlertJob> {
        (*self.alert_storage).clone()
    }

    /// Create email job storage
    pub fn create_email_storage(&self) -> RedisStorage<EmailJob> {
        (*self.email_storage).clone()
    }
}
