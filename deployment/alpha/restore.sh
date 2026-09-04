#!/bin/sh
set -eu
if [ "$#" -lt 1 ]; then echo "usage: restore.sh BACKUP [TARGET_DATABASE]" >&2; exit 2; fi
compose_file=${COMPOSE_FILE:-deployment/alpha/compose.yaml}
backup=$1
target=${2:-gameverse_restore}
test -s "$backup"
if [ -f "$backup.sha256" ]; then sha256sum --check "$backup.sha256"; fi
case "$target" in
  gameverse) test "${ALLOW_REPLACE_PRIMARY:-}" = "yes" || { echo "set ALLOW_REPLACE_PRIMARY=yes to replace the primary database" >&2; exit 2; } ;;
  gameverse_restore|gameverse_restore_*) ;;
  *) echo "restore target must be gameverse_restore, gameverse_restore_* or explicitly authorized primary gameverse" >&2; exit 2 ;;
esac
docker compose -f "$compose_file" exec -T postgres dropdb --username gameverse --if-exists "$target"
docker compose -f "$compose_file" exec -T postgres createdb --username gameverse "$target"
if ! docker compose -f "$compose_file" exec -T postgres pg_restore --username gameverse --dbname "$target" --exit-on-error --no-owner --no-acl < "$backup"; then
  docker compose -f "$compose_file" exec -T postgres dropdb --username gameverse --if-exists "$target"
  exit 1
fi
tables=$(docker compose -f "$compose_file" exec -T postgres psql --username gameverse --dbname "$target" -tAc "SELECT count(*) FROM information_schema.tables WHERE table_schema='public'")
test "$tables" -ge 18
printf '{"status":"restored","database":"%s","tables":%s}\n' "$target" "$tables"
