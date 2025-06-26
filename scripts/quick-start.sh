#!/bin/bash

# GameVerse Framework - Quick Start Script
# Быстрая настройка сервера для тестировщиков

set -e

# ----------------------------------------------------------------------------
# Branding – ASCII logo (same as CLI)
# ----------------------------------------------------------------------------
print_logo() {
  local green='\033[0;32m'
  local cyan='\033[0;36m'
  local reset='\033[0m'
  local lines=(
"   ██████╗  █████╗ ███╗   ███╗███████╗██╗   ██╗███████╗██████╗ ███████╗ ███████╗"
"  ██╔════╝ ██╔══██╗████╗ ████║██╔════╝██║   ██║██╔════╝██╔══██╗██╔════╝ ██╔════╝"
"  ██║  ███╗███████║██╔████╔██║█████╗  ██║   ██║█████╗  ██████╔╝███████╗ ███████╗"
"  ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝  ╚██╗ ██╔╝██╔══╝  ██╔══██╗╚════██║ ██╔════╝"
"  ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗ ╚████╔╝ ███████╗██║  ██║███████║ ███████╗"
"   ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝╚══════╝ ╚══════╝"
  )
  for i in "${!lines[@]}"; do
    if (( i % 2 == 0 )); then
      echo -e "${green}${lines[$i]}${reset}"
    else
      echo -e "${cyan}${lines[$i]}${reset}"
    fi
  done
  echo
}

# ----------------------------------------------------------------------------
# Help
# ----------------------------------------------------------------------------
show_help() {
  cat <<EOF
🎮 GameVerse Framework – Quick Start

Usage: ./scripts/quick-start.sh [OPTIONS]

Options:
  --skip-build            Пропустить компиляцию сервера (если уже собран)
  --skip-natives          Не генерировать TypeScript natives
  --no-basic-gamemode     Не копировать пример ресурса basic-gamemode
  --clean                 Удалить существующие build-артефакты и server-data перед запуском
  -h, --help              Показать эту справку

Пример:
  ./scripts/quick-start.sh --skip-build --skip-natives
EOF
}

# Default flags
SKIP_BUILD=false
SKIP_NATIVES=false
WITH_BASIC_GAMEMODE=true
CLEAN=false

# Parse arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    --skip-build) SKIP_BUILD=true ; shift ;;
    --skip-natives) SKIP_NATIVES=true ; shift ;;
    --no-basic-gamemode|--without-basic-gamemode) WITH_BASIC_GAMEMODE=false ; shift ;;
    --clean) CLEAN=true ; shift ;;
    -h|--help) show_help ; exit 0 ;;
    *) echo "Unknown option: $1" ; show_help ; exit 1 ;;
  esac
done

# ----------------------------------------------------------------------------
# Begin
# ----------------------------------------------------------------------------
print_logo

echo "🎮 GameVerse Framework - Quick Start Setup"
echo "============================================="
# Developer info (bright white)
echo -e "\033[1;37mdev: https://x.com/genecental  |  GitHub: https://github.com/igoryanba  |  Telegram: https://t.me/igoryan34\033[0m"

# Clean up if requested
if [[ "$CLEAN" == true ]]; then
  echo "🧹 Cleaning previous builds and server-data..."
  rm -rf core/target server-data
fi

# Check prerequisites
echo "📋 Checking prerequisites..."

# Helper to verify that a command exists
need_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo -e "\033[0;31m❌ '$1' not found. См. docs/GETTING_STARTED.md → Установка зависимостей.\033[0m"
    exit 1
  fi
}

# Required tools
need_cmd git
need_cmd rustc
need_cmd cargo
need_cmd node

echo "✅ Prerequisites check passed"

# Build server (unless skipped)
if [[ "$SKIP_BUILD" == false ]]; then
  echo "🔨 Building GameVerse server..."
  pushd core >/dev/null
  cargo build --release --bin gameverse_server
  popd >/dev/null
else
  echo "⏭️  Skipping server build (--skip-build)"
fi

# Setup basic resources
echo "📦 Setting up basic resources..."
mkdir -p server-data/{resources,logs,config}

# Copy basic gamemode (optional)
if [[ "$WITH_BASIC_GAMEMODE" == true ]]; then
  echo "📄 Copying basic-gamemode resource..."
  cp -r resources/basic-gamemode server-data/resources/
else
  echo "⏭️  Skipping basic-gamemode copy (--no-basic-gamemode)"
fi

# Copy configuration
cp core/config/gameverse.toml server-data/config/

# Generate TypeScript definitions (unless skipped)
if [[ "$SKIP_NATIVES" == false ]]; then
  echo "📘 Generating TypeScript definitions..."
  pushd sdk/native-generator >/dev/null
  cargo run -- generate --target type-script --intellisense -o ../../server-data/natives
  popd >/dev/null
else
  echo "⏭️  Skipping natives generation (--skip-natives)"
fi

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