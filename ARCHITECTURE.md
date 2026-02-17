# Clean Architecture Job Processing with Apalis

This project demonstrates a clean architecture pattern for building job processing systems with Rust and Apalis.

## Architecture Overview

The project is organized into four main layers following clean architecture principles:

### 1. Domain Layer (`src/domain/`)
Contains all job types and business entities.

- **`jobs.rs`**: All job type definitions (`OtaTimeoutJob`, `AlertJob`, `EmailJob`)
- **`enums.rs`**: Domain enums (`AlertType`, `Severity`)

### 2. Workflow Layer (`src/workflow/`)
Contains job handlers and business logic.

- **`handler.rs`**: The `JobHandler` trait that all handlers implement
- **`handlers/`**: Individual handler implementations (e.g., `ota_timeout.rs`)

### 3. Server Layer (`src/server/`)
Contains worker implementations and monitoring.

- **`monitor.rs`**: `JobRegistry` for registering and running all workers
- **`worker_config.rs`**: Configuration types for workers (retry policy, etc.)
- **`workers/`**: Individual worker implementations (handler functions)

### 4. Storage Layer (`src/storage/`)
Provides storage abstractions for job queues.

- **`redis.rs`**: `StorageFactory` for creating Redis-backed storages

## Project Structure

```
rust-apalis-test/
├── src/
│   ├── lib.rs                 # Public API exports
│   ├── domain/                # Job types & domain logic
│   │   ├── mod.rs
│   │   ├── jobs.rs           # All job definitions
│   │   └── enums.rs          # AlertType, Severity, etc.
│   ├── workflow/              # Job handlers (business logic)
│   │   ├── mod.rs
│   │   ├── handler.rs        # JobHandler trait
│   │   └── handlers/
│   │       ├── mod.rs
│   │       └── ota_timeout.rs
│   ├── server/                # Worker implementations
│   │   ├── mod.rs
│   │   ├── monitor.rs        # JobRegistry for worker registration
│   │   ├── worker_config.rs  # Worker configuration types
│   │   └── workers/
│   │       ├── mod.rs
│   │       └── ota_timeout.rs
│   └── storage/               # Storage abstraction
│       ├── mod.rs
│       └── redis.rs
├── bins/                      # Binary executables
│   ├── worker/main.rs        # Worker binary
│   └── producer/main.rs      # Producer binary
├── Cargo.toml
└── README.md
```

## How to Add a New Job Type

Follow these steps to add a new job type:

### Step 1: Define the Job in Domain Layer

Add your job type to `src/domain/jobs.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyNewJob {
    pub id: String,
    pub data: String,
}
```

### Step 2: Implement the Handler

Create `src/workflow/handlers/my_new_job.rs`:

```rust
use crate::domain::jobs::MyNewJob;
use crate::workflow::handler::JobHandler;

#[derive(Clone)]
pub struct MyNewJobHandler;

impl Default for MyNewJobHandler {
    fn default() -> Self {
        Self
    }
}

impl JobHandler for MyNewJobHandler {
    type Job = MyNewJob;

    async fn handle(
        &self,
        job: Self::Job,
        attempt: usize,
        max_retries: usize,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Your business logic here
        println!("Processing job: {}", job.id);
        Ok(())
    }
}
```

Update `src/workflow/handlers/mod.rs`:

```rust
pub mod my_new_job;
pub use my_new_job::MyNewJobHandler;
```

### Step 3: Create Worker Handler Function

Create `src/server/workers/my_new_job.rs`:

```rust
use apalis::prelude::*;
use apalis_core::task::attempt::Attempt;
use std::sync::Arc;
use crate::domain::jobs::MyNewJob;
use crate::workflow::{JobHandler, handlers::MyNewJobHandler};

pub async fn my_new_job_handler_fn(
    job: MyNewJob,
    handler: Data<Arc<MyNewJobHandler>>,
    attempt: Attempt,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    handler.handle(job, attempt.current(), 3).await
}
```

Update `src/server/workers/mod.rs`:

```rust
pub mod my_new_job;
pub use my_new_job::my_new_job_handler_fn;
```

### Step 4: Register the Job

Update `src/server/monitor.rs`:

1. Add storage field to `JobRegistry`:

```rust
pub struct JobRegistry {
    // ... existing fields
    pub my_new_job_storage: Option<RedisStorage<MyNewJob>>,
    pub my_new_job_handler: Arc<MyNewJobHandler>,
    pub my_new_job_retry_config: WorkerRetryConfig,
}
```

2. Add builder methods:

```rust
pub fn with_my_new_job_storage(mut self, storage: RedisStorage<MyNewJob>) -> Self {
    self.my_new_job_storage = Some(storage);
    self
}
```

3. Register worker in `run()` method:

```rust
if let Some(storage) = self.my_new_job_storage {
    let handler = self.my_new_job_handler;
    let config = self.my_new_job_retry_config;

    monitor = monitor.register(move |count| {
        // ... worker configuration
        WorkerBuilder::new(format!("my-new-job-worker-{}", count))
            .backend(storage.clone())
            .retry(/* ... */)
            .data(handler.clone())
            .build(my_new_job_handler_fn)
    });
}
```

4. Add storage factory method to `src/storage/redis.rs`:

```rust
pub fn create_my_new_job_storage(&self) -> RedisStorage<MyNewJob> {
    RedisStorage::new(self.conn.clone())
}
```

### Step 5: Update Worker Binary

Update `bins/worker/main.rs` to include the new storage:

```rust
JobRegistry::new()
    .with_my_new_job_storage(storage_factory.create_my_new_job_storage())
    .run()
    .await?;
```

## Running the Project

### Start Redis

```bash
redis-server
```

### Start the Worker

```bash
cargo run --bin worker
```

### Produce Jobs

```bash
cargo run --bin producer
```

## Design Benefits

1. **Separation of Concerns**: Each layer has a clear responsibility
2. **Easy to Extend**: Adding new jobs requires touching only a few files
3. **Type Safety**: Rust's type system ensures job handlers match job types
4. **Testability**: Handlers can be tested independently of workers
5. **Reusability**: Common patterns (retry policy, storage) are abstracted

## Key Patterns

### JobHandler Trait
All handlers implement the `JobHandler` trait, ensuring consistent interfaces:

```rust
pub trait JobHandler: Clone + Send + Sync + 'static {
    type Job: Send + Sync;

    fn handle(
        &self,
        job: Self::Job,
        attempt: usize,
        max_retries: usize,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send;
}
```

### JobRegistry
The `JobRegistry` provides a fluent API for configuring and running all workers:

```rust
JobRegistry::new()
    .with_ota_timeout_storage(storage)
    .with_retry_config(config)
    .run()
    .await?;
```

### StorageFactory
Centralized storage creation for all job types:

```rust
let factory = StorageFactory::new(conn);
let storage = factory.create_ota_timeout_storage();
```
