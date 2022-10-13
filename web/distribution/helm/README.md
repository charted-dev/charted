# charted web :: Helm Chart

This is the official Helm Chart for deploying the Web UI as a seperate deployment from the main **charted-server** deployment.

## Configuring `Caddyfile`

Since the web UI's Docker Image is basically a **Caddy** server running with the [file_server](https://caddyserver.com/docs/caddyfile/directives/file_server) directive, you can configure the Caddyfile to your liking by adding [server metrics](https://caddyserver.com/docs/caddyfile/directives/metrics) and much more.

### Example: Reverse Proxying `charted-server`

Caddy supports reverse proxying endpoints to a different server on the host. You can use the local Kubernetes cluster URL to access it (i.e, `noel-system.charted-server.svc.cluster.local`):

```caddyfile
# You can use any type here, doesn't matter. We only require that the file_server directive
# points to /app/noelware/charted/web.
localhost:2134 {
  encode gzip

  handle {
    root * /app/noelware/charted/web
    file_server
  }

  handle /api/* {
    reverse_proxy <location to charted-server>:<port>
  }
}
```

### Example: Prometheus Metrics

> **Note** - You can set `web.metrics` to be `true` (`--set web.metrics=true`) to let the Helm Chart do it for you.

```caddyfile
# You can use any type here, doesn't matter. We only require that the file_server directive
# points to /app/noelware/charted/web.
localhost:2134 {
  encode gzip
  metrics /metrics

  handle {
    root * /app/noelware/charted/web
    file_server
  }
}
```

Now, you can configure your Prometheus server to collect metrics:

```yaml
global:
  scrape_interval: 15s # default is 1 minute

scrape_configs:
  - job_name: caddy
    static_configs:
      # localhost:2019 is the Caddy admin server.
      - targets: ['localhost:2019']
```
