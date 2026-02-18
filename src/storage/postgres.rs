use apalis_postgres::PostgresStorage;
use sqlx::PgPool;

use crate::domain::jobs::{AlertJob, EmailJob, OrderJob};

/// Storage factory for creating all job queue storages
pub struct StorageFactory {
    pool: PgPool,
}

impl StorageFactory {
    /// Create a new storage factory with a PostgreSQL connection pool
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create order job storage
    pub fn create_order_storage(&self) -> PostgresStorage<OrderJob> {
        PostgresStorage::new(&self.pool)
    }

    /// Create alert job storage
    pub fn create_alert_storage(&self) -> PostgresStorage<AlertJob> {
        PostgresStorage::new(&self.pool)
    }

    /// Create email job storage
    pub fn create_email_storage(&self) -> PostgresStorage<EmailJob> {
        PostgresStorage::new(&self.pool)
    }
}
