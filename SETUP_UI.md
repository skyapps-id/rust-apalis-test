# Setup Apalis Board UI

## Prerequisites

```bash
# Install Node.js jika belum ada
# macOS
brew install node

# Ubuntu/Debian
sudo apt install nodejs npm

# Verifikasi
node --version
npm --version
```

## Cara Setup UI

### Step 1: Build Frontend

```bash
# Jalankan script setup
./setup-ui.sh
```

Script ini akan:
1. Clone repo apalis
2. Extract package web
3. Install npm dependencies
4. Build frontend dengan Leptos
5. Copy build output ke folder `static/`

### Step 2: Jalankan Aplikasi

```bash
# Build Rust app
cargo build --bin rest

# Jalankan server
cargo run --bin rest
```

### Step 3: Buka UI

Buka browser:
```
http://localhost:3000/board
```

## Manual Setup (Jika Script Gagal)

```bash
# 1. Clone apalis repo
git clone --depth 1 https://github.com/apalis-dev/apalis.git
cd apalis/packages/web

# 2. Install dependencies
npm install

# 3. Build frontend
npm run build

# 4. Copy ke project
cp -r dist/* /path/to/rust-apalis-test/static/

# 5. Jalankan aplikasi
cd /path/to/rust-apalis-test
cargo run --bin rest
```

## Troubleshooting

### Error: "trunk not found"

Install trunk Rust tool:
```bash
cargo install trunk
```

### Error: "npm not found"

Install Node.js:
```bash
# macOS
brew install node

# Linux
sudo apt install nodejs npm
```

### Build Gagal - Memory Error

Coba increase Node.js memory:
```bash
export NODE_OPTIONS="--max-old-space-size=4096"
npm run build
```

### Folder static/ tidak ada

Buat manual:
```bash
mkdir -p static
```

Lalu copy build output:
```bash
cp -r apalis-board-web/dist/* static/
```

## Integrasi API dengan UI

Untuk connect UI dengan backend, perlu setup API:

### Update Router dengan API

```rust
// src/server/rest/router.rs
use apalis_board::axum::framework::{ApiBuilder, RegisterRoute};

pub fn create_router(state: ServerState) -> Router {
    // Setup Apalis Board API
    let board_api = ApiBuilder::new(axum::routing::Router::new())
        .mount("/api/v1")
        .register(state.container.storage.create_order_storage())
        .register(state.container.storage.create_email_storage())
        .build();

    Router::new()
        .route("/orders", post(create_order))
        .route("/health", get(health_check))
        .merge(board_api)  // Tambahkan ini
        .nest_service("/board", ServeDir::new("static"))
        .with_state(state)
}
```

**Catatan**: Integrasi API mungkin memerlukan penyesuaian karena perbedaan state type.

## Verifikasi Setup

### 1. Cek Static Files

```bash
ls -la static/
# Harus ada: index.html, assets/, dll
```

### 2. Test Serve Files

```bash
# Start server
cargo run --bin rest

# Buka di browser
open http://localhost:3000/board
# atau
curl http://localhost:3000/board/
```

### 3. Cek API

```bash
curl http://localhost:3000/api/v1/queues
```

## Struktur Folder

```
rust-apalis-test/
├── static/              # Frontend build output
│   ├── index.html
│   ├── assets/
│   └── ...
├── apalis-board-web/    # Frontend source (untuk development)
├── setup-ui.sh          # Script setup UI
└── src/
```

## Development Mode

Untuk development frontend dengan hot-reload:

```bash
cd apalis-board-web

# Install dependencies
npm install

# Start dev server
npm run dev

# Frontend akan jalan di port lain (misal: 3000)
# Backend Rust di port 3000
```

Lalu update CORS untuk allow cross-origin.

## Quick Start

```bash
# 1. Setup UI (one-time)
./setup-ui.sh

# 2. Setup database (one-time)
./setup-db.sh
cargo run --bin setup_migration

# 3. Jalankan server
cargo run --bin rest

# 4. Buka browser
open http://localhost:3000/board
```

## UI Features

Setelah setup berhasil, UI menyediakan:

- 📊 **Dashboard** - Overview semua queues dan jobs
- 📋 **Queue List** - Lihat semua queues
- 🔍 **Job Details** - Inspect individual jobs
- ⚡ **Actions** - Retry, kill jobs
- 👷 **Workers** - Monitor active workers
- 📈 **Stats** - Queue statistics

## Catatan Penting

1. **Build Time**: Frontend build bisa memakan waktu 2-5 menit
2. **Disk Space**: Folder `static/` ~ 1-2 MB
3. **API Integration**: Untuk full functionality, API perlu di-setup dengan benar
4. **Updates**: Untuk update UI, jalankan `./setup-ui.sh` lagi

## Alternative: Use Pre-built

Jika build terlalu lama, coba download pre-built dari release:

```bash
# Download dari apalis releases (jika available)
wget https://github.com/apalis-dev/apalis/releases/download/vX.X.X/apalis-board-dist.tar.gz

# Extract
tar -xzf apalis-board-dist.tar.gz
cp -r dist/* static/
```
