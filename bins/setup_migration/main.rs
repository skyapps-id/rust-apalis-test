#!/usr/bin/env cargo
//! Contoh script untuk menjalankan migration apalis-postgres
//!
//! Jalankan dengan: cargo run --bin setup_migration

use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Setting up Apalis PostgreSQL migrations...\n");

    // Connect ke database
    let database_url = "postgres://root:root@localhost:5432/apalis-database";
    println!("📡 Connecting to: {}", database_url);
    
    let pool = PgPool::connect(database_url).await?;
    println!("✅ Connected!\n");

    // Jalankan migration
    println!("🚀 Running migrations...");
    apalis_postgres::PostgresStorage::setup(&pool).await?;
    println!("✅ Migrations completed successfully!\n");

    // Verifikasi tabel
    println!("📊 Checking tables...");
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM information_schema.tables WHERE table_name = 'tasks'")
        .fetch_one(&pool)
        .await?;
    
    if row.0 > 0 {
        println!("✅ Table 'tasks' exists!\n");
        
        // Tampilkan struktur tabel
        println!("📋 Table structure:");
        let columns = sqlx::query_as::<_, (String, String, String)>(
            "SELECT column_name, data_type, is_nullable 
             FROM information_schema.columns 
             WHERE table_name = 'tasks' 
             ORDER BY ordinal_position"
        )
        .fetch_all(&pool)
        .await?;
        
        for (col, dtype, nullable) in columns {
            println!("  - {}: {} (nullable: {})", col, dtype, nullable);
        }
    } else {
        println!("❌ Table 'tasks' not found!");
    }

    println!("\n✨ Setup complete!");
    Ok(())
}
