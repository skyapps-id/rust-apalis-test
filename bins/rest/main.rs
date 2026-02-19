use rust_apalis_test::server::rest::{run_server, ServerState};
use rust_apalis_test::storage::amqp::StorageFactory;
use rust_apalis_test::AppContainer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting REST API Server...");
    println!("Press Ctrl+C to shutdown gracefully...");
    println!();

    let amqp_addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://admin:password@127.0.0.1:5672".to_string());
    let conn_name = std::env::var("AMQP_CONN_NAME").unwrap_or_else(|_| "rust-apalis-rest".to_string());
    let storage_factory = Arc::new(StorageFactory::with_connection_name(&amqp_addr, &conn_name).await?);
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
