# Setup Database PostgreSQL untuk Apalis

## Connection String yang Digunakan

**Semua file menggunakan connection string yang sama:**
```
postgres://root:root@localhost:5432/apalis-database
```

**Format:** `postgres://[USER]:[PASSWORD]@[HOST]:[PORT]/[DATABASE]`

| Parameter | Value |
|-----------|-------|
| User | `root` |
| Password | `root` |
| Host | `localhost` |
| Port | `5432` |
| Database | `apalis-database` |

---

## Cara Setup Database

### Opsi 1: Menggunakan Script Setup (Disarankan)

Jalankan script yang sudah disediakan:

```bash
./setup-db.sh
```

Script ini akan:
- ✅ Membuat user `root` dengan password `root`
- ✅ Membuat database `apalis-postgres`
- ✅ Grant privileges yang diperlukan

### Opsi 2: Manual Setup

**1. Connect ke PostgreSQL:**
```bash
psql -U postgres
```

**2. Buat user:**
```sql
CREATE ROLE root WITH LOGIN PASSWORD 'root';
```

**3. Buat database:**
```sql
CREATE DATABASE apalis_database OWNER root;
```

**4. Grant privileges:**
```sql
GRANT ALL PRIVILEGES ON DATABASE apalis_database TO root;
```

**5. Keluar:**
```sql
\q
```

### Opsi 3: Via Docker

**Jalankan PostgreSQL di Docker:**

```bash
docker run --name apalis-database \
  -e POSTGRES_USER=root \
  -e POSTGRES_PASSWORD=root \
  -e POSTGRES_DB=apalis-database \
  -p 5432:5432 \
  -d postgres:16-alpine
```

**Verifikasi:**
```bash
docker ps | grep apalis-database

# Test connection
docker exec -it apalis-database psql -U root -d apalis-database
```

---

## Menjalankan Migration

Setelah database siap, jalankan migration:

```bash
cargo run --bin setup_migration
```

Output yang diharapkan:
```
🔄 Setting up Apalis PostgreSQL migrations...

📡 Connecting to: postgres://root:root@localhost:5432/apalis-database
✅ Connected!

🚀 Running migrations...
✅ Migrations completed successfully!

📊 Checking tables...
✅ Table 'tasks' exists!

📋 Table structure:
  - id: uuid (nullable: NO)
  ...
```

---

## Verifikasi Setup

**Test connection via psql:**
```bash
psql -h localhost -U root -d apalis-database
```

**Cek tabel:**
```sql
\dt

-- Harus ada tabel: tasks
```

**Query test:**
```sql
SELECT COUNT(*) FROM tasks;
```

---

## Troubleshooting

### Error: "database \"apalis-postgres\" does not exist"

**Solusi:** Jalankan setup database dulu

```bash
# Via script
./setup-db.sh

# Atau manual
psql -U postgres -c "CREATE DATABASE apalis_database OWNER root;"
```

### Error: "authentication failed"

**Solusi 1:** Update pg_hba.conf

```bash
# Cari lokasi pg_hba.conf
psql -U postgres -c "SHOW hba_file;"

# Edit file, ubah ke 'trust' atau 'md5'
local   all             all                                     trust
host    all             all    127.0.0.1/32                    trust

# Restart PostgreSQL
brew services restart postgresql@16  # macOS
sudo systemctl restart postgresql  # Linux
```

**Solusi 2:** Ganti password user root

```bash
psql -U postgres -c "ALTER ROLE root WITH PASSWORD 'root';"
```

### Error: "role \"root\" does not exist"

**Solusi:** Buat user root

```bash
psql -U postgres -c "CREATE ROLE root WITH LOGIN PASSWORD 'root';"
psql -U postgres -c "CREATE DATABASE apalis_database OWNER root;"
```

---

## Running Application

Setelah database dan migration siap:

**REST API:**
```bash
cargo run --bin rest
```

**Worker:**
```bash
cargo run --bin worker
```

Kedua binary akan otomatis menjalankan migration saat startup.

---

## Monitoring Jobs

**Connect ke database:**
```bash
psql -h localhost -U root -d apalis-database
```

**Query penting:**

```sql
-- Lihat semua jobs
SELECT id, queue, status, created_at
FROM tasks
ORDER BY created_at DESC
LIMIT 10;

-- Jobs by status
SELECT queue, status, COUNT(*) as total
FROM tasks
GROUP BY queue, status;

-- Jobs yang pending
SELECT * FROM tasks WHERE status = 'pending';

-- Jobs yang failed
SELECT * FROM tasks WHERE status = 'failed'
ORDER BY created_at DESC;

-- Kill job yang stuck
UPDATE tasks SET status = 'failed'
WHERE id = 'task-id-here';
```

---

## Reset Database (Jika Perlu)

```bash
# Drop dan recreate
psql -U postgres -c "DROP DATABASE IF EXISTS apalis_database;"
psql -U postgres -c "CREATE DATABASE apalis_database OWNER root;"
```

---

## Quick Start Command

```bash
# 1. Setup database
./setup-db.sh

# 2. Jalankan migration
cargo run --bin setup_migration

# 3. Jalankan aplikasi
cargo run --bin rest
cargo run --bin worker
```
