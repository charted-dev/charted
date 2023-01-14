---
title: Configuring charted-server
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
