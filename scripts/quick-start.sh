#!/bin/bash

# GameVerse Framework - Quick Start Script
# Быстрая настройка сервера для тестировщиков

set -e

echo "🎮 GameVerse Framework - Quick Start Setup"
echo "============================================="

# Check prerequisites
echo "📋 Checking prerequisites..."

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Install from: https://rustup.rs/"
    exit 1
fi

if ! command -v node &> /dev/null; then
    echo "❌ Node.js not found. Install from: https://nodejs.org/"
    exit 1
fi

echo "✅ Prerequisites check passed"

# Build server
echo "🔨 Building GameVerse server..."
cd core
cargo build --release --bin gameverse_server
cd ..

# Setup basic resources
echo "📦 Setting up basic resources..."
mkdir -p server-data/{resources,logs,config}

# Copy basic gamemode
cp -r resources/basic-gamemode server-data/resources/

# Copy configuration
cp core/config/gameverse.toml server-data/config/

# Generate TypeScript definitions
echo "📘 Generating TypeScript definitions..."
cd sdk/native-generator
cargo run -- generate --target type-script --intellisense -o ../../server-data/natives
cd ../..

# Create launch script
cat > server-data/start-server.sh << 'EOF'
#!/bin/bash
echo "🚀 Starting GameVerse server..."
../core/target/release/gameverse_server config/gameverse.toml --dev
EOF

chmod +x server-data/start-server.sh

echo "🎉 GameVerse server setup complete!"
echo ""
echo "💡 Next steps:"
echo "1. cd server-data"  
echo "2. ./start-server.sh"
echo "3. Connect to localhost:30120"
echo ""
echo "📚 Documentation: https://docs.gameverse.dev"
echo "💬 Discord: https://discord.gg/gameverse" 