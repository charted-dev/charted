# Unreleased

## ✅ Additions

-   Helm Plugin for easily integrating charted-server into your CI/CD pipeline ([@auguwu](https://github.com/auguwu), [@spotlightishere](https://github.com/spotlightishere))
-   [cli] **charted accounts create** and **charted accounts list** subcommands ([@auguwu](https://github.com/auguwu))
-   [tools] **kt-to-rust** tool, very bare-bones and not meant to be outside charted. ([@auguwu](https://github.com/auguwu))

## :bug: Infrastructure

-   [deps] Upgrade `alpine` Docker tag to v3.17 ([@renovate-bot](https://github.com/apps/renovate), #)
-   [deps] Upgrade `elastic-apm` to v1.35.0 ([@renovate-bot](https://github.com/apps/renovate), #)
-   [deps] Kotlin's Explicit API now enabled on configuration DSL and common modules ([@auguwu](https://github.com/auguwu))
-   [deps] Upgrade Spring Security Crypto to v6.0.1 ([@renovate-bot](https://github.com/apps/renovate), #)
-   [deps] Upgrade Gradle Enterprise plugin to v3.12.1 ([@renovate-bot](https://github.com/apps/renovate), #)
-   [deps] Upgrade `prometheus` Helm release to `~19.2.0` ([@renovate-bot](https://github.com/apps/renovate), #)
-   [deps] Upgrade `serde` Rust create to `1.0.152` ([@renovate-bot](https://github.com/apps/renovate), #)
-   [deps] Upgrade `com.github.ajalt.clikt:clikt` to `3.5.1` ([@renovate-bot](https://github.com/apps/renovate), #)
-   [deps] Upgrade `org.mockito:mockito-core` to `4.11.0` ([@renovate-bot](https://github.com/apps/renovate), #)
-   [deps] Upgrade `io.insert-koin:koin-core` to `3.3.2` ([@renovate-bot](https://github.com/apps/renovate), #)
-   [deps] Upgrade `once_cell` Rust crate to `1.17.0` ([@renovate-bot](https://github.com/apps/renovate), #)
-   [deps] Migrate to [charted-dev/snowflake](https://github.com/charted-dev/snowflake) for more accurate snowflake generation ([@auguwu](https://github.com/auguwu))
-   [ci] Allow PRs to run in `.github/` folders ([@auguwu](https://github.com/auguwu))
-   [ides/fleet] Add Fleet settings to allow development with JetBrains Fleet

## :whale: Docker Registry

-   Authorization is now ready, but hasn't been tested yet ([@auguwu](https://github.com/auguwu))

## :package: Server

-   Fix multiple server tests ([@auguwu](https://github.com/auguwu))
-   Fix OpenAPI runtime errors when parsing dates ([@auguwu](https://github.com/auguwu))
-   Added `/features` endpoint ([@auguwu](https://github.com/auguwu))
-   Allow embedding Swagger UI as an endpoint on the server (configurable via `config.swagger_ui`) ([@auguwu](https://github.com/auguwu))
-   Organizations feature is now implemented ([@auguwu](https://github.com/auguwu))
-   The api key scopes' bit values have been changed ([@auguwu](https://github.com/auguwu))
-   OpenAPI definitions are now up-to-date ([@auguwu](https://github.com/auguwu))
-   CDN routing now support caching headers ([@auguwu](https://github.com/auguwu))

## API Key Scope Updates

### Users

| Name                  | Old Value   | New Value   |
| --------------------- | ----------- | ----------- |
| `user:access`         | 1 << 0 (1)  | 1 << 0 (1)  |
| `user:update`         | 1 << 1 (2)  | 1 << 1 (2)  |
| `user:delete`         | 1 << 2 (4)  | 1 << 2 (4)  |
| `user:connections`    | 1 << 4 (16) | 1 << 3 (8)  |
| `users:notifications` | 1 << 3 (8)  | 1 << 4 (16) |
| `user:avatar:update`  | 1 << 5 (32) | 1 << 5 (32) |
| `user:sessions:list`  | 1 << 6 (64) | 1 << 6 (64) |

### Repositories

-   `repo:view` is now removed in this release, use the `repo:access` scope instead
-   `repo:invites` and `repo:member:invites` is now removed, use `repo:members:invites:access` instead
-   `repo:member:*` scopes are now prefixed `repo:members:`

| Name                          | Old Value                | New Value          |
| ----------------------------- | ------------------------ | ------------------ |
| `repo:access`                 | 1 << 7 (128)             | 1 << 7 (128)       |
| `repo:create`                 | 1 << 8 (256)             | 1 << 8 (128)       |
| `repo:delete`                 | 1 << 9 (512)             | 1 << 9 (512)       |
| `repo:update`                 | 1 << 10 (1024)           | 1 << 10 (1024)     |
| `repo:write`                  | 1 << 12 (4096)           | 1 << 11 (2048)     |
| `repo:releases:create`        | 1 << 14 (16384)          | 1 << 12 (4096)     |
| `repo:releases:delete`        | 1 << 15 (32768)          | 1 << 13 (8192)     |
| `repo:releases:update`        | 1 << 16 (65535)          | 1 << 14 (16384)    |
| `repo:members:list`           | (nothing)                | 1 << 15 (32768)    |
| `repo:members:update`         | 1 << 19 (524288)         | 1 << 16 (65535)    |
| `repo:members:kick`           | 1 << 18 (262144)         | 1 << 17 (131072)   |
| `repo:members:invites:access` | 1 << 13 (8192)           | 1 << 18 (262144)   |
| `repo:members:invites:create` | 1 << 40 (1099511627776)  | 1 << 19 (524288)   |
| `repo:members:invites:delete` | 1 << 41 (2199023255552)  | 1 << 20 (1048576)  |
| `repo:webhooks:list`          | 1 << 20 (1048576)        | 1 << 21 (2097152)  |
| `repo:webhooks:create`        | 1 << 21 (2097152)        | 1 << 22 (4194304)  |
| `repo:webhooks:delete`        | 1 << 23 (8388608)        | 1 << 23 (8388608)  |
| `repo:webhooks:update`        | 1 << 22 (4194304)        | 1 << 24 (16777216) |
| `repo:webhooks:events:access` | 1 << 44 (17592186044416) | 1 << 25 (33554432) |
| `repo:webhooks:events:delete` | 1 << 45 (35184372088832) | 1 << 26 (67108864) |

### API Keys

| Name                 | Old Value              | New Value            |
| -------------------- | ---------------------- | -------------------- |
| `apikeys:view`       | 1 << 36 (68719476736)  | 1 << 27 (134217728)  |
| `apikeys:create`     | 1 << 37 (137438953472) | 1 << 28 (268435456)  |
| `apikeys:delete`     | 1 << 38 (274877906944) | 1 << 29 (536870912)  |
| `apikeys:update`     | (nothing)              | 1 << 30 (1073741824) |
| `apikeys:edit:perms` | 1 << 39 (549755813888) | 1 << 31 (2147483648) |

### Organizations

-   `org:invites` has been removed, please use `org:members:invites`
-   `org:member:*` scopes have use the `org:members:` prefix

| Name                         | Old Value | New Value                |
| ---------------------------- | --------- | ------------------------ |
| `org:access`                 |           | 1 << 32 (4294967296)     |
| `org:create`                 |           | 1 << 33 (8589934592)     |
| `org:update`                 |           | 1 << 34 (17179869184)    |
| `org:delete`                 |           | 1 << 35 (34359738368)    |
| `org:members:invites`        |           | 1 << 36 (68719476736)    |
| `org:members:list`           |           | 1 << 37 (137438953472)   |
| `org:members:kick`           |           | 1 << 38 (274877906944)   |
| `org:members:update`         |           | 1 << 39 (549755813888)   |
| `org:webhooks:list`          |           | 1 << 40 (1099511627776)  |
| `org:webhooks:create`        |           | 1 << 41 (2199023255552)  |
| `org:webhooks:update`        |           | 1 << 42 (4398046511104)  |
| `org:webhooks:delete`        |           | 1 << 43 (8796093022208)  |
| `org:webhooks:events:list`   |           | 1 << 44 (17592186044416) |
| `org:webhooks:events:delete` |           | 1 << 45 (35184372088832) |

## :octocat: GitHub

-   Add support for GitHub Codespaces for remote development ([@auguwu](https://github.com/auguwu))

# v0.3.2-nightly

## Fixes

-   [cli] Make sure `--config`/`--logback-config` can only read files and not error when the file is not writable ([@auguwu](https://github.com/auguwu))
-   [chart] Default to the chart's AppVersion if `image.tag` is not defined ([@auguwu](https://github.com/auguwu))

**Full Changelog**: https://github.com/charted-dev/charted/compare/0.3.1-nightly...0.3.2-nightly

# v0.3.1-nightly

## Fixes

-   Allow `--config`/`--logback-config` to be readonly when loading ([@auguwu](https://github.com/auguwu))

**Full Changelog**: https://github.com/charted-dev/charted/compare/0.3.0-nightly...0.3.1-nightly

# v0.3.0-nightly

This is the most anticipated release of this project. This contains a full refactored version of the old project which includes:

-   a new CLI to do management stuff with, in the future, you will be able to create accounts/repos/orgs and such through the CLI.
-   new and fresh codebase that is easier to navigate.
-   and more!!!

## Additions

-   Partial support for **Noelware Analytics** ([@auguwu](https://github.com/auguwu), [@IceeMC](https://github.com/IceeMC))
-   The major components are fully testable (mainly the HTTP server, but most tests aren't finished) ([@auguwu](https://github.com/auguwu))
-   OpenAPI definitions are more clear now and will be available at `charts.noelware.org/api/openapi?format=json`. ([@auguwu](https://github.com/auguwu))
-   Elastic APM is fully supported for tracing, not all methods are traceable yet. ([@auguwu](https://github.com/auguwu))
-   Repositories can now list their `Chart.yaml`, `index.yaml` and template files from their tarball and not by releasing. ([@auguwu](https://github.com/auguwu))
-   Patching repository metadata is now added ([@auguwu](https://github.com/auguwu))
-   Server now has preconditions, so we don't have to repeat most preconditions in Repositories and Organizations API ([@auguwu](https://github.com/auguwu))
-   Repository members are partially added ([@auguwu](https://github.com/auguwu))
-   **charted** now comes with a fully working PowerShell script, useful for Windows users. ([@auguwu](https://github.com/auguwu))
-   All artifacts will be pushed to `artifacts.noelware.cloud` as well on GitHub releases. ([@auguwu](https://github.com/auguwu))
-   Elasticsearch SSL connections are now fully supported. ([@IceeMC](https://github.com/IceeMC))

# Updates/Fixes

-   A bunch of dependency updates by @renovate-bot

# Removed

-   Cassandra has been swapped with ClickHouse due to Noelware's infrastructure conflicts. ([@auguwu](https://github.com/auguwu))

**Full Changelog**: https://github.com/charted-dev/charted/compare/v0.2.0-nightly.1...v0.3-nightly

# v0.2.0-nightly.1

This release fixes some issues with our Release (Nightly) pipeline for the Docker images.

# v0.2.0-nightly

This release comes with a few changes and dependency upgrades. v0.3-nightly should contain actual features like repository and organization member support and the web UI.

## Additions

-   OpenAPI support is available on the server, which you can access from `<server url>/openapi.json` or `<server url>/openapi.yaml`. Documentation for **charted-server** will be relied on the official instance for API routing and official SDKs for **charted-server** will exist around October to November 2022.
-   Elastic APM tracing is supported, but it is very limiting. We do plan to support anything that supports the OpenTelemetry API.
-   Images for the Cassandra database should be available from the `ghcr.io/charted-dev/charted/migrations` image, not sure!
-   Generating instance UUIDs for Noelware Analytics can be disabled with the `CHARTED_NO_ANALYTICS` environment variable.
-   Docker image will use JDK 19 instead of JDK 18 (thanks #122 by @renovate-bot)

## Dependency Updates

-   Upgrade **io.insert-koin:koin-core** from v3.2.0 to v3.2.1 by @renovate-bot in #89
-   Upgrade Helm release **postgresql** from ~11.8.0 to ~11.9.0 by @renovate-bot in #59
-   Upgrade Helm release **cassandra** from to ~9.5.0 by @renovate-bot in #90
-   Upgrade **org.slf4j** dependencies (`org.slf4j:slf4j-api`, `org.slf4j:slf4j-simple`) from 2.0.0 to 2.0.1 by @renovate-bot in #94, #104, #125, #106
-   Upgrade **com.google.protobuf:protoc** dependency from 3.21.5 to 3.21.7 by @renovate-bot in #64

(this is not all the dependency updates, check the changelog below to go through all of them)

**Full Changelog**: https://github.com/charted-dev/charted/compare/1.1.1-nightly...1.2.0-nightly

# v1.1.1-nightly

The Docker image now makes the **charted-server** binary executable, which is the only change in this release.

# v1.1.0-nightly

Many fixes with Logback and the Docker image not properly running... sorry for taking a while, Logback was not wanting to work until me being
sleepy at 8am trying to fix the problem, which I did, so please be proud of me.

# v1.0.0-nightly

This is the first alpha release of **charted-server**. At the moment, we only distribute the tarballs and ZIP files in this release,
and Docker images that are available on the [GitHub Container Registry](https://github.com/charted-dev/charted/pkgs/container/charted).
