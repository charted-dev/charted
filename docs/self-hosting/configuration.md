---
title: Configuring charted-server
description: Reference of all the values and configuration options charted-server supports!
---

You can easily configure **charted-server** with a `config.yaml` file that can be loaded from:

- The **CHARTED_CONFIG_FILE** environment variable when running the `charted server` CLI command.
- The `--config` or `-c` command-line flags when running the `charted server` CLI command.
- In the `./config/charted.yaml` file where `./` is the directory of a **charted-server** installation.
- In `./config.yml` where `./` is the directory of a **charted-server** installation.

This documentation page is meant to give all the information you need to configure **charted-server** since in reality,
charted-server is a big project and a lot of stuff can be configured tailoured to your environment. You can create a base
configuration file with the `charted generate-config` CLI command.

The document will also discuss valid types that the configuration can take.

- `Type` is referenced as the given type that is acceptable by the configuration host, types can also be suffixed
  with `?` indicating that it can be nullable and will not do anything.
- `Default` is usually referenced when a specific value has a default value.

## Kotlin Script

**charted-server** also has experimental support for using a Kotlin Script for configurating **charted-server** in case
you don't want to read this and want to have intellisense, which we get! The CLI for charted supports validating and the server
will pick up the configuration file and initialize it.

You can get the configuration DSL from Noelware's Maven Repository, which you can either use Gradle or Maven
to manage the dependency or just download it and place it in a `lib/` directory.

