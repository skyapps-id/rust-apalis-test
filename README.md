# Rust Apalis - Clean Architecture Job Processing

A production-ready example project using **Apalis 1.0.0-rc.4** for background job processing with REST API and worker architecture.

## Features

- ✅ **Trait-based Usecase Pattern** - Clean separation of business logic
- ✅ **REST API** - Create/schedule jobs via HTTP endpoints (Axum)
- ✅ **Worker Processing** - Configurable concurrency, async job consumers
- ✅ **Dependency Injection** - Centralized AppContainer
- ✅ **Redis Storage** - Shared storage instances for producer & consumer
- ✅ **Graceful Shutdown** - Ctrl+C handler for clean exit
- ✅ **Unique Worker ID** - Timestamp-based worker identification
- ✅ **Type Safety** - Trait objects for flexibility

## Prerequisites

- Rust 2024 edition
- Redis server (required)

```bash
# Install Redis
brew install redis  # macOS
sudo apt install redis-server  # Ubuntu

# Start Redis
redis-server
```

## Quick Start

### 1. Start Redis

```bash
redis-server
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

### 4. Send Job via REST API

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

### 5. Worker Processes Job

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
│   │   ├── redis.rs      # StorageFactory (shared instances)
│   │   └── mod.rs
│   ├── container.rs      # AppContainer (DI container)
│   └── lib.rs            # Public API exports
├── bins/                 # Binary executables
│   ├── rest/main.rs      # REST API server binary
│   └── worker/main.rs    # Worker binary
├── Cargo.toml
├── ARCHITECTURE.md       # Detailed architecture guide
└── README.md
```

## Architecture

This project follows **Clean Architecture** principles with clear separation of concerns:

- **Domain Layer** - Pure business entities (no dependencies)
- **Usecase Layer** - Business logic traits & implementations with private helper methods
- **Handler Layer** - HTTP request handlers & job handlers
- **Server Layer** - Worker registration & REST server setup
- **Storage Layer** - Redis storage abstraction
- **Container** - Dependency injection

### Private Helper Methods Pattern

Services in the usecase layer use private helper methods for task scheduling:

- `OrderService::schedule_order_task()` - Schedules order jobs to Redis queue
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

## Troubleshooting

### Worker not consuming tasks

**Problem:** Tasks are pushed to Redis but worker doesn't process them.

**Solution:** Check task status in Redis:
```bash
redis-cli KEYS "*order*"
redis-cli HGETALL "rust_apalis_test::domain::jobs::OrderJob:meta:<TASK_ID>"
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

**Solution:** Ensure `StorageFactory` creates shared instances:
```rust
// In StorageFactory::new()
let order_storage = Arc::new(RedisStorage::new(conn.clone()));

// In create_order_storage()
pub fn create_order_storage(&self) -> RedisStorage<OrderJob> {
    (*self.order_storage).clone()  // Return clone of shared instance
}
```

### Worker restart error: "worker is still active"

**Problem:** Redis still has worker metadata from previous run.

**Solution:** Two options:

1. **Flush Redis (quick fix):**
```bash
redis-cli FLUSHALL
```

2. **Wait for worker timeout (default 60 seconds)** - Worker metadata will expire automatically.

**Note:** This project uses unique worker IDs (timestamp-based), so restart usually works without flushing.

## Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| `apalis` | 1.0.0-rc.4 | Job processing framework |
| `apalis-redis` | 1.0.0-rc.3 | Redis storage backend |
| `axum` | 0.8 | HTTP server framework |
| `redis` | 0.32 | Redis client |
| `tokio` | 1 | Async runtime |
| `serde` | 1 | Serialization |
| `async-trait` | 0.1 | Async trait support |

## Design Benefits

1. **Trait-based Architecture** - Flexible business logic via traits
2. **Dependency Injection** - Centralized AppContainer
3. **Shared Storage** - Single RedisStorage instance per job type
4. **Type Safety** - Trait objects ensure compile-time checks
5. **Testability** - Each layer can be tested independently
6. **Scalability** - Easy to add new job types
7. **Graceful Shutdown** - Clean exit on Ctrl+C
8. **Worker Concurrency** - Configurable parallel processing

## Contributing

When adding new job types, follow the guide in [ARCHITECTURE.md](ARCHITECTURE.md#how-to-add-a-new-job-type).

## License

MIT
