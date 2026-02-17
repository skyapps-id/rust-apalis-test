use apalis_redis::RedisStorage;
use redis::aio::ConnectionManager;

use crate::domain::jobs::{AlertJob, EmailJob, OrderJob};

/// Storage factory for creating all job queue storages
pub struct StorageFactory {
    conn: ConnectionManager,
}

impl StorageFactory {
    /// Create a new storage factory with a Redis connection
    pub fn new(conn: ConnectionManager) -> Self {
        Self { conn }
    }

    /// Create order job storage
    pub fn create_order_storage(&self) -> RedisStorage<OrderJob> {
        RedisStorage::new(self.conn.clone())
    }

    /// Create alert job storage
    pub fn create_alert_storage(&self) -> RedisStorage<AlertJob> {
        RedisStorage::new(self.conn.clone())
    }

    /// Create email job storage
    pub fn create_email_storage(&self) -> RedisStorage<EmailJob> {
        RedisStorage::new(self.conn.clone())
    }
}
