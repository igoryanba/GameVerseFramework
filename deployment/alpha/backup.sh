#!/bin/sh
set -eu
compose_file=${COMPOSE_FILE:-deployment/alpha/compose.yaml}
backup_dir=${BACKUP_DIR:-deployment/alpha/backups}
mkdir -p "$backup_dir"
output=${1:-"$backup_dir/gameverse-$(date -u +%Y%m%dT%H%M%SZ).dump"}
case "$output" in
  "$backup_dir"/*) ;;
  *) echo "backup path must be inside $backup_dir" >&2; exit 2 ;;
esac
temporary="$output.partial"
docker compose -f "$compose_file" exec -T postgres pg_dump --username gameverse --dbname gameverse --format custom --no-owner --no-acl > "$temporary"
test -s "$temporary"
mv "$temporary" "$output"
sha256sum "$output" > "$output.sha256"
printf '{"status":"created","path":"%s","sha256_file":"%s.sha256"}\n' "$output" "$output"
