# GameVerse Admin Panel (React)

## Development

```bash
cd admin-panel
npm install
npm run dev
```
Dev server runs at `http://localhost:5173` and proxies API calls to localhost:30121.

## Production build

```bash
npm run build
```
Output is placed in `admin-panel/dist/`. To serve via Nginx container, mount/copy `dist/` to `/var/www` and ensure Nginx serves `/`.

Helm chart values can override by adding:
```yaml
extraVolumes:
  - name: static-site
    hostPath:
      path: /opt/gameverse/admin
extraVolumeMounts:
  - name: static-site
    mountPath: /var/www
```

## Features
- Live metrics line chart (avg tick).
- Real-time logs viewer via SSE.

Endpoints used:
- `GET /api/server/metrics/stream` (SSE)
- `GET /api/server/logs/stream` (SSE) 