use apalis_board_api::framework::{ApiBuilder, RegisterRoute};
use apalis_postgres::PostgresStorage;
use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Apalis Board UI...");
    println!("Press Ctrl+C to shutdown gracefully...");
    println!();

    let database_url = "postgres://root:root@localhost:5432/apalis-database";
    let pool = sqlx::PgPool::connect(database_url).await?;

    PostgresStorage::setup(&pool).await?;

    let storage: PostgresStorage<serde_json::Value> = PostgresStorage::new(&pool);

    let api_routes = ApiBuilder::new(Router::new())
        .register(storage)
        .build();

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await?;
    
    println!("🚀 Apalis Board starting...");
    println!("Board UI: http://localhost:9000");
    println!("API: http://localhost:9000/api");
    println!();

    axum::serve(listener, app).await?;

    Ok(())
}
