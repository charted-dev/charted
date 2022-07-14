# ClickHouse Migrations for charted-server
This is a main CLI tool to run ClickHouse migrations on a ClickHouse cluster.

## Usage
### Docker
```shell
$ docker run --rm docker.noelware.org/charted/ch-migrations --help
Runs the ClickHouse migrations for charted-server.

Usage:
  ch:migrations [ARGS...] [flags]

Flags:
  -c, --cluster string    The cluster name to create the tables if ClickHouse is distributed.
  -d, --db string         The database name (default "charted")
  -h, --help              help for ch:migrations
      --host string       The host to connect to your ClickHouse server (default "127.0.0.1")
  -p, --password string   The password to connect to your ClickHouse server if authentication is enabled [env: CLICKHOUSE_AUTH_PASSWORD]
      --port int          The port to connect to your ClickHouse server (default 9000)
  -u, --username string   The username to connect to your ClickHouse server if authentication is enabled [env: CLICKHOUSE_AUTH_USERNAME]
```

### Binary
```shell
$ cd tools/migrations
$ go build -o ./bin/ch-migrations # append `.exe` if on Windows
$ ./bin/ch-migrations[.exe]
```
