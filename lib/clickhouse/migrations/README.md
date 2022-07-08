# charted-server - ClickHouse Migrations
This is the migration tool to keep migrations up to date with the ClickHouse database, if you ever need it for anything.

## How to use
### Docker Image (via Docker Hub)
```shell
docker run --rm --name charted-ch-migrations -v ./lib/clickhouse/migrations/migrations:/migrations \
  --env CLICKHOUSE_HOST=localhost \
  --env CLICKHOUSE_PORT=9000 \
  --env CLICKHOUSE_USERNAME={username} \
  --env CLICKHOUSE_PASSWORD={password} \
  --env CLICKHOUSE_DATABASE=charted \
  charted/ch-migrations:latest
```

### Locally
You are required to have **Go 1.18** before running the script.

```shell
# in root directory where charted-server lives
$ make run.migrations
```
