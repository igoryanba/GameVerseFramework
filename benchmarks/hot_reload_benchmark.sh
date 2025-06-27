#!/usr/bin/env bash
# =============================================================
# Hot-Reload Benchmark – GameVerse vs FiveM
# -------------------------------------------------------------
# Меритика: время (мс) между изменением файла плагина и готовностью
# сервера после hot-reload. Для FiveM выполняется restart ресурса,
# для GameVerse – встроенная команда `gameverse plugin watch`.
# -------------------------------------------------------------
# Требования:
#   – hyperfine (https://github.com/sharkdp/hyperfine)
#   – FiveM test-server запущен на 30120, GameVerse на 30121
# =============================================================
set -euo pipefail

TEST_PLUGIN="example_plugin"
GV_CMD="gameverse plugin watch --name $TEST_PLUGIN --once"
FIVEM_CMD="bash -c 'echo "restart $TEST_PLUGIN" | nc -u -w1 127.0.0.1 30120'"

hyperfine --warmup 3 --export-json hot_reload_results.json \
  --prepare "touch ../resources/$TEST_PLUGIN/src/main.lua" \
  "$GV_CMD" \
  "$FIVEM_CMD"

echo "\n🟢 Benchmark complete. Results written to hot_reload_results.json" 