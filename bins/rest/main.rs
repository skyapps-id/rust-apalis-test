use rust_apalis_test::server::rest::{run_server, ServerState};
use rust_apalis_test::storage::postgres::StorageFactory;
use rust_apalis_test::AppContainer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting REST API Server...");
    println!("Press Ctrl+C to shutdown gracefully...");
    println!();

    let database_url = "postgres://root:root@localhost:5432/apalis-database";
    let pool = sqlx::PgPool::connect(database_url).await?;

    // Setup PostgreSQL tables for apalis
    apalis_postgres::PostgresStorage::setup(&pool).await?;

    let storage_factory = Arc::new(StorageFactory::new(pool));
    let container = AppContainer::new(storage_factory);
    let state = ServerState::new(container);

    // Setup signal handler for graceful shutdown
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                println!("\nReceived shutdown signal, stopping server...");
                let _ = shutdown_tx.send(());
            }
            Err(err) => {
                eprintln!("Error listening for shutdown signal: {}", err);
            }
        }
    });

    // Run server until shutdown signal received
    tokio::select! {
        result = run_server("0.0.0.0:3000".parse()?, state) => {
            result?;
        }
        _ = &mut shutdown_rx => {
            println!("Server shutting down gracefully...");
        }
    }

    println!("Server stopped. Goodbye!");
    Ok(())
}
