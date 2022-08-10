# Cassandra Migrations for charted-server
This is a main CLI tool to run ClickHouse migrations on a ClickHouse cluster.

## Usage
### Docker
```shell
$ docker run --rm docker.noelware.org/charted/migrations:latest --help
Runs the Cassandra migrations for charted-server

Usage:
  migrations [ARGS...] [flags]

Flags:
  -h, --help              help for migrations
      --hosts strings     The cluster hosts to connect to (default [127.0.0.1])
  -k, --keyspace string   The keyspace to run migrations on. (default "charted")
  -p, --password string   The password to connect to your Cassandra cluster if authentication is enabled. [env: CASSANDRA_PASSWORD]
      --port int          The port to connect to your Cassandra cluster (default 9042)
      --protocol int      Cassandra protocol to use.
      --table string      The migrations table to use (default "migrations")
  -t, --timeout string    The dial timeout to use when connecting. (default "1m")
  -u, --username string   The username to connect to your Cassandra cluster if authentication is enabled. [env: CASSANDRA_USERNAME]
      --version int32     The migration version to run.

```

### Binary
```shell
$ cd tools/migrations
$ go build -o ./bin/migrations # append `.exe` if on Windows
$ ./bin/migrations[.exe]
```
