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

Operational endpoints bind to localhost on the host by default:

- `http://127.0.0.1:30123/v1/health`
- `http://127.0.0.1:30123/v1/ready`
- `http://127.0.0.1:30123/v1/version`
- `http://127.0.0.1:30123/v1/metrics`

The admin mutation API and backup automation remain closed-alpha release work.
