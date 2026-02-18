# Setup PostgreSQL untuk Apalis

## Prerequisites

### macOS (Homebrew)
```bash
# Install PostgreSQL
brew install postgresql@16
brew services start postgresql@16

# Buat database
createdb apalis
```

### Docker
```bash
# Jalankan PostgreSQL di Docker
docker run --name apalis-postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=apalis \
  -p 5432:5432 \
  -d postgres:16-alpine

# Verifikasi
docker ps | grep apalis-postgres
```

### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql

# Buat database
sudo -u postgres createdb apalis
```

## Connection String

Semua binary menggunakan connection string yang sama:

```rust
postgres://postgres:postgres@localhost:5432/apalis
```

**Format:** `postgres://[USER]:[PASSWORD]@[HOST]:[PORT]/[DATABASE]`

| Parameter | Value |
|-----------|-------|
| User | `postgres` |
| Password | `postgres` |
| Host | `localhost` |
| Port | `5432` |
| Database | `apalis` |

## Menjalankan Migration

### Option 1: Via Script Setup (Disarankan)
```bash
cargo run --bin setup_migration
```

Ini akan:
- ✅ Connect ke database
- ✅ Jalankan semua migration
- ✅ Verifikasi tabel terbuat
- ✅ Tampilkan struktur tabel

### Option 2: Otomatis saat Startup
Migration otomatis dijalankan saat menjalankan REST API atau Worker:

```bash
# REST API (migration otomatis dijalankan)
cargo run --bin rest

# Worker (migration otomatis dijalankan)
cargo run --bin worker
```

## Verifikasi Setup

### 1. Test Connection
```bash
# Via psql
psql -h localhost -U postgres -d apalis

# Atau via Docker
docker exec -it apalis-postgres psql -U postgres -d apalis
```

### 2. Cek Tabel
```sql
-- Lihat semua tabel
\dt

-- Cek tabel tasks
SELECT table_name, table_type 
FROM information_schema.tables 
WHERE table_name = 'tasks';
```

### 3. Lihat Struktur Tabel
```sql
\d+ tasks

-- Atau query manual
SELECT column_name, data_type, is_nullable 
FROM information_schema.columns 
WHERE table_name = 'tasks' 
ORDER BY ordinal_position;
```

### 4. Cek Data
```sql
-- Lihat semua jobs
SELECT id, queue, status, created_at 
FROM tasks 
ORDER BY created_at DESC 
LIMIT 10;

-- Count by status
SELECT queue, status, COUNT(*) as total
FROM tasks
GROUP BY queue, status
ORDER BY queue, status;
```

## Troubleshooting

### Error: "unexpected response from SSLRequest"
**Penyebab:** Database belum berjalan atau port salah

**Solusi:**
```bash
# Cek apakah PostgreSQL berjalan
ps aux | grep postgres

# Atau cek port
lsof -i :5432

# Start PostgreSQL jika belum berjalan
# macOS
brew services start postgresql@16

# Linux
sudo systemctl start postgresql

# Docker
docker start apalis-postgres
```

### Error: "database \"apalis\" does not exist"
**Solusi:**
```bash
# Buat database
createdb apalis

# Atau via psql
psql -U postgres -c "CREATE DATABASE apalis;"
```

### Error: "authentication failed"
**Solusi:**
```bash
# Update pg_hba.conf untuk trust authentication
# Lokasi file:
# - macOS: /opt/homebrew/var/postgresql@16/pg_hba.conf
# - Linux: /etc/postgresql/16/main/pg_hba.conf

# Ubah method ke 'trust' untuk local connections:
# local   all             all                                     trust

# Restart PostgreSQL
brew services restart postgresql@16
# atau
sudo systemctl restart postgresql
```

### Error: "connection refused"
**Solusi:**
```bash
# Cek apakah PostgreSQL listening di port 5432
netstat -an | grep 5432

# Atau telnet
telnet localhost 5432
```

## Reset Database (Jika Perlu)

```bash
# Drop dan recreate database
dropdb apalis
createdb apalis

# Jalankan ulang migration
cargo run --bin setup_migration
```

## Configuration Lainnya

### Custom Connection String
Jika ingin menggunakan configuration berbeda, update semua file:

1. **bins/rest/main.rs**
2. **bins/worker/main.rs**
3. **bins/setup_migration/main.rs**

Ubah baris:
```rust
let database_url = "postgres://user:pass@host:port/dbname";
```

### Environment Variables (Recommended)
Untuk production, gunakan environment variables:

```bash
# .env file
DATABASE_URL=postgres://postgres:postgres@localhost:5432/apalis

# Di kode Rust
use std::env;
let database_url = env::var("DATABASE_URL")
    .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/apalis".to_string());
```

## Monitoring Jobs

Setelah aplikasi berjalan, monitor jobs via SQL:

```sql
-- Jobs yang pending
SELECT * FROM tasks WHERE status = 'pending' ORDER BY run_at;

-- Jobs yang failed
SELECT * FROM tasks WHERE status = 'failed' ORDER BY created_at DESC;

-- Jobs yang running
SELECT * FROM tasks WHERE status = 'running';

-- Kill job yang stuck (emergency only)
UPDATE tasks SET status = 'failed' WHERE id = 'task-id';
```

## Best Practices

1. **Backup Database:**
   ```bash
   pg_dump -U postgres apalis > backup.sql
   ```

2. **Monitoring:**
   - Setup logging untuk worker
   - Monitor table size growth
   - Cleanup old jobs periodically

3. **Performance:**
   - Create indexes untuk frequently queried columns
   - Partition tasks table by date jika data sangat besar
   - Use connection pooling (PgPool sudah di-setup)
