# Rust Apalis Test

A simple example project using Apalis 1.0.0-rc.4 for background job processing.

## Prerequisites

- Rust 2024 edition
- Redis server (optional, for future implementation)

## Installation

```bash
cargo build
```

## Running

Run the worker example:
```bash
cargo run --bin main
```

Run the producer example:
```bash
cargo run --bin producer
```

## Project Structure

- `src/main.rs` - Worker implementation with job handler example
- `src/producer.rs` - Job structure definition and enqueue example
- `Cargo.toml` - Dependencies including apalis 1.0.0-rc.4

## Current Status

This project demonstrates the basic structure for using Apalis 1.0.0-rc.4. The full worker implementation with Redis storage requires additional trait implementations that are still being stabilized in the release candidate version.

## Dependencies

- `apalis` 1.0.0-rc.4
- `apalis-redis` 0.7.0-rc.4
- `apalis-core` 1.0.0-rc.4
- `tokio` with full features
- `serde` with derive
- `redis` 0.32 with tokio-comp
- `chrono` for timestamps
