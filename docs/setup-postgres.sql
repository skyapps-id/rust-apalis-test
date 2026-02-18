-- Setup database apalis
-- Jalankan ini di PostgreSQL sebelum menjalankan aplikasi

-- 1. Buat database
CREATE DATABASE IF NOT EXISTS apalis;

-- 2. Connect ke database apalis
\c apalis

-- 3. Migration akan otomatis dijalankan oleh aplikasi saat startup
-- melalui: apalis_postgres::PostgresStorage::setup(&pool).await;

-- 4. Untuk melihat tabel setelah migration:
\d+ tasks

-- 5. Query untuk monitoring jobs
SELECT 
    queue,
    status,
    COUNT(*) as total
FROM tasks
GROUP BY queue, status
ORDER BY queue, status;

-- 6. Query untuk melihat jobs yang pending/running
SELECT 
    id,
    queue,
    status,
    task->>'event_id' as event_id,
    attempts,
    created_at,
    run_at
FROM tasks
WHERE status IN ('pending', 'running')
ORDER BY run_at;
