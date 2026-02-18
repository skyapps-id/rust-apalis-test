use axum::{routing::get, routing::post, Router};
use std::net::SocketAddr;

use crate::handler::rest::{create_order, health_check};
use super::ServerState;

pub fn create_router(state: ServerState) -> Router {
    Router::new()
        .route("/orders", post(create_order))
        .route("/health", get(health_check))
        .with_state(state)
}

pub async fn run_server(addr: SocketAddr, state: ServerState) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("REST API running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
