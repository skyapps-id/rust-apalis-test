# Setup Apalis Board

Apalis Board adalah web UI untuk monitoring dan managing jobs. Berikut cara setup-nya:

## 1. Tambah Dependency

Sudah ditambahkan di `Cargo.toml`:
```toml
apalis-board = { version = "1.0.0-rc.3", features = ["axum"] }
```

## 2. Integrasi dengan Axum

Ada dua cara mengintegrasikan apalis-board:

### Cara 1: API Only (Recommended untuk sekarang)

Setup API endpoints untuk board, tanpa UI:

```rust
// src/server/rest/router.rs
use apalis_board::axum::framework::{ApiBuilder, RegisterRoute};

pub fn create_router(state: ServerState) -> Router {
    // Setup Apalis Board API
    let board_api = ApiBuilder::new(axum::routing::Router::new()) // Base router kosong
        .mount("/api/v1") // Mount di path ini
        .register(state.container.storage.create_order_storage())
        .register(state.container.storage.create_email_storage())
        .build();

    Router::new()
        .route("/orders", post(create_order))
        .route("/health", get(health_check))
        .merge(board_api)
        .with_state(state)
}
```

### Cara 2: Separate Port

Jalankan apalis-board di port terpisah:

```rust
// bins/board/main.rs (baru)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::PgPool::connect("postgres://root:root@localhost:5432/apalis-postgres").await?;
    let storage_factory = Arc::new(StorageFactory::new(pool));

    use apalis_board::axum::framework::{ApiBuilder, RegisterRoute};

    let board_api = ApiBuilder::new(axum::routing::Router::new())
        .mount("/")
        .register(storage_factory.create_order_storage())
        .register(storage_factory.create_email_storage())
        .build();

    let app = Router::new().merge(board_api);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001".parse()?).await?;
    println!("Apalis Board running on http://0.0.0.0:3001");

    axum::serve(listener, app).await?;
    Ok(())
}
```

## 3. Menggunakan Apalis Board

### Via API

```bash
# List all queues
curl http://localhost:3000/api/v1/queues

# List tasks in a queue
curl http://localhost:3000/api/v1/queues/order-jobs/tasks

# Get task details
curl http://localhost:3000/api/v1/queues/order-jobs/tasks/{task_id}

# Kill a task
curl -X DELETE http://localhost:3000/api/v1/queues/order-jobs/tasks/{task_id}

# Get queue stats
curl http://localhost:3000/api/v1/queues/order-jobs/stats
```

### Via Frontend (UI)

Frontend apalis-board perlu di-build dan serve secara terpisah:

```bash
# Clone apalis repo
git clone https://github.com/apalis-dev/apalis.git
cd apalis/packages/web

# Install dependencies dan build
npm install
npm run build

# Serve static files dari dist/ folder
```

## 4. API Endpoints

Setelah setup, endpoints yang tersedia:

```
GET  /api/v1/queues                    - List all queues
GET  /api/v1/queues/:name              - Get queue details
GET  /api/v1/queues/:name/tasks        - List tasks in queue
GET  /api/v1/queues/:name/tasks/:id    - Get task by ID
POST /api/v1/queues/:name/tasks/:id    - Update task
DEL  /api/v1/queues/:name/tasks/:id    - Kill/Delete task
GET  /api/v1/queues/:name/stats        - Queue statistics
GET  /api/v1/workers                   - List all workers
GET  /api/v1/jobs                      - List all jobs across queues
```

## 5. Example Queries

### Monitoring Jobs

```bash
# Lihat semua pending tasks
curl http://localhost:3000/api/v1/queues/order-jobs/tasks?status=pending

# Lihat failed tasks
curl http://localhost:3000/api/v1/queues/order-jobs/tasks?status=failed

# Lihat queue stats
curl http://localhost:3000/api/v1/queues/email-jobs/stats | jq
```

### Managing Jobs

```bash
# Retry failed task
curl -X POST http://localhost:3000/api/v1/queues/order-jobs/tasks/{task_id}/retry

# Kill stuck task
curl -X DELETE http://localhost:3000/api/v1/queues/order-jobs/tasks/{task_id}
```

## 6. Integrasi Frontend

Untuk full UI experience, ada beberapa opsi:

### Opsi 1: Serve Static Files

```rust
// Tambah ke router
use tower_http::services::ServeDir;

Router::new()
    .merge(board_api)
    .nest_service("/board", ServeDir::new("static/apalis-board"))
```

### Opsi 2: External Frontend

Jalankan frontend dev server terpisah dan proxy API:

```bash
# Frontend dev server
cd apalis/packages/web
npm run dev

# Atau build dan serve
npm run build
npx serve dist
```

## Current Limitations

1. **Frontend Integration**: UI perlu di-build terpisah
2. **State Compatibility**: Router dengan berbeda state butuh penanganan khusus
3. **Features**: Tidak semua features apalis-board available untuk rc.3

## Rekomendasi untuk Sekarang

Gunakan **API only approach** untuk monitoring jobs:

```bash
# Setup sederhana
curl http://localhost:3000/api/v1/queues | jq

# Monitor jobs
watch -n 5 'curl -s http://localhost:3000/api/v1/queues/order-jobs/stats | jq'
```