Once you have the configuration DSL library, you can now create a **.charted.kts** file to configure **charted-server**! You can
see the [Config.kt class](https://github.com/charted-dev/charted/blob/main/modules/config/dsl/src/main/kotlin/Config.kt) for future reference.

You can use the `charted validate-script` CLI command to validate the script for potential errors.

## Example file

```kts
jwt_secret_key = "some secure string"
swagger_ui = false
debug = true
storage {
    filesystem("./data")
}
```

It'll be transformed into:

```yaml
jwt_secret_key: some secure string
swagger_ui: false
debug: true
storage:
  filesystem:
    directory: ./data
```

### Gradle (Kotlin DSL)

```kotlin
repositories {
    // You can also use the `/snapshots` suffix to use the snapshots
    // repository.
    maven("https://maven.noelware.org")
}

dependencies {
    implementation("org.noelware.charted:modules-config-dsl:{{ .Project.Version }}")
}
```

### Gradle (Groovy DSL)

```groovy
repositories {
  // You can also use the `/snapshots` suffix to use the snapshots
  // repository.
  maven "https://maven.noelware.org"
}

dependencies {
  implementation "org.noelware.charted:modules-config-dsl:{{ .Project.Version }}"
}
```

### Maven

```xml
<repositories>
  <repository>
    <url>https://maven.noelware.org</url>
  </repository>
</repositories>
<dependencies>
  <dependency>
    <groupId>org.noelware.charted</groupId>
    <artifactId>modules-config-dsl</artifactId>
    <version>{{ .Project.Version }}</version>
    <type>pom</type>
  </dependency>
</dependencies>
```

### JAR

```shell
# Linux/macOS
$ curl -Lo ./lib/modules-config-dsl.jar https://maven.noelware.org/org/noelware/charted/modules-config-dsl/{{ .Project.Version }}/modules-config-dsl-sources.jar
```

## Logback Configuration

**charted-server** uses the [Logback](https://logback.qos.ch) framework to provide robust logging! You can configure
how **charted-server** performs logging when running the API server by using a `logback.properties` file in the following
locations:

- Under the `$ROOT/config/logback.properties` path (where `$ROOT` is the directory of your **charted-server** instance)
- Using the `--logback-config` command line flag or the `CHARTED_LOGBACK_CONFIG_PATH` environment variable when invoking `charted server`

### charted.log.level

The default log level to use when logging to the console and when using other appenders. Sentry is the only configurable
appender that doesn't use this, it uses ERROR.

- Type: String
- Default: "info"
- Acceptable Values: info, debug, error, warn, trace

### charted.log.json

This property will transform all logs from the default, pretty printed logs into a stream of JSON
printed out. If you wish to fall back to the default, comment this out or not provide this in the final
logback.properties file.

- Type: anything
- Default:

### charted.appenders

A list of configurable appenders that can be used in your tech stack. **charted-server** supports configuring
Sentry and Logstash for logging appenders.

- Type: String
- Default: "\<empty string>"
- Acceptable Values: "sentry", "logstash"

### charted.sentry.dsn

The DSN (if not already defined in [`config.sentry_dsn`](#sentrydsn)) to use when configuring Sentry to
report any errors from the console.

- Type: DSN
- Default: \<not defined>
- Acceptable Value: [Sentry Docs](https://docs.sentry.io/product/sentry-basics/dsn-explainer)

### charted.logstash.type

The connection type if [`charted.appenders`](#chartedappenders) is configured to initialize the Logstash appender. By default,
it will use a TCP connection to connect to Logstash. This would be similarly be used if you have the [TCP input plugin](https://www.elastic.co/guide/en/logstash/current/plugins-inputs-tcp.html) initialized:

```conf
inputs {
  tcp {
    port => 4400
    codec => json
  }
}

outputs {
  elasticsearch {
    hosts => ["http://localhost:9200"]
  }
}
```

- Type: String
- Default: "tcp"
- Acceptable Values: "tcp"

### charted.logstash.endpoint

The full endpoint where we should output charted-server's logs into Logstash. This should not include the URL scheme,
as per example: `localhost:4400`

- Type: URL (without scheme)
- Default: "\<not defined>"

## Types

**charted-server**'s configuration host extends all of YAML's scalar types with custom-defined ones
to make it easier to write the configuration file.

### Secure String

**Secure String**s are used to define that this configuration value needs to load up an environment variable and use
the value from that environment variable. You can define secure string with the `${}` syntax:

```shell
# Loads up `ENV_VAR_NAME` and uses it, but returns an empty string if it was not defined
${ENV_VAR_NAME}

# Loads up `ENV_VAR_NAME` and uses it, or returns "default" if the environment variable doesn't exist.
${ENV_VAR_NAME:-default}
```

### Server Features

The configuration file can also enable features that are otherwise disabled by default. The `audit_logs` and
`webhooks` features require a ClickHouse connection to be established.

### audit_logs

This feature enables the **Audit Logs** features, which is a way of introspecting events that might have occurred in your
Helm repository or organization. This can be useful to determine what actions have been done and can be reverted.

### docker_registry

> **Warning**: Enabling this feature is experimental.

The **docker_registry** features uses a local Docker Registry instance to store your Helm Charts instead of persisting them
in `$DATA_DIR/repositories/[id]/releases/[version].tar.gz` or `$DATA_DIR/metadata/[id]/index.yaml`. It uses the OCI standard
to store Helm Charts.

This feature is useful to people who want to have their data centralized with the registry that is already running instead
of worrying about what will happen with the standardized way, which is completely ok.

### webhooks

The **webhooks** features is useful for introspecting events that might occur when configuring a repository or organization. You
can read to pretty much any event that might occur when interacting with **charted-server**.

All webhook events are stored in ClickHouse while webhook settings are stored in PostgreSQL.

## jwt_secret_key

**jwt_secret_key** is a [secure string](#secure-string) that is used to encode JWT tokens for users to authenticate with their user. It
is recommended to set this to a random seed, so it is not predicable when generating. You can use the following command to generate a random
seed:

```shell
$ openssl rand -hex 32
```

- Type: [Secure String](#secure-string)
- Default: None

## invite_only

Whether if the server is invite only or not. If this value is set to `true`, you will have to manually invite users
to your instance with

- The `charted accounts invite` subcommand
- administration portal if the [web interface](https://charts.noelware.org/docs/web-ui/latest) is available

> **Note**: [`invite_only`](#inviteonly) and [`registrations`](#registrations) are mutally exclusive. Both keys
> cannot be set to `true`, you will have to choose one or the other.

- Type: Boolean (`true`/`false`)
- Default: false

## registrations

Whether if registrations are enabled on the server or not. If this value is set to `false`, then user creation will have to be manually executed
with

- The `charted accounts create` subcommand
- administration portal if the [web interface](https://charts.noelware.org/docs/web-ui/latest) is available

> **Note**: [`invite_only`](#inviteonly) and [`registrations`](#registrations) are mutally exclusive. Both keys
> cannot be set to `true`, you will have to choose one or the other.

- Type: Boolean (`true`/`false`)
- Default: false

## telemetry

Whether if Noelware's Telemetry Services is enabled on this instance or not. This completely optional
and opt-in if you wish to send out anonymous telemetry packets to Noelware for helping out with the development
of charted-server and other products and services. Noelware's telemetry server is completely open sourced,
which you can view at https://github.com/Noelware/telemetry.

- Type: Boolean (`true`/`false`)
- Default: false

## swagger_ui

Whether if **charted-server** should host the [Swagger UI](https://swagger.io/tools/swagger-ui) that is
easily accessible with the `/_swagger` endpoint.

- Type: Boolean (`true`/`false`)
- Default: false

## sentry_dsn

Whether if Sentry is enabled on the server or not. This will make use of [Sentry](https://sentry.io) to capture realtime events
and exceptions that the server might throw while running. It will also trace anything that is traceable to Sentry that you can see
what happened.

- Type: [Secure String](#secure-string)?
- Default: <not set>

## base_url

The absolute base URL to determine where this instance lives. This value _can be changed_, but it is not recommended to be
updated since you will have to update any existing entries from all the registered Helm repository indexes.

- Type: String?
- Default: `http://{server.host}:{server.port}` or `https://{server.host}:{server.port}` is [server.ssl.enabled]() is set to `true`.

## debug

Whether if debug mode is enabled or not. This should be only suited for development purposes since it will allow you
to debug stacktrace(s) from [Kotlin Coroutines]().

- Type: Boolean (`true`/`false`)
- Default: false

## ClickHouse Configuration

### clickhouse.database

The database name to connect to. You will have to create the database with the following command,
assuming a ClickHouse server is running:

```shell
$ clickhouse-client -q "CREATE DATABASE <name>;"
```

> **Warning**: You will need to substitute the **\<name>** with any database name you want.

- Type: String
- Default: `"charted"`

### clickhouse.username

The user's name to connect to if the ClickHouse server is protected with authentication.

- Type: String?
- Default: nothing

### clickhouse.password

Similarly to [`clickhouse.username`](#clickhouseusername), this is the user's password that can access
the ClickHouse server if under authentication.

- Type: String?
- Default: nothing

### clickhouse.host

Hostname to connect to the ClickHouse server. This can be an IPv4, IPv6, or domain name, it just needs
to be resolvable by the API server.

- Type: Domain Name, IPv4 Address, or IPv6 Address
- Default: `127.0.0.1`

### clickhouse.port

The port to connect to ClickHouse. The API server uses the HTTP interface when connecting to ClickHouse,
so this should always be `8123`, unless the ClickHouse server connects different ports together.

- Type: Port Range (1024..65535)
- Default: `8123`

## Noelware Analytics

### analytics.grpc_bind_ip

The bind address when creating the Noelware Analytics gRPC Daemon that is allowed by the [Analytics API Server](https://analytics.noelware.org/docs/api-server/current)
and can collect metrics from.

- Type: IPv4 Address or IPv6 Address
- Default: `0.0.0.0`

### analytics.endpoint_auth

The authentication token that is used to authenticate this daemon to the Noelware Analytics API server.

- Type: [Secure String](#secure-string)
- Default: (unknown)

### analytics.endpoint

The Noelware Analytics API server endpoint to connect to. This will use the official instance's endpoint
if this is not set (`https://analytics.noelware.org/api`)

- Type: Domain Name, IPv4 Address, or IPv6 Address
- Default: https://analytics.noelware.org/api

### analytics.port

Port range to listen to when the gRPC daemon is listening for incoming requests.

- Type: Port Range (1024..65535)
- Default: `10234`

## PostgreSQL configuration

### database.username

Username for connecting to Postgres if authentication is enabled.

- Type: String?
- Default: nothing

### database.password

Password for connecting to Postgres if authentication is enabled.

- Type: String?
- Default: nothing

### database.database

PostgreSQL database name to connect to store data.

- Type: String
- Default: `charted`

### database.schema

The schema to use where the [database](#databasedatabase) lives in, this is `public` by default.

- Type: String
- Default: `public`

### database.host

Hostname to connect to the PostgreSQL server. This can be an IPv4, IPv6, or domain name, it just needs
to be resolvable by the API server.

- Type: Domain Name, IPv4 Address, or IPv6 Address
- Default: `127.0.0.1`

### database.port

The port to connect to PostgreSQL. By default, it will use `5432`.

- Type: Port Range (1024..65535)
- Default: `5432`
