#!/bin/bash
# Setup apalis-board UI

set -e

echo "🎨 Setting up Apalis Board UI..."
echo ""

# Clone apalis-board repo jika belum ada
if [ ! -d "apalis-board-web" ]; then
    echo "📥 Cloning apalis-board repository..."
    git clone --depth 1 --branch main https://github.com/apalis-dev/apalis-board.git temp-apalis
    mv temp-apalis/crates/web apalis-board-web
    rm -rf temp-apalis
else
    echo "✅ apalis-board-web already exists"
fi

cd apalis-board-web

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    echo "❌ trunk is not installed. Install it with:"
    echo "   cargo install trunk"
    exit 1
fi

# Build frontend
echo "🔨 Building frontend with trunk..."
trunk build --release

cd ..

# Buat direktori static jika belum ada
mkdir -p static

# Copy build output ke static
echo "📂 Copying build output..."
cp -r apalis-board-web/dist/* static/

echo ""
echo "✅ Apalis Board UI setup complete!"
echo ""
echo "Static files served from: ./static/"
echo "UI akan available di: http://localhost:3000/board"
