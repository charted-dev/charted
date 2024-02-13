# `charted-server` changelog

## v0.1.0-beta [unreleased]

:tada: **_FIRST BETA RELEASE!!!_** :tada:

This is the first ever beta release of `charted-server` where we will keep working to make charted a production-ready product. You are invited to run charted on-premise and give us feedback via [GitHub Issues](https://github.com/charted-dev/charted), Noelware will personally dogfeed all charts into.

We restructured the codebase to be in Rust as it is easier to maintain and to keep a collection of multiple projects together as well.

Sorry it took a year to do, this will never happen again.

~ **Noel Towa**

-   Rework all code into Rust as the Kotlin codebase became too complex to work on and Rust makes it easier (@auguwu, @spotlightishere)
-   **ClickHouse** is not required anymore for any feature. **ClickHouse** will become a critical component for some features that haven't been implemented already. (@auguwu)
-   OpenAPI document is now more stable and is the recommended way to generate the APIs necessary (@auguwu)
-   Added `migrations` subcommand with `migrations list` and `migrations run` sub-subcommands (@auguwu)
-   Java is no longer necessary, `charted` is now a flat binary that can be installed from Noelware's Artifacts Registry (@auguwu)
-   `accounts` subcommand is no longer available (@auguwu)
-   LDAP is now supported when authenticating users, while it is basic and experimental (@auguwu)
-   `logback.properties` is no longer available, please use the `logging` block in your `config.yml` (@auguwu)

## v0.4.0-unstable.3

### :heavy_plus_sign: Additions

-   Added **Elastic APM** support for tracing HTTP requests (@auguwu)
-   Added user information to be attached to Sentry if Sentry is enabled (@auguwu)
-   Sentry will now collect thread and stacktrace information for better debugging exceptions (@auguwu)
-   Cache workers now have a dedicated configuration object under `config.server.caching`, at the moment you can only set the driver to use (`InMemory` or `Redis`) and it does nothing at the moment. (@auguwu)

### :x: Removed

-   Noelware Analytics is no longer supported due to its removal internally by the Noelware team (@auguwu)

### :bug: Fixes

-   Fix serialization for Elastic APM tracing (@auguwu)
-   Disable any Logback-available properties when using `-Dlogback.[key]=[value]` in any command that isn't `charted server`. (@auguwu)

### :sparkles: Dependency Updates

-   Update [Eclipse Temurin](https://hub.docker.com/_/eclipse-temurin) Docker images to **17.0.7_7** (@renovate-bot)
-   Update **Sentry** dependencies to v6.19.1 (@renovate-bot)
-   Update **Elasticsearch Rest Client** dependencies to v8.7.1 (@renovate-bot)
-   Update **com.gradle.enterprise** Gradle plugin to v3.13.2 (@renovate-bot)
-   Update **Elastic APM** dependencies to v1.38.0 (@renovate-bot)
-   Update **com.google.protobuf:protobuf-kotlin** to v3.23.0 (@renovate-bot)
-   Update **com.google.protobuf:protoc** to v3.23.1 (@renovate-bot)

[there is more but I'll probably automate this part]

**Full Changelog**: https://github.com/charted-dev/charted/compare/0.4.0-unstable.2...0.4.0-unstable.3

## v0.4.0-unstable.2

### :heavy_plus_sign: Additions

-   [cli] Added new subcommand `accounts` (@auguwu)
-   [metrics] Added key-sets for Elasticsearch and Redis, which means you can filter out ES and Redis Prometheus metrics (@auguwu)
-   [server] Tracing is lightly supported, at the moment, only a Sentry tracer is implemented, we plan to add Elastic APM and OpenTelemetry (@auguwu)
-   [server] New endpoints:
    -   `GET /organizations/{idOrName}/repositories`
    -   `PUT /organizations/{idOrName}/repositories`
    -   `GET /organizations/{idOrName}/repositories/{repoIdOrName}`
    -   `PATCH /organizations/{idOrName}/repositories/{id}`
    -   `GET /repositories/{id}/releases`
    -   `PATCH /repositories/{id}/releases/{version}`

### :bug: Fixes

-   Let any [ConfigAwareCliktCommand](https://github.com/charted-dev/charted/blob/0.4.0-unstable.2/cli/src/main/kotlin/commands/abstractions/ConfigAwareCliktCommand.kt) load Kotlin Script configuration files (@auguwu)
-   Logback will not run on any CLI commands that aren't the `server` subcommand (@auguwu)

### :wrench: Development Updates

-   Decommission `buildSrc` with `build-logic` (read more about about Gradle [composite builds](https://proandroiddev.com/stop-using-gradle-buildsrc-use-composite-builds-instead-3c38ac7a2ab3) to know our decision on why) (@auguwu)

### :sparkles: Dependency Updates

-   Update dependency **com.github.ben-manes.caffeine:caffeine** to v3.1.6 (@renovate-bot, #651)
-   Update dependency **com.github.ajalt.mordant:mordant** to v2.0.0-beta13 (@renovate-bot, #652)
-   Update dependency **org.bouncycastle:bcpkix-jdk15to18** to v1.73 (@renovate-bot, #653)
-   Update **Gradle** to v8.1.1 (@renovate-bot, #654, #669)
-   Update dependency **com.google.protobuf:protobuf-kotlin** to v3.22.3 (@renovate-bot, #656)
-   Update dependency **com.google.protobuf:protoc** to v3.22.3 (@renovate-bot, #657)
-   Update Gradle plugin **com.gradle.enterprise** to v3.13 (@renovate-bot, #655)
-   Update dependency **io.grpc:protoc-gen-grpc-java** to v1.54.1 (@renovate-bot, #658)
-   Update gRPC libraries to v1.54.1 (@renovate-bot, #659)
-   Update **bitnami/redis** Docker images to v7.0.11 (@renovate-bot, #660)
-   Update **org.springframework.security:spring-security-crypto** to v6.0.3 (@renovate-bot, #661)
-   Update OpenTelemetry libraries to v1.25.0 (@renovate-bot, #663, #664)
-   Update Ktor libraries to v2.3.0 (@renovate-bot, #665)
-   Update **io.lettuce:lettuce-core** to v6.2.4.RELEASE (@renovate-bot, #662)
-   Update Terraform provider **hashicorp/kubernetes** to v2.20.0 (@renovate-bot, #668)
-   Update Logback libraries to v1.4.7 (@renovate-bot, #667)
-   Update Helm dependency **redis** ([bitnami/redis](https://github.com/bitnami/charts/tree/main/bitnami/redis)) to v17.10.0 (@renovate-bot, #676)
-   Update Gradle plugin **org.gradle.toolchains:foojay-resolver** and **org.gradle.toolchains.foojay-resolver-convention** to v0.5.0 (@renovate-bot, #673, #674)
-   Update Jackson libraries to v2.15.0 (@renovate-bot, #670)
-   Update Kotlin toolchain to v1.8.21 (@renovate-bot, #675)
-   Update Helm dependency **postgresql** ([bitnami/redis](https://github.com/bitnami/charts/tree/main/bitnami/postgresql)) to v12.4.0 (@renovate-bot, #677)
-   Update JUnit5 libraries to v5.9.3 (@renovate-bot, #679)
-   Update GitHub action **JetBrains/qodana-action** to 2023 (@renovate-bot, #671)
-   Update Gradle plugin **com.netflix.nebula:gradle-ospackage-plugin** to v11.3.0 (@renovate-bot, #678)
-   Update Gradle plugin **com.google.protobuf:protobuf-gradle-plugin** to v0.9.3 (@renovate-bot, #680)

**Full Changelog**: https://github.com/charted-dev/charted/compare/0.4.0-unstable.1...0.4.0-unstable.2

## v0.4.0-unstable.1

### :bug: Fixes

-   Fixes Sentry appender from overwriting the Console/Logstash appenders (if configured)

## v0.4.0-unstable

unstable release of v0.4-nightly (for now), will be deleted once (and any other artifact) it is ready to be released.

this is the 2nd iteration of v0.4-unstable:

-   adds in `CHARTED_JAVA_OPTS` to CLI launcher

## v0.3.2-nightly

### :bug: Fixes

-   [cli] Make sure `--config`/`--logback-config` can only read files and not error when the file is not writable (@auguwu)
-   [chart] Default to the chart's AppVersion if `image.tag` is not defined (@auguwu)

**Full Changelog**: https://github.com/charted-dev/charted/compare/0.3.1-nightly...0.3.2-nightly

## v0.3.1-nightly

### :bug: Fixes

-   Allow `--config`/`--logback-config` to be readonly when loading (@auguwu)

**Full Changelog**: https://github.com/charted-dev/charted/compare/0.3.0-nightly...0.3.1-nightly

## v0.3.0-nightly

This is the most anticipated release of this project. This contains a full refractored version of the old project which includes:

-   a new CLI to do management stuff with, in the future, you will be able to create accounts/repos/orgs and such through the CLI.
-   new and fresh codebase that is easier to navigate.
-   and more!!!

This release will also be the first release Noelware launches into production at `charts.noelware.org/api`. The frontend design will not be finished until Q1/Q2 2023 as Noelware has a lot of work to do in that regard.

### :heavy_plus_sign: Additions

-   Partial support for **Noelware Analytics** (@auguwu, @IceeMC)
-   The major components are fully testable (mainly the HTTP server, but most tests aren't finished) (@auguwu)
-   OpenAPI definitions are more clear now and will be available at `charts.noelware.org/api/openapi?format=json`. (@auguwu)
-   Elastic APM is fully supported for tracing, not all methods are traceable yet. (@auguwu)
-   Repositories can now list their `Chart.yaml`, `index.yaml` and template files from their tarball and not by releasing. (@auguwu)
-   Patching repository metadata is now added (@auguwu)
-   Server now has preconditions, so we don't have to repeat most preconditions in Repositories and Organizations API (@auguwu)
-   Repository members are partially added (@auguwu)
-   **charted** now comes with a fully working PowerShell script, useful for Windows users. (@auguwu)
-   All artifacts will be pushed to `artifacts.noelware.cloud` as well on GitHub releases. (@auguwu)
-   Elasticsearch SSL connections are now fully supported. (@IceeMC)

### :bug: Updates/Fixes

-   A whole bunch of dependency updates by @renovate-bot

### :x: Removed

-   Cassandra has been swapped with ClickHouse due to Noelware's infrastructure conflicts. (@auguwu)

**Full Changelog**: https://github.com/charted-dev/charted/compare/v0.2.0-nightly.1...v0.3-nightly

## v0.2.0-nightly.1

This release fixes some issues with our Release (Nightly) pipeline for the Docker images.

## v0.2.0-nightly

This release comes with a few changes and dependency upgrades. v0.3.0-nightly should contain actual features like repository and organization member support and the web UI.

# Additions

-   OpenAPI support is available on the server, which you can access from `<server url>/openapi.json` or `<server url>/openapi.yaml`. Documentation for **charted-server** will be relied on the official instance for API endpoints and official SDKs for **charted-server** will exist around October to November 2022.
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

(this is not all of the dependency updates, check the changelog below to go through all of them)

**Full Changelog**: https://github.com/charted-dev/charted/compare/1.1.1-nightly...1.2.0-nightly

## v1.1.1-nightly (originally `v1.1.1-nightly`)

The Docker image now makes the **charted-server** binary executable, which is the only change in this release.

## v1.1.1-nightly (originally `v1.1.1-nightly`)

Many fixes with Logback and the Docker image not properly running... sorry for taking a while, Logback was not wanting to work until me being sleepy at 8am trying to fix the problem, which I did, so please be proud of me.

## v0.1.0-nightly (originally `v1.0.0-nightly`)

This is the first alpha release of **charted-server**. At the moment, we only distribute the tarballs and ZIP files in this release, and Docker images that are available on the [GitHub Container Registry](https://github.com/charted-dev/charted/pkgs/container/charted).
