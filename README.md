# Rust Apalis - Clean Architecture Job Processing

A production-ready example project using **Apalis 1.0.0-rc.4** for background job processing with REST API and worker architecture.

## Features

- ✅ **Trait-based Usecase Pattern** - Clean separation of business logic
- ✅ **REST API** - Create/schedule jobs via HTTP endpoints (Axum)
- ✅ **Worker Processing** - Async job consumers with retry policy
- ✅ **Dependency Injection** - Centralized AppContainer
- ✅ **Redis Storage** - Shared storage instances for producer & consumer
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
Registering order worker...
Registering email worker...
Starting monitor...
Starting order worker instance 0
Starting email worker instance 0
```

### 3. Start REST API (Terminal 2)

```bash
cargo run --bin rest
```

Output:
```
REST API running on http://0.0.0.0:3000
POST /orders - Create order email job
GET  /health - Health check
```

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
- **Usecase Layer** - Business logic traits & implementations
- **Handler Layer** - HTTP request handlers & job handlers
- **Server Layer** - Worker registration & REST server setup
- **Storage Layer** - Redis storage abstraction
- **Container** - Dependency injection

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

## Contributing

When adding new job types, follow the guide in [ARCHITECTURE.md](ARCHITECTURE.md#how-to-add-a-new-job-type).

## License

MIT
