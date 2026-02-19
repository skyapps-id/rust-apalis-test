use rust_apalis_test::server::worker::register::{run_jobs_with_config, WorkerConfig};
use rust_apalis_test::storage::amqp::StorageFactory;
use rust_apalis_test::AppContainer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Apalis Job Worker...");
    println!("Press Ctrl+C to shutdown gracefully...");
    println!();

    let amqp_addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://admin:password@127.0.0.1:5672".to_string());
    let conn_name = std::env::var("AMQP_CONN_NAME").unwrap_or_else(|_| "rust-apalis-worker".to_string());
    let storage_factory = Arc::new(StorageFactory::with_connection_name(&amqp_addr, &conn_name).await?);
    let container = AppContainer::new(storage_factory.clone());

    // Configure worker concurrency
    let worker_config = WorkerConfig {
        order_concurrency: 3,  // 3 concurrent order workers
        email_concurrency: 2,  // 2 concurrent email workers
    };

    // Setup signal handler for graceful shutdown
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                println!("\nReceived shutdown signal, stopping worker...");
                let _ = shutdown_tx.send(());
            }
            Err(err) => {
                eprintln!("Error listening for shutdown signal: {}", err);
            }
        }
    });

    // Run worker until shutdown signal received
    tokio::select! {
        result = run_jobs_with_config(&storage_factory, container, worker_config) => {
            result?;
        }
        _ = &mut shutdown_rx => {
            println!("Worker shutting down gracefully...");
            println!("Cleaning up AMQP worker metadata...");
        }
    }

    println!("Worker stopped. Goodbye!");
    Ok(())
}
