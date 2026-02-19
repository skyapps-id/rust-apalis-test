# Install Apalis Board UI (Manual Build)

Dokumentasi ini menjelaskan cara build dan install Apalis Board UI secara manual menggunakan `trunk`.

## Prerequisites

### 1. Install Trunk

**macOS:**
```bash
brew install trunk
```

**Linux:**
```bash
cargo install trunk
```

**Verifikasi instalasi:**
```bash
trunk --version
# Output: trunk 0.21.14
```

### 2. Install wasm32 target untuk Rust

```bash
rustup target add wasm32-unknown-unknown
```

### 3. Install Node.js dan npm

Pastikan Node.js sudah terinstall:
```bash
node --version
npm --version
```

## Langkah-langkah Build Manual

### 1. Clone Apalis Board Repository

Clone repository dan download hanya folder yang diperlukan:

```bash
cd /path/to/your/project

# Clone repo dengan sparse checkout
git clone --depth 1 --branch main --filter=blob:none --sparse https://github.com/apalis-dev/apalis-board.git temp-apalis-board

# Setup sparse checkout untuk hanya mengambil web dan types
cd temp-apalis-board
git sparse-checkout set crates/web crates/types

# Kembali ke project root
cd ..
```

### 2. Copy Folder ke Project

```bash
# Copy folder web dan types ke project
cp -r temp-apalis-board/crates/web apalis-board-web
cp -r temp-apalis-board/crates/types apalis-board-types

# Hapus temporary repo
rm -rf temp-apalis-board
```

### 3. Fix Dependencies

Edit `apalis-board-types/Cargo.toml` untuk menghapus workspace configuration:

**File: `apalis-board-types/Cargo.toml`**
```toml
# Hapus bagian ini:
[lints]
workspace = true
```

**File: `apalis-board-web/Cargo.toml`**
```toml
# Ubah path dependencies:
apalis-board-types = { path = "../apalis-board-types", version = "1.0.0-rc.3" }
```

### 4. Install npm Dependencies

```bash
cd apalis-board-web

# Install tailwindcss plugins
npm install tailwindcss-animate
npm install @tailwindcss/container-queries @tailwindcss/forms @tailwindcss/typography
npm install tailwind-scrollbar tailwind-scrollbar-hide tailwindcss-textshadow

cd ..
```

### 5. Build Frontend dengan Trunk

```bash
cd apalis-board-web
trunk build
```

**Output yang diharapkan:**
```
INFO 🚀 Starting trunk 0.21.14
INFO 📦 starting build
Finished `dev` profile [unoptimized + debuginfo] target(s) in XmXXs
INFO applying new distribution
INFO ✅ success
```

### 6. Verifikasi Build

Cek apakah folder `dist` sudah terbuat:

```bash
ls -la apalis-board-web/dist/
```

**Seharusnya ada file:**
- `index.html`
- `apalis-board-web-[hash].js`
- `apalis-board-web-[hash]_bg.wasm`
- `input-[hash].css`

## Update Backend Configuration

### 1. Tambahkan Dependencies ke `Cargo.toml`

```toml
[dependencies]
# ... dependencies lainnya
tower-http = { version = "0.6", features = ["cors", "fs"] }
```

### 2. Update `bins/board/main.rs`

**Import ServeDir:**
```rust
use tower_http::services::fs::ServeDir;
```

**Gunakan ServeDir sebagai fallback:**
```rust
let app = Router::new()
    .nest("/api/v1", api_routes)
    .fallback_service(ServeDir::new("apalis-board-web/dist"))
    .layer(cors)
    .layer(Extension(broadcaster.clone()));
```

