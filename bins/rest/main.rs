use redis::{Client, aio::ConnectionManager};
use rust_apalis_test::server::rest::{run_server, ServerState};
use rust_apalis_test::storage::redis::StorageFactory;
use rust_apalis_test::AppContainer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_client = Client::open("redis://127.0.0.1:6379")?;
    let conn = ConnectionManager::new(redis_client).await?;

    let storage_factory = Arc::new(StorageFactory::new(conn));
    let container = AppContainer::new(storage_factory);
    let state = ServerState::new(container);

    run_server("0.0.0.0:3000".parse()?, state).await?;

    Ok(())
}
