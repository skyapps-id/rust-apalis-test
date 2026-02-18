# Quick Start: Apalis Board UI

## Setup UI (One Time)

```bash
# 1. Jalankan script setup
./setup-ui.sh
```

Tunggu sampai selesai (2-5 menit). Script akan:
- ✅ Clone apalis repo
- ✅ Install npm dependencies
- ✅ Build frontend
- ✅ Copy ke folder `static/`

## Jalankan Aplikasi

```bash
# Build dan jalankan REST server
cargo run --bin rest
```

## Buka UI

Buka browser:
```
http://localhost:3000/board
```

## Troubleshooting

### Script gagal?

```bash
# Install Node.js dulu
brew install node  # macOS
# atau
sudo apt install nodejs npm  # Linux

# Ulangi setup
./setup-ui.sh
```

### Build lama sekali?

Normal! Leptos build bisa 2-5 menit pertama kali.

### Halaman blank?

```bash
# Cek static files
ls -la static/

# Harus ada index.html dan folder assets/
```

### Error saat build?

```bash
# Coba manual:
cd apalis-board-web
npm install
npm run build
cd ..
cp -r apalis-board-web/dist/* static/
```

## Need API Integration?

UI saat ini dalam mode read-only. Untuk full functionality (manage jobs dari UI), perlu setup API endpoints.

Lihat `SETUP_UI.md` untuk detail lengkap.

---

**Quick Check:**

```bash
# 1. Prerequisites
node --version  # >= v18
npm --version

# 2. Setup
./setup-ui.sh

# 3. Run
cargo run --bin rest

# 4. Browse
open http://localhost:3000/board
```
