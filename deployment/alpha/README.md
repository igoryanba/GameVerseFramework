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

The migration currently represents the RP persistence contract. Database-backed account and RP command handling is the next integration step; the M2 presence server starts only after the schema has been applied.
