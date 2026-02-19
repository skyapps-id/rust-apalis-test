use rust_apalis_test::server::worker::register::{run_jobs_with_config, WorkerConfig};
use rust_apalis_test::storage::postgres::StorageFactory;
use rust_apalis_test::AppContainer;
use std::sync::Arc;
use apalis_board::axum::{
    sse::{TracingBroadcaster, TracingSubscriber},
};
use apalis_board_api::framework::{ApiBuilder, RegisterRoute};
use apalis_postgres::PostgresStorage;
use axum::{Extension, Router, response::Html};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::fs::ServeDir;
use tracing_subscriber::{EnvFilter, Layer as TraceLayer, layer::SubscriberExt, util::SubscriberInitExt};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Apalis Board + Worker...");
    println!("Press Ctrl+C to shutdown gracefully...");
    println!();

    let database_url = "postgres://root:root@localhost:5432/apalis-database";
    let pool = sqlx::PgPool::connect(database_url).await?;

    PostgresStorage::setup(&pool).await?;

    // Setup SHARED tracing broadcaster
    let broadcaster = TracingBroadcaster::create();

    let line_sub = TracingSubscriber::new(&broadcaster);
    let tracer = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(EnvFilter::builder().parse("debug").unwrap()),
        )
        .with(
            line_sub
                .layer()
                .with_filter(EnvFilter::builder().parse("apalis=debug,info").unwrap()),
        );
    tracer.try_init()?;

    let storage_factory = Arc::new(StorageFactory::new(pool.clone()));

    // Setup board API
    let api_routes = ApiBuilder::new(Router::new())
        .register(storage_factory.create_email_storage())
        .register(storage_factory.create_order_storage())
        .register(storage_factory.create_alert_storage())
        .build();

    async fn serve_spa() -> Html<String> {
        let index_html = fs::read_to_string("static/index.html")
            .unwrap_or_else(|_| "<html><body>UI not found</body></html>".to_string());
        Html(index_html)
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", axum::routing::get(serve_spa))
        .route("/queues", axum::routing::get(serve_spa))
        .route("/workers", axum::routing::get(serve_spa))
        .route("/tasks", axum::routing::get(serve_spa))
        .route("/logs", axum::routing::get(serve_spa))
        .nest("/api/v1", api_routes)
        .fallback_service(ServeDir::new("static"))
        .layer(cors)
        .layer(Extension(broadcaster.clone()));

    // Setup worker
    let container = AppContainer::new(storage_factory.clone());
    let worker_config = WorkerConfig {
        order_concurrency: 3,
        email_concurrency: 2,
    };

    // Run worker & board together
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await?;

    println!("🚀 Apalis Board + Worker starting...");
    println!("Board UI: http://localhost:9000");
    println!("API: http://localhost:9000/api/v1");
    println!("SSE Events: http://localhost:9000/api/v1/events");
    println!();

    let storage_factory_ref = storage_factory.clone();
    tokio::select! {
        result = axum::serve(listener, app) => {
            result?;
        }
        result = run_jobs_with_config(&storage_factory_ref, container, worker_config) => {
            result?;
        }
        _ = tokio::signal::ctrl_c() => {
            println!("\nShutting down...");
        }
    }

    Ok(())
}