**Full example:**
```rust
use apalis_board::axum::{
    sse::{TracingBroadcaster, TracingSubscriber},
};
use apalis_board_api::framework::{ApiBuilder, RegisterRoute};
use apalis_postgres::PostgresStorage;
use axum::{Extension, Router};
use rust_apalis_test::storage::postgres::StorageFactory;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::fs::ServeDir;
use tracing_subscriber::{EnvFilter, Layer as TraceLayer, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let broadcaster = TracingBroadcaster::create();

    let line_sub = TracingSubscriber::new(&broadcaster);
    let tracer = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(EnvFilter::builder().parse("info").unwrap()),
        )
        .with(
            line_sub
                .layer()
                .with_filter(EnvFilter::builder().parse("info").unwrap()),
        );
    tracer.try_init()?;

    let database_url = "postgres://root:root@localhost:5432/apalis-database";
    let pool = sqlx::PgPool::connect(database_url).await?;

    PostgresStorage::setup(&pool).await?;

    let storage_factory = StorageFactory::new(pool.clone());

    let api_routes = ApiBuilder::new(Router::new())
        .register(storage_factory.create_email_storage())
        .register(storage_factory.create_order_storage())
        .register(storage_factory.create_alert_storage())
        .build();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api/v1", api_routes)
        .fallback_service(ServeDir::new("apalis-board-web/dist"))
        .layer(cors)
        .layer(Extension(broadcaster.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await?;

    println!("🚀 Apalis Board starting...");
    println!("Board UI: http://localhost:9000");
    println!("API: http://localhost:9000/api/v1");
    println!("SSE Events: http://localhost:9000/api/v1/events");
    println!();

    axum::serve(listener, app).await?;

    Ok(())
}
```

### 3. Build dan Run Backend

```bash
# Build backend
cargo build --bin board

# Run server
cargo run --bin board
```

## Verifikasi

Buka browser dan akses:
- **UI**: http://localhost:9000
- **API**: http://localhost:9000/api/v1/queues
- **SSE Events**: http://localhost:9000/api/v1/events

## Troubleshooting

### Error: `wasm32-unknown-unknown target not installed`

**Solusi:**
```bash
rustup target add wasm32-unknown-unknown
```

### Error: `Cannot find module 'tailwindcss-animate'`

**Solusi:**
```bash
cd apalis-board-web
npm install tailwindcss-animate
```

### Error: `failed to find workspace root`

**Solusi:**
Hapus bagian `[lints] workspace = true` dari `apalis-board-types/Cargo.toml`

### Error: `fs` module not found di tower-http

**Solusi:**
Tambahkan feature "fs" ke `Cargo.toml`:
```toml
tower-http = { version = "0.6", features = ["cors", "fs"] }
```

### Frontend tidak memanggil API

**Cek:**
1. Buka DevTools (F12) → Network tab
2. Refresh halaman
3. Lihat apakah ada request ke `/api/v1/queues`

**Kalau tidak ada request:**
- Pastikan WASM file ter-load (tab Network)
- Cek Console untuk error JavaScript
- Pastikan backend berjalan di port yang benar

### Error CORS di browser

**Pastikan CORS layer sudah di-setup:**
```rust
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);

let app = Router::new()
    // ...
    .layer(cors)
    // ...
```

## Struktur Folder Akhir

```
project-root/
├── apalis-board-web/
│   ├── dist/              # Generated by trunk build
│   │   ├── index.html
│   │   ├── apalis-board-web-[hash].js
│   │   ├── apalis-board-web-[hash]_bg.wasm
│   │   └── input-[hash].css
│   ├── src/               # Source code
│   ├── Cargo.toml
│   ├── index.html
│   ├── Trunk.toml
│   └── tailwind.config.js
├── apalis-board-types/
│   ├── src/
│   └── Cargo.toml
├── bins/
│   └── board/
│       └── main.rs        # Backend server
├── Cargo.toml             # Project dependencies
└── ...
```

## Rebuild Frontend (Jika ada perubahan)

Kalau ada perubahan di source code frontend:

```bash
cd apalis-board-web
trunk build
cd ..
cargo run --bin board
```

## Keuntungan Build Manual

1. **Versi terbaru**: Selalu menggunakan frontend dari latest source code
2. **Customizable**: Bisa modifikasi source code frontend
3. **Debuggable**: Bisa inspect dan debug WASM module
4. **No embedded bugs**: Hindari bugs dari embedded files di crate

## Alternatif: ServeUI dari Crate

Kalau tidak mau build manual, gunakan `ServeUI` dari crate:

```rust
use apalis_board::axum::ui::ServeUI;

let app = Router::new()
    .nest("/api/v1", api_routes)
    .fallback_service(ServeUI::new())  // Embedded files
    .layer(cors);
```

**Tapi perhatikan:** Embedded files di crate mungkin outdated atau memiliki bug.
