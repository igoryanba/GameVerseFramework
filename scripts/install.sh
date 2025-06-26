#!/usr/bin/env bash
# GameVerse installer script (Linux/macOS)
set -euo pipefail

REPO="igoryanba/GameVerseFramework"
TAG="${1:-latest}"
BINDIR="$HOME/.gameverse/bin"
mkdir -p "$BINDIR"

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || { echo "❌ '$1' not found. Please install it."; exit 1; }
}

need_cmd curl
need_cmd tar

OS=$(uname -s)
ARCH=$(uname -m)
case "$OS" in
  Linux) PLATFORM="linux" ;;
  Darwin) PLATFORM="macos" ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

if [[ "$TAG" == "latest" ]]; then
  echo "🛰️  Fetching latest release tag…"
  TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep -oP '"tag_name":\s*"\K([^"]+)')
fi

ASSET_NAME="gameverse-${PLATFORM}-${ARCH}.tar.gz"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$TAG/$ASSET_NAME"

echo "📥 Downloading $DOWNLOAD_URL"
curl -L "$DOWNLOAD_URL" -o "/tmp/$ASSET_NAME"

echo "📦 Extracting to $BINDIR"
tar -xzf "/tmp/$ASSET_NAME" -C "$BINDIR"
rm "/tmp/$ASSET_NAME"

if [[ ":$PATH:" != *":$BINDIR:"* ]]; then
  echo "➕ Add the following line to your shell profile (~/.bashrc / ~/.zshrc):"
  echo "export PATH=\"\$PATH:$BINDIR\""
fi

echo "✅ GameVerse installed. Test with: gameverse --help" 