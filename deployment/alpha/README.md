# Closed-alpha deployment

This is the supported development deployment for the independent M2 runtime. It contains only the M2 server and PostgreSQL; Redis and the legacy services are intentionally absent.

```sh
cd deployment/alpha
cp .env.example .env
# replace the password in .env
docker compose up --build -d
docker compose ps
```

The server identity is generated once and retained in `server-identity`. Copy `/var/lib/gameverse/identity/server-cert.der` from the server container to an authenticated distribution location for the Windows launcher; clients trust that pinned certificate.

The M2 server applies every embedded SQLx migration in filename order after PostgreSQL becomes healthy and before it starts accepting sessions. Database-backed account, character, courier, shop and reconnect handling are part of the M2 server.

The admin API is disabled unless `GAMEVERSE_ADMIN_BIND`, `GAMEVERSE_ADMIN_TOKEN` (at least 32 characters) and `GAMEVERSE_ADMIN_ACTOR_ID` are all set. Bind it to `0.0.0.0:30124` inside Compose; the host mapping remains localhost-only. Every call requires `Authorization: Bearer …`, and the configured actor must still have a live moderator or administrator role in PostgreSQL. The API exposes `/v1/admin/sessions`, `/v1/admin/audit`, and administrator-audited account `ban`/`unban` operations.

Operational endpoints bind to localhost on the host by default:

- `http://127.0.0.1:30123/v1/health`
- `http://127.0.0.1:30123/v1/ready`
- `http://127.0.0.1:30123/v1/version`
- `http://127.0.0.1:30123/v1/metrics`

Create a verified PostgreSQL backup from the repository root:

```sh
deployment/alpha/backup.sh
deployment/alpha/restore.sh deployment/alpha/backups/gameverse-YYYYMMDDTHHMMSSZ.dump
```

Restore uses a separate `gameverse_restore` database by default and verifies its schema. Replacing the primary database requires both the explicit `gameverse` target and `ALLOW_REPLACE_PRIMARY=yes`. Keep backup files outside the repository and test restoration before every release.
