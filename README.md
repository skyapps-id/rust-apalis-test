# Rust Apalis - Clean Architecture Job Processing

A production-ready example project using **Apalis 1.0.0-rc.4** for background job processing with REST API and worker architecture.

## Features

- ✅ **Trait-based Usecase Pattern** - Clean separation of business logic
- ✅ **REST API** - Create/schedule jobs via HTTP endpoints (Axum)
- ✅ **Worker Processing** - Configurable concurrency, async job consumers
- ✅ **Dependency Injection** - Centralized AppContainer
- ✅ **PostgreSQL Storage** - Persistent storage with apalis-postgres
- ✅ **Graceful Shutdown** - Ctrl+C handler for clean exit
- ✅ **Unique Worker ID** - Timestamp-based worker identification
- ✅ **Type Safety** - Trait objects for flexibility
- ✅ **Apalis Board** - Web UI for monitoring jobs (optional)

## Prerequisites

- Rust 2024 edition
- PostgreSQL server (required)

```bash
# Install PostgreSQL
brew install postgresql@16  # macOS
sudo apt install postgresql  # Ubuntu

# Start PostgreSQL
brew services start postgresql@16  # macOS
sudo systemctl start postgresql  # Linux
```

See [DATABASE_SETUP.md](DATABASE_SETUP.md) for detailed database setup.

## Quick Start

### 1. Setup Database

```bash
# Create database
psql -U postgres -c "CREATE DATABASE apalis_database OWNER root;"

# Run migrations
cargo run --bin setup_migration
```

### 2. Start Worker (Terminal 1)

```bash
cargo run --bin worker
```

Output:
```
Starting Apalis Job Worker...
Press Ctrl+C to shutdown gracefully...

Worker ID: 1771412612
Worker Concurrency:
  - Order: 3 instances
  - Email: 2 instances

Registering order worker...
Registering email worker...

Starting monitor...
All workers registered successfully!

  → Starting order worker instance 1/3
  → Starting email worker instance 1/2
```

### 3. Start REST API (Terminal 2)

```bash
cargo run --bin rest
```

Output:
```
Starting REST API Server...
Press Ctrl+C to shutdown gracefully...

REST API running on http://0.0.0.0:3000
POST /orders - Create order email job
GET  /health - Health check
```

**Graceful Shutdown:** Both worker and REST API support Ctrl+C for clean shutdown.

### 3. Send Job via REST API

```bash
curl -X POST http://localhost:3000/orders \
  -H "Content-Type: application/json" \
  -d '{
    "event_id": "EVT-001",
    "device_uuid": "DEV-123"
  }'
```

Response:
```json
{
  "message": "Order task scheduled successfully",
  "event_id": "EVT-001"
}
```

### 4. Worker Processes Job

Worker terminal output:
```
=== ORDER HANDLER CALLED ===
Attempt: 1
[ORDER] Event: EVT-001 | Device: DEV-123 | Time: 14:23:45.123
Order processed successfully!
```

## API Endpoints

### Create Order Job

```http
POST /orders
Content-Type: application/json

{
  "event_id": "EVT-001",
  "device_uuid": "DEV-123"
}
```

**Response:** `201 Created`

### Health Check

```http
GET /health
```

**Response:** `200 OK` with `OK`

## Project Structure

```
rust-apalis-test/
├── src/
│   ├── domain/           # Job types & domain entities
│   │   ├── jobs.rs       # OrderJob, EmailJob, AlertJob
│   │   ├── enums.rs      # Domain enums
│   │   └── mod.rs
│   ├── usecase/          # Business logic (trait-based)
│   │   ├── order.rs      # OrderUsecase trait + OrderService
│   │   ├── email.rs      # EmailSender trait + EmailService
│   │   └── mod.rs
│   ├── handler/          # Request/Job handlers
│   │   ├── rest/         # HTTP handlers (Axum)
│   │   │   ├── order.rs  # create_order endpoint
│   │   │   ├── health.rs # health_check endpoint
│   │   │   └── mod.rs
│   │   ├── workflow/     # Job handlers (Apalis)
│   │   │   ├── order.rs  # order_handler_fn
│   │   │   ├── email.rs  # email_handler_fn
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── server/           # Server setup & worker registration
│   │   ├── rest/         # REST API router & server
│   │   │   ├── router.rs # create_router, run_server
│   │   │   └── mod.rs    # ServerState
│   │   ├── worker/       # Worker registration
│   │   │   ├── register.rs # run_jobs, monitor setup
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── storage/          # Storage abstraction
│   │   ├── postgres.rs   # StorageFactory for PostgreSQL
│   │   └── mod.rs
│   ├── container.rs      # AppContainer (DI container)
│   └── lib.rs            # Public API exports
├── bins/                 # Binary executables
│   ├── rest/main.rs      # REST API server binary
│   ├── worker/main.rs    # Worker binary
│   ├── board/main.rs     # Apalis Board UI binary
│   └── setup_migration/  # Database migration binary
├── docs/
│   └── APALIS_BOARD.md   # Apalis Board setup guide
├── Cargo.toml
├── DATABASE_SETUP.md     # Database setup guide
├── ARCHITECTURE.md       # Detailed architecture guide
└── README.md
```

## Architecture

This project follows **Clean Architecture** principles with clear separation of concerns:

- **Domain Layer** - Pure business entities (no dependencies)
- **Usecase Layer** - Business logic traits & implementations with private helper methods
- **Handler Layer** - HTTP request handlers & job handlers
- **Server Layer** - Worker registration & REST server setup
- **Storage Layer** - PostgreSQL storage abstraction
- **Container** - Dependency injection

### Private Helper Methods Pattern

Services in the usecase layer use private helper methods for task scheduling:

- `OrderService::schedule_order_task()` - Schedules order jobs to PostgreSQL queue
- `OrderService::send_order_email()` - Schedules email notifications

This pattern keeps trait methods clean and delegates task creation/storage logic to private methods.

## Architecture Diagrams

### System Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           RUST APALIS ARCHITECTURE                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐   │
│  │   REST API  │───▶│   HANDLERS  │───▶│   USECASE   │───▶│   DOMAIN    │   │
│  │   (Axum)    │    │  (rest/)    │    │  (traits)   │    │  (jobs)     │   │
│  └─────────────┘    └─────────────┘    └──────┬──────┘    └─────────────┘   │
│                                                  │                          │
│                                                  ▼                          │
│                                          ┌─────────────┐                    │
│                                          │   STORAGE   │                    │
│                                          │   (Redis)   │                    │
│                                          └──────┬──────┘                    │
│                                                 │                           │
│                                                 ▼                           │
│                                          ┌─────────────┐                    │
│                                          │   WORKER    │                    │
│                                          │  (Apalis)   │                    │
│                                          └─────────────┘                    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Project Structure

```
rust-apalis-test/
│
├── bins/
│   ├── rest/main.rs          ──▶  REST API Server Entry Point
│   └── worker/main.rs        ──▶  Worker Entry Point
│
├── src/
│   ├── domain/               ──▶  Job Types (OrderJob, EmailJob)
│   ├── usecase/              ──▶  Business Logic (Traits + Impl)
│   │   └── order.rs          ──▶  OrderUsecase + OrderService
│   │       ├── create_order()
│   │       ├── process_order()
│   │       ├── schedule_order_task()  ← private helper
│   │       └── send_order_email()     ← private helper
│   ├── handler/
│   │   ├── rest/             ──▶  HTTP Endpoints
│   │   └── workflow/         ──▶  Job Handlers
│   ├── server/
│   │   ├── rest/             ──▶  Router & Server
│   │   └── worker/           ──▶  Worker Registration
│   ├── storage/
│   │   └── redis.rs          ──▶  StorageFactory (Shared Instances)
│   └── container.rs          ──▶  Dependency Injection
│
└── Cargo.toml
```

For detailed architecture information, see [ARCHITECTURE.md](ARCHITECTURE.md).

## Apalis Board (Web UI)

Monitor and manage jobs via web interface:

```bash
cargo run --bin board
```

Access at: http://localhost:9000

See [docs/APALIS_BOARD.md](docs/APALIS_BOARD.md) for details.

## Troubleshooting

### Worker not consuming tasks

**Problem:** Tasks are pushed to PostgreSQL but worker doesn't process them.

**Solution:** Check task status in database:
```bash
psql -h localhost -U root -d apalis-database -c "SELECT id, status, queue FROM tasks ORDER BY created_at DESC LIMIT 5;"
```

If status is `Failed`, check handler signature matches trait object type:
```rust
// ✅ CORRECT - Trait object
ctx: Data<std::sync::Arc<dyn OrderUsecase>>

// ❌ WRONG - Concrete type
ctx: Data<std::sync::Arc<OrderService>>
```

### Storage mismatch error

**Problem:** Producer and consumer use different storage instances.

**Solution:** Ensure `StorageFactory` uses the same PgPool:
```rust
// In StorageFactory::new()
pub fn new(pool: PgPool) -> Self {
    Self { pool }
}

// All storage methods use the same pool
pub fn create_order_storage(&self) -> PostgresStorage<OrderJob> {
    PostgresStorage::new(self.pool.clone())
}
```

### Worker restart error: "worker is still active"

**Problem:** Database still has worker metadata from previous run.

**Solution:** Clean up worker metadata:
```bash
psql -h localhost -U root -d apalis-database -c "DELETE FROM workers WHERE last_seen < NOW() - INTERVAL '5 minutes';"
```

**Note:** This project uses unique worker IDs (timestamp-based), so restart usually works without cleanup.

## Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| `apalis` | 1.0.0-rc.4 | Job processing framework |
| `apalis-postgres` | 1.0.0-rc.3 | PostgreSQL storage backend |
| `apalis-board` | 1.0.0-rc.3 | Web UI for monitoring |
| `apalis-board-api` | 1.0.0-rc.3 | API for Apalis Board |
| `axum` | 0.8 | HTTP server framework |
| `sqlx` | 0.8 | Database client |
| `tokio` | 1 | Async runtime |
| `serde` | 1 | Serialization |
| `async-trait` | 0.1 | Async trait support |

## Design Benefits

1. **Trait-based Architecture** - Flexible business logic via traits
2. **Dependency Injection** - Centralized AppContainer
3. **Persistent Storage** - PostgreSQL with connection pooling
4. **Type Safety** - Trait objects ensure compile-time checks
5. **Testability** - Each layer can be tested independently
6. **Scalability** - Easy to add new job types
7. **Graceful Shutdown** - Clean exit on Ctrl+C
8. **Worker Concurrency** - Configurable parallel processing
9. **Web Monitoring** - Apalis Board for job visualization

## Contributing

When adding new job types, follow the guide in [ARCHITECTURE.md](ARCHITECTURE.md#how-to-add-a-new-job-type).

## License

MIT
